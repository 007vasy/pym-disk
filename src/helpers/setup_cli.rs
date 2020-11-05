use structopt::StructOpt;

custom_derive! {
    #[derive(Debug, PartialEq, EnumDisplay, EnumFromStr, Copy, Clone)]
    pub enum VolumeType {
        gp2,
        io1,
        io2,
    }
}

custom_derive! {
    #[derive(Debug, PartialEq, EnumDisplay, EnumFromStr, Copy, Clone)]
    pub enum FileSystem {
        btrfs,
        xfs,
        ext4,
    }
}

#[derive(StructOpt, Clone)]
pub struct CliOptions {
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
    pub min_disk_size: u64,
    // Assumption: all size imput should come in GiB, abbreviation: high
    #[structopt(short = "h", long = "max", default_value = "16")]
    pub maximal_capacity: u64,
    #[structopt(short = "s", long = "striping-level", default_value = "8")]
    pub striping_level: u64,
    // Last used device name to attach after
    #[structopt(
        short = "d",
        long = "last-used-device",
        default_value = "/dev/sda",
        parse(from_os_str)
    )]
    pub last_used_device: std::path::PathBuf,
    // Checking available disk space every <p> second
    #[structopt(short = "p", long = "poll", default_value = "60")]
    pub poll: u64,
    // No polling, just runnig pym-disk once (useful for creating desired volume setup)
    #[structopt(short, long)]
    pub oneshot: bool,

    #[structopt(short = "t", long = "volume_type", default_value = "gp2")]
    pub volume_type: VolumeType,

    #[structopt(short = "f", long = "file_system", default_value = "ext4")]
    pub file_system: FileSystem,

    // We are assuming that the user of the cli tool will give in the correct value for the appropiate volume type
    #[structopt(short = "i", long = "iops", default_value = "16000")]
    pub iops: u64,

    #[structopt(skip)]
    pub ec2_metadata: crate::helpers::setup_aws_credentials::EC2Metadata,

    #[structopt(skip)]
    pub disk_sizes: Vec<u64>,
}
