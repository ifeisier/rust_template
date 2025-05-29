#!/bin/bash

ntpdate ntp.aliyun.com

service ssh start
tail -f /dev/null
