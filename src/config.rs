//! CLI 参数与子命令配置。

use clap::{Parser, Subcommand};

/// CLI 全局参数。
#[derive(Parser, Debug)]
#[command(name = "fekit")]
#[command(version)]
#[command(about = "Front-end toolkit CLI.", long_about = None)]
pub struct Args {
    /// 全局调试选项，各个子命令都可用。
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// 子命令。
    #[command(subcommand)]
    pub command: Commands,
}

/// 子命令定义。
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 创建前端项目的 git tag
    Tag {
        /// 指定版本号。
        #[arg(short, long)]
        version: Option<String>,
        /// 是否推送到远程。
        #[arg(long, default_value_t = false)]
        push: bool,
    },
    /// 升级到最新版本
    Upgrade,
}
