use console::{style, Term};
use std::io::{self, Write};

pub fn info(message: &str) {
    println!("ℹ️  {}", style(message).cyan());
}

pub fn success(message: &str) {
    println!("✅ {}", style(message).green());
}

pub fn warn(message: &str) {
    println!("⚠️  {}", style(message).yellow());
}

pub fn error(message: &str) {
    eprintln!("❌ {}", style(message).red());
}

pub fn plain(message: &str) {
    println!("• {message}");
}

pub fn prompt(message: &str) -> Result<(), String> {
    print!("👉 {} ", style(message).cyan());
    io::stdout()
        .flush()
        .map_err(|err| format!("输出提示失败: {err}"))?;
    Ok(())
}

pub fn confirm_enter(message: &str) -> Result<bool, String> {
    prompt(message)?;
    let term = Term::stdout();
    let ch = term
        .read_char()
        .map_err(|err| format!("读取输入失败: {err}"))?;
    term.write_line("").ok();
    Ok(ch == '\n' || ch == '\r')
}

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
