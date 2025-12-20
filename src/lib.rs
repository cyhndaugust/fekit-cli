use crate::config::{Args, Commands};
use crate::subcommands::tag::run_tag_command;

pub mod config;
pub mod subcommands;

pub fn match_command(args: &Args) {
    // 处理全局 debug 参数
    if args.debug {
        println!("开启全局 Debug 模式");
    }

    // 匹配具体子命令
    match &args.command {
        Commands::Tag { version, push } => {
            if let Err(err) = run_tag_command(version.as_deref(), *push) {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    }
}
