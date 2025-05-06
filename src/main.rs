use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::cmp::Ordering;
use chrono::{DateTime, Local, NaiveDate, Datelike};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Language {
    Chinese, 
    English,
    Japanese,
}

impl Language {
    fn from_str(lang_str: &str) -> Self {
        match lang_str.to_lowercase().as_str() {
            "en" | "english" => Language::English,
            "ja" | "jp" | "japanese" => Language::Japanese,
            _ => Language::Chinese, //默认语言
        }
    }
}

// 语言资源结构
struct Translations {
    lang: Language,
}

impl Translations {
    fn new(lang: Language) -> Self {
        Self { lang }
    }

    // 翻译方法
    fn task_added(&self, task: &str) -> String {
        match self.lang {
            Language::English => format!("Task \"{}\" added", task),
            Language::Japanese => format!("タスク \"{}\" が追加されました", task),
            Language::Chinese => format!("任务 \"{}\" 已添加", task),
        }
    }

    fn due_date(&self) -> String {
        match self.lang {
            Language::English => "Due date",
            Language::Japanese => "期限",
            Language::Chinese => "截止日期",
        }.to_string()
    }

    fn incomplete_tasks(&self) -> String {
        match self.lang {
            Language::English => "Incomplete Tasks:",
            Language::Japanese => "未完了のタスク:",
            Language::Chinese => "未完成任务:",
        }.to_string()
    }

    fn completed_tasks(&self) -> String {
        match self.lang {
            Language::English => "Completed Tasks:",
            Language::Japanese => "完了したタスク:",
            Language::Chinese => "已完成任务:",
        }.to_string()
    }

    fn none(&self) -> String {
        match self.lang {
            Language::English => "None",
            Language::Japanese => "なし",
            Language::Chinese => "无",
        }.to_string()
    }

    fn id(&self) -> String {
        match self.lang {
            Language::English => "ID",
            Language::Japanese => "ID",
            Language::Chinese => "ID",
        }.to_string()
    }

    fn description(&self) -> String {
        match self.lang {
            Language::English => "Description",
            Language::Japanese => "説明",
            Language::Chinese => "描述",
        }.to_string()
    }

    fn error(&self, msg: &str) -> String {
        match self.lang {
            Language::English => format!("Error: {}", msg),
            Language::Japanese => format!("エラー: {}", msg),
            Language::Chinese => format!("错误: {}", msg),
        }
    }

    fn content_updated(&self, id: &str) -> String {
        match self.lang {
            Language::English => format!("Content of task {} has been updated", id),
            Language::Japanese => format!("タスク {} の内容が更新されました", id),
            Language::Chinese => format!("已更新任务 {} 的内容", id),
        }
    }

    fn due_date_updated(&self, id: &str) -> String {
        match self.lang {
            Language::English => format!("Due date of task {} has been updated", id),
            Language::Japanese => format!("タスク {} の期限が更新されました", id),
            Language::Chinese => format!("已更新任务 {} 的截止日期", id),
        }
    }

    fn task_completed(&self, id: &str) -> String {
        match self.lang {
            Language::English => format!("Task {} marked as completed", id),
            Language::Japanese => format!("タスク {} が完了としてマークされました", id),
            Language::Chinese => format!("任务 {} 已标记为完成", id),
        }
    }

    fn task_uncompleted(&self, id: &str) -> String {
        match self.lang {
            Language::English => format!("Task {} marked as incomplete", id),
            Language::Japanese => format!("タスク {} が未完了としてマークされました", id),
            Language::Chinese => format!("任务 {} 已标记为未完成", id),
        }
    }

    fn task_starred(&self, id: &str) -> String {
        match self.lang {
            Language::English => format!("Task {} marked as important", id),
            Language::Japanese => format!("タスク {} が重要としてマークされました", id),
            Language::Chinese => format!("任务 {} 已标记为重要", id),
        }
    }

    fn task_unstarred(&self, id: &str) -> String {
        match self.lang {
            Language::English => format!("Task {} unmarked as important", id),
            Language::Japanese => format!("タスク {} の重要マークが解除されました", id),
            Language::Chinese => format!("任务 {} 已取消重要标记", id),
        }
    }

    fn task_deleted(&self, id: &str) -> String {
        match self.lang {
            Language::English => format!("Task {} deleted", id),
            Language::Japanese => format!("タスク {} が削除されました", id),
            Language::Chinese => format!("任务 {} 已删除", id),
        }
    }

