use std::io::{self, stdout, Write};
// 移除未使用的导入: use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor, style};

use crate::{parse_date, TodoList};

pub fn run_interactive_mode(todo_list: &mut TodoList) -> Result<(), io::Error> {
    // 初始化终端
    let mut stdout = stdout().into_raw_mode()?;
    
    // 显示初始界面
    render_screen(todo_list, &mut stdout, "")?;
    
    // 命令行缓冲区
    let mut command_buffer = String::new();
    let mut modified = false;
    
    // 进入事件循环
    let stdin = io::stdin();
    let mut keys = stdin.keys();
    
    loop {
        if let Some(key_result) = keys.next() {
            match key_result? {
                // 退出
                Key::Ctrl('c') | Key::Char('q') => {
                    if modified {
                        // 如果有未保存的修改，询问用户
                        write!(stdout, "{}{}有未保存的修改。确定要退出吗？(y/n) ", 
                               cursor::Goto(1, termion::terminal_size()?.1), clear::CurrentLine)?;
                        stdout.flush()?;
                        
                        loop {
                            if let Some(key_result) = keys.next() {
                                match key_result? {
                                    Key::Char('y') | Key::Char('Y') => {
                                        clean_exit(&mut stdout)?;
                                        return Ok(());
                                    },
                                    Key::Char('n') | Key::Char('N') | Key::Esc => {
                                        render_screen(todo_list, &mut stdout, "")?;
                                        break;
                                    },
                                    _ => {}
                                }
                            }
                        }
                    } else {
                        clean_exit(&mut stdout)?;
                        return Ok(());
                    }
                },
                
                // 命令输入
                Key::Char(':') => {
                    command_buffer.clear();
                    command_buffer.push(':');
                    render_command(&mut stdout, &command_buffer)?;
                    
                    // 进入命令输入模式
                    loop {
                        if let Some(key_result) = keys.next() {
                            match key_result? {
                                Key::Char('\n') => {
                                    // 解析并执行命令
                                    let (message, save_performed) = execute_command(todo_list, &command_buffer)?;
                                    if save_performed {
                                        modified = false;
                                    } else if command_buffer == ":q" || command_buffer == ":quit" {
                                        if modified {
                                            write!(stdout, "{}{}有未保存的修改。使用 :q! 强制退出或 :wq 保存并退出。", 
                                                   cursor::Goto(1, termion::terminal_size()?.1), clear::CurrentLine)?;
                                            stdout.flush()?;
                                            std::thread::sleep(std::time::Duration::from_secs(2));
                                            render_screen(todo_list, &mut stdout, "")?;
                                            command_buffer.clear();
                                            break;
                                        } else {
                                            clean_exit(&mut stdout)?;
                                            return Ok(());
                                        }
                                    } else if command_buffer == ":q!" {
                                        clean_exit(&mut stdout)?;
                                        return Ok(());
                                    } else if command_buffer == ":wq" {
                                        // 保存并退出
                                        if let Err(e) = todo_list.save() {
                                            render_screen(todo_list, &mut stdout, &format!("保存失败: {}", e))?;
                                        } else {
                                            clean_exit(&mut stdout)?;
                                            return Ok(());
                                        }
                                    } else if command_buffer == ":w" {
                                        // 仅保存
                                        if let Err(e) = todo_list.save() {
                                            render_screen(todo_list, &mut stdout, &format!("保存失败: {}", e))?;
                                        } else {
                                            modified = false;
                                            render_screen(todo_list, &mut stdout, "保存成功")?;
                                        }
                                    }
                                    
                                    // 检查命令是否修改了任务
                                    if command_buffer.starts_with(":a ") || 
                                       command_buffer.starts_with(":e ") || 
                                       command_buffer.starts_with(":d ") || 
                                       command_buffer.starts_with(":c ") {
                                        modified = true;
                                    }
                                    
                                    command_buffer.clear();
                                    render_screen(todo_list, &mut stdout, &message)?;
                                    break;
                                },
                                Key::Char(c) => {
                                    command_buffer.push(c);
                                    render_command(&mut stdout, &command_buffer)?;
                                },
                                Key::Backspace => {
                                    if command_buffer.len() > 1 {
                                        command_buffer.pop();
                                        render_command(&mut stdout, &command_buffer)?;
                                    }
                                },
                                Key::Esc => {
                                    command_buffer.clear();
                                    render_screen(todo_list, &mut stdout, "")?;
                                    break;
                                },
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }
}

fn render_screen(todo_list: &TodoList, stdout: &mut RawTerminal<io::Stdout>, message: &str) -> Result<(), io::Error> {
    // 清屏并定位光标
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;
    
    // 绘制标题
    writeln!(stdout, "{}JODO - 任务管理器{}\n", style::Bold, style::Reset)?;
    
    // 绘制任务列表
    let tasks = todo_list.get_tasks();
    if tasks.is_empty() {
        writeln!(stdout, "没有任务\n")?;
    } else {
        // 找出最大ID的宽度，用于对齐
        let max_id_width = tasks.iter().map(|t| t.id.to_string().len()).max().unwrap_or(1);
        
        // 计算合适的描述宽度，留出空间给状态和截止日期
        let terminal_width = termion::terminal_size()?.0 as usize;
        let desc_width = terminal_width.saturating_sub(max_id_width + 20); // 为其他列预留空间
        
        for task in tasks {
            let status_marker = if task.completed { "[✓]" } else { "[ ]" };
            let due_date = match task.due_date {
                Some(date) => format!("截止: {}", date.format("%Y-%m-%d")),
                None => String::new(),
            };
            
            // 截断过长的描述并添加省略号
            let desc = if task.description.len() > desc_width {
                format!("{}...", &task.description[..desc_width.saturating_sub(3)])
            } else {
                task.description.clone()
            };
            
            // 使用固定宽度格式化
            writeln!(
                stdout,
                "{} {:width1$} {:<width2$} {}",
                status_marker,
                format!("{}.", task.id),
                desc,
                due_date,
                width1 = max_id_width + 1,  // ID宽度（+1是点号）
                width2 = desc_width         // 描述固定宽度
            )?;
        }
        writeln!(stdout)?;
    }
    
    // 显示消息（如果有）
    if !message.is_empty() {
        writeln!(stdout, "\n{}", message)?;
    }
    
    // 显示简短的提示
    write!(stdout, "\n输入 :help 查看帮助，:q 退出")?;
    
    stdout.flush()?;
    Ok(())
}

fn render_command(stdout: &mut RawTerminal<io::Stdout>, command: &str) -> Result<(), io::Error> {
    write!(
        stdout,
        "{}{}{}{}",
        cursor::Goto(1, termion::terminal_size()?.1),
        clear::CurrentLine,
        command,
        cursor::Goto((command.len() + 1) as u16, termion::terminal_size()?.1)
    )?;
    stdout.flush()?;
    Ok(())
}

fn show_help(stdout: &mut RawTerminal<io::Stdout>) -> Result<(), io::Error> {
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;
    
    writeln!(stdout, "{}JODO - 帮助{}\n", style::Bold, style::Reset)?;
    writeln!(stdout, "命令:")?;
    writeln!(stdout, "  :e <id> <新内容>     - 编辑任务内容")?;
    writeln!(stdout, "  :e <id> -t <日期>    - 编辑任务截止日期 (YYYY-MM-DD)")?;
    writeln!(stdout, "  :d <id>              - 删除任务")?;
    writeln!(stdout, "  :c <id>              - 标记任务为已完成")?;
    writeln!(stdout, "  :a <内容>            - 添加新任务")?;
    writeln!(stdout, "  :a <内容> -t <日期>  - 添加带截止日期的任务")?;
    writeln!(stdout, "  :w                   - 保存更改")?;
    writeln!(stdout, "  :q                   - 退出 (有未保存更改时会提示)")?;
    writeln!(stdout, "  :wq                  - 保存并退出")?;
    writeln!(stdout, "  :q!                  - 强制退出 (不保存更改)")?;
    writeln!(stdout, "  :help                - 显示此帮助信息")?;
    
    writeln!(stdout, "\n按任意键返回...")?;
    stdout.flush()?;
    Ok(())
}

fn execute_command(todo_list: &mut TodoList, command: &str) -> Result<(String, bool), io::Error> {
    let cmd_parts: Vec<&str> = command.split_whitespace().collect();
    
    if cmd_parts.is_empty() {
        return Ok(("请输入命令".to_string(), false));
    }
    
    // 命令已经以冒号开头（因为我们在UI中自动添加了），所以无需再检查冒号
    // 只需检查命令是否只有冒号
    if cmd_parts[0] == ":" {
        return Ok(("请输入命令".to_string(), false));
    }
    
    // 解析命令类型 - 现在cmd_parts[0]一定是以冒号开头的
    let cmd = if cmd_parts[0].starts_with(':') {
        &cmd_parts[0][1..] // 去掉冒号
    } else {
        &cmd_parts[0]      // 以防万一，但实际上不应该到这里
    };
    
    match cmd {
        "q" | "quit" => {
            // 在外层处理，因为需要检查修改状态
            return Ok(("".to_string(), false));
        },
        "q!" => {
            // 在外层处理强制退出
            return Ok(("".to_string(), false));
        },
        "w" => {
            // 在外层处理保存
            return Ok(("".to_string(), true));
        },
        "wq" => {
            // 在外层处理保存并退出
            return Ok(("".to_string(), true));
        },
        "help" => {
            // 显示帮助界面
            let mut stdout = stdout().into_raw_mode()?;
            show_help(&mut stdout)?;
            
            // 等待任意键
            let stdin = io::stdin();
            let mut keys = stdin.keys();
            if let Some(key_result) = keys.next() {
                key_result?; // 忽略按键值，只要按了任意键
            }
            return Ok(("".to_string(), false));
        },
        "e" | "edit" => {
            if cmd_parts.len() < 3 {
                return Ok(("语法错误: :e <id> <新内容> 或 :e <id> -t <日期>".to_string(), false));
            }
            
            let id = match cmd_parts[1].parse::<usize>() {
                Ok(id) => id,
                Err(_) => return Ok(("无效的任务ID".to_string(), false)),
            };
            
            // 检查是否为修改日期
            if cmd_parts[2] == "-t" && cmd_parts.len() >= 4 {
                let date = match parse_date(cmd_parts[3]) {
                    Ok(date) => date,
                    Err(e) => return Ok((format!("日期格式错误: {}", e), false)),
                };
                
                match todo_list.edit_task(id, None, Some(date)) {
                    Ok(_) => return Ok((format!("已更新任务 {} 的截止日期", id), false)),
                    Err(e) => return Ok((format!("错误: {}", e), false)),
                }
            } else {
                // 修改任务内容
                let new_content = cmd_parts[2..].join(" ");
                match todo_list.edit_task(id, Some(new_content), None) {
                    Ok(_) => return Ok((format!("已更新任务 {}", id), false)),
                    Err(e) => return Ok((format!("错误: {}", e), false)),
                }
            }
        },
        "d" | "del" | "delete" => {
            if cmd_parts.len() != 2 {
                return Ok(("语法错误: :d <id>".to_string(), false));
            }
            
            let id = match cmd_parts[1].parse::<usize>() {
                Ok(id) => id,
                Err(_) => return Ok(("无效的任务ID".to_string(), false)),
            };
            
            match todo_list.remove_task(id) {
                Ok(_) => return Ok((format!("已删除任务 {}", id), false)),
                Err(e) => return Ok((format!("错误: {}", e), false)),
            }
        },
        "c" | "complete" => {
            if cmd_parts.len() != 2 {
                return Ok(("语法错误: :c <id>".to_string(), false));
            }
            
            let id = match cmd_parts[1].parse::<usize>() {
                Ok(id) => id,
                Err(_) => return Ok(("无效的任务ID".to_string(), false)),
            };
            
            match todo_list.mark_done(id) {
                Ok(_) => return Ok((format!("已完成任务 {}", id), false)),
                Err(e) => return Ok((format!("错误: {}", e), false)),
            }
        },
        "a" | "add" => {
            if cmd_parts.len() < 2 {
                return Ok(("语法错误: :a <内容> 或 :a <内容> -t <日期>".to_string(), false));
            }
            
            let t_pos = cmd_parts.iter().position(|&s| s == "-t");
            let (content, due_date) = if let Some(pos) = t_pos {
                if pos + 1 >= cmd_parts.len() {
                    return Ok(("语法错误: 缺少日期参数".to_string(), false));
                }
                
                let content = cmd_parts[1..pos].join(" ");
                let date = match parse_date(cmd_parts[pos + 1]) {
                    Ok(date) => Some(date),
                    Err(e) => return Ok((format!("日期格式错误: {}", e), false)),
                };
                
                (content, date)
            } else {
                (cmd_parts[1..].join(" "), None)
            };
            
            match todo_list.add_task(content.clone(), due_date) {
                Ok(_) => return Ok((format!("已添加任务: {}", content), false)),
                Err(e) => return Ok((format!("添加任务失败: {}", e), false)),
            }
        },
        _ => return Ok(("未知命令".to_string(), false)),
    }
}

fn clean_exit(stdout: &mut RawTerminal<io::Stdout>) -> Result<(), io::Error> {
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;
    stdout.flush()?;
    Ok(())
}
