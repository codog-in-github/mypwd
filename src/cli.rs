use clap::Parser;

#[derive(Parser)]
pub(crate) struct Args {
    /// 初始化盐值，已有盐文件时默认复用
    #[arg(short = 'i', long, value_name = "salt")]
    pub(crate) init: Option<String>,

    /// 备份旧盐并强制替换，未指定 --init 时生成随机盐
    #[arg(short = 'f', long)]
    pub(crate) force: bool,

    /// 要计算的文本，使用 --init 时可省略
    #[arg(required_unless_present = "init")]
    pub(crate) text: Option<String>,
}
