use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_ec2::{
    AttachVolumeRequest,
    CreateVolumeRequest,
    Ec2,
    Ec2Client,
    //RunInstancesRequest,
    Tag,
    TagSpecification,
};
// use rusoto_sts::StsAssumeRoleSessionCredentialsProvider;
// use rusoto_sts::StsClient;
use std::default::Default;
// use std::io::{stdin, stdout, Write};
use std::{thread, time};
use sysinfo::{DiskExt, SystemExt};
use uuid::Uuid;

use crate::helpers::setup_aws_credentials::fetch_credentials;
use crate::helpers::setup_cli::Cli;
use crate::helpers::setup_tokio::create_runtime;

fn calculate_next_disk_size() {}

fn generate_next_disk_name() {}

fn volume_availability_waiter() {}

fn get_used_mount_point_memory_percent() {
    let mut system = sysinfo::System::new_all();

    // First we update all information of our system struct.
    system.refresh_all();

    // And then all disks' information:
    for disk in system.get_disks() {
        println!("{:?}", disk);
        println!("{}", disk.get_available_space());
        println!("{}", disk.get_total_space());
    }

    // And finally the RAM and SWAP information:
    println!("total memory: {} KB", system.get_total_memory());
    println!("used memory : {} KB", system.get_used_memory());
    println!("total swap  : {} KB", system.get_total_swap());
    println!("used swap   : {} KB", system.get_used_swap());
}

fn create_attach_and_init_volume() {
    let client = Ec2Client::new_with(
        HttpClient::new().unwrap(),
        cred_provider,
        Region::ApSoutheast2, //TODO get it from underlying EC2
    );

    let mut volume_id_holder = String::new();
    let create_volume_rqst = CreateVolumeRequest {
        availability_zone: "ap-southeast-2b".to_string(), //TODO get it from underlying EC2
        size: Some(8), // increase with every new addition, Fibonacci?
        volume_type: Some("gp2".to_string()), //Todo get it from cli parameters
        tag_specifications: Some(vec![TagSpecification {
            resource_type: Some("volume".to_string()),
            tags: Some(vec![Tag {
                key: Some("createdBy".to_string()),
                value: Some("pym-disk".to_string()),
            }]),
        }]),
        ..Default::default() // TODO add delete on termination setting
    };
    match _rt.block_on(client.create_volume(create_volume_rqst)) {
        Ok(output) => match output.volume_id {
            Some(volume_id) => {
                volume_id_holder = volume_id;
            }
            None => println!("no instances instantiated!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    let attach_volume_rqst = AttachVolumeRequest {
        device: "/dev/xvdf".to_string(),
        instance_id: "i-0cb68a3d1a173fe0c".to_string(), //TODO get it from underlying EC2
        volume_id: volume_id_holder,
        ..Default::default()
    };
    let ten_sec = time::Duration::from_millis(10000);
    thread::sleep(ten_sec);
    match _rt.block_on(client.attach_volume(attach_volume_rqst)) {
        Ok(output) => match output.volume_id {
            Some(volume_id) => {
                println!("{}", volume_id);
            }
            None => println!("no instances instantiated!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    // TODO volume init (formatting)
}

fn make_volumes_available() {
    create_attach_and_init_volume()
}

fn setup_mount_point(cli_args,_rt,cred_provider) {
    make_volumes_available()
    // Create VG
    // Add volumes
    // Create LG with settings
    // Mount LG
}


fn extend_mount_point() {
    make_volumes_available()
    // Extend VG
    // Extend LG: lvextend vg/stripe -l +100%FREE --resizefs
}

pub fn pym_disk_handler(cli_args: Cli) {
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
