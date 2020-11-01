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
    let args = CliOptions::from_args();
    println!("Mount point: {:?}", args.mount_point);
    println!("Starting disk size: {:?}", args.min_disk_size);
    println!("Maximal overall available size: {:?}", args.maximal_capacity);
    println!("Striping (Raid 0) level: {:?}", args.striping_level);
    println!("Polling frequency: {:?}", args.poll);
    println!("OneShotMode is {:?}", args.oneshot);
    println!("OneShotMode is {:?}", args.volume_type);
    println!("OneShotMode is {:?}", args.file_system);
    println!("OneShotMode is {:?}", args.iops);

    pym_disk_handler(args);
}