    fn task_not_exist(&self) -> &'static str {
        match self.lang {
            Language::English => "Task does not exist",
            Language::Japanese => "タスクが存在しません",
            Language::Chinese => "任务不存在",
        }
    }

    fn task_already_completed(&self) -> &'static str {
        match self.lang {
            Language::English => "Task does not exist or is already completed",
            Language::Japanese => "タスクが存在しないか、すでに完了しています",
            Language::Chinese => "任务不存在或已完成",
        }
    }

    fn task_not_completed(&self) -> &'static str {
        match self.lang {
            Language::English => "Task does not exist or is not completed",
            Language::Japanese => "タスクが存在しないか、完了していません",
            Language::Chinese => "任务不存在或未完成",
        }
    }

    fn save_failed(&self) -> &'static str {
        match self.lang {
            Language::English => "Failed to save task",
            Language::Japanese => "タスクの保存に失敗しました",
            Language::Chinese => "保存任务失败",
        }
    }

    fn invalid_date_format(&self) -> &'static str {
        match self.lang {
            Language::English => "Invalid date format, please use YYYY-MM-DD format",
            Language::Japanese => "日付の形式が無効です。YYYY-MM-DD形式を使用してください",
            Language::Chinese => "日期格式错误，请使用 YYYY-MM-DD 格式",
        }
    }

    fn invalid_datetime(&self) -> &'static str {
        match self.lang {
            Language::English => "Invalid date and time",
            Language::Japanese => "無効な日付と時刻",
            Language::Chinese => "无效的日期时间",
        }
    }

    fn no_tasks(&self) -> String {
        match self.lang {
            Language::English => "No tasks".yellow().to_string(),
            Language::Japanese => "タスクはありません".yellow().to_string(),
            Language::Chinese => "没有任务".yellow().to_string(),
        }
    }

    fn task_details(&self) -> String {
        match self.lang {
            Language::English => "Task Details:".bold().to_string(),
            Language::Japanese => "タスクの詳細:".bold().to_string(),
            Language::Chinese => "任务详情:".bold().to_string(),
        }
    }

    fn status(&self) -> String {
        match self.lang {
            Language::English => "Status",
            Language::Japanese => "状態",
            Language::Chinese => "状态",
        }.to_string()
    }

    fn status_completed(&self) -> String {
        match self.lang {
            Language::English => "Completed".green().to_string(),
            Language::Japanese => "完了".green().to_string(),
            Language::Chinese => "已完成".green().to_string(),
        }
    }

    fn status_incomplete(&self) -> String {
        match self.lang {
            Language::English => "Incomplete".yellow().to_string(),
            Language::Japanese => "未完了".yellow().to_string(),
            Language::Chinese => "未完成".yellow().to_string(),
        }
    }

    fn starred(&self) -> String {
        match self.lang {
            Language::English => "Starred",
            Language::Japanese => "重要マーク",
            Language::Chinese => "重要标记",
        }.to_string()
    }

    fn yes(&self) -> String {
        match self.lang {
            Language::English => "Yes ★".yellow().to_string(),
            Language::Japanese => "はい ★".yellow().to_string(),
            Language::Chinese => "是 ★".yellow().to_string(),
        }
    }

    fn no(&self) -> String {
        match self.lang {
            Language::English => "No".normal().to_string(),
            Language::Japanese => "いいえ".normal().to_string(),
            Language::Chinese => "否".normal().to_string(),
        }
    }

    fn created_at(&self) -> String {
        match self.lang {
            Language::English => "Created at",
            Language::Japanese => "作成日時",
            Language::Chinese => "创建时间",
        }.to_string()
    }

    fn provide_content_or_date(&self) -> String {
        match self.lang {
            Language::English => "Please provide content or due date to modify",
            Language::Japanese => "変更するコンテンツまたは期限を提供してください",
            Language::Chinese => "请提供要修改的内容或截止日期",
        }.to_string()
    }

    fn help_title(&self) -> String {
        match self.lang {
            Language::English => "Jodo - A simple command-line Todo application".bold().to_string(),
            Language::Japanese => "Jodo - シンプルなコマンドラインTodoアプリケーション".bold().to_string(),
            Language::Chinese => "Jodo - 简单的命令行Todo应用".bold().to_string(),
        }
    }

    fn basic_usage(&self) -> String {
        match self.lang {
            Language::English => "Basic Usage:",
            Language::Japanese => "基本的な使い方:",
            Language::Chinese => "基本用法:",
        }.to_string()
    }

    fn task_management(&self) -> String {
        match self.lang {
            Language::English => "Task Management Commands:",
            Language::Japanese => "タスク管理コマンド:",
            Language::Chinese => "任务管理命令:",
        }.to_string()
    }

    fn note_completed_tasks(&self) -> String {
        match self.lang {
            Language::English => "Note: Completed tasks have a 'c' suffix, e.g. '1c'",
            Language::Japanese => "注意: 完了したタスクは'c'サフィックスが付きます。例: '1c'",
            Language::Chinese => "注意: 已完成的任务ID会在末尾显示'c'，例如 '1c'",
        }.to_string()
    }

    fn other_options(&self) -> String {
        match self.lang {
            Language::English => "Other Options:",
            Language::Japanese => "その他のオプション:",
            Language::Chinese => "其他选项:",
        }.to_string()
    }

    fn examples(&self) -> String {
        match self.lang {
            Language::English => "Examples:",
            Language::Japanese => "例:",
            Language::Chinese => "示例:",
        }.to_string()
    }

    fn version_info(&self) -> String {
        match self.lang {
            Language::English => "Jodo version",
            Language::Japanese => "Jodoバージョン",
            Language::Chinese => "Jodo 版本",
        }.to_string()
    }

    fn app_description(&self) -> String {
        match self.lang {
            Language::English => "A simple command-line Todo application",
            Language::Japanese => "シンプルなコマンドラインTodoアプリケーション",
            Language::Chinese => "一个简单的命令行Todo应用",
        }.to_string()
    }

    fn author(&self) -> String {
        match self.lang {
            Language::English => "Author",
            Language::Japanese => "作者",
            Language::Chinese => "作者",
        }.to_string()
    }

    fn init_failed(&self, e: &str) -> String {
        match self.lang {
            Language::English => format!("Failed to initialize task list: {}", e),
            Language::Japanese => format!("タスクリストの初期化に失敗しました: {}", e),
            Language::Chinese => format!("初始化任务列表失败: {}", e),
        }
    }

    fn due_status_legend(&self) -> String {
        match self.lang {
            Language::English => "Due Date Colors",
            Language::Japanese => "期限の色分け",
            Language::Chinese => "截止日期颜色说明",
        }.to_string()
    }
    
    fn overdue(&self) -> String {
        match self.lang {
            Language::English => "Overdue",
            Language::Japanese => "期限切れ",
            Language::Chinese => "已过期",
        }.to_string()
    }
    
    fn urgent(&self) -> String {
        match self.lang {
            Language::English => "Urgent (Today)",
            Language::Japanese => "緊急 (今日)",
            Language::Chinese => "紧急 (今天)",
        }.to_string()
    }
    
    fn soon(&self) -> String {
        match self.lang {
            Language::English => "Soon (Within 3 days)",
            Language::Japanese => "まもなく (3日以内)",
            Language::Chinese => "即将到期 (3天内)",
        }.to_string()
    }

    fn task_view(&self) -> String {
        match self.lang {
            Language::English => "Task View",
            Language::Japanese => "タスクビュー",
            Language::Chinese => "任务视图",
        }.to_string()
    }
    fn multi_mode_start(&self) -> String {
        match self.lang {
            Language::English => "Batch mode (type 'exit' to quit):",
            Language::Japanese => "バッチモード ('exit'で終了):",
            Language::Chinese => "批量添加模式 (输入 'exit' 结束):",
        }.to_string()
    }

    fn multi_mode_prompt(&self) -> String {
        "jodo$> ".to_string()
    }

    fn tasks_completed(&self, ids: &[String]) -> String {
        let id_list = ids.join(", ");
        match self.lang {
            Language::English => format!("Tasks {} marked as completed", id_list),
            Language::Japanese => format!("タスク {} が完了としてマークされました", id_list),
            Language::Chinese => format!("任务 {} 已标记为完成", id_list),
        }
    }
    fn add_task(&self) -> String {
        match self.lang {
            Language::English => "Add new task",
            Language::Japanese => "新しいタスクを追加",
            Language::Chinese => "添加新任务",
        }.to_string()
    }

    fn add_task_with_date(&self) -> String {
        match self.lang {
            Language::English => "Add task with due date",
            Language::Japanese => "期限付きでタスクを追加",
            Language::Chinese => "添加带截止日期的任务",
        }.to_string()
    }

    fn list_tasks_cmd(&self) -> String {
        match self.lang {
            Language::English => "List all tasks",
            Language::Japanese => "すべてのタスクを表示",
            Language::Chinese => "列出所有任务",
        }.to_string()
    }

    fn edit_task_content(&self) -> String {
        match self.lang {
            Language::English => "Edit task content",
            Language::Japanese => "タスク内容を編集",
            Language::Chinese => "编辑任务内容",
        }.to_string()
    }

    fn edit_task_due_date(&self) -> String {
        match self.lang {
            Language::English => "Edit task due date",
            Language::Japanese => "タスクの期限を編集",
            Language::Chinese => "修改任务截止日期",
        }.to_string()
    }

    fn mark_task_completed(&self) -> String {
        match self.lang {
            Language::English => "Mark task as completed",
            Language::Japanese => "タスクを完了としてマーク",
            Language::Chinese => "标记任务为已完成",
        }.to_string()
    }

    fn mark_task_incomplete(&self) -> String {
        match self.lang {
            Language::English => "Mark task as incomplete",
            Language::Japanese => "タスクを未完了としてマーク",
            Language::Chinese => "取消任务完成标记",
        }.to_string()
    }

    fn star_task_cmd(&self) -> String {
        match self.lang {
            Language::English => "Mark task as important (pin to top)",
            Language::Japanese => "タスクを重要としてマーク（トップに固定）",
            Language::Chinese => "标记任务为重要（置顶）",
        }.to_string()
    }

    fn unstar_task_cmd(&self) -> String {
        match self.lang {
            Language::English => "Unmark task as important",
            Language::Japanese => "タスクの重要マークを解除",
            Language::Chinese => "取消任务的重要标记",
        }.to_string()
    }

    fn delete_task(&self) -> String {
        match self.lang {
            Language::English => "Delete task",
            Language::Japanese => "タスクを削除",
            Language::Chinese => "删除任务",
        }.to_string()
    }

    fn show_task_details(&self) -> String {
        match self.lang {
            Language::English => "Show task details",
            Language::Japanese => "タスクの詳細を表示",
            Language::Chinese => "显示任务的详细信息",
        }.to_string()
    }

    fn batch_mode(&self) -> String {
        match self.lang {
            Language::English => "Enter batch mode to add multiple tasks",
            Language::Japanese => "バッチモードで複数のタスクを追加",
            Language::Chinese => "进入批量添加任务模式",
        }.to_string()
    }

    fn set_language(&self) -> String {
        match self.lang {
            Language::English => "Set language (zh-cn/en/ja)",
            Language::Japanese => "言語を設定 (zh-cn/en/ja)",
            Language::Chinese => "设置语言 (zh-cn/en/ja)",
        }.to_string()
    }

    fn show_help_info(&self) -> String {
        match self.lang {
            Language::English => "Show help information",
            Language::Japanese => "ヘルプ情報を表示",
            Language::Chinese => "显示帮助信息",
        }.to_string()
    }

    fn show_version_info(&self) -> String {
        match self.lang {
            Language::English => "Show version information",
            Language::Japanese => "バージョン情報を表示",
            Language::Chinese => "显示版本信息",
        }.to_string()
    }

    fn views(&self) -> String {
        match self.lang {
            Language::English => "Views",
            Language::Japanese => "ビュー",
            Language::Chinese => "视图",
        }.to_string()
    }
    fn cmd_error(&self, msg: &str) -> String {
        match self.lang {
            Language::English => format!("Error: {}", msg),
            Language::Japanese => format!("エラー: {}", msg),
            Language::Chinese => format!("错误: {}", msg),
        }
    }

    fn tasks_deleted(&self, ids: &[usize]) -> String {
        let id_list = ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
            
        match self.lang {
            Language::English => format!("Tasks {} deleted", id_list),
            Language::Japanese => format!("タスク {} が削除されました", id_list),
            Language::Chinese => format!("任务 {} 已删除", id_list),
        }
    }
    fn example_task(&self) -> String {
        match self.lang {
            Language::English => "Complete project report",
            Language::Japanese => "プロジェクトレポートを完成させる",
            Language::Chinese => "完成项目报告",
        }.to_string()
    }

    fn example_edit_task(&self) -> String {
        match self.lang {
            Language::English => "Updated task content",
            Language::Japanese => "更新されたタスク内容",
            Language::Chinese => "已修改的任务内容",
        }.to_string()
    }
    fn language_changed(&self, lang_name: &str) -> String {
        match self.lang {
            Language::English => format!("Language switched to {}", lang_name),
            Language::Japanese => format!("言語が{}に切り替えられました", lang_name),
            Language::Chinese => format!("语言已切换为{}", lang_name),
        }
    }

    fn tasks_uncompleted(&self, ids: &[String]) -> String {
        let id_list = ids.join(", ");
        match self.lang {
            Language::English => format!("Tasks {} marked as incomplete", id_list),
            Language::Japanese => format!("タスク {} が未完了としてマークされました", id_list),
            Language::Chinese => format!("任务 {} 已标记为未完成", id_list),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    language: Language,
}

impl Config {
    fn new() -> Self {
        Self {
            language: Language::Chinese,
        }
    }
    
    fn load() -> Self {
        let config_path = get_config_path();
        
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(contents) => {
                    match serde_json::from_str(&contents) {
                        Ok(config) => return config,
                        Err(_) => return Config::new(),
                    }
                },
                Err(_) => return Config::new(),
            }
        }
        
        Config::new()
    }
    
    fn save(&self) -> Result<(), io::Error> {
        let config_path = get_config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let contents = serde_json::to_string_pretty(self)?;
        let mut file = File::create(&config_path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}

// 获取配置文件路径
fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_default();
    path.push(".jodo");
    path.push("config.json");
    path
}

