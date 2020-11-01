use rusoto_core::{HttpClient, Region};
use rusoto_credential::{StaticProvider,ProvideAwsCredentials};
use rusoto_ec2::{
    AttachVolumeRequest,
    CreateVolumeRequest,
    Ec2,
    Ec2Client,
    DescribeVolumesRequest,
    ModifyInstanceAttributeRequest,
    Tag,
    TagSpecification,
    InstanceBlockDeviceMappingSpecification,
    EbsInstanceBlockDeviceSpecification,
    Volume
};

use std::default::Default;
use std::{thread, time};
use sysinfo::{DiskExt, SystemExt};
use std::future::Future;

use crate::helpers::setup_aws_credentials::fetch_credentials;
use crate::helpers::setup_cli::CliOptions;
use crate::helpers::setup_tokio::create_runtime;


fn calculate_next_wolume_size(last_size:int64) -> int64 {
    // Strat 10x because of the limited amount of EBS volumes could be attached
    last_size * 10
}

fn generate_next_device_name(current_device_name: String) -> Result<String, String> {
    static ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z',
    ];
    
    let lookup_char = current_device_name.chars().last().unwrap();
    
    if lookup_char == 'z'{
        return Err("No device names left".to_string())
    }
    
    let mut new_device_name = current_device_name.clone();
    new_device_name.truncate(current_device_name.len()-1);// + new_char;
    new_device_name = new_device_name + &ASCII_LOWER[ASCII_LOWER.iter().position(|&r| r == lookup_char).unwrap() + 1].to_string();

    Ok(new_device_name)
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
  

    #[test]
    fn test_nice() {
        assert_eq!(Ok("/dev/sdc".to_string()),generate_next_device_name("/dev/sdb".to_string()));
    }

    #[test]
    fn test_error() {
        assert_eq!(Err("No device names left".to_string()),generate_next_device_name("/dev/sdz".to_string()));

    }

}

fn get_used_mount_point_memory_percent() {
    let sys = System::new();

    match sys.mounts() {
        Ok(mounts) => {
            println!("\nMounts:");
            for mount in mounts.iter() {
                if mount.fs_mounted_on == "/" {
                    println!("mount point: > {} < (available {} of {}) Extra space needed: {}", 
                        mount.fs_mounted_on, mount.avail, mount.total, saturating_sub_bytes(mount.total, mount.avail) < mount.avail);
                }
            }
        }
        Err(x) => println!("\nMounts: error: {}", x)
    }

}

fn extract_volume_state_info(volumes:Vec<Volume>,desired_state:String) -> bool {
    desired_state == match desired_state.as_ref() {
        "available" => volumes[0].state.as_ref().unwrap().to_string(),
        "attached" => volumes[0].attachments.as_ref().unwrap()[0].state.as_ref().unwrap().to_string(),
        _ => unimplemented!(),
    }
}

async fn volume_state_waiter(client:&Ec2Client, volume_id:String, desired_state:String) -> Result<String, String>{

    let small_sleep = time::Duration::from_millis(200);

    
    let describe_volume_request = DescribeVolumesRequest {
        volume_ids: Some([volume_id.to_string()].to_vec()),
        ..Default::default()
    };

    let mut count = 0u32;

    println!("Wait until volume: >{}< is in state: >{}<", volume_id, desired_state);

    // Infinite loop
    loop {
        count += 1;
        let mut _describe_volume_request = describe_volume_request.clone();
        match client.describe_volumes(_describe_volume_request).await {
            Ok(ref output) => match &output.volumes {
                Some(volumes) => {
                    if extract_volume_state_info(volumes.to_vec(), desired_state.to_string()){
                        return Ok(format!("Desired state {} Polled", desired_state))
                    }
                }
                None => println!("no volumes to describe"),
            },
            Err(error) => {
                println!("Error: {:?}", error);
            }
    
        }


        if count == 600 { // Timeout after 600 * 200ms = 2 mins
            println!("OK, that's enough");

            // Exit this loop
            return Err("Timeout".to_string())
        }
        thread::sleep(small_sleep);
    }

    
    
}

