use rusoto_credential::{
    AwsCredentials, ChainProvider, ProfileProvider,
    ProvideAwsCredentials,
};
use std::time::Duration;

pub async fn fetch_credentials() -> (AwsCredentials, String) {
    let profile_name = "pym-disk";
    //let profile_name = "default";


    let mut pp = ProfileProvider::new().unwrap();
    pp.set_profile(profile_name);
    let mut cp = ChainProvider::with_profile_provider(pp);
    // out expectation is to be running in AWS so this is plenty of time for it to
    // get any EC2 role credentials
    cp.set_timeout(Duration::from_millis(500));
    let creds = cp.credentials().await.unwrap();

    (
        creds.clone(),
        format!("Profile `{}` -> {:?}", profile_name, creds,),
    )
}