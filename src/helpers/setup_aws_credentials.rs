use rusoto_credential::{
    AwsCredentials,
    ChainProvider,
    DefaultCredentialsProvider,
    StaticProvider,
    ProvideAwsCredentials,
};

use curl::easy::Easy;

struct EC2Metadata {
    instance_id: String,
    availability_zone: String,
    region: String,
}

// export AWS_AZ=$(curl -s  http://169.254.169.254/latest/meta-data/placement/availability-zone/)
// export AWS_REGION=$(echo ${AWS_AZ} | sed -e 's/[a-z]$//')
// export INSTANCE_ID=$(curl -s  http://169.254.169.254/latest/meta-data/instance-id)

use std::time::Duration;

async fn fetch_credentials() -> ProvideAwsCredentials{
    //let profile_name = "pym-disk";
    //let profile_name = "default";

    //let mut pp = ProfileProvider::new().unwrap();
    //pp.set_profile(profile_name);
    //let mut cp = ChainProvider::with_profile_provider(pp);
    let mut cp = ChainProvider::new();
    // out expectation is to be running in AWS so this is plenty of time for it to
    // get any EC2 role credentials
    cp.set_timeout(Duration::from_millis(500));
    //let creds = cp.credentials().await.unwrap();
    let creds = DefaultCredentialsProvider::new()
        .unwrap()
        .credentials()
        .await
        .unwrap();
    let cred_provider = StaticProvider::new(
        creds.aws_access_key_id().to_string(),
        creds.aws_secret_access_key().to_string(),
        creds.token().clone(),
        None,
    );
    cred_provider.clone()
}

fn get_instance_metadata() -> EC2Metadata {
    let mut get_az = Easy::new();
    let mut get_i_id = Easy::new();
    let ec2_metadata: EC2Metadata;

    let AZ_URL = "http://169.254.169.254/latest/meta-data/placement/availability-zone/"
    let INSTANCE_ID_URL = " http://169.254.169.254/latest/meta-data/instance-id"

    get_i_id.url(INSTANCE_ID_URL).unwrap();
    get_i_id.write_function(|data| {
        Ok(data)
    }).unwrap();
    
    get_az.url(AZ_URL).unwrap();
    get_az.write_function(|data| {
        Ok(data)
    }).unwrap();

    let instance_id:String = get_i_id.perform.unwrap();
    let availability_zone:String = get_az.perform().unwrap();
    let region:String = availability_zone.clone().truncate(availability_zone.len()-1);


    {
        instance_id: instance_id.to_string(),
        availability_zone: availability_zone.to_string(),
        region: region.to_string()
    }
}