async fn create_and_attach_volume() -> String {
    let device_name = "/dev/sdh"; //Todo get it from cli parameters
    let instance_id = "i-0cb68a3d1a173fe0c"; //TODO get it from underlying EC2
    let availability_zone = "ap-southeast-2b"; //TODO get it from underlying EC2
    let volume_type = "gp2"; //Todo get it from cli parameters
    let size = 8; //Todo get it from cli + algs
    let (creds, creds_msg) = fetch_credentials().await;
    let cred_provider = StaticProvider::new(
        creds.aws_access_key_id().to_string(),
        creds.aws_secret_access_key().to_string(),
        creds.token().clone(),
        None,
    );
    let client = Ec2Client::new_with(
        HttpClient::new().unwrap(),
        cred_provider,
        Region::ApSoutheast2, //TODO get it from underlying EC2
    );

    let mut volume_id_holder = String::new();
    let create_volume_request = CreateVolumeRequest {
        availability_zone: availability_zone.to_string(),
        size: Some(size),
        volume_type: Some(volume_type.to_string()), 
        tag_specifications: Some(vec![TagSpecification {
            resource_type: Some("volume".to_string()),
            tags: Some(vec![Tag {
                key: Some("createdBy".to_string()),
                value: Some("pym-disk".to_string()),
            }]),
        }]),
        ..Default::default() 
    };
    match client.create_volume(create_volume_request).await {
        Ok(ref output) => match &output.volume_id {
            Some(volume_id) => {
                volume_id_holder = volume_id.to_string();
            }
            None => println!("no volumes created"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }

    }

    let attach_volume_request = AttachVolumeRequest {
        device: device_name.to_string(),
        instance_id: instance_id.to_string(), 
        volume_id: volume_id_holder.to_string(),
        ..Default::default()
    };
    volume_state_waiter(&client, volume_id_holder.to_string(), "available".to_string()).await;
    match client.attach_volume(attach_volume_request).await {
        Ok(ref output) => match &output.volume_id {
            Some(volume_id) => {
                println!("{}", volume_id);
                //println!("{:?}", output);
            }
            None => println!("no volumes attached"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
    volume_state_waiter(&client, volume_id_holder.to_string(), "attached".to_string()).await;
    let modify_instance_attribute_request = ModifyInstanceAttributeRequest {
        block_device_mappings: Some(
            [InstanceBlockDeviceMappingSpecification{
                device_name: Some(device_name.to_string()),
                ebs: Some(EbsInstanceBlockDeviceSpecification{
                    delete_on_termination: Some(true), // TODO this currently set the delete on termination flag to true
                    ..Default::default()
                }),
                ..Default::default()
            }].to_vec()
        ),
        instance_id: instance_id.to_string(), 
        ..Default::default()
    };

    match client.modify_instance_attribute(modify_instance_attribute_request).await{
        Ok(_) =>{
            println!("Ok")
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    };

    volume_id_holder

}

fn make_volumes_available() {
    //check size
    create_and_attach_volume()
    //calculate next size
}

fn setup_mount_point(cli_args,_rt,cred_provider) {
    make_volumes_available()
    // vgcreate vg /dev/sdb /dev/sdc
    // lvcreate -n stripe -l +100%FREE -i 2 vg
    // mkdir /stratch
    // mkfs.ext4 /dev/vg/stripe
    // mount /dev/vg/stripe /stratch
}


fn extend_mount_point() {
    make_volumes_available()
    // vgextend vg /dev/sdd /dev/sde
    // lvextend vg/stripe -l +100%FREE --resizefs
}

pub fn pym_disk_handler(cli_args: CliOptions) {
    // we use tokio runtime for various async activity
    let (mut _rt, _rt_msg) = create_runtime();

    // a single set of credentials which we are assuming will last throughout the whole operation
    let (creds, _creds_msg) = _rt.block_on(fetch_credentials());

    let cred_provider = StaticProvider::new(
        creds.aws_access_key_id().to_string(),
        creds.aws_secret_access_key().to_string(),
        creds.token().clone(),
        None,
    );

    setup_mount_point(cli_args,_rt,cred_provider);
    if cli_args.oneshot {
        // TODO: Coloring, loading, other fancy stuff
        println!(">>> Pym Disk is in One Shot Mode! <<<");
    } else {
        println!(">>> Pym Disk is in Watch Dog Mode! <<<");

    };
}
