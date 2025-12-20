//! `fk` 入口，作为 `fekit` 的便捷别名。

use clap::Parser;
use fekit_cli::{config::Args, match_command};

/// CLI 主入口（`fk` 别名）。
///
/// 参数：
/// - 无。
///
/// 返回：
/// - 无。
fn main() {
    let args = Args::parse();

    match_command(&args);
}
