# Jodo - Simple な Command Line Todo Application

JodoはRustで開発されたCommand Line Todo Applicationで、Terminalでのtask管理を支援します。

[English](./README_EN.md) | 日本語 | [简体中文](./README.md)

## 主な機能

- Task listの管理、追加、編集、削除、完了状態のmark付けを対応
- Taskに期限を設定
- 重要なtaskをmarkしてtopに表示
- 完了したtaskを自動的に分類
- Taskの詳細情報の表示
- 複数のtaskを一度に削除する機能

## Install方法

### GitHubから直接install

```bash
# Repositoryを取得
git clone https://github.com/JoyinJoester/JODO.git
cd JODO

# 構築とinstall
cargo build --release
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

### CargoでGitHubから直接install

```bash
cargo install --git https://github.com/JoyinJoester/JODO.git
```

### Linuxにinstall

1. まずrelease buildを実行します:

```bash
cargo build --release
```

2. 提供されているinstall scriptを実行します:

```bash
sudo chmod +x ./debian_install.sh
sudo ./debian_install.sh
```

このscriptは、構築されたbinaryを `/usr/local/bin` directoryにコピーし、必要に応じてPATH環境変数を設定します。

3. 手動install方法:

Install scriptを使いたくない場合は、手動でinstallできます:

```bash
sudo cp ./target/release/jodo /usr/local/bin/
sudo chmod +x /usr/local/bin/jodo
```

4. Installの確認:

Install後、新しいterminalを開いて以下を入力します:

```bash
jodo --version
```

Version情報が表示されれば、installは成功です。

### Cargoを使ってinstall

RustとCargoがすでにinstallされている場合は、次のcommandを使ってinstallすることもできます:

```bash
cargo install --path .
```

## 使用方法

### 基本操作

```bash
jodo "プロジェクトレポートを完成させる"         # 新しいtaskを追加
jodo "プロジェクトレポートを完成させる" -t 2023-12-31 # 期限付きでtaskを追加
jodo -l, --list                           # すべてのtaskを表示
jodo                                      # 上記と同じ、すべてのtaskを表示
```

### Task管理command

```bash
jodo -e, --edit <id> --content "内容"   # Task内容を編集
jodo -e, --edit <id> -t, --time 日付    # Taskの期限を編集
jodo -d, --delete <id>                 # 一つのtaskを削除
jodo -d, --delete <id1> <id2> <id3>    # 複数のtaskを一度に削除
jodo -c, --complete <id>               # Taskを完了としてmark
jodo -c, --complete <id1> <id2>        # 複数のtaskを一度に完了としてmark
jodo -u, --undo <id>                   # Taskの完了markを解除
jodo --star <id>                       # Taskを重要としてmark（topに固定）
jodo --unstar <id>                     # 重要markを解除
jodo --show <id>                       # Taskの詳細情報を表示
```

### Sub-command形式

```bash
jodo list                  # すべてのtaskを表示
jodo done <id>             # Taskを完了としてmark
jodo undo <id>             # Taskの完了markを解除
jodo remove <id>           # Taskを削除
jodo edit <id> "内容"       # Task内容を編集
jodo edit <id> -t 日付      # Taskの期限を編集
jodo star <id>             # Taskを重要としてmark
jodo unstar <id>           # 重要markを解除
jodo show <id>             # Taskの詳細情報を表示
```

### Batch Mode

```bash
jodo -m                    # Batch modeを開始
jodo$> 最初のtaskを完了する    # 新しいtaskを追加
jodo$> 次のtaskを完了する      # 別のtaskを追加
jodo$> exit                # Batch modeを終了
```

### その他のoption

```bash
jodo -h, --help            # Help情報を表示
jodo -v, --version         # Version情報を表示
jodo -L, --language <lang> # 言語を設定 (zh-cn/en/ja)
```

### 期限の色について

- **赤色 (太字)**: 期限切れ
- **明るい赤色**: 今日が期限 (緊急)
- **黄色**: 3日以内が期限 (もうすぐ)
- **通常色**: その他の日付

### 注意事項

- 完了したtaskのIDには「c」suffixが付きます（例: `1c`）
- 重要markされたtaskはlistのtopに表示されます
- Task IDは変更や削除後に自動的に再割り当てされ、連続性を維持します

## File保存

Task dataは `~/.jodo/tasks.json` fileに保存されます。各操作後に変更は自動的に保存されます。

## 使用例

```bash
# 新しいtaskを追加
jodo "プロジェクトレポートを完成させる"

# 期限付きでtaskを追加
jodo "プロジェクトレポートを完成させる" -t 2023-12-31

# すべてのtaskを表示
jodo -l

# Task内容を編集
jodo -e 1 --content "修正された内容"

# Taskの期限を設定
jodo -e 1 -t 2023-12-31

# 直接期限を設定（簡単な方法）
jodo -t 1 2023-12-31

# Taskを重要としてmark
jodo --star 1

# Task詳細を表示
jodo --show 1

# Taskを完了としてmark
jodo -c 1

# 複数のtaskを一度に完了としてmark
jodo -c 1 3 5

# Taskの完了markを解除
jodo -u 1c

# 一つのtaskを削除
jodo -d 1

# 複数のtaskを削除
jodo -d 1 3 5

# 中国語interfaceに切り替え
jodo -L zh-cn
```
