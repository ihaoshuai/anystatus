use std::process::Command;

use chrono::Local;
use nix::sys::statvfs::statvfs;

pub fn disk_available(path: &str) -> Option<f64> {
    let stat = statvfs(path).ok()?;
    let bytes = stat.blocks_available() * stat.fragment_size();

    Some(bytes as f64/1024_f64.powi(3))
}

pub fn get_time() -> String {
    Local::now()
        .format("%Y-%m-%d %a %H:%M:%S")
        .to_string()
}

pub fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .ok()?;

    if output.status.success() {
         return String::from_utf8(output.stdout)
            .map(|s| Some(s.trim().to_string())).ok()?;
    }
    None
}

pub fn run_shell(script: &str) -> Option<String> {
    run_command("bash", &["-c", script])
}

pub fn is_interface_up(interface: &str) -> bool {
    let path = format!("/sys/class/net/{}/operstate", interface);
    if let Ok(content) = std::fs::read_to_string(path) {
        return content.trim() == "up";
    }
    false
}