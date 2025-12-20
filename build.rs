//! 构建脚本：在编译时注入版本号信息。
//!
//! 逻辑：
//! 1. 若环境变量 `FEKIT_VERSION` 存在且非空，则直接使用该值。
//! 2. 否则尝试从 git 获取最近的 tag 作为版本号。
//! 3. 成功获取后通过 `cargo:rustc-env` 注入到编译期环境中。

use std::env;
use std::process::Command;

/// 构建脚本入口。
///
/// 参数：
/// - 无。
///
/// 返回：
/// - 无。通过输出指令与 Cargo 通信。
fn main() {
    if let Ok(version) = env::var("FEKIT_VERSION") {
        if !version.trim().is_empty() {
            println!("cargo:rustc-env=FEKIT_VERSION={}", version.trim());
            return;
        }
    }

    if let Some(tag) = latest_git_tag() {
        let normalized = tag.trim();
        if !normalized.is_empty() {
            println!("cargo:rustc-env=FEKIT_VERSION={normalized}");
        }
    }
}

/// 获取当前仓库中最近的 tag。
///
/// 参数：
/// - 无。
///
/// 返回：
/// - `Some(String)`：成功获取到 tag。
/// - `None`：获取失败或未找到 tag。
fn latest_git_tag() -> Option<String> {
    let output = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let tag = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if tag.is_empty() {
        None
    } else {
        Some(tag)
    }
}