// 全局语言设置
static mut CURRENT_LANGUAGE: Language = Language::Chinese;

fn get_translations() -> Translations {
    unsafe {
        Translations::new(CURRENT_LANGUAGE)
    }
}
fn build_cli() -> Cli {
    Cli {
        task: None,
        due_date: None,
        language: None,
        help: false,
        version: false,
        edit_id: None,
        edit_content_arg: None,
        edit_content: None,
        complete_ids: Vec::new(),
        undo_id: None,
        star_id: None,
        unstar_id: None,
        delete_ids: Vec::new(),
        list: false,
        show_id: None,
        multi_mode: false,
        command: None,
    }
}

#[derive(Parser)]
#[clap(name = "jodo")]
#[clap(about = "", long_about = None)]
#[clap(version)]
struct Cli {
    /// 任务内容 (直接添加任务)
    #[clap(value_name = "CONTENT")]
    task: Option<String>,

    /// 设置任务截止日期 (格式: YYYY-MM-DD)
    #[clap(short = 't', long = "time")]
    due_date: Option<String>,

    /// 设置语言 (zh-cn: 中文, en: 英文, ja: 日语)
    #[clap(short = 'L', long = "language")]
    language: Option<String>,

    /// 显示详细的帮助信息
    #[clap(short = 'h', long = "help", conflicts_with = "task")]
    help: bool,

    /// 显示版本信息
    #[clap(short = 'v', long = "version")]
    version: bool,

    /// 编辑任务内容
    #[clap(short = 'e', long = "edit", value_name = "ID")]
    edit_id: Option<String>,

    /// 编辑任务时的新内容 (可以直接作为命令行参数)
    #[clap(value_name = "CONTENT")]
    edit_content_arg: Option<String>,

    /// 编辑任务时的新内容 (通过--content参数)
    #[clap(long = "content", requires = "edit_id")]
    edit_content: Option<String>,

    /// 标记任务为完成
    #[clap(short = 'c', long = "complete", num_args = 1.., value_name = "ID", conflicts_with_all = &["edit_id", "task"])]
    complete_ids: Vec<String>,

    /// 标记任务为未完成
    #[clap(short = 'u', long = "undo", value_name = "ID", conflicts_with_all = &["complete_id", "edit_id", "task"])]
    undo_id: Option<String>,

    /// 将任务标记为重要（置顶）
    #[clap(long = "star", value_name = "ID", conflicts_with_all = &["complete_id", "undo_id", "edit_id", "task"])]
    star_id: Option<String>,

    /// 取消任务的重要标记
    #[clap(long = "unstar", value_name = "ID", conflicts_with_all = &["star_id", "complete_id", "undo_id", "edit_id", "task"])]
    unstar_id: Option<String>,

    /// 删除任务 (可多个ID，空格分隔)
    #[clap(short = 'd', long = "delete", num_args = 1.., value_name = "ID", conflicts_with_all = &["edit_id", "complete_id", "undo_id", "star_id", "unstar_id", "task"])]
    delete_ids: Vec<String>,

    /// 列出所有任务
    #[clap(short = 'l', long = "list", conflicts_with_all = &["edit_id", "complete_id", "delete_ids", "task"])]
    list: bool,

    /// 显示任务的详细信息
    #[clap(long = "show", value_name = "ID", conflicts_with_all = &["edit_id", "complete_id", "undo_id", "star_id", "unstar_id", "delete_ids", "task"])]
    show_id: Option<String>,

    /// 启用批量添加任务模式
    #[clap(short = 'm', long = "multi", conflicts_with_all = &["edit_id", "complete_id", "undo_id", "star_id", "unstar_id", "delete_ids", "task", "show_id"])]
    multi_mode: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 列出所有任务
    List,
    /// 完成一个任务
    Done {
        /// 任务ID
        id: String,
    },
    /// 将完成的任务标记为未完成
    Undo {
        /// 任务ID
        id: String,
    },
    /// 删除一个任务
    Remove {
        /// 任务ID
        id: String,
    },
    /// 编辑一个任务
    Edit {
        /// 任务ID
        id: String,
        /// 新的任务内容
        #[clap(required_unless_present = "time")]
        content: Option<String>,
        /// 设置任务截止日期 (格式: YYYY-MM-DD)
        #[clap(short = 't', long = "time")]
        time: Option<String>,
    },
    /// 标记任务为重要（置顶）
    Star {
        /// 任务ID
        id: String,
    },
    /// 取消任务重要标记
    Unstar {
        /// 任务ID
        id: String,
    },
    /// 显示任务的详细信息
    Show {
        /// 任务ID
        id: String,
    },
    /// 显示帮助信息
    Help,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    created_at: DateTime<Local>,
    due_date: Option<DateTime<Local>>,
    starred: bool,    
    deleted: bool,    
}
#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
    file_path: PathBuf,   
    next_id: usize,      
}

// 辅助函数：解析任务ID（支持后缀'c'表示已完成任务）
fn parse_task_id(id_str: &str) -> (usize, bool) {
    if id_str.ends_with('c') {
        if let Ok(id) = id_str[0..id_str.len()-1].parse::<usize>() {
            return (id, true); // 返回ID和"是否完成"标志       
        }
    }

    if let Ok(id) = id_str.parse::<usize>() {
        return (id, false);        
    }

    (0, false) // 解析失败    
}

// 截断字符串，同时考虑显示宽度
fn truncate_str(s: &str, max_width: usize) -> String {
    let mut width = 0;
    let mut end_idx = 0;
        
    for (idx, c) in s.char_indices() {
        let char_width = if c.is_ascii() { 1 } else { 2 };
        
        if width + char_width > max_width {
            break;    
        }
        
        width += char_width;   
        end_idx = idx + c.len_utf8();
    }
        
    if end_idx < s.len() {
        format!("{}...", &s[..end_idx])
    } else { 
        s.to_string()       
    }    
}

