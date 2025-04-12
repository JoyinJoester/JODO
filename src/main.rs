use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::cmp::Ordering;

use chrono::{DateTime, Local, NaiveDate};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};

// 定义支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    Chinese, // 默认语言
    English,
    Japanese,
}

impl Language {
    fn from_str(lang_str: &str) -> Self {
        match lang_str.to_lowercase().as_str() {
            "en" | "english" => Language::English,
            "ja" | "jp" | "japanese" => Language::Japanese,
            _ => Language::Chinese, // 默认为中文
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

    // 保留基本翻译方法
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
    
    // 批量模式的翻译保留
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
}

// 全局语言设置
static mut CURRENT_LANGUAGE: Language = Language::Chinese;

fn get_translations() -> Translations {
    unsafe {
        Translations::new(CURRENT_LANGUAGE)
    }
}

#[derive(Parser)]
#[clap(name = "jodo")]
#[clap(about = "一个简单的命令行Todo应用", long_about = None)]
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
}

// 修改Task结构体，移除group字段
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

#[derive(Debug)]
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

    // 修改add_task方法，移除group参数
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

    // 列出任务，移除group参数
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
            
        // 修改排序逻辑：首先按照星标排序，然后严格按照ID数值排序
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

    // 重构ID分配逻辑，使用顺序ID而不是递增ID
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
        
        // 重新分配ID
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
}

fn parse_date(date_str: &str) -> Result<DateTime<Local>, &'static str> {
    let t = get_translations();
    
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| t.invalid_date_format())?;
    
    let naive_datetime = naive_date.and_hms_opt(23, 59, 59)
        .ok_or(t.invalid_datetime())?;
    
    Ok(DateTime::from_naive_utc_and_offset(naive_datetime, Local::now().offset().clone()))
}

fn show_help() {
    let t = get_translations();
    println!("{}", t.help_title());
    println!("{}", "=======================".bold());
    
    // 基本视图概念说明
    println!("\n【视图】");
    println!("  jodo               {}", t.task_view());
    
    // 基本用法
    println!("\n{}", t.basic_usage());
    println!("  jodo \"任务内容\"              添加新任务");
    println!("  jodo \"任务内容\" -t 日期      添加带截止日期的任务");
    println!("  jodo -l                    列出所有任务");
    
    // 任务管理命令
    println!("\n{}", t.task_management());
    println!("  jodo -e ID 新内容           编辑任务内容");
    println!("  jodo -e ID -t 日期          修改任务截止日期");
    println!("  jodo -c ID                  标记任务为已完成");
    println!("  jodo -u ID                  取消任务完成标记");
    println!("  jodo --star ID              标记任务为重要（置顶）");
    println!("  jodo --unstar ID            取消任务重要标记");
    println!("  jodo -d ID                  删除任务");
    println!("  jodo --show ID              显示任务的详细信息");
    println!("  jodo -m                     进入批量添加任务模式");
    
    // 其他选项
    println!("\n{}", t.other_options());
    println!("  jodo -L 语言代码             设置语言 (zh-cn/en/ja)");
    println!("  jodo -h, --help              显示帮助信息");
    println!("  jodo -v, --version           显示版本信息");
    
    // 注意事项
    println!("\n{}", t.note_completed_tasks());
    
    // 示例
    println!("\n{}", t.examples());
    println!("  jodo \"完成项目报告\"");
    println!("  jodo \"完成项目报告\" -t 2023-12-31");
    println!("  jodo -e 1 \"已修改的任务内容\" -t 2023-12-25");

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
}

