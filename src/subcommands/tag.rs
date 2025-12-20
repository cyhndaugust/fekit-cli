use crate::output;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
struct ParsedVersion {
    prefix: Option<String>,
    major: u64,
    minor: u64,
    patch: u64,
}

pub fn run_tag_command(input_version: Option<&str>, push_remote: bool) -> Result<(), String> {
    ensure_git_repo()?;
    let package_path = Path::new("package.json");
    if !package_path.exists() {
        return Err("未检测到 package.json，请在前端项目根目录执行。".to_string());
    }

    let package_content =
        fs::read_to_string(package_path).map_err(|err| format!("读取 package.json 失败: {err}"))?;
    let current_version = extract_package_version(&package_content)
        .ok_or_else(|| "package.json 中未找到 version 字段，请补充后重试。".to_string())?;

    let base_input = input_version.unwrap_or(&current_version);
    let mut parsed = parse_version(base_input).map_err(|err| format!("版本格式错误: {err}"))?;

    let mut candidate = build_tag(&parsed);
    while tag_exists_local(&candidate)? {
        parsed.patch += 1;
        candidate = build_tag(&parsed);
    }

    if push_remote && remote_tag_exists(&candidate)? {
        return Err(format!("远程已存在 tag：{candidate}，请调整版本后重试。"));
    }

    print_tag_preview(&current_version, &candidate)?;
    if !confirm_proceed()? {
        output::warn("已取消执行。");
        return Ok(());
    }

    let updated_package = replace_package_version(&package_content, &candidate)?;
    fs::write(package_path, updated_package)
        .map_err(|err| format!("写入 package.json 失败: {err}"))?;

    git_command(&["add", "package.json"])?;
    git_command(&["commit", "-m", &format!("tag@{candidate}")])?;
    git_command(&["tag", &candidate])?;

    if push_remote {
        git_command(&["push"])?;
        git_command(&["push", "origin", &candidate])?;
        output::success(&format!("已完成 tag 发布并推送远程：{candidate}"));
    } else {
        output::success(&format!("已完成本地 tag 创建：{candidate}"));
    }
    Ok(())
}

fn ensure_git_repo() -> Result<(), String> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map_err(|err| format!("无法执行 git 命令: {err}"))?;
    if !output.status.success() {
        return Err("当前目录不是 git 仓库，请在仓库内执行。".to_string());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim() != "true" {
        return Err("当前目录不是 git 仓库，请在仓库内执行。".to_string());
    }
    Ok(())
}

fn extract_package_version(content: &str) -> Option<String> {
    let regex = Regex::new(r#""version"\s*:\s*"([^"]+)""#).ok()?;
    regex
        .captures(content)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

fn replace_package_version(content: &str, new_version: &str) -> Result<String, String> {
    let regex = Regex::new(r#""version"\s*:\s*"([^"]+)""#)
        .map_err(|err| format!("正则初始化失败: {err}"))?;
    if !regex.is_match(content) {
        return Err("package.json 中未找到 version 字段，无法更新。".to_string());
    }
    Ok(regex
        .replacen(content, 1, format!("\"version\": \"{new_version}\""))
        .to_string())
}

fn parse_version(input: &str) -> Result<ParsedVersion, String> {
    let mut parts = input.split('/');
    let (prefix, version) = match (parts.next(), parts.next(), parts.next()) {
        (Some(v), None, None) => (None, v),
        (Some(p), Some(v), None) if !p.is_empty() => (Some(p), v),
        _ => return Err("版本格式应为 1.0.0 或 xxx/1.0.0".to_string()),
    };

    let nums: Vec<&str> = version.split('.').collect();
    if nums.len() != 2 && nums.len() != 3 {
        return Err("版本号需为两段或三段数字，如 1.0 或 1.0.0".to_string());
    }

    let major = parse_number(nums[0])?;
    let minor = parse_number(nums[1])?;
    let patch = if nums.len() == 3 {
        parse_number(nums[2])?
    } else {
        0
    };

    Ok(ParsedVersion {
        prefix: prefix.map(|p| p.to_string()),
        major,
        minor,
        patch,
    })
}

fn parse_number(value: &str) -> Result<u64, String> {
    if value.is_empty() || !value.chars().all(|c| c.is_ascii_digit()) {
        return Err("版本号必须为数字".to_string());
    }
    value
        .parse::<u64>()
        .map_err(|_| "版本号数字解析失败".to_string())
}

fn build_tag(version: &ParsedVersion) -> String {
    let core = format!("{}.{}.{}", version.major, version.minor, version.patch);
    match &version.prefix {
        Some(prefix) => format!("{prefix}/{core}"),
        None => core,
    }
}

fn tag_exists_local(tag: &str) -> Result<bool, String> {
    let output = git_command(&["tag", "--list", tag])?;
    Ok(!output.trim().is_empty())
}

fn remote_tag_exists(tag: &str) -> Result<bool, String> {
    let remotes = git_command(&["remote"])?;
    if !remotes.lines().any(|line| line.trim() == "origin") {
        return Err("未检测到远程 origin，请先配置远程仓库。".to_string());
    }
    let output = git_command(&["ls-remote", "--tags", "origin", &format!("refs/tags/{tag}")])?;
    Ok(!output.trim().is_empty())
}

fn print_tag_preview(current: &str, target: &str) -> Result<(), String> {
    output::info(&format!(
        "即将创建 tag：{target}（当前版本：{current}）"
    ));
    Ok(())
}

fn confirm_proceed() -> Result<bool, String> {
    output::confirm_ynq("确认继续？(y=继续, n=取消, q=退出)：")
}

fn git_command(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|err| format!("git 命令执行失败: {err}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git {:?} 执行失败: {}", args, stderr.trim()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
