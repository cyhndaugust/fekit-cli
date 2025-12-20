use std::env;
use std::process::Command;

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