fn main() {
    let args = Cli::parse();
    
    // 处理语言设置
    if let Some(lang_str) = &args.language {
        unsafe {
            CURRENT_LANGUAGE = Language::from_str(lang_str);
        }
    }
    
    let t = get_translations();
    
    // 处理帮助选项
    if args.help {
        show_help();
        return;
    }
    
    // 处理版本选项
    if args.version {
        show_version();
        return;
    }
    
    let mut todo_list = match TodoList::new() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("{}", t.init_failed(&e.to_string()));
            return;
        }
    };

    // 处理添加新任务
    if let Some(task_str) = args.task {
        let due_date = match &args.due_date {
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
                println!("{}: {}", t.due_date(), args.due_date.as_ref().unwrap());
            }
        }
        return;
    }

    // 处理编辑任务
    if let Some(id_str) = args.edit_id.clone() {
        // 检查是否误解析为task
        let mut desc = args.edit_content_arg.as_deref().or_else(|| args.edit_content.as_deref());
        
        // 如果没有编辑内容且task存在，尝试使用task作为编辑内容
        if desc.is_none() && args.task.is_some() {
            desc = args.task.as_deref();
        }
        
        // 如果指定了日期，更新截止日期
        let due_date = match &args.due_date {
            Some(date_str) => match parse_date(date_str) {
                Ok(date) => Some(date),
                Err(e) => {
                    eprintln!("{}", t.error(e));
                    return;
                }
            },
            None => None,
        };
        
        // 确保至少有一项要修改
        if desc.is_none() && due_date.is_none() {
            eprintln!("{}", t.error(&t.provide_content_or_date()));
            return;
        }
        
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
    
    // 处理直接修改任务截止日期的命令
    // 格式: jodo -t ID DATE
    if args.due_date.is_some() && args.task.is_none() && args.edit_id.is_none() && args.command.is_none() {
        // 尝试解析第一个参数为任务ID
        if let Some(first_arg) = std::env::args().nth(2) {
            // 检查它是不是-t或--time选项
            if !first_arg.starts_with('-') {
                // 获取日期
                if let Some(date_str) = &args.due_date {
                    let due_date = match parse_date(date_str) {
                        Ok(date) => Some(date),
                        Err(e) => {
                            eprintln!("{}", t.error(e));
                            return;
                        }
                    };
                    
                    match todo_list.edit_task(&first_arg, None, due_date) {
                        Ok(_) => {
                            println!("{}", t.due_date_updated(&first_arg));
                        },
                        Err(e) => eprintln!("{}", t.error(e)),
                    }
                    return;
                }
            }
        }
    }
    
    // 处理完成任务
    if !args.complete_ids.is_empty() {
        match todo_list.mark_done_multiple(&args.complete_ids) {
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
    if let Some(id_str) = args.undo_id {
        match todo_list.mark_undone(&id_str) {
            Ok(_) => println!("{}", t.task_uncompleted(&id_str)),
            Err(e) => eprintln!("{}", t.error(e)),
        }
        return;
    }
    
    // 处理标记重要任务
    if let Some(id_str) = args.star_id {
        match todo_list.star_task(&id_str) {
            Ok(_) => println!("{}", t.task_starred(&id_str)),
            Err(e) => eprintln!("{}", t.error(e)),
        }
        return;
    }
    
    // 处理取消重要标记
    if let Some(id_str) = args.unstar_id {
        match todo_list.unstar_task(&id_str) {
            Ok(_) => println!("{}", t.task_unstarred(&id_str)),
            Err(e) => eprintln!("{}", t.error(e)),
        }
        return;
    }
    
    // 处理删除任务 (更改为处理多个ID)
    if !args.delete_ids.is_empty() {
        // 确保用户删除的是他们看到的内容，而不是在ID重新分配后的内容
        match todo_list.remove_tasks(&args.delete_ids) {
            Ok(ids) => {
                if ids.len() == 1 {
                    println!("{}", t.task_deleted(&ids[0].to_string()));
                } else {
                    let id_list = ids.iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    
                    // 使用国际化翻译
                    println!("任务 {} 已删除", id_list);
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
    if let Some(id_str) = args.show_id {
        match todo_list.show_task_detail(&id_str) {
            Ok(_) => {},
            Err(e) => eprintln!("{}", t.error(e)),
        }
        return;
    }
    
    // 处理批量添加模式
    if args.multi_mode {
        println!("{}", t.multi_mode_start());
        let mut line = String::new();
        
        loop {
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

            // 添加任务
            if let Err(e) = todo_list.add_task(input.to_string(), None) {
                eprintln!("{}", t.error(&e.to_string()));
            } else {
                println!("{}", t.task_added(input));
            }
        }
        return;
    }
    
    // 处理列出所有任务
    if args.list || args.command.is_none() {
        todo_list.list_tasks();
        return;
    }
    
    // 处理子命令
    match &args.command {
        Some(Commands::List) => todo_list.list_tasks(),
        Some(Commands::Done { id }) => {
            match todo_list.mark_done(&id) {
                Ok(_) => println!("任务 {} 已标记为完成", id),
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Undo { id }) => {
            match todo_list.mark_undone(&id) {
                Ok(_) => println!("任务 {} 已标记为未完成", id),
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Remove { id }) => {
            match todo_list.remove_task(&id) {
                Ok(_) => println!("{}", t.task_deleted(&id)),
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Edit { id, content, time }) => {
            let desc = content.as_deref();
            
            let due_date = match time {
                Some(date_str) => match parse_date(&date_str) {
                    Ok(date) => Some(date),
                    Err(e) => {
                        eprintln!("错误: {}", e);
                        return;
                    }
                },
                None => None,
            };
            
            match todo_list.edit_task(&id, desc, due_date) {
                Ok(_) => {
                    if desc.is_some() {
                        println!("已更新任务 {} 的内容", id);
                    }
                    if due_date.is_some() {
                        println!("已更新任务 {} 的截止日期", id);
                    }
                },
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Star { id }) => {
            match todo_list.star_task(&id) {
                Ok(_) => println!("任务 {} 已标记为重要", id),
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Unstar { id }) => {
            match todo_list.unstar_task(&id) {
                Ok(_) => println!("任务 {} 已取消重要标记", id),
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Show { id }) => {
            match todo_list.show_task_detail(&id) {
                Ok(_) => {},
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        None => {}
    }
}
