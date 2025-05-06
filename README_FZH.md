# JODO - 簡易命令行待辦事項應用

[English] (./README.md)| [日本語](./README_JA.md) | [簡體中文](./README_CN.md)|FakeChiese

JODO乃一輕量級命令行待辦事項應用，助君高效管理日常任務。

## 功能特點

- 簡潔命令行界面
- 支援任務截止日期
- 重要任務標記
- 截止日期顏色顯示（逾期、緊急、即將到期）
- 批量任務添加與操作
- 多語言支援（中文、英文、日文）

## 安裝方法

### Windows安裝

1. 前往[GitHub Releases頁面](https://github.com/JoyinJoester/JODO/releases)，下載最新安裝包 (`jodo-1.2.0-x86_64.msi`)
2. 雙擊安裝文件，按向導完成安裝。
3. 安裝完成後，打開命令提示符或PowerShell，輸入`jodo --version`驗證安裝是否成功。

### 從GitHub直接安裝

```bash
# 克隆倉庫
git clone https://github.com/JoyinJoester/JODO.git
cd JODO

# 編譯並安裝
cargo build --release
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

### 使用Cargo從GitHub安裝

```bash
cargo install --git https://github.com/JoyinJoester/JODO.git
```

### Linux安裝

1. 首先，構建發布版本：

```bash
cargo build --release
```

2. 運行提供的安裝腳本：

```bash
sudo chmod +x ./debian_install.sh
sudo ./debian_install.sh
```

此腳本將編譯後的二進制文件複製到`/usr/local/bin`目錄，並可選設置PATH環境變量。

3. 手動安裝方法：

若不願使用安裝腳本，可手動安裝：

```bash
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

4. 驗證安裝：

安裝完成後，打開新終端，輸入：

```bash
jodo --version
```

若顯示版本信息，則安裝成功。

### 使用Cargo安裝

若已安裝Rust和Cargo，也可使用以下命令安裝：

```bash
cargo install --path .
```

### 常見問題

#### Cargo.lock版本相容性問題

若在不同設備上編譯時遇到`Cargo.lock`文件相關錯誤，可能是格式版本不相容所致。本項目`Cargo.lock`使用版本4格式（文件第3行顯示`version = 4`），但某些舊版Rust或Cargo僅支援版本3。

**解決方案**：

1. 修改Cargo.lock文件：
   ```bash
   # 將"version = 4"改為"version = 3"
   sed -i 's/version = 4/version = 3/' Cargo.lock
   ```

2. 或更新Rust工具鏈：
   ```bash
   rustup update
   ```

3. 或完全重新生成Cargo.lock：
   ```bash
   rm Cargo.lock
   cargo build
   ```

注意：Cargo.lock版本4自Rust 1.62.0引入，若使用舊版Rust可能遇到相容性問題。

## 使用方法

### 基本操作

```bash
# 添加新任務
jodo "完成項目報告"

# 添加帶截止日期的任務
jodo "完成項目報告" -t 2023-12-31

# 列出所有任務
jodo -l
jodo
```

### 任務管理

```bash
# 編輯任務內容
jodo -e 1 "更新任務內容"

# 修改任務截止日期
jodo -e 1 -t 2023-12-25
jodo -t 1 2023-12-25  # 快捷方式

# 標記任務為已完成
jodo -c 1
jodo -c 1 2 3  # 一次完成多個任務

# 標記任務為未完成
jodo -u 1

# 標記任務為重要
jodo --star 1

# 取消任務重要標記
jodo --unstar 1

# 刪除任務
jodo -d 1
jodo -d 1 2 3  # 一次刪除多個任務

# 查看任務詳情
jodo --show 1
```

### 批量模式

```bash
# 進入批量模式
jodo -m

# 示例會話：
jodo$> 完成第一任務
jodo$> 完成第二任務
jodo$> 學習Rust編程
jodo$> exit  # 退出批量模式
```

### 批量操作

```bash
# 批量編輯任務截止日期（將任務5-8設置為本月22日）
jodo -e 5to8 -t 22
jodo -e 5-8 -t 22  # 等效語法

# 批量標記任務為已完成
jodo -c 1-5    # 標記任務1到5為已完成
jodo -c 1to5   # 等效語法

# 批量標記任務為未完成
jodo -u 1c-5c    # 標記已完成任務1c到5c為未完成
jodo -u 1cto5c   # 等效語法

# 批量刪除任務
jodo -d 1-3    # 刪除任務1到3
jodo -d 1to3   # 等效語法
```

### 其他選項

```bash
# 顯示幫助信息
jodo -h

# 顯示版本信息
jodo -v

# 切換語言
jodo -L en  # 英文
jodo -L zh-cn  # 中文
jodo -L ja  # 日文
```

## 截止日期顏色說明

- **粗體紅色**：已逾期
- **亮紅色**：今日到期（緊急）
- **黃色**：3日內到期（即將到期）
- **普通顏色**：其他日期

## 數據文件

JODO將所有任務數據存儲於以下位置：

- 任務數據：`~/.jodo/tasks.json`

每次任務操作後，數據會自動保存至此文件。若需備份數據，诸君仅需複製此文件。

## 許可協議

[MIT許可協議](LICENSE)