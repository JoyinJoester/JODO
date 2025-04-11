# Jodo - シンプルなコマンドラインTodoアプリケーション

JodoはRustで開発されたコマンドラインTodoアプリケーションで、ターミナルでタスク管理を支援します。

[English](./README_EN.md) | 日本語 | [简体中文](./README.md)

## 主な機能

- タスクリストの管理、追加、編集、削除、完了状態のマーク付けをサポート
- タスクに期限を設定
- 重要なタスクをマークしてトップに表示
- 完了したタスクを自動的に分類
- タスクの詳細情報の表示
- 複数のタスクを一度に削除する機能

## インストール

### GitHubから直接インストール

```bash
# リポジトリをクローン
git clone https://github.com/JoyinJoester/JODO.git
cd JODO

# コンパイルとインストール
cargo build --release
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

### Cargoを使用してGitHubから直接インストール

```bash
cargo install --git https://github.com/JoyinJoester/JODO.git
```

### Linuxにインストール

1. まずリリースビルドを行います:

```bash
cargo build --release
```

2. 提供されているインストールスクリプトを実行します:

```bash
sudo chmod +x ./debian_install.sh
sudo ./debian_install.sh
```

このスクリプトは、コンパイルされたバイナリを `/usr/local/bin` ディレクトリにコピーし、オプションでPATH環境変数を設定します。

3. 手動インストール方法:

インストールスクリプトを使用したくない場合は、手動でインストールできます:

```bash
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

4. インストールの確認:

インストール後、新しいターミナルを開いて以下を入力します:

```bash
jodo --version
```

バージョン情報が表示されれば、インストールは成功です。

### Cargoを使用してインストール

RustとCargoがすでにインストールされている場合は、次のコマンドを使用してインストールすることもできます:

```bash
cargo install --path .
```

## 使用方法

### 基本操作

```bash
jodo "プロジェクトレポートを完成させる"         # 新しいタスクを追加
jodo "プロジェクトレポートを完成させる" -t 2023-12-31 # 期限付きでタスクを追加
jodo -l, --list                           # すべてのタスクを表示
jodo                                      # 上記と同じ、すべてのタスクを表示
```

### タスク管理コマンド

```bash
jodo -e, --edit <id> --content "内容"   # タスク内容を編集
jodo -e, --edit <id> -t, --time 日付    # タスクの期限を編集
jodo -d, --delete <id>                 # 一つのタスクを削除
jodo -d, --delete <id1> <id2> <id3>    # 複数のタスクを一度に削除
jodo -c, --complete <id>               # タスクを完了としてマーク
jodo -u, --undo <id>                   # タスクの完了マークを解除
jodo --star <id>                       # タスクを重要としてマーク（トップに固定）
jodo --unstar <id>                     # 重要マークを解除
jodo --show <id>                       # タスクの詳細情報を表示
```

### サブコマンド形式

```bash
jodo list                  # すべてのタスクを表示
jodo done <id>             # タスクを完了としてマーク
jodo undo <id>             # タスクの完了マークを解除
jodo remove <id>           # タスクを削除
jodo edit <id> "内容"       # タスク内容を編集
jodo edit <id> -t 日付      # タスクの期限を編集
jodo star <id>             # タスクを重要としてマーク
jodo unstar <id>           # 重要マークを解除
jodo show <id>             # タスクの詳細情報を表示
```

### その他のオプション

```bash
jodo -h, --help            # ヘルプ情報を表示
jodo -v, --version         # バージョン情報を表示
jodo -L, --language <lang> # 言語を設定 (zh-cn/en/ja)
```

### 注意事項

- 完了したタスクのIDには「c」サフィックスが付きます（例: `1c`）
- 重要マークされたタスクはリストのトップに表示されます
- タスクIDは変更や削除後に自動的に再割り当てされ、連続性を維持します

## ファイル保存

タスクデータは `~/.jodo/tasks.json` ファイルに保存されます。各操作後に変更は自動的に保存されます。

## 使用例

```bash
# 新しいタスクを追加
jodo "プロジェクトレポートを完成させる"

# 期限付きでタスクを追加
jodo "プロジェクトレポートを完成させる" -t 2023-12-31

# すべてのタスクを表示
jodo -l

# タスク内容を編集
jodo -e 1 --content "修正された内容"

# タスクの期限を設定
jodo -e 1 -t 2023-12-31

# タスクを重要としてマーク
jodo --star 1

# タスク詳細を表示
jodo --show 1

# タスクを完了としてマーク
jodo -c 1

# タスクの完了マークを解除
jodo -u 1c

# 一つのタスクを削除
jodo -d 1

# 複数のタスクを削除
jodo -d 1 3 5

# 中国語インターフェースに切り替え
jodo -L zh-cn
```
