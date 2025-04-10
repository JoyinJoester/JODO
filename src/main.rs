use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::cmp::Ordering;

use chrono::{DateTime, Local, NaiveDate};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};

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
    
    /// 显示详细的帮助信息
    #[clap(short = 'h', long = "help", conflicts_with = "task")]
    help: bool,

    /// 显示版本信息
    #[clap(short = 'v', long = "version")]
    version: bool,
    
    /// 编辑任务内容
    #[clap(short = 'e', long = "edit", value_name = "ID", conflicts_with = "task")]
    edit_id: Option<String>,
    
    /// 编辑任务时的新内容
    #[clap(long = "content", requires = "edit_id")]
    edit_content: Option<String>,
    
    /// 标记任务为完成
    #[clap(short = 'c', long = "complete", value_name = "ID", conflicts_with_all = &["edit_id", "task"])]
    complete_id: Option<String>,
    
    /// 标记任务为未完成
    #[clap(short = 'u', long = "undo", value_name = "ID", conflicts_with_all = &["complete_id", "edit_id", "task"])]
    undo_id: Option<String>,
    
    /// 将任务标记为重要（置顶）
    #[clap(long = "star", value_name = "ID", conflicts_with_all = &["complete_id", "undo_id", "edit_id", "task"])]
    star_id: Option<String>,
    
    /// 取消任务的重要标记
    #[clap(long = "unstar", value_name = "ID", conflicts_with_all = &["star_id", "complete_id", "undo_id", "edit_id", "task"])]
    unstar_id: Option<String>,
    
    /// 删除任务
    #[clap(short = 'd', long = "delete", value_name = "ID", conflicts_with_all = &["edit_id", "complete_id", "undo_id", "star_id", "unstar_id", "task"])]
    delete_id: Option<String>,
    
    /// 列出所有任务
    #[clap(short = 'l', long = "list", conflicts_with_all = &["edit_id", "complete_id", "delete_id", "task"])]
    list: bool,

    /// 显示任务的详细信息
    #[clap(long = "show", value_name = "ID", conflicts_with_all = &["edit_id", "complete_id", "undo_id", "star_id", "unstar_id", "delete_id", "task"])]
    show_id: Option<String>,

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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    created_at: DateTime<Local>,
    due_date: Option<DateTime<Local>>,
    starred: bool,    // 新增重要标记
    deleted: bool,    // 标记任务是否被删除，而不是实际删除
}

#[derive(Debug)]
struct TodoList {
    tasks: Vec<Task>,
    file_path: PathBuf,
    next_id: usize,   // 跟踪下一个可用ID
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

        // 计算下一个可用ID
        let next_id = tasks.iter()
            .filter(|task| !task.deleted)
            .map(|task| task.id)
            .max()
            .unwrap_or(0) + 1;

