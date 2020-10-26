![Continuous integration](https://github.com/007vasy/pym-disk/workflows/Continuous%20integration/badge.svg?branch=dev)

# pym-disk

Rust based ebs volume autoscaling tool for AWS

## Using it on AWS EC2

`git clone --depth 1 git@github.com:007vasy/pym-disk.git`

aws ec2 run-instances --image-id ami-173d747e --count 1 --instance-type t1.micro --key-name MyKeyPair --security-groups my-sg

aws ec2 create-volume \
 --volume-type gp2 \
 --size 80 \
 --availability-zone ap-southeast-2b
