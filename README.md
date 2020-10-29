![Continuous integration](https://github.com/007vasy/pym-disk/workflows/Continuous%20integration/badge.svg?branch=dev)

# pym-disk

Rust based ebs volume autoscaling tool for AWS

# TODOS

    * input checking, everything is higher than 0, min < max, stripe % 2 == 0, min * 2^x == max? x>=1
    * device naming conventions
    * add volume types
    * add fs types
    * add parse config from environment variables (AWS region, etc)
    * add parse config from file
    * add logging to file
    * poll is > speed of adding new drives
    * add end-to-end demo to examples?
    * tests, long due
    * add deploy binary description
    * add contributions (fork + PR)
    * add fibonacci growing strat next to doubling
