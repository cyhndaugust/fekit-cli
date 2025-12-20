//! CLI 参数与子命令配置。

use clap::{Parser, Subcommand};

/// CLI 全局参数。
#[derive(Parser, Debug)]
#[command(name = "fekit")]
#[command(version)]
#[command(about = "Front-end toolkit CLI.", long_about = None)]
pub struct Args {
    /// 全局调试选项，各个子命令都可用。
    ///
    /// 参数：
    /// - `debug`：是否开启调试模式。
    ///
    /// 返回：
    /// - 无。
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// 子命令。
    ///
    /// 参数：
    /// - `command`：具体子命令枚举。
    ///
    /// 返回：
    /// - 无。
    #[command(subcommand)]
    pub command: Commands,
}

/// 子命令定义。
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 创建前端项目的 git tag
    Tag {
        /// 指定版本号。
        ///
        /// 参数：
        /// - `version`：用户指定的版本字符串。
        ///
        /// 返回：
        /// - 无。
        #[arg(short, long)]
        version: Option<String>,
        /// 是否推送到远程。
        ///
        /// 参数：
        /// - `push`：是否推送提交与 tag 到远程。
        ///
        /// 返回：
        /// - 无。
        #[arg(long, default_value_t = false)]
        push: bool,
    },
    /// 升级到最新版本
    Upgrade,
}
