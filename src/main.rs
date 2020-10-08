use rusoto_core::{
    Region
  };
use rusoto_ec2::{
    Ec2,
    Ec2Client,
    RunInstancesRequest
  };
use uuid::Uuid;



pub fn spawn(worker_type: String) {
    println!("worker type: {}", worker_type);
  
    let client_token = format!("{}-{}", worker_type, Uuid::new_v4());
    println!("client token: {}", client_token);
  
    let client = Ec2Client::new(Region::ApSoutheast2);
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
  
    match client.run_instances(run_instances_request).unwrap() {
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
    spawn("BenTest".to_string())
}
