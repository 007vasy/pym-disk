use structopt::StructOpt;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

custom_derive! {
    #[derive(Debug, PartialEq, EnumDisplay, EnumFromStr)]
    pub enum VolumeType {
        gp2,
        io1,
        io2,
    }
}

custom_derive! {
    #[derive(Debug, PartialEq, EnumDisplay, EnumFromStr)]
    pub enum FileSystem {
        btrfs,
        xfs,
        ext4,
    }
}

#[derive(StructOpt)]
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
    pub min_disk_size: i64,
    // Assumption: all size imput should come in GiB, abbreviation: high
    #[structopt(short = "h", long = "max", default_value = "16")]
    pub maximal_capacity: i64,
    #[structopt(short = "s", long = "striping-level", default_value = "8")]
    pub striping_level: i64,
    // Last used device name to attach after
    #[structopt(
        short = "d",
        long = "first-device",
        default_value = "/dev/sdb",
        parse(from_os_str)
    )]
    pub last_used_device: std::path::PathBuf,
    // Checking available disk space every <p> second
    #[structopt(short = "p", long = "poll", default_value = "10")]
    pub poll: i64,
    // No polling, just runnig pym-disk once (useful for creating desired volume setup)
    #[structopt(short, long)]
    pub oneshot: bool,
    
    #[structopt(short = "t", long = "volume_type", default_value = "gp2")]
    pub volume_type: VolumeType,

    #[structopt(short = "f", long = "file_system", default_value = "ext4")]
    pub file_system: FileSystem,

    // We are assuming that the user of the cli tool will give in the correct value for the appropiate volume type
    #[structopt(short = "i", long = "iops", default_value = "16000")]
    pub iops: i64,
    
    #[structopt(skip)]
    pub ec2_metadata: crate::helpers::setup_aws_credentials::EC2Metadata,
    
    #[structopt(skip)]
    pub disk_sizes: Vec<i64>
}