// 计算字符串的显示宽度，考虑中文字符占两个宽度
fn display_width(s: &str) -> usize {
    s.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum()    
}

// 创建固定显示宽度的字符串
fn fixed_width_string(s: &str, width: usize) -> String {
    let actual_width = display_width(s);
    if actual_width >= width {
        s.to_string()
    } else {
        // 添加空格以达到固定宽度   
        format!("{}{}", s, " ".repeat(width - actual_width))       
    }    
}

// 截止日期临近程度的枚举
enum DueStatus {
    Overdue,    // 已过期
    Urgent,     // 紧急 (1天内) 
    Soon,       // 即将到期 (3天内)
    Normal,     // 正常   
    NoDue,      // 无截止日期    
}

// 判断截止日期的临近程度
fn get_due_status(due_date: Option<&DateTime<Local>>) -> DueStatus {
    if let Some(date) = due_date {
        let now = Local::now();
        let days_remaining = (*date - now).num_days();
        
        if days_remaining < 0 {
            DueStatus::Overdue
        } else if days_remaining == 0 {
            DueStatus::Urgent
        } else if days_remaining <= 3 {
            DueStatus::Soon
        } else { 
            DueStatus::Normal
        }
    } else { 
        DueStatus::NoDue       
    }    
}

// 根据截止日期状态返回颜色化的日期字符串
fn format_due_date(date: Option<&DateTime<Local>>, t: &Translations) -> ColoredString {
    match date {
        Some(date) => {
            let formatted = date.format("%Y-%m-%d").to_string();
            match get_due_status(Some(date)) {
                DueStatus::Overdue => formatted.red().bold(),
                DueStatus::Urgent => formatted.bright_red(),
                DueStatus::Soon => formatted.yellow(),
                DueStatus::Normal => formatted.normal(),   
                DueStatus::NoDue => t.none().normal(),      
            }
        },   
        None => t.none().normal()       
    }    
}

impl TodoList {
    fn new() -> Result<Self, io::Error> {
        let mut file_path = dirs::home_dir().unwrap_or_default();
        file_path.push(".jodo");
        file_path.push("tasks.json");
        
        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;    
        }
        
        let tasks: Vec<Task> = if file_path.exists() {
            let mut file = File::open(&file_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Vec::new()        
        };
    
        // 创建TodoList实例
        let mut todo_list = Self { 
            tasks,
            file_path,  
            next_id: 1,  // 临时值，会在reassign_ids中更新        
        };
    
        // 程序启动时就重新分配ID，确保任务总是从1开始连续编号
        todo_list.reassign_ids();    
        todo_list.save()?; 
    
        Ok(todo_list)        
    }
    fn add_task(&mut self, description: String, due_date: Option<DateTime<Local>>) -> Result<(), io::Error> {
        let task = Task {
            id: self.next_id, // 临时ID 
            description,
            completed: false,
            created_at: Local::now(),
            due_date,
            starred: false,  
            deleted: false,        
        };
    
        self.tasks.push(task);
        self.reassign_ids(); // 重新分配所有ID   
        self.save()        
    }

    // 列出任务
    fn list_tasks(&self) {
        let t = get_translations();
    
        // 过滤出未删除的任务    
        let filtered_tasks: Vec<&Task> = self.tasks.iter().filter(|task| !task.deleted).collect();
    
        if filtered_tasks.is_empty() {
            println!("{}", t.no_tasks());   
            return;        
        }
    
        // 按照完成状态、星标和ID排序
        let mut incomplete_tasks: Vec<&Task> = filtered_tasks.iter()
            .filter(|t| !t.completed) 
            .cloned()        
            .collect();
    
        let mut completed_tasks: Vec<&Task> = filtered_tasks.iter()
            .filter(|t| t.completed) 
            .cloned()        
            .collect();
    
        // 排序逻辑：首先按照星标排序，然后严格按照ID数值排序
        incomplete_tasks.sort_by(|a, b| {
            match (a.starred, b.starred) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,   
                _ => a.id.cmp(&b.id)  // 按ID升序排序     
            }        
        });
    
        // 已完成任务严格按ID排序    
        completed_tasks.sort_by_key(|t| t.id);
    
        println!("{}",t.incomplete_tasks());
        if incomplete_tasks.is_empty() {
            println!("  {}", t.none().italic());
        } else {
            println!("{:<5} {:<40} {:<15}", t.id(), t.description(), t.due_date());        
            println!("{}", "-".repeat(70));
    
            for task in &incomplete_tasks {
                let star_marker = if task.starred { "★ ".yellow() } else { "  ".into() };
                // 使用新的格式化函数显示彩色的截止日期
                let due_date = format_due_date(task.due_date.as_ref(), &t);
                
                // 根据可用空间计算描述的最大长度，考虑中文字符
                let max_desc_width = 36; // 留4个字符的余量
                let truncated_desc = truncate_str(&task.description, max_desc_width);            
                let formatted_desc = fixed_width_string(&truncated_desc, 40);
    
                println!(
                    "{}{:<5} {} {:<15}",
                    star_marker,
                    task.id.to_string().blue(),
                    formatted_desc,  
                    due_date       
                );       
            }        
        }
    
