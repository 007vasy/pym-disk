use uuid::Uuid;
use std::default::Default;

use rusoto_core::{Region, HttpClient};

use rusoto_ec2::{Ec2Client, Ec2, RunInstancesRequest};
use rusoto_credential::StaticProvider;
mod setup_tokio;
use setup_tokio::create_runtime;

mod setup_aws_credentials;
use setup_aws_credentials::fetch_credentials;


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

    let client = Ec2Client::new_with(HttpClient::new().unwrap(), cred_provider, Region::ApSoutheast2);

    let run_instances_request: RunInstancesRequest = RunInstancesRequest {
      min_count: 1,
      max_count: 1,
      key_name: Some(format!("ben-worker-autoscaler-{}", worker_type)),
      client_token: Some(client_token),
      image_id: Some("ami-0da8269b3b0487036".to_string()),
      instance_type: Some("t2.micro".to_string()),
      //security_groups: Some(vec!["rdp-only".to_string(), "ssh-only".to_string()]),
      //security_group_ids: Some(vec!["sg-3bd7bf41".to_string(), "sg-s5bd6be21".to_string()]),
      //subnet_id: Some("subnet-f94cb29f".to_string()),
      instance_initiated_shutdown_behavior: Some("stop".to_string()),
      ..Default::default()
    };
  
    match rt.block_on(client.run_instances(run_instances_request)) {
      Ok(output) => {
        match output.instances {
          Some(instances) => {
            println!("instances instantiated:");
            for instance in instances {
              println!("{:?}", instance.instance_id);
            }
          }
          None => println!("no instances instantiated!"),
        }
      }
      Err(error) => {
        println!("Error: {:?}", error);
      }
    }
  }


fn main() {
  spawn("BenTest".to_string());
}
