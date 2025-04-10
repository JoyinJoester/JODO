#!/bin/bash

# Jodo - 简单命令行Todo应用安装脚本

# 彩色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}开始安装 Jodo...${NC}"

# 检查是否已安装Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}未检测到Rust环境，是否要安装Rust？ [y/N]${NC}"
    read -r install_rust
    
    if [[ "$install_rust" =~ ^[Yy]$ ]]; then
        echo -e "${GREEN}正在安装Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo -e "${RED}Rust未安装，无法继续安装Jodo。${NC}"
        exit 1
    fi
fi

# 确定安装方式
echo -e "${YELLOW}请选择安装方式:${NC}"
echo "1) 从本地安装 (需要先克隆仓库)"
echo "2) 从GitHub直接安装 (推荐)"
read -r install_method

case $install_method in
    1)
        # 检查是否在项目目录下
        if [ ! -f "Cargo.toml" ]; then
            echo -e "${RED}错误: 未找到Cargo.toml，请确认您在项目目录下。${NC}"
            exit 1
        fi
        
        echo -e "${GREEN}从本地构建并安装...${NC}"
        cargo build --release
        
        if [ $? -ne 0 ]; then
            echo -e "${RED}构建失败!${NC}"
            exit 1
        fi
        
        # 安装到用户路径
        if [ -d "$HOME/.cargo/bin" ]; then
            cp ./target/release/jodo "$HOME/.cargo/bin/"
            echo -e "${GREEN}已安装到 $HOME/.cargo/bin/jodo${NC}"
        else
            echo -e "${YELLOW}无法找到Cargo bin目录，尝试安装到本地bin目录...${NC}"
            mkdir -p "$HOME/.local/bin"
            cp ./target/release/jodo "$HOME/.local/bin/"
            
            # 添加本地bin到PATH
            if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
                echo -e "${YELLOW}建议将 ~/.local/bin 添加到您的PATH中:${NC}"
                echo 'export PATH="$HOME/.local/bin:$PATH"'
            fi
            
            echo -e "${GREEN}已安装到 $HOME/.local/bin/jodo${NC}"
        fi
        ;;
        
    2)
        echo -e "${GREEN}从GitHub直接安装...${NC}"
        cargo install --git https://github.com/JoyinJoester/JODO.git
        
        if [ $? -ne 0 ]; then
            echo -e "${RED}安装失败!${NC}"
            exit 1
        fi
        
        echo -e "${GREEN}安装成功!${NC}"
        ;;
        
    *)
        echo -e "${RED}无效的选择，退出安装程序。${NC}"
        exit 1
        ;;
esac

# 验证安装
echo -e "${GREEN}验证安装...${NC}"
jodo --version

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Jodo安装成功!${NC}"
    echo -e "可以使用 ${YELLOW}jodo --help${NC} 命令查看使用说明"
else
    echo -e "${RED}无法找到jodo命令，安装可能不成功。${NC}"
    echo -e "请确保安装路径在您的PATH环境变量中。"
fi
