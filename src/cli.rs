use clap::Parser;
use std::io::{self, Write};

#[derive(Parser)]
pub(crate) struct Args {
    /// 初始化盐值，已有盐文件时默认复用
    #[arg(short = 'i', long, value_name = "salt")]
    pub(crate) init: Option<String>,

    /// 备份旧盐并强制替换，未指定 --init 时生成随机盐
    #[arg(short = 'f', long)]
    pub(crate) force: bool,

    /// 确认后删除历史盐备份，保留当前盐文件
    #[arg(short = 'c', long, conflicts_with_all = ["init", "force", "text"])]
    pub(crate) history_clear: bool,

    /// 要计算的文本，使用 --init 或 --history-clear 时可省略
    #[arg(required_unless_present_any = ["init", "history_clear"])]
    pub(crate) text: Option<String>,
}

pub(crate) fn confirm_history_clear() -> io::Result<bool> {
    eprintln!("警告：将删除所有历史盐备份，删除后无法恢复。当前盐文件会保留。");
    eprint!("确认清除？[y/N] ");
    io::stderr().flush()?;

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    Ok(matches!(
        answer.trim().to_ascii_lowercase().as_str(),
        "y" | "yes"
    ))
}
