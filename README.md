# JODO - 简单的命令行 Todo 应用

[English](./README_EN.md) | [日本語](./README_JA.md) | 简体中文

JODO 是一个轻量级命令行 Todo 应用程序，帮助您高效管理日常任务。

## 特性

- 简洁的命令行界面
- 支持任务截止日期设置
- 重要任务标记
- 截止日期彩色显示（逾期、紧急、即将到期）
- 批量添加和操作任务
- 多语言支持（中文、英文、日语）

## 安装

### 从GitHub直接安装

```bash
# 克隆仓库
git clone https://github.com/JoyinJoester/JODO.git
cd JODO

# 编译安装
cargo build --release
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

### 使用Cargo从GitHub直接安装

```bash
cargo install --git https://github.com/JoyinJoester/JODO.git
```

### Linux安装

1. 首先进行发布构建:

```bash
cargo build --release
```

2. 运行提供的安装脚本:

```bash
sudo chmod +x ./debian_install.sh
sudo ./debian_install.sh
```

该脚本会将编译好的二进制文件复制到 `/usr/local/bin` 目录，并可选地设置 PATH 环境变量。

3. 手动安装方式:

如果不想使用安装脚本，可以手动安装:

```bash
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

4. 验证安装:

安装后，打开新的终端并输入:

```bash
jodo --version
```

如果显示版本信息，则安装成功。

### 使用Cargo安装

如果已经安装了Rust和Cargo，也可以使用以下命令安装:

```bash
cargo install --path .
```

### 可能遇到的问题

#### Cargo.lock 版本兼容性问题

如果您在其他设备上编译时遇到与 `Cargo.lock` 文件相关的错误，这可能是因为 `Cargo.lock` 文件中的格式版本不兼容。在本项目中，`Cargo.lock` 使用了版本 4 格式（文件的第 3 行显示 `version = 4`），但某些旧版本的 Rust 或 Cargo 可能只支持版本 3。

**解决方法**:

1. 修改 Cargo.lock 文件:
   ```bash
   # 将 Cargo.lock 文件中的 "version = 4" 改为 "version = 3"
   sed -i 's/version = 4/version = 3/' Cargo.lock
   ```

2. 或者更新您的 Rust 工具链:
   ```bash
   rustup update
   ```

3. 也可以完全重新生成 Cargo.lock:
   ```bash
   rm Cargo.lock
   cargo build
   ```

注意：Cargo.lock 版本 4 是在 Rust 1.62.0 中引入的，如果您使用的是较旧的 Rust 版本，将会遇到兼容性问题。

## 使用方法

### 基本操作

```bash
# 添加新任务
jodo "完成项目报告"

# 添加带有截止日期的任务
jodo "完成项目报告" -t 2023-12-31

# 列出所有任务
jodo -l
jodo
```

### 任务管理

```bash
# 编辑任务内容
jodo -e 1 "更新后的任务内容"

# 修改任务截止日期
jodo -e 1 -t 2023-12-25
jodo -t 1 2023-12-25  # 快捷方式

# 标记任务为已完成
jodo -c 1
jodo -c 1 2 3  # 一次完成多个任务

# 取消任务完成标记
jodo -u 1

# 标记任务为重要
jodo --star 1

# 取消任务重要标记
jodo --unstar 1

# 删除任务
jodo -d 1
jodo -d 1 2 3  # 一次删除多个任务

# 查看任务详情
jodo --show 1
```

### 批量添加模式

```bash
# 进入批量添加模式
jodo -m

# 示例会话:
jodo$> 完成第一项任务
jodo$> 完成第二项任务
jodo$> 学习Rust编程
jodo$> exit  # 退出批量模式
```

### 其他选项

```bash
# 显示帮助信息
jodo -h

# 显示版本信息
jodo -v

# 切换语言
jodo -L en  # 英文
jodo -L zh-cn  # 中文
jodo -L ja  # 日语
```

## 截止日期颜色说明

- **红色加粗**: 已过期
- **亮红色**: 今天到期（紧急）
- **黄色**: 3天内到期（即将到期）
- **正常颜色**: 其他日期

## 数据文件

JODO 将所有任务数据保存在以下位置：

- 任务数据： `~/.jodo/tasks.json`

每次对任务进行操作后，数据会自动保存到此文件。如需备份数据，只需复制该文件即可.


