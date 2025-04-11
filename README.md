# Jodo - 简单的命令行Todo应用

Jodo是一个使用Rust开发的命令行Todo应用程序，可以帮助你在终端中管理任务。
[English](./README_EN.md) | [日本語](./README_JA.md) | 简体中文

## 功能亮点

- 管理待办事项清单，支持添加、编辑、删除和标记完成状态
- 为任务设置截止日期
- 标记重要任务并置顶显示
- 已完成任务自动归类
- 支持查看任务详细信息
- 支持一次性删除多个任务

## 安装

### 直接从GitHub安装

```bash
# 克隆仓库
git clone https://github.com/JoyinJoester/JODO.git
cd JODO

# 编译并安装
cargo build --release
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

### 使用Cargo直接从GitHub安装

```bash
cargo install --git https://github.com/JoyinJoester/JODO.git
```

### 在Linux上安装

1. 首先构建发行版:

```bash
cargo build --release
```

2. 然后运行提供的安装脚本:

```bash
sudo chmod +x ./debian_install.sh
sudo ./debian_install.sh
```

此脚本会将编译好的二进制文件复制到`/usr/local/bin`目录，并可选择配置PATH环境变量。

3. 手动安装方式:

如果你不想使用安装脚本，可以手动安装:

```bash
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

4. 验证安装:

安装完成后，你可以打开一个新终端并输入:

```bash
jodo --version
```

如果显示版本信息，则安装成功。

### 使用Cargo安装

如果你已经安装了Rust和Cargo，也可以直接使用以下命令安装:

```bash
cargo install --path .
```

## 使用方法

### 基本操作

```bash
jodo "完成项目报告"               # 添加新任务
jodo "完成项目报告" -t 2023-12-31 # 添加带截止日期的任务
jodo -l, --list                 # 列出所有任务
jodo                           # 同上，列出所有任务
```

### 任务管理命令

```bash
jodo -e, --edit <id> --content "内容"   # 编辑任务内容
jodo -e, --edit <id> -t, --time 日期    # 编辑任务截止日期
jodo -d, --delete <id>                 # 删除单个任务
jodo -d, --delete <id1> <id2> <id3>    # 同时删除多个任务
jodo -c, --complete <id>               # 标记任务为已完成
jodo -u, --undo <id>                   # 取消任务完成标记
jodo --star <id>                       # 标记任务为重要（置顶）
jodo --unstar <id>                     # 取消任务重要标记
jodo --show <id>                       # 显示任务的详细信息
```

### 子命令形式

```bash
jodo list                  # 列出所有任务
jodo done <id>             # 标记任务为已完成
jodo undo <id>             # 取消任务完成标记
jodo remove <id>           # 删除任务
jodo edit <id> "内容"       # 编辑任务内容
jodo edit <id> -t 日期      # 编辑任务截止日期
jodo star <id>             # 标记任务为重要
jodo unstar <id>           # 取消任务重要标记
jodo show <id>             # 显示任务的详细信息
```

### 其他选项

```bash
jodo -h, --help            # 显示帮助信息
jodo -v, --version         # 显示版本信息
jodo -L, --language <lang> # 设置语言 (zh-cn/en/ja)
```

### 注意事项

- 已完成的任务ID会在末尾显示'c'，例如 `1c`
- 星标任务会在列表中置顶显示
- 任务ID在修改或删除后会自动重新分配，保持连续性

## 文件存储

任务数据存储在 `~/.jodo/tasks.json` 文件中。每次操作会自动保存更改。

## 示例

```bash
# 添加一个新任务
jodo "完成项目报告"

# 添加一个带截止日期的任务
jodo "完成项目报告" -t 2023-12-31

# 查看所有任务
jodo -l

# 编辑任务内容
jodo -e 1 --content "修改后的内容"

# 设置任务截止日期
jodo -e 1 -t 2023-12-31

# 标记任务为重要
jodo --star 1

# 查看任务详情
jodo --show 1

# 标记任务为完成
jodo -c 1

# 取消任务完成标记
jodo -u 1c

# 删除单个任务
jodo -d 1

# 删除多个任务
jodo -d 1 3 5

# 切换到英文界面
jodo -L en
```

