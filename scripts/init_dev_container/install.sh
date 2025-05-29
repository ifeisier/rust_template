#!/bin/bash

current_dir=$(pwd)
LOCK_FILE="$current_dir/init.lock"
if [ -f "$LOCK_FILE" ]; then
    exit 0
fi
echo "" > $LOCK_FILE

export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y wget curl git bzip2 unzip locales ntpdate g++ gcc libudev-dev pkg-config build-essential libssl-dev
apt-get install -y openssh-server
mkdir /var/run/sshd
echo 'root:VUI&sfp$8@VY' | chpasswd
sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
sed -i 's/#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config

echo "Asia/Shanghai" > /etc/timezone
ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
