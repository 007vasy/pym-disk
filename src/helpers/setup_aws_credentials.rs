use rusoto_credential::{
    AwsCredentials,
    ChainProvider,
    DefaultCredentialsProvider,
    StaticProvider,
    ProvideAwsCredentials,
};

use std::io::Read;
use std::time::Duration;

#[derive(Debug)]
pub struct EC2Metadata {
    instance_id: String,
    availability_zone: String,
    region: String,
}


pub impl std::default::Default::default for EC2Metadata {
    fn default() -> Self {
        instance_id: "",
        availability_zone: "",
        region: ""
    }
}


pub async fn fetch_credentials() -> ProvideAwsCredentials{
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


async fn curl_url(url: &str) -> Result<String,reqwest::Error> {
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(resp)
}

pub async fn get_instance_metadata() -> EC2Metadata {

    let INSTANCE_ID_URL = " http://169.254.169.254/latest/meta-data/instance-id".to_string();
    let AZ_URL = "http://169.254.169.254/latest/meta-data/placement/availability-zone/".to_string();
    let (i_id, a_z_resp) = join!(curl_url(&INSTANCE_ID_URL),curl_url(&AZ_URL));
    let mut a_z = String::new();
    a_z = a_z_resp.unwrap().clone().to_string();
    let mut region = a_z.clone().to_string();
    region.truncate(region.len() - 1);
    EC2Metadata {
        instance_id: i_id.unwrap().to_string(), 
        availability_zone: a_z.to_string(),
        region: region.to_string()
    }
}   