        println!("\n{}",t.completed_tasks());
        if completed_tasks.is_empty() {
            println!("  {}", t.none().italic());
        } else {
            println!("{:<5} {:<40} {:<15}", t.id(), t.description(), t.due_date());        
            println!("{}", "-".repeat(70));
    
            for task in completed_tasks.iter() {
                let task_id = format!("{}c", task.id);
                
                // 已完成任务的截止日期不需要特殊颜色标记
                let due_date = match task.due_date {
                    Some(date) => date.format("%Y-%m-%d").to_string().normal(),  
                    None => t.none().normal(),    
                };
                
                // 对已完成任务也处理中文显示问题
                let max_desc_width = 36;
                let truncated_desc = truncate_str(&task.description, max_desc_width);
                let formatted_desc = fixed_width_string(&truncated_desc, 40);
                
                println!(
                    "{:<5} {} {:<15}",
                    task_id.green(),
                    formatted_desc,  
                    due_date       
                );       
            }       
        }        
    }

    // 修改edit_task方法，移除group参数
    fn edit_task(&mut self, id_str: &str, new_desc: Option<&str>, due_date: Option<DateTime<Local>>) -> Result<(), &'static str> {
        let (id, is_completed) = parse_task_id(id_str);
    
        if let Some(task) = self.tasks.iter_mut()
            .filter(|t| !t.deleted && t.completed == is_completed && t.id == id)        
            .next() {
    
            if let Some(desc) = new_desc {
                task.description = desc.to_string();    
            }
            
            if due_date.is_some() {
                task.due_date = due_date;    
            }
            
            self.save().map_err(|_| "保存任务失败")?;
            Ok(())
        } else { 
            Err("任务不存在")       
        }        
    }

    fn mark_done(&mut self, id_str: &str) -> Result<(), &'static str> {
        let t = get_translations();
        let (id, _) = parse_task_id(id_str);
        
        if let Some(task) = self.tasks.iter_mut()
            .find(|t| !t.deleted && !t.completed && t.id == id) {
            task.completed = true;
            self.reassign_ids(); // 重新分配ID
            self.save().map_err(|_| t.save_failed())?;
            Ok(())
        } else { 
            Err(t.task_already_completed())       
        }    
    }
    
    fn mark_undone(&mut self, id_str: &str) -> Result<(), &'static str> {
        let t = get_translations();
        let (id, _) = parse_task_id(id_str);
        
        if let Some(task) = self.tasks.iter_mut()
            .find(|t| !t.deleted && t.completed && t.id == id) {
            task.completed = false;
            self.reassign_ids(); // 重新分配ID
            self.save().map_err(|_| t.save_failed())?;
            Ok(())
        } else { 
            Err(t.task_not_completed())       
        }    
    }
    
    fn star_task(&mut self, id_str: &str) -> Result<(), &'static str> {
        let (id, is_completed) = parse_task_id(id_str);
        
        if let Some(task) = self.tasks.iter_mut()
            .find(|t| !t.deleted && t.completed == is_completed && t.id == id) {
            task.starred = true;
            self.reassign_ids(); // 重新分配ID
            self.save().map_err(|_| "保存任务失败")?;
            Ok(())
        } else { 
            Err("任务不存在")       
        }    
    }
    
    fn unstar_task(&mut self, id_str: &str) -> Result<(), &'static str> {
        let (id, is_completed) = parse_task_id(id_str);
        
        if let Some(task) = self.tasks.iter_mut()
            .find(|t| !t.deleted && t.completed == is_completed && t.id == id) {
            task.starred = false;
            self.reassign_ids(); // 重新分配ID
            self.save().map_err(|_| "保存任务失败")?;
            Ok(())
        } else { 
            Err("任务不存在")       
        }    
    }
    
    fn remove_tasks(&mut self, id_strs: &[String]) -> Result<Vec<usize>, &'static str> {
        // 首先将任务ID映射到内部任务索引，避免中途ID变化
        let mut task_indices_to_delete: Vec<(usize, usize, bool)> = Vec::new();  // (内部索引, 显示ID, 是否完成)    
        let mut display_ids = Vec::new();  // 用于显示的任务ID 
    
        // 第一步：收集所有要删除的任务信息
        for id_str in id_strs {    
            let (display_id, is_completed) = parse_task_id(id_str);
    
            // 找到对应的任务在tasks数组中的实际索引位置
            if let Some((index, task)) = self.tasks.iter()
                .enumerate()
                .find(|(_, t)| !t.deleted && t.completed == is_completed && t.id == display_id) {
                
                task_indices_to_delete.push((index, task.id, is_completed));
                display_ids.push(task.id);
            } else { 
                return Err("任务不存在");       
            }        
        }
    
        // 第二步：按索引标记删除，避免重排序导致的问题
        for (index, _, _) in &task_indices_to_delete {
            self.tasks[*index].deleted = true;        
        }
    
        // 保存更改    
        self.save().map_err(|_| "保存任务失败")?;
    
        // 重新分配ID
        self.reassign_ids();    
        self.save().map_err(|_| "保存任务失败")?;
    
        Ok(display_ids)    
    }
    
    // 修改原来的remove_task方法，使用新的批量删除方法
    fn remove_task(&mut self, id_str: &str) -> Result<(), &'static str> {
        self.remove_tasks(&[id_str.to_string()])?;   
        Ok(())        
    }

    fn show_task_detail(&self, id_str: &str) -> Result<(), &'static str> {
        let t = get_translations();    
        let (id, is_completed) = parse_task_id(id_str);
    
        if let Some(task) = self.tasks.iter()
            .find(|t| !t.deleted && t.completed == is_completed && t.id == id) {
            
            println!("{}", t.task_details());
            println!("{}", "=".repeat(50));
            println!("{:<10}: {}", t.id(), if is_completed { 
                format!("{}c", task.id).green()
            } else {
                task.id.to_string().blue()
            });
            println!("{:<10}: {}", t.status(), if task.completed { 
                t.status_completed()
            } else { 
                t.status_incomplete()
            });
            println!("{:<10}: {}", t.starred(), if task.starred {
                t.yes()
            } else {
                t.no()
            });
            println!("{:<10}: {}", t.created_at(), task.created_at.format("%Y-%m-%d %H:%M:%S"));
    
            // 在详细信息中也显示彩色截止日期
            let due_date_str = match &task.due_date {
                Some(date) => {
                    if task.completed {
                        date.format("%Y-%m-%d").to_string().normal()
                    } else { 
                        format_due_date(Some(date), &t)      
                    }
                },  
                None => t.none().normal(),
            };
            println!("{:<10}: {}", t.due_date(), due_date_str);
            println!("{}", "-".repeat(50));
            println!("{:<10}: ", t.description());
            println!("{}", task.description);
            println!("{}", "=".repeat(50));
            
            Ok(())
        } else { 
            Err(t.task_not_exist())       
        }        
    }

    fn save(&self) -> Result<(), io::Error> {
        let contents = serde_json::to_string_pretty(&self.tasks)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(contents.as_bytes())?;   
        Ok(())        
    }

    #[allow(dead_code)]
    fn get_task(&self, id_str: &str) -> Option<&Task> {
        let (id, is_completed) = parse_task_id(id_str);
    
        self.tasks.iter()   
            .find(|t| !t.deleted && t.completed == is_completed && t.id == id)
    }
    fn reassign_ids(&mut self) {
        let mut next_id = 1;
        // 先处理未完成的任务
        let mut incomplete_tasks: Vec<&mut Task> = self.tasks.iter_mut()
            .filter(|task| !task.deleted && !task.completed)        
            .collect();
    
        // 排序规则（星标优先，然后按原ID）
        incomplete_tasks.sort_by(|a, b| {
            match (a.starred, b.starred) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,   
                _ => a.id.cmp(&b.id)     
            }        
        });
    
        for task in incomplete_tasks {
            task.id = next_id;   
            next_id += 1;        
        }
    
        // 再处理已完成的任务
        let mut completed_tasks: Vec<&mut Task> = self.tasks.iter_mut()
            .filter(|task| !task.deleted && task.completed)        
            .collect();
    
        // 严格按照原ID排序
        completed_tasks.sort_by_key(|t| t.id);
                
        for task in completed_tasks {
            task.id = next_id;   
            next_id += 1;        
        }
    
        // 更新下一个可用ID   
        self.next_id = next_id;        
    }

    fn mark_done_multiple(&mut self, id_strs: &[String]) -> Result<Vec<String>, &'static str> {
        let t = get_translations();    
        let mut completed_ids = Vec::new(); 
    
        for id_str in id_strs {    
            let (id, _) = parse_task_id(id_str);
    
            if let Some(task) = self.tasks.iter_mut()
                .find(|t| !t.deleted && !t.completed && t.id == id) {
                task.completed = true;   
                completed_ids.push(id_str.clone());       
            }        
        }
    
        if completed_ids.is_empty() {
            return Err(t.task_already_completed());
        }
            
        self.reassign_ids(); // 重新分配ID    
        self.save().map_err(|_| t.save_failed())?;
    
        Ok(completed_ids)       
    }    

    fn mark_undone_multiple(&mut self, id_strs: &[String]) -> Result<Vec<String>, &'static str> {
        let t = get_translations();    
        let mut uncompleted_ids = Vec::new(); 
    
        for id_str in id_strs {    
            let (id, is_completed) = parse_task_id(id_str);
            
            // 只处理已完成的任务
            if !is_completed && id > 0 {
                continue;
            }
    
            if let Some(task) = self.tasks.iter_mut()
                .find(|t| !t.deleted && t.completed && t.id == id) {
                task.completed = false;   
                uncompleted_ids.push(id_str.clone());       
            }        
        }
    
        if uncompleted_ids.is_empty() {
            return Err(t.task_not_completed());
        }
            
        self.reassign_ids(); // 重新分配ID    
        self.save().map_err(|_| t.save_failed())?;
    
        Ok(uncompleted_ids)       
    }

    // 方法：批量编辑任务的截止日期
    fn edit_tasks_due_date(&mut self, id_strs: &[String], due_date: DateTime<Local>) -> Result<Vec<String>, &'static str> {
        let t = get_translations();
        let mut updated_ids = Vec::new();

        for id_str in id_strs {
            let (id, is_completed) = parse_task_id(id_str);
            
            if let Some(task) = self.tasks.iter_mut()
                .find(|t| !t.deleted && t.completed == is_completed && t.id == id) {
                task.due_date = Some(due_date);
                updated_ids.push(id_str.clone());
            }
        }

        if updated_ids.is_empty() {
            return Err(t.task_not_exist());
        }

        self.save().map_err(|_| t.save_failed())?;
        Ok(updated_ids)
    }
}
fn parse_date(date_str: &str) -> Result<DateTime<Local>, &'static str> {
    let t = get_translations();
    let now = Local::now();
    
    // 处理特殊关键字
    match date_str.to_lowercase().as_str() {
        "" | "today" | "td" => {
            // 今天，使用当前日期的00:00:00
            let today = now.date_naive().and_hms_opt(0, 0, 0)
                .ok_or(t.invalid_datetime())?;
            return Ok(DateTime::from_naive_utc_and_offset(today, now.offset().clone()));
        },
        "tomorrow" | "tm" => {
            // 明天，使用当前日期+1天的00:00:00
            let tomorrow = (now + chrono::Duration::days(1)).date_naive()
                .and_hms_opt(0, 0, 0).ok_or(t.invalid_datetime())?;
            return Ok(DateTime::from_naive_utc_and_offset(tomorrow, now.offset().clone()));
        },
        _ => {}
    }
    
    // 尝试解析不同格式的日期
    
    // 1. 只有一个数字，解释为当月的某一天
    if let Ok(day) = date_str.parse::<u32>() {
        if day >= 1 && day <= 31 {
            let year = now.year();
            let month = now.month();
            
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let datetime = date.and_hms_opt(0, 0, 0).ok_or(t.invalid_datetime())?;
                return Ok(DateTime::from_naive_utc_and_offset(datetime, now.offset().clone()));
            } else {
                return Err(t.invalid_date_format());
            }
        }
    }
    
    // 2. MM-DD格式，解释为当年的某月某日
    if let Some(pos) = date_str.find('-') {
        let (month_str, day_str) = date_str.split_at(pos);
        let day_str = &day_str[1..]; // 去掉'-'
        
        if let (Ok(month), Ok(day)) = (month_str.parse::<u32>(), day_str.parse::<u32>()) {
            if month >= 1 && month <= 12 && day >= 1 && day <= 31 {
                let year = now.year();
                
                if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                    let datetime = date.and_hms_opt(0, 0, 0).ok_or(t.invalid_datetime())?;
                    return Ok(DateTime::from_naive_utc_and_offset(datetime, now.offset().clone()));
                }
            }
        }
    }
    
    // 3. 标准YYYY-MM-DD格式
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| t.invalid_date_format())?;
    
    // 修改为00:00:00而不是23:59:59，避免时区转换导致日期偏移
    let naive_datetime = naive_date.and_hms_opt(0, 0, 0)
        .ok_or(t.invalid_datetime())?;
       
    Ok(DateTime::from_naive_utc_and_offset(naive_datetime, Local::now().offset().clone()))
}