        Ok(Self { tasks, file_path, next_id })
    }

    fn add_task(&mut self, description: String, due_date: Option<DateTime<Local>>) -> Result<(), io::Error> {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
            created_at: Local::now(),
            due_date,
            starred: false,
            deleted: false,
        };

        self.next_id += 1;
        self.tasks.push(task);
        self.save()
    }

    fn list_tasks(&self) {
        let active_tasks: Vec<&Task> = self.tasks.iter()
            .filter(|task| !task.deleted)
            .collect();
            
        if active_tasks.is_empty() {
            println!("{}", "没有任务".yellow());
            return;
        }

        // 按照完成状态、星标和ID排序
        let mut incomplete_tasks: Vec<&Task> = active_tasks.iter()
            .filter(|t| !t.completed)
            .cloned()
            .collect();
            
        let mut completed_tasks: Vec<&Task> = active_tasks.iter()
            .filter(|t| t.completed)
            .cloned()
            .collect();
            
        // 星标任务置顶，同状态下按ID排序
        incomplete_tasks.sort_by(|a, b| {
            match (a.starred, b.starred) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.id.cmp(&b.id)
            }
        });
        
        completed_tasks.sort_by_key(|t| t.id);
        
        println!("未完成任务:");
        if incomplete_tasks.is_empty() {
            println!("  {}", "无".italic());
        } else {
            println!("{:<5} {:<40} {:<15}", "ID", "描述", "截止日期");
            println!("{}", "-".repeat(70));

            for task in &incomplete_tasks {
                let star_marker = if task.starred { "★ ".yellow() } else { "  ".into() };

                let due_date = match task.due_date {
                    Some(date) => date.format("%Y-%m-%d").to_string(),
                    None => "无".to_string(),
                };
                
                // 根据可用空间计算描述的最大长度，考虑中文字符
                let max_desc_width = 36; // 留4个字符的余量
                let truncated_desc = truncate_str(&task.description, max_desc_width);

                println!(
                    "{}{:<5} {:<40} {:<15}",
                    star_marker,
                    task.id.to_string().blue(),
                    truncated_desc,
                    due_date
                );
            }
        }
        
        println!("\n已完成任务:");
        if completed_tasks.is_empty() {
            println!("  {}", "无".italic());
        } else {
            println!("{:<5} {:<40} {:<15}", "ID", "描述", "截止日期");
            println!("{}", "-".repeat(70));

            for task in completed_tasks.iter() {
                let task_id = format!("{}c", task.id);
                
                let due_date = match task.due_date {
                    Some(date) => date.format("%Y-%m-%d").to_string(),
                    None => "无".to_string(),
                };
                
                // 对已完成任务也处理中文显示问题
                let max_desc_width = 36;
                let truncated_desc = truncate_str(&task.description, max_desc_width);
                
                println!(
                    "{:<5} {:<40} {:<15}",
                    task_id.green(),
                    truncated_desc,
                    due_date
                );
            }
        }
    }

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
        let (id, _) = parse_task_id(id_str);
        
        if let Some(task) = self.tasks.iter_mut()
            .find(|t| !t.deleted && !t.completed && t.id == id) {
            task.completed = true;
            self.save().map_err(|_| "保存任务失败")?;
            Ok(())
        } else {
            Err("任务不存在或已完成")
        }
    }
    
    fn mark_undone(&mut self, id_str: &str) -> Result<(), &'static str> {
        let (id, _) = parse_task_id(id_str);
        
        if let Some(task) = self.tasks.iter_mut()
            .find(|t| !t.deleted && t.completed && t.id == id) {
            task.completed = false;
            self.save().map_err(|_| "保存任务失败")?;
            Ok(())
        } else {
            Err("任务不存在或未完成")
        }
    }
    
    fn star_task(&mut self, id_str: &str) -> Result<(), &'static str> {
        let (id, is_completed) = parse_task_id(id_str);
        
        if let Some(task) = self.tasks.iter_mut()
            .find(|t| !t.deleted && t.completed == is_completed && t.id == id) {
            task.starred = true;
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
            self.save().map_err(|_| "保存任务失败")?;
            Ok(())
        } else {
            Err("任务不存在")
        }
    }

    fn remove_task(&mut self, id_str: &str) -> Result<(), &'static str> {
        let (id, is_completed) = parse_task_id(id_str);
        
        if let Some(task) = self.tasks.iter_mut()
            .find(|t| !t.deleted && t.completed == is_completed && t.id == id) {
            // 标记为删除而不实际移除
            task.deleted = true;
            self.save().map_err(|_| "保存任务失败")?;
            Ok(())
        } else {
            Err("任务不存在")
        }
    }

    fn show_task_detail(&self, id_str: &str) -> Result<(), &'static str> {
        let (id, is_completed) = parse_task_id(id_str);
        
        if let Some(task) = self.tasks.iter()
            .find(|t| !t.deleted && t.completed == is_completed && t.id == id) {
            
            println!("{}", "任务详情:".bold());
            println!("{}", "=".repeat(50));
            println!("{:<10}: {}", "ID", if is_completed { 
                format!("{}c", task.id).green()
            } else {
                task.id.to_string().blue()
            });
            println!("{:<10}: {}", "状态", if task.completed { 
                "已完成".green() 
            } else { 
                "未完成".yellow()
            });
            println!("{:<10}: {}", "重要标记", if task.starred {
                "是 ★".yellow()
            } else {
                "否".normal()
            });
            println!("{:<10}: {}", "创建时间", task.created_at.format("%Y-%m-%d %H:%M:%S"));
            println!("{:<10}: {}", "截止日期", match task.due_date {
                Some(date) => date.format("%Y-%m-%d").to_string(),
                None => "无".to_string(),
            });
            println!("{}", "-".repeat(50));
            println!("{:<10}: ", "描述");
            println!("{}", task.description);
            println!("{}", "=".repeat(50));
            
            Ok(())
        } else {
            Err("任务不存在")
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
}

