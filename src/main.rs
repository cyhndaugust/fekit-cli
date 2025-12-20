//! 程序入口，解析命令行参数并执行对应逻辑。

use clap::Parser;
use fekit_cli::{config::Args, match_command};

fn main() {
    let args = Args::parse();

    match_command(&args);
}
