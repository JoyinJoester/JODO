#!/bin/bash

# 确保脚本以root权限运行
if [ "$EUID" -ne 0 ]; then
  echo "请使用sudo运行此脚本"
  exit 1
fi

# 创建安装目录
install_dir="/usr/local/bin"
mkdir -p "$install_dir"

# 假设编译后的二进制文件在target/release目录
binary_path="./target/release/jodo"

if [ ! -f "$binary_path" ]; then
  echo "错误: 未找到jodo二进制文件。请先运行 'cargo build --release'"
  exit 1
fi

# 复制二进制文件到安装目录
cp "$binary_path" "$install_dir/"
chmod +x "$install_dir/jodo"

echo "jodo已安装到 $install_dir/jodo"
echo "你可以现在运行 'jodo' 命令"

# 添加环境变量到.bashrc (如果需要)
echo "是否要将jodo添加到PATH环境变量? [y/N]"
read add_to_path

if [[ "$add_to_path" =~ ^[Yy]$ ]]; then
  for profile in /etc/profile.d/*.sh; do
    echo "export PATH=\$PATH:$install_dir" >> "$profile"
  done
  echo "已添加jodo到系统PATH环境变量"
fi

echo "安装完成！"
