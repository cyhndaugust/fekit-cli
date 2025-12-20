//! 命令行输出封装，统一样式与交互提示。

use console::{Term, style};
use std::io::{self, Write};

/// 输出提示信息（信息级别）。
///
/// 参数：
/// - `message`：要输出的内容。
///
/// 返回：
/// - 无。
pub fn info(message: &str) {
    println!("ℹ️  {}", style(message).cyan());
}

/// 输出成功信息。
///
/// 参数：
/// - `message`：要输出的内容。
///
/// 返回：
/// - 无。
pub fn success(message: &str) {
    println!("✅ {}", style(message).green());
}

/// 输出警告信息。
///
/// 参数：
/// - `message`：要输出的内容。
///
/// 返回：
/// - 无。
pub fn warn(message: &str) {
    println!("⚠️  {}", style(message).yellow());
}

/// 输出错误信息到标准错误。
///
/// 参数：
/// - `message`：要输出的内容。
///
/// 返回：
/// - 无。
pub fn error(message: &str) {
    eprintln!("❌ {}", style(message).red());
}

/// 输出普通信息。
///
/// 参数：
/// - `message`：要输出的内容。
///
/// 返回：
/// - 无。
pub fn plain(message: &str) {
    println!("• {message}");
}

/// 输出提示语并刷新终端，适用于后续读取用户输入。
///
/// 参数：
/// - `message`：提示文案。
///
/// 返回：
/// - `Ok(())`：提示输出成功。
/// - `Err(String)`：输出失败的错误信息。
pub fn prompt(message: &str) -> Result<(), String> {
    print!("👉 {} ", style(message).cyan());
    io::stdout()
        .flush()
        .map_err(|err| format!("输出提示失败: {err}"))?;
    Ok(())
}

/// 等待用户按键确认，回车表示同意。
///
/// 参数：
/// - `message`：提示文案。
///
/// 返回：
/// - `Ok(true)`：用户按回车确认。
/// - `Ok(false)`：用户输入非回车字符。
/// - `Err(String)`：读取输入失败。
pub fn confirm_enter(message: &str) -> Result<bool, String> {
    prompt(message)?;
    let term = Term::stdout();
    let ch = term
        .read_char()
        .map_err(|err| format!("读取输入失败: {err}"))?;
    term.write_line("").ok();
    Ok(ch == '\n' || ch == '\r')
}

/// 等待用户输入 y/n/q：y 继续，n 取消，q 退出。
///
/// 参数：
/// - `message`：提示文案。
///
/// 返回：
/// - `Ok(true)`：用户输入 y/Y。
/// - `Ok(false)`：用户输入 n/N/q/Q。
/// - `Err(String)`：读取输入失败。
pub fn confirm_ynq(message: &str) -> Result<bool, String> {
    loop {
        prompt(message)?;
        let term = Term::stdout();
        let ch = term
            .read_char()
            .map_err(|err| format!("读取输入失败: {err}"))?;
        term.write_line("").ok();
        match ch {
            'y' | 'Y' => return Ok(true),
            'n' | 'N' | 'q' | 'Q' => return Ok(false),
            _ => {
                warn("请输入 y/n/q");
            }
        }
    }
}
