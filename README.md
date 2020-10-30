![Continuous integration](https://github.com/007vasy/pym-disk/workflows/Continuous%20integration/badge.svg?branch=dev)

# pym-disk

Rust based ebs volume autoscaling tool for AWS with striping

# TODOS

- speed test with s3bfg
- input checking, everything is higher than 0, min < max, stripe % 2 == 0, min \* 2^x == max? x>=1
- device naming conventions
- add correctly parsed volume types
- add correctly parsed fs types
- add correctly parsed disk paths
- add parse config from environment variables (AWS region, etc)
- add parse config from file
- add logging to file
- poll is > speed of adding new drives
- add end-to-end demo to examples?
- tests, long due
- add deploy binary description
- add contributions (fork + PR)
- add fibonacci and doubling to the 10x strat (because limited device names and striping)
- change every command line invocation to Rust code
- check pre-reqs
- more status messages
- add IO speed as a parameter
- test delete on termination to make sure
- regex mount point and device name

# Steps to do striped autoscaling manually

## Pre-reqs

yum update -y
yum install -y btrfs-progs xfsprogs e4fsprogs lvm2

## Setup

- vgcreate vg /dev/sdb /dev/sdc
- lvcreate -n stripe -l +100%FREE -i 2 vg
- mkdir /stratch
- mkfs.ext4 /dev/vg/stripe
- mount /dev/vg/stripe /stratch

## Extension

- vgextend vg /dev/sdd /dev/sde
- lvextend vg/stripe -l +100%FREE --resizefs
