use crate::config::{Args, Commands};

pub mod config;

pub fn match_command(args: &Args) {
    // 处理全局 debug 参数
    if args.debug {
        println!("开启全局 Debug 模式");
    }

    // 匹配具体子命令
    match &args.command {
        Commands::Init { force } => {
            println!("running init, force: {}", force);
        }
        Commands::Build { target } => {
            println!("running build, target: {}", target);
        }
        Commands::Deploy { env, verbose } => {
            println!("running deploy, env: {}, verbose: {}", env, verbose);
        }
    }
}