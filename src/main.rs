use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_ec2::{
    AttachVolumeRequest, CreateVolumeRequest, Ec2, Ec2Client, RunInstancesRequest, Tag,
    TagSpecification,
};
use rusoto_sts::StsAssumeRoleSessionCredentialsProvider;
use rusoto_sts::StsClient;
use std::default::Default;
use std::io::{stdin, stdout, Write};
use std::{thread, time};
use uuid::Uuid;
mod setup_tokio;
use setup_tokio::create_runtime;

mod setup_aws_credentials;
use setup_aws_credentials::fetch_credentials;

use sysinfo::{DiskExt, ProcessExt, System, SystemExt};

pub fn spawn(worker_type: String) {
    println!("worker type: {}", worker_type);

    let client_token = format!("{}-{}", worker_type, Uuid::new_v4());
    println!("client token: {}", client_token);

    // we use tokio runtime for various async activity
    let (mut rt, rt_msg) = create_runtime();

    // a single set of credentials which we are assuming will last throughout the whole copy
    let (creds, creds_msg) = rt.block_on(fetch_credentials());

    let cred_provider = StaticProvider::new(
        creds.aws_access_key_id().to_string(),
        creds.aws_secret_access_key().to_string(),
        creds.token().clone(),
        None,
    );

    // let sts = StsClient::new_with(HttpClient::new().unwrap(),cred_provider,Region::ApSoutheast2);

    // let mut provider = StsAssumeRoleSessionCredentialsProvider::new(
    //     sts,
    //     "arn:aws:iam::667213777749:role/OrganizationAccountAccessRole".to_owned(),
    //     "default".to_owned(),
    //     None, None, None,
    //     Some("arn:aws:iam::355186423092:mfa/bence.vass".to_owned()),
    // );
    // let mut s=String::new();
    // print!("Please enter the MFA code: ");
    // let _=stdout().flush();
    // stdin().read_line(&mut s).expect("Did not enter a correct string");
    // if let Some('\n')=s.chars().next_back() {
    //     s.pop();
    // }
    // if let Some('\r')=s.chars().next_back() {
    //     s.pop();
    // }
    // println!("You typed: {}",s);
    // provider.set_mfa_code(s);

    // let client = Ec2Client::new_with(HttpClient::new().unwrap(), provider, Region::ApSoutheast2);
    let client = Ec2Client::new_with(
        HttpClient::new().unwrap(),
        cred_provider,
        Region::ApSoutheast2,
    );

    // let run_instances_request: RunInstancesRequest = RunInstancesRequest {
    //   min_count: 1,
    //   max_count: 1,
    //   key_name: Some("pym-disk-temp-key".to_string()),
    //   client_token: Some(client_token),
    //   image_id: Some("ami-0f96495a064477ffb".to_string()),
    //   instance_type: Some("t2.micro".to_string()),
    //   //security_groups: Some(vec!["rdp-only".to_string(), "ssh-only".to_string()]),
    //   //security_group_ids: Some(vec!["sg-3bd7bf41".to_string(), "sg-s5bd6be21".to_string()]),
    //   subnet_id: Some("subnet-03e605e7cac782459".to_string()),
    //   instance_initiated_shutdown_behavior: Some("stop".to_string()),
    //   ..Default::default()
    // };

    let mut system = sysinfo::System::new_all();

    // First we update all information of our system struct.
    system.refresh_all();

    // Now let's print every process' id and name:
    // for (pid, proc_) in system.get_processes() {
    //     println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
    // }

    // Then let's print the temperature of the different components:
    // for component in system.get_components() {
    //     println!("{:?}", component);
    // }

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

    let mut volume_id_holder = String::new();
    let create_volume_rqst = CreateVolumeRequest {
        availability_zone: "ap-southeast-2b".to_string(), //Todo get it from config
        size: Some(8), // increase with every new addition, Fibonacci?
        volume_type: Some("gp2".to_string()), //Todo get it from config
        tag_specifications: Some(vec![TagSpecification {
            resource_type: Some("volume".to_string()),
            tags: Some(vec![Tag {
                key: Some("createdBy".to_string()),
                value: Some("pym-disk".to_string()),
            }]),
        }]),
        ..Default::default() // TODO add delete on termination
    };
    match rt.block_on(client.create_volume(create_volume_rqst)) {
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
        instance_id: "i-0cb68a3d1a173fe0c".to_string(), // TODO get it from config
        volume_id: volume_id_holder,
        ..Default::default()
    };
    let ten_sec = time::Duration::from_millis(10000);
    thread::sleep(ten_sec);
    match rt.block_on(client.attach_volume(attach_volume_rqst)) {
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

    //   match rt.block_on(client.run_instances(run_instances_request)) {
    //     Ok(output) => {
    //       match output.instances {
    //         Some(instances) => {
    //           println!("instances instantiated:");
    //           for instance in instances {
    //             println!("{:?}", instance.instance_id);
    //           }
    //         }
    //         None => println!("no instances instantiated!"),
    //       }
    //     }
    //     Err(error) => {
    //       println!("Error: {:?}", error);
    //     }
    //   }
}

fn main() {
    spawn("BenTest".to_string());
}