fn parse_date(date_str: &str) -> Result<DateTime<Local>, &'static str> {
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| "日期格式错误，请使用 YYYY-MM-DD 格式")?;
    
    let naive_datetime = naive_date.and_hms_opt(23, 59, 59)
        .ok_or("无效的日期时间")?;
    
    Ok(DateTime::from_naive_utc_and_offset(naive_datetime, Local::now().offset().clone()))
}

fn show_help() {
    println!("{}", "Jodo - 简单的命令行Todo应用".bold());
    println!("{}", "=======================".bold());
    println!();
    println!("基本用法:");
    println!("  jodo \"任务内容\"         添加新任务");
    println!("  jodo \"任务内容\" -t 日期  添加带截止日期的任务");
    println!("  jodo -l, --list        列出所有任务");
    println!();
    println!("任务管理命令:");
    println!("  jodo -e, --edit <id> --content \"内容\"   编辑任务内容");
    println!("  jodo -e, --edit <id> -t, --time 日期    编辑任务截止日期");
    println!("  jodo -d, --delete <id>                 删除任务");
    println!("  jodo -c, --complete <id>               标记任务为已完成");
    println!("  jodo -u, --undo <id>                   取消任务完成标记");
    println!("  jodo --star <id>                       标记任务为重要（置顶）");
    println!("  jodo --unstar <id>                     取消任务重要标记");
    println!("  jodo --show <id>                       显示任务的详细信息");
    println!();
    println!("注意: 已完成的任务ID会在末尾显示'c'，例如 '1c'");
    println!();
    println!("子命令:");
    println!("  jodo list                              列出所有任务");
    println!("  jodo done <id>                         标记任务为已完成");
    println!("  jodo undo <id>                         取消任务完成标记");
    println!("  jodo remove <id>                       删除任务");
    println!("  jodo edit <id> \"内容\"                  编辑任务内容");
    println!("  jodo edit <id> -t 日期                 编辑任务截止日期");
    println!("  jodo star <id>                         标记任务为重要");
    println!("  jodo unstar <id>                       取消任务重要标记");
    println!("  jodo show <id>                         显示任务的详细信息");
    println!();
    println!("其他选项:");
    println!("  -h, --help             显示此帮助信息");
    println!("  -v, --version          显示版本信息");
    println!();
    println!("示例:");
    println!("  jodo \"完成项目报告\"");
    println!("  jodo \"完成项目报告\" -t 2023-12-31");
    println!("  jodo -l");
    println!("  jodo -e 1 --content \"修改后的内容\"");
    println!("  jodo -e 1 -t 2023-12-31");
    println!("  jodo -d 1");
    println!("  jodo -c 1");
}

fn show_version() {
    println!("Jodo 版本 {}", env!("CARGO_PKG_VERSION"));
    println!("一个简单的命令行Todo应用");
    println!("作者: {}", env!("CARGO_PKG_AUTHORS"));
}

