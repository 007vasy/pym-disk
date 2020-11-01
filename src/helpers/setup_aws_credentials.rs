use rusoto_credential::{
    AwsCredentials,
    ChainProvider,
    DefaultCredentialsProvider,
    StaticProvider,
    ProvideAwsCredentials,
};

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