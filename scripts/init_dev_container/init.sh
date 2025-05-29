#!/bin/bash

# 配置容器
project_name="rust_template"
mapping_port='["31874:22"]'
mapping_port=$(echo $mapping_port | tr -d '[]"')
mapping_port=$(echo $mapping_port | tr ',' ' ')
mapping_port_output=""
for port in $mapping_port; do
    mapping_port_output="$mapping_port_output -p $port"
done

# 配置命令参数
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
project_dir="$(cd "$script_dir/../../../" && pwd)"
volume_mount="$project_dir/$project_name"

init_lock="$volume_mount/scripts/init_dev_container/init.lock"
if [ -f "$init_lock" ]; then
    rm "$init_lock"
fi

sudo docker run -itd --restart="always" \
--privileged \
--name $project_name \
--entrypoint /bin/bash \
$mapping_port_output \
-v $volume_mount:/root/$project_name \
debian:12.10-slim \
-c "cd /root/$project_name/scripts/init_dev_container && ./install.sh && ./startup.sh"
