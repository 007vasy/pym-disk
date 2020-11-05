#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
mod helpers {
    pub mod setup_aws_credentials;
    pub mod setup_cli;
    pub mod setup_tokio;
}
mod disk_utils;
use crate::disk_utils::pym_disk_handler;
use structopt::StructOpt;

fn main() {
    let args = helpers::setup_cli::CliOptions::from_args();
    println!("Mount point: {:?}", args.mount_point);
    println!("Starting disk size: {:?}", args.min_disk_size);
    println!(
        "Maximal overall available size: {:?}",
        args.maximal_capacity
    );
    println!("Striping (Raid 0) level: {:?}", args.striping_level);
    println!("Polling frequency: {:?}", args.poll);
    println!("OneShotMode is {:?}", args.oneshot);
    println!("Volume type is {:?}", args.volume_type);
    println!("File system is {:?}", args.file_system);
    println!("IOPs is {:?}", args.iops);

    pym_disk_handler(args);
}
