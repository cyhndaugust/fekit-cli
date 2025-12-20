use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "fekit")]
#[command(version)]
#[command(about = "Front-end toolkit CLI.", long_about = None)]
pub struct Args {
    /// 全局调试选项，各个子命令都可用
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// 子命令
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 创建前端项目的 git tag
    Tag {
        /// 指定版本号
        #[arg(short, long, default_value = "1.0.0")]
        version: String, 
    },
}
