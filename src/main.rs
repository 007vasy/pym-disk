mod helpers {
    pub mod setup_aws_credentials;
    pub mod setup_cli;
    pub mod setup_tokio;
}
mod disk_utils;

use disk_utils::pym_disk_handler;
use helpers::setup_cli::Cli;

use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    println!("Mount point: {:?}", args.mount_point);
    println!("Starting disk size: {:?}", args.min);
    println!("Maximal overall available size: {:?}", args.max);
    println!("Striping (Raid 0) level: {:?}", args.striping_level);
    println!("First device: {:?}", args.first_device);
    println!("Polling frequency: {:?}", args.poll);
    println!("OneShotMode is {:?}", args.oneshot);
    // TODO: input checking, everything is higher than 0, min < max, stripe % 2 == 0, min * 2^x == max? x>=1
    // TODO: device naming conventions

    pym_disk_handler(args);
}
