use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "fekit")]
#[command(version)]
#[command(about = "Front-end toolkit CLI.", long_about = None)]
pub struct Args {
    /// 全局调试选项，各个子命令都可用
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// 子命令: init / build / deploy / ...
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 初始化项目
    Init {
        /// 是否强制初始化
        #[arg(short, long)]
        force: bool,
    },

    /// 编译项目
    Build {
        /// 设置编译目标
        #[arg(short, long, default_value = "debug")]
        target: String,
    },

    /// 部署项目
    Deploy {
        /// 指定部署环境
        #[arg(short, long)]
        env: String,

        /// 是否详细输出
        #[arg(short, long)]
        verbose: bool,
    },
}