// 添加日期范围解析函数 - 修改为解析日期序列而不是直接生成日期
fn parse_date_range(range_str: &str) -> Option<Vec<DateTime<Local>>> {
    // 检查是否包含"to"关键字
    if !range_str.contains("to") {
        return None;
    }
    
    let parts: Vec<&str> = range_str.split("to").collect();
    if parts.len() != 2 {
        return None;
    }
    
    let start_str = parts[0].trim();
    let end_str = parts[1].trim();
    
    // 尝试解析开始和结束日期
    let start_date = match parse_date(start_str) {
        Ok(date) => date,
        Err(_) => return None,
    };
    
    let end_date = match parse_date(end_str) {
        Ok(date) => date,
        Err(_) => return None,
    };
    
    // 收集日期范围内的所有日期
    let mut dates = Vec::new();
    // 修复警告：移除未使用的可变变量
    let start_date = start_date;
    
    // 处理跨年或月份大小问题
    // 如果结束日期的月份小于开始日期的月份，可能是跨年的情况
    if end_date.month() < start_date.month() {
        // 假设是下一年，这仅适用于跨年情况
        let next_year = start_date.year() + 1;
        let next_year_date = DateTime::<Local>::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(next_year, end_date.month(), end_date.day())
                .unwrap_or_else(|| NaiveDate::from_ymd_opt(next_year, end_date.month(), 28).unwrap())
                .and_hms_opt(0, 0, 0).unwrap(),
            Local::now().offset().clone()
        );
        dates.push(start_date);
        dates.push(next_year_date);
        return Some(dates);
    }
    
    // 正常情况：开始日期不大于结束日期
    if start_date <= end_date {
        // 添加开始和结束日期
        dates.push(start_date);
        if start_date != end_date {
            dates.push(end_date);
        }
        return Some(dates);
    }
    
    // 无效范围
    return None;
}

fn show_help() {
    let t = get_translations();
    println!("{}", t.help_title());
    println!("{}", "=======================".bold());
    // 基本视图概念说明
    println!("\n【{}】", t.views());
    println!("  jodo               {}", t.task_view());
    // 基本用法
    println!("\n{}", t.basic_usage());
    println!("  jodo \"{}\"              {}", t.example_task(), t.add_task());
    println!("  jodo \"{}\" -t 2023-12-31      {}", t.example_task(), t.add_task_with_date());
    println!("  jodo -l                    {}", t.list_tasks_cmd()); 
    // 任务管理命令
    println!("\n{}", t.task_management());
    println!("  jodo -e 1 \"{}\"           {}", t.example_edit_task(), t.edit_task_content());
    println!("  jodo -e 1 -t 2023-12-25          {}", t.edit_task_due_date());
    println!("  jodo -c 1                  {}", t.mark_task_completed());
    println!("  jodo -u 1                  {}", t.mark_task_incomplete());
    println!("  jodo --star 1              {}", t.star_task_cmd());
    println!("  jodo --unstar 1            {}", t.unstar_task_cmd());
    println!("  jodo -d 1                  {}", t.delete_task());
    println!("  jodo --show 1              {}", t.show_task_details());
    println!("  jodo -m                     {}", t.batch_mode());    
    // 其他选项
    println!("\n{}", t.other_options());
    println!("  jodo -L <lang>             {}", t.set_language());
    println!("  jodo -h, --help              {}", t.show_help_info());
    println!("  jodo -v, --version           {}", t.show_version_info());
    // 注意事项
    println!("\n{}", t.note_completed_tasks());
    // 示例
    println!("\n{}", t.examples());
    println!("  jodo \"{}\"", t.example_task());
    println!("  jodo \"{}\" -t 2023-12-31", t.example_task()); 
    println!("  jodo -e 1 \"{}\" -t 2023-12-25", t.example_edit_task()); 
    // 截止日期颜色图例
    println!("\n{}", t.due_status_legend());
    println!("  {} - {}", t.overdue().red().bold(), t.overdue());
    println!("  {} - {}", t.urgent().bright_red(), t.urgent());   
    println!("  {} - {}", t.soon().yellow(), t.soon());
}

fn show_version() {
    let t = get_translations();
    println!("{} {}", t.version_info(), env!("CARGO_PKG_VERSION"));
    println!("{}", t.app_description());   
    println!("{}: {}", t.author(), env!("CARGO_PKG_AUTHORS"));    
    
    // ASCII图，当前版本是牛牛
    println!("\n         (_____)    ");
    println!("          (o o)    ");
    println!("   /------\\oo/     ");
    println!("  / |    ||      ");
    println!(" *  /\\---/\\    ");
    println!("    ~~   ~~      ");
}
fn parse_id_range(id_str: &str) -> Vec<String> {
    // 检查是否包含范围分隔符"-"或"to"
    if id_str.contains('-') || id_str.contains("to") {
        // 确定使用的分隔符和分割点
        let (start_str, end_str) = if id_str.contains('-') {
            id_str.split_once('-').unwrap()
        } else {
            id_str.split_once("to").unwrap()
        };
        
        // 检查是否两端都有"c"后缀，确保一致性
        let start_has_c = start_str.ends_with('c');
        let end_has_c = end_str.ends_with('c');
        
        // 如果后缀不一致，当作单个ID处理
        if start_has_c != end_has_c {
            return vec![id_str.to_string()];
        }
        
        // 解析开始和结束的ID数字
        let start_num = if start_has_c {
            start_str[0..start_str.len()-1].parse::<usize>()
        } else {
            start_str.parse::<usize>()
        };
        
        let end_num = if end_has_c {
            end_str[0..end_str.len()-1].parse::<usize>()
        } else {
            end_str.parse::<usize>()
        };
        
        // 如果解析成功并且范围有效，生成ID序列
        if let (Ok(start), Ok(end)) = (start_num, end_num) {
            if start <= end && end - start <= 100 { // 限制范围大小，防止滥用
                let mut ids = Vec::new();
                for id in start..=end {
                    if start_has_c {
                        ids.push(format!("{}c", id));
                    } else {
                        ids.push(id.to_string());
                    }
                }
                return ids;
            }
        }
    }
    
    // 如果不是有效范围，当作单个ID处理
    vec![id_str.to_string()]
}

