use tokio::runtime::Runtime;

/// Returns a tokio runtime configured based on our command line settings.
///
pub fn create_runtime() -> (Runtime, String) {
    let rt: Runtime;
    let mut rt_description: String = String::new();


    let rt = tokio::runtime::Builder::new()
            .enable_all()
            .basic_scheduler()
            .build()
            .unwrap();
    rt_description.push_str("basic");


    (rt, rt_description)
}