fn main() {
    let args = Cli::parse();
    
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
            eprintln!("初始化任务列表失败: {}", e);
            return;
        }
    };

    // 处理添加新任务
    if let Some(task_str) = args.task {
        let due_date = match &args.due_date {
            Some(date_str) => match parse_date(date_str) {
                Ok(date) => Some(date),
                Err(e) => {
                    eprintln!("错误: {}", e);
                    return;
                }
            },
            None => None,
        };

        if let Err(e) = todo_list.add_task(task_str.clone(), due_date) {
            eprintln!("添加任务失败: {}", e);
        } else {
            println!("任务 \"{}\" 已添加", task_str);
            if due_date.is_some() {
                println!("截止日期: {}", args.due_date.as_ref().unwrap());
            }
        }
        return;
    }

    // 处理编辑任务
    if let Some(id_str) = args.edit_id {
        // 如果指定了新内容，更新任务内容
        let desc = if args.edit_content.is_some() {
            args.edit_content.as_deref()
        } else {
            None
        };
        
        // 如果指定了日期，更新截止日期
        let due_date = match &args.due_date {
            Some(date_str) => match parse_date(date_str) {
                Ok(date) => Some(date),
                Err(e) => {
                    eprintln!("错误: {}", e);
                    return;
                }
            },
            None => None,
        };
        
        if desc.is_none() && due_date.is_none() {
            eprintln!("错误: 请提供要修改的内容或截止日期");
            return;
        }
        
        match todo_list.edit_task(&id_str, desc, due_date) {
            Ok(_) => {
                if desc.is_some() {
                    println!("已更新任务 {} 的内容", id_str);
                }
                if due_date.is_some() {
                    println!("已更新任务 {} 的截止日期", id_str);
                }
            },
            Err(e) => eprintln!("错误: {}", e),
        }
        return;
    }
    
    // 处理完成任务
    if let Some(id_str) = args.complete_id {
        match todo_list.mark_done(&id_str) {
            Ok(_) => println!("任务 {} 已标记为完成", id_str),
            Err(e) => eprintln!("错误: {}", e),
        }
        return;
    }
    
    // 处理取消完成任务
    if let Some(id_str) = args.undo_id {
        match todo_list.mark_undone(&id_str) {
            Ok(_) => println!("任务 {} 已标记为未完成", id_str),
            Err(e) => eprintln!("错误: {}", e),
        }
        return;
    }
    
    // 处理标记重要任务
    if let Some(id_str) = args.star_id {
        match todo_list.star_task(&id_str) {
            Ok(_) => println!("任务 {} 已标记为重要", id_str),
            Err(e) => eprintln!("错误: {}", e),
        }
        return;
    }
    
    // 处理取消重要标记
    if let Some(id_str) = args.unstar_id {
        match todo_list.unstar_task(&id_str) {
            Ok(_) => println!("任务 {} 已取消重要标记", id_str),
            Err(e) => eprintln!("错误: {}", e),
        }
        return;
    }
    
    // 处理删除任务
    if let Some(id_str) = args.delete_id {
        match todo_list.remove_task(&id_str) {
            Ok(_) => println!("任务 {} 已删除", id_str),
            Err(e) => eprintln!("错误: {}", e),
        }
        return;
    }
    
    // 处理显示详细信息
    if let Some(id_str) = args.show_id {
        match todo_list.show_task_detail(&id_str) {
            Ok(_) => {},
            Err(e) => eprintln!("错误: {}", e),
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
            match todo_list.mark_done(id) {
                Ok(_) => println!("任务 {} 已标记为完成", id),
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Undo { id }) => {
            match todo_list.mark_undone(id) {
                Ok(_) => println!("任务 {} 已标记为未完成", id),
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Remove { id }) => {
            match todo_list.remove_task(id) {
                Ok(_) => println!("任务 {} 已删除", id),
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Edit { id, content, time }) => {
            let desc = content.as_deref();
            
            let due_date = match time {
                Some(date_str) => match parse_date(date_str) {
                    Ok(date) => Some(date),
                    Err(e) => {
                        eprintln!("错误: {}", e);
                        return;
                    }
                },
                None => None,
            };
            
            match todo_list.edit_task(id, desc, due_date) {
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
            match todo_list.star_task(id) {
                Ok(_) => println!("任务 {} 已标记为重要", id),
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Unstar { id }) => {
            match todo_list.unstar_task(id) {
                Ok(_) => println!("任务 {} 已取消重要标记", id),
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        Some(Commands::Show { id }) => {
            match todo_list.show_task_detail(id) {
                Ok(_) => {},
                Err(e) => eprintln!("错误: {}", e),
            }
        },
        None => {}
    }
}