// 扩展ID解析，支持范围
fn expand_id_ranges(id_strs: &[String]) -> Vec<String> {
    let mut expanded_ids = Vec::new();
    
    for id_str in id_strs {
        expanded_ids.extend(parse_id_range(id_str));
    }
    
    expanded_ids
}

fn main() {
    // 首先加载配置
    let mut config = Config::load();
    
    // 从配置中设置当前语言
    unsafe {
        CURRENT_LANGUAGE = config.language;
    }
    
    // 特殊处理语言切换: 检查是否只有语言切换参数
    // 如果是，则更新配置并显示成功消息
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 && (args[1] == "-L" || args[1] == "--language") {
        let lang_str = &args[2];
        let new_language = Language::from_str(lang_str);
        
        // 获取语言名称，用于显示切换提示
        let lang_display_name = match new_language {
            Language::English => "English",
            Language::Japanese => "日本語",
            Language::Chinese => "中文",
        };
        
        unsafe {
            CURRENT_LANGUAGE = new_language;
        }
        config.language = new_language;
        if let Err(e) = config.save() {
            let t = get_translations();
            eprintln!("{}", t.error(&format!("无法保存语言设置: {}", e)));
        }
        
        // 只显示语言切换成功的消息，不显示帮助
        let t = get_translations();
        println!("{}", t.language_changed(lang_display_name));
        return;
    }
    
    // 如果是help命令，直接显示帮助并退出
    if args.len() == 2 && (args[1] == "help" || args[1] == "-h" || args[1] == "--help") {
        show_help();
        return;
    }
    
    // 使用clap解析命令行参数
    let matches = Cli::parse();
    
    // 复制解析后的参数到我们的cli实例
    let mut cli = build_cli();
    cli.task = matches.task;
    cli.due_date = matches.due_date;
    cli.language = matches.language;
    cli.help = matches.help;
    cli.version = matches.version;
    cli.edit_id = matches.edit_id;
    cli.edit_content_arg = matches.edit_content_arg;
    cli.edit_content = matches.edit_content;
    cli.complete_ids = matches.complete_ids;
    cli.undo_id = matches.undo_id;
    cli.star_id = matches.star_id;
    cli.unstar_id = matches.unstar_id;
    cli.delete_ids = matches.delete_ids;
    cli.list = matches.list;
    cli.show_id = matches.show_id;
    cli.multi_mode = matches.multi_mode;    
    cli.command = matches.command;

    let t = get_translations();
    
    // 处理帮助选项
    if cli.help || (cli.command.is_some() && matches!(cli.command.as_ref().unwrap(), Commands::Help)) {
        show_help();
        return;
    }
    
    // 处理版本选项
    if cli.version {
        show_version();
        return;
    }
    
    // 更新语言设置 (从命令行参数)
    if let Some(lang_str) = &cli.language {
        let new_language = Language::from_str(lang_str);
        unsafe {
            CURRENT_LANGUAGE = new_language;
        }
        
        // 更新并保存配置
        config.language = new_language;
        if let Err(e) = config.save() {
            eprintln!("{}", t.error(&format!("无法保存语言设置: {}", e)));
        }
        
        // 语言切换后立即显示帮助
        show_help();
        return;
    }

    // 初始化TodoList实例
    let mut todo_list = match TodoList::new() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("{}", t.init_failed(&e.to_string()));
            return;
        }
    };
    
    // 处理添加新任务
    if let Some(ref task_str) = cli.task {
        // 判断是否为编辑模式
        if cli.edit_id.is_some() {
            // 编辑模式下不执行添加任务逻辑
        } else {
            // 添加新任务
            let due_date = match &cli.due_date {
                Some(date_str) => match parse_date(date_str) {
                    Ok(date) => Some(date),
                    Err(e) => {
                        eprintln!("{}", t.error(e));   
                        return;      
                    }
                },  
                None => None,        
            };
        
            if let Err(e) = todo_list.add_task(task_str.clone(), due_date) {
                eprintln!("{}", t.error(&e.to_string()));
            } else {
                println!("{}", t.task_added(&task_str));
                if due_date.is_some() {
                    println!("{}: {}", t.due_date(), cli.due_date.as_ref().unwrap());       
                }
            }   
            return;        
        }
    }

    // 处理编辑任务
    if let Some(id_str) = cli.edit_id.clone() {
        // 检测批量编辑格式
        if id_str.contains("to") || id_str.contains('-') {
            // 解析ID范围
            let parts: Vec<&str>;
            let delimiter = if id_str.contains("to") { "to" } else { "-" };
            parts = id_str.split(delimiter).collect();
            
            if parts.len() == 2 && 
               parts[0].parse::<usize>().is_ok() && 
               parts[1].parse::<usize>().is_ok() {
                let start: usize = parts[0].parse().unwrap();
                let end: usize = parts[1].parse().unwrap();
                if start <= end && end - start <= 100 { // 限制范围大小
                    let mut ids = Vec::new();
                    for id in start..=end {
                        ids.push(id.to_string());
                    }
                    
                    // 处理批量编辑截止日期
                    if let Some(date_str) = &cli.due_date {
                        let due_date = match parse_date(date_str) {
                            Ok(date) => date,
                            Err(e) => {
                                eprintln!("{}", t.error(e));
                                return;
                            }
                        };
                        
                        match todo_list.edit_tasks_due_date(&ids, due_date) {
                            Ok(updated_ids) => {
                                if updated_ids.len() == 1 {
                                    println!("{}", t.due_date_updated(&updated_ids[0]));
                                } else {
                                    println!("已更新任务 {} 的截止日期", updated_ids.join(", "));
                                }
                                return;
                            },
                            Err(e) => {
                                eprintln!("{}", t.error(e));
                                return;
                            }
                        }
                    }
                }
            }
        }

        // 单任务编辑处理
        // 按优先级确定编辑内容: 编辑参数 > 主参数 > 内容选项
        let desc = cli.edit_content_arg.as_deref()
            .or_else(|| cli.task.as_deref())
            .or_else(|| cli.edit_content.as_deref());
        
        // 解析截止日期
        let due_date = match &cli.due_date {
            Some(date_str) => match parse_date(date_str) {
                Ok(date) => Some(date),
                Err(e) => {
                    eprintln!("{}", t.error(e));   
                    return;      
                }
            },  
            None => None,    
        };
        
        // 验证编辑参数存在
        if desc.is_none() && due_date.is_none() {
            eprintln!("{}", t.error(&t.provide_content_or_date()));   
            return;
        }
            
        // 执行编辑操作
        match todo_list.edit_task(&id_str, desc, due_date) {
            Ok(_) => {
                if desc.is_some() {
                    println!("{}", t.content_updated(&id_str));
                }
                if due_date.is_some() {
                    println!("{}", t.due_date_updated(&id_str));      
                }
            },   
            Err(e) => eprintln!("{}", t.error(e)),
        }
        return;        
    }

    // 处理完成任务
    if !cli.complete_ids.is_empty() {
        // 扩展可能的ID范围
        let expanded_ids = expand_id_ranges(&cli.complete_ids);
        
        match todo_list.mark_done_multiple(&expanded_ids) {
            Ok(ids) => {
                if ids.len() == 1 {
                    println!("{}", t.task_completed(&ids[0]));
                } else { 
                    println!("{}", t.tasks_completed(&ids));      
                }
            },   
            Err(e) => eprintln!("{}", t.error(e)),
        }   
        return;    
    }
    
    // 处理取消完成任务
    if let Some(id_str) = cli.undo_id.clone() {
        // 扩展可能的ID范围
        let expanded_ids = parse_id_range(&id_str);
        
        if expanded_ids.len() == 1 {
            // 单个ID的情况，保持原有行为
            match todo_list.mark_undone(&id_str) {
                Ok(_) => println!("{}", t.task_uncompleted(&id_str)),   
                Err(e) => eprintln!("{}", t.error(e)),
            }
        } else {
            // 多个ID的情况，调用新的批量处理函数
            match todo_list.mark_undone_multiple(&expanded_ids) {
                Ok(ids) => {
                    if ids.len() == 1 {
                        println!("{}", t.task_uncompleted(&ids[0]));
                    } else {
                        println!("{}", t.tasks_uncompleted(&ids));
                    }
                },
                Err(e) => eprintln!("{}", t.error(e)),
            }
        }
        return;    
    }
    
    // 处理标记重要任务
    if let Some(id_str) = cli.star_id {
        match todo_list.star_task(&id_str) {
            Ok(_) => println!("{}", t.task_starred(&id_str)),   
            Err(e) => eprintln!("{}", t.error(e)),
        }   
        return;    
    }
    
    // 处理取消重要标记
    if let Some(id_str) = cli.unstar_id {
        match todo_list.unstar_task(&id_str) {
            Ok(_) => println!("{}", t.task_unstarred(&id_str)),   
            Err(e) => eprintln!("{}", t.error(e)),
        }   
        return;    
    }
    
    // 处理删除任务 (更改为处理多个ID和ID范围)
    if !cli.delete_ids.is_empty() {
        // 扩展可能的ID范围（如"1-5"）
        let expanded_ids = expand_id_ranges(&cli.delete_ids);
        
        // 确保用户删除的是他们看到的内容，而不是在ID重新分配后的内容
        match todo_list.remove_tasks(&expanded_ids) {
            Ok(ids) => {
                if ids.len() == 1 {
                    println!("{}", t.task_deleted(&ids[0].to_string()));
                } else {
                    println!("{}", t.tasks_deleted(&ids));    
                }
                
                // 显示当前任务列表，以便用户看到删除后的结果
                println!();  
                todo_list.list_tasks();
            },   
            Err(e) => eprintln!("{}", t.error(e)),
        }   
        return;    
    }
    
    // 处理显示详细信息
    if let Some(id_str) = cli.show_id {
        match todo_list.show_task_detail(&id_str) {
            Ok(_) => {},
            Err(e) => eprintln!("{}", t.error(e)),
        }   
        return;    
    }
    
    // 处理批量添加模式
    if cli.multi_mode {
        // 检查是否有日期范围参数
        let mut date_sequence: Option<Vec<DateTime<Local>>> = None;
        let mut date_index = 0;
        
        // 尝试检索-t参数，它可能在args中的不同位置
        for i in 0..args.len() - 1 {
            if (args[i] == "-t" || args[i] == "--time") && i + 1 < args.len() {
                // 尝试解析日期范围
                if let Some(range) = parse_date_range(&args[i + 1]) {
                    date_sequence = Some(range);
                    break;
                }
            }
        }
        
        // 进入交互式批量添加模式
        println!("{}", t.multi_mode_start());
        
        // 如果有日期序列，显示日期范围提示
        if let Some(dates) = &date_sequence {
            if dates.len() >= 2 {
                let start_date = dates[0].format("%Y-%m-%d").to_string();
                let end_date = dates[1].format("%Y-%m-%d").to_string();
                println!("日期范围模式：从 {} 到 {}", start_date, end_date);
                
                // 计算任务数量
                let days_diff = (dates[1] - dates[0]).num_days().abs() as usize + 1;
                println!("将创建 {} 个任务，每个任务依次对应从开始到结束日期", days_diff);
            }
        }
        
        let mut line = String::new();
        
        loop {
            // 确认是否已达到日期序列末尾
            if let Some(dates) = &date_sequence {
                // 如果是日期范围的第一个和最后一个日期
                if dates.len() == 2 {
                    let start_date = dates[0];
                    let end_date = dates[1];
                    
                    // 计算当前应该使用的日期
                    if date_index > 0 {
                        // 根据已添加的任务数计算当前日期
                        if start_date <= end_date {
                            // 正常顺序
                            let days_to_add = date_index as i64;
                            if start_date + chrono::Duration::days(days_to_add) > end_date {
                                println!("已完成所有指定日期范围内的任务添加");
                                break;
                            }
                        } else {
                            // 跨年情况
                            if date_index >= 2 {
                                println!("已完成所有指定日期范围内的任务添加");
                                break;
                            }
                        }
                    }
                }
            }
            
            // 输出提示符
            print!("{}", t.multi_mode_prompt());
            io::stdout().flush().unwrap();
            
            // 读取用户输入
            line.clear();
            if io::stdin().read_line(&mut line).is_err() {
                eprintln!("{}", t.error("读取输入失败"));   
                continue;                
            }

            let input = line.trim();
            
            // 检查退出命令
            if input.to_lowercase() == "exit" {
                break;                
            }

            // 跳过空行
            if input.is_empty() {
                continue;                
            }

            // 确定当前任务的截止日期
            let due_date = if let Some(dates) = &date_sequence {
                if dates.len() == 2 {
                    let start_date = dates[0];
                    let end_date = dates[1];
                    
                    if start_date <= end_date {
                        // 常规情况：按天递增
                        let days_to_add = date_index as i64;
                        Some(start_date + chrono::Duration::days(days_to_add))
                    } else if date_index == 0 {
                        // 跨年情况：第一个任务使用开始日期
                        Some(start_date)
                    } else if date_index == 1 {
                        // 跨年情况：第二个任务使用结束日期
                        Some(end_date)
                    } else {
                        None
                    }
                } else if !dates.is_empty() {
                    // 使用指定的单个日期
                    Some(dates[0])
                } else {
                    None
                }
            } else {
                None
            };
            
            // 添加任务
            if let Err(e) = todo_list.add_task(input.to_string(), due_date) {
                eprintln!("{}", t.error(&e.to_string()));
            } else { 
                if let Some(date) = due_date {
                    let date_str = date.format("%Y-%m-%d").to_string();
                    println!("{} ({})", t.task_added(input), date_str);
                } else {
                    println!("{}", t.task_added(input));
                }
                
                // 增加日期索引
                date_index += 1;
            }
        }   
        return;    
    }
    
    // 处理列出所有任务
    if cli.list || cli.command.is_none() {
        todo_list.list_tasks();   
        return;    
    }
    
    // 处理子命令
    match cli.command {
        Some(Commands::List) => todo_list.list_tasks(),
        Some(Commands::Done { id }) => {
            match todo_list.mark_done(&id) {
                Ok(_) => println!("{}", t.task_completed(&id)),
                Err(e) => eprintln!("{}", t.cmd_error(e)),
            }
        },
        Some(Commands::Undo { id }) => {
            match todo_list.mark_undone(&id) {
                Ok(_) => println!("{}", t.task_uncompleted(&id)),
                Err(e) => eprintln!("{}", t.cmd_error(e)),
            }
        },
        Some(Commands::Remove { id }) => {
            match todo_list.remove_task(&id) {
                Ok(_) => println!("{}", t.task_deleted(&id)),
                Err(e) => eprintln!("{}", t.cmd_error(e)),
            }
        },
        Some(Commands::Edit { id, content, time }) => {
            let desc = content.as_deref();
            
            let due_date = match time {
                Some(date_str) => match parse_date(&date_str) {
                    Ok(date) => Some(date),
                    Err(e) => {
                        eprintln!("{}", t.cmd_error(e));   
                        return;
                    }
                },
                None => None,
            };
            
            match todo_list.edit_task(&id, desc, due_date) {
                Ok(_) => {
                    if desc.is_some() {
                        println!("{}", t.content_updated(&id));
                    }
                    if due_date.is_some() {
                        println!("{}", t.due_date_updated(&id));      
                    }
                },   
                Err(e) => eprintln!("{}", t.cmd_error(e)),
            }
        },
        Some(Commands::Star { id }) => {
            match todo_list.star_task(&id) {
                Ok(_) => println!("{}", t.task_starred(&id)),
                Err(e) => eprintln!("{}", t.cmd_error(e)),
            }
        },
        Some(Commands::Unstar { id }) => {
            match todo_list.unstar_task(&id) {
                Ok(_) => println!("{}", t.task_unstarred(&id)),
                Err(e) => eprintln!("{}", t.cmd_error(e)),
            }
        },
        Some(Commands::Show { id }) => {
            match todo_list.show_task_detail(&id) {
                Ok(_) => {},
                Err(e) => eprintln!("{}", t.cmd_error(e)),
            }
        },   
        Some(Commands::Help) => {       
            show_help();        
            return;
        },
        None => {}
    }
}
