//! upgrade 子命令实现：自动下载并替换为最新版本。

use crate::output;
use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 执行 upgrade 子命令，下载并更新到最新版本。
///
/// 参数：
/// - 无。
///
/// 返回：
/// - `Ok(())`：升级成功或已给出后续操作提示。
/// - `Err(String)`：升级失败并附带错误信息。
pub fn run_upgrade_command() -> Result<(), String> {
    if let Some(latest) = fetch_latest_version().unwrap_or(None) {
        let current = normalize_version(CURRENT_VERSION);
        let latest_norm = normalize_version(&latest);
        if current == latest_norm {
            output::success(&format!("已是最新版本：{CURRENT_VERSION}"));
            return Ok(());
        }
        output::info(&format!(
            "当前版本：{CURRENT_VERSION}，最新版本：{latest}，开始升级..."
        ));
    }

    let (os, arch, ext) = detect_platform()?;
    let asset = format!("fekit-{os}-{arch}{ext}");
    let url = format!(
        "https://github.com/cyhndaugust/fekit-cli/releases/latest/download/{asset}"
    );

    output::info(&format!("准备下载最新版本：{asset}"));
    let exe_path = current_exe_path()?;
    let tmp_path = temp_path_for(&exe_path);

    download_to_path(&url, &tmp_path)?;
    set_executable(&tmp_path)?;

    match replace_executable(&tmp_path, &exe_path) {
        Ok(()) => {
            output::success("升级完成，已替换当前可执行文件。");
            Ok(())
        }
        Err(err) => {
            let fallback = fallback_path_for(&exe_path, &ext);
            fs::rename(&tmp_path, &fallback)
                .map_err(|e| format!("写入备用文件失败: {e}"))?;
            output::warn(&format!(
                "无法直接替换当前可执行文件：{err}"
            ));
            output::warn(&format!(
                "已下载新版本到：{}，请退出后手动替换。",
                fallback.display()
            ));
            Ok(())
        }
    }
}

/// 获取当前可执行文件路径。
///
/// 参数：
/// - 无。
///
/// 返回：
/// - `Ok(PathBuf)`：当前可执行文件路径。
/// - `Err(String)`：读取失败。
fn current_exe_path() -> Result<PathBuf, String> {
    env::current_exe().map_err(|err| format!("获取当前可执行文件失败: {err}"))
}

/// 识别当前平台，返回 OS、ARCH 和扩展名。
///
/// 参数：
/// - 无。
///
/// 返回：
/// - `Ok((String, String, String))`：分别为 OS、ARCH、扩展名。
/// - `Err(String)`：不支持的平台。
fn detect_platform() -> Result<(String, String, String), String> {
    let os = match env::consts::OS {
        "linux" => "linux",
        "macos" => "macos",
        "windows" => "windows",
        other => return Err(format!("暂不支持的操作系统: {other}")),
    };

    let arch = match env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "arm64",
        other => return Err(format!("暂不支持的架构: {other}")),
    };

    let ext = if os == "windows" { ".exe" } else { "" };
    Ok((os.to_string(), arch.to_string(), ext.to_string()))
}

/// 获取最新发布版本号（从 GitHub release）。
///
/// 参数：
/// - 无。
///
/// 返回：
/// - `Ok(Some(String))`：成功获取到版本号。
/// - `Ok(None)`：未解析到版本号。
/// - `Err(String)`：请求失败或解析失败。
fn fetch_latest_version() -> Result<Option<String>, String> {
    let url = "https://api.github.com/repos/cyhndaugust/fekit-cli/releases/latest";
    let response = ureq::get(url)
        .set("User-Agent", "fekit-cli")
        .call()
        .map_err(|err| format!("获取最新版本失败: {err}"))?;
    let body = response
        .into_string()
        .map_err(|err| format!("读取版本信息失败: {err}"))?;

    let regex = Regex::new(r#""tag_name"\s*:\s*"([^"]+)""#)
        .map_err(|err| format!("版本解析失败: {err}"))?;
    Ok(regex
        .captures(&body)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string())))
}

/// 规范化版本号，去掉前导 v（兼容旧标签）。
///
/// 参数：
/// - `version`：原始版本字符串。
///
/// 返回：
/// - `String`：规范化后的版本号。
fn normalize_version(version: &str) -> String {
    version.trim().trim_start_matches('v').to_string()
}

/// 生成临时下载文件路径。
///
/// 参数：
/// - `exe_path`：当前可执行文件路径。
///
/// 返回：
/// - `PathBuf`：临时文件路径。
fn temp_path_for(exe_path: &Path) -> PathBuf {
    let mut tmp = exe_path.to_path_buf();
    let suffix = if exe_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        == "exe"
    {
        "tmp.exe"
    } else {
        "tmp"
    };
    tmp.set_extension(suffix);
    tmp
}

/// 下载文件到指定路径。
///
/// 参数：
/// - `url`：下载地址。
/// - `dest`：目标路径。
///
/// 返回：
/// - `Ok(())`：下载成功。
/// - `Err(String)`：下载失败。
fn download_to_path(url: &str, dest: &Path) -> Result<(), String> {
    let response = ureq::get(url)
        .call()
        .map_err(|err| format!("下载失败: {err}"))?;

    let mut reader = response
        .into_reader();
    let mut file = fs::File::create(dest).map_err(|err| format!("创建文件失败: {err}"))?;
    std::io::copy(&mut reader, &mut file)
        .map_err(|err| format!("写入文件失败: {err}"))?;
    Ok(())
}

/// 设置可执行权限（非 Windows）。
///
/// 参数：
/// - `path`：目标文件路径。
///
/// 返回：
/// - `Ok(())`：设置成功或无需设置。
/// - `Err(String)`：设置失败。
fn set_executable(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)
            .map_err(|err| format!("读取权限失败: {err}"))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)
            .map_err(|err| format!("设置执行权限失败: {err}"))?;
    }
    Ok(())
}

/// 替换当前可执行文件。
///
/// 参数：
/// - `tmp`：临时文件路径。
/// - `dest`：目标文件路径。
///
/// 返回：
/// - `Ok(())`：替换成功。
/// - `Err(String)`：替换失败。
fn replace_executable(tmp: &Path, dest: &Path) -> Result<(), String> {
    fs::rename(tmp, dest).map_err(|err| format!("替换失败: {err}"))
}

/// 生成无法替换时的备用文件路径。
///
/// 参数：
/// - `dest`：目标文件路径。
/// - `ext`：目标扩展名。
///
/// 返回：
/// - `PathBuf`：备用文件路径。
fn fallback_path_for(dest: &Path, ext: &str) -> PathBuf {
    let mut fallback = dest.to_path_buf();
    if ext == ".exe" {
        fallback.set_extension("new.exe");
    } else {
        fallback.set_extension("new");
    }
    fallback
}
