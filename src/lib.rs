//! CLI 入口与命令分发。

use crate::config::{Args, Commands};
use crate::subcommands::tag::run_tag_command;

pub mod config;
pub mod output;
pub mod subcommands;

/// 根据解析后的参数分发子命令。
///
/// 参数：
/// - `args`：命令行解析后的参数集合。
///
/// 返回：
/// - 无。若执行失败会直接退出进程。
pub fn match_command(args: &Args) {
    // 处理全局 debug 参数
    if args.debug {
        println!("开启全局 Debug 模式");
    }

    // 匹配具体子命令
    match &args.command {
        Commands::Tag { version, push } => {
            if let Err(err) = run_tag_command(version.as_deref(), *push) {
                output::error(&err);
                std::process::exit(1);
            }
        }
    }
}
