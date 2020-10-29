use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    // Mountpoint to attach the volumes
    #[structopt(
        short = "m",
        long = "mount-point",
        default_value = "/stratch",
        parse(from_os_str)
    )]
    pub mount_point: std::path::PathBuf,
    // Assumption: all size imput should come in GiB, abbreviation: low
    #[structopt(short = "l", long = "min", default_value = "4")]
    pub min: i64,
    // Assumption: all size imput should come in GiB, abbreviation: high
    #[structopt(short = "h", long = "max", default_value = "16")]
    pub max: i64,
    #[structopt(short = "s", long = "striping-level", default_value = "8")]
    pub striping_level: i64,
    // First device name to attach
    #[structopt(
        short = "f",
        long = "first-device",
        default_value = "/dev/sdb",
        parse(from_os_str)
    )]
    pub first_device: std::path::PathBuf,
    // Checking available disk space every <p> second
    #[structopt(short = "p", long = "poll", default_value = "5")]
    pub poll: i64,
    // No polling, just runnig pym-disk once (useful for creating desired volume setup)
    #[structopt(short, long)]
    pub oneshot: bool,
}
