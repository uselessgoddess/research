use std::process::Command;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DepError {
    #[error("command `{cmd}` not found — install it first")]
    NotFound { cmd: String },
    #[error("`{cmd}` version {found} is below minimum {required}")]
    VersionTooLow {
        cmd: String,
        found: String,
        required: String,
    },
    #[error("kernel module `{module}` is not loaded")]
    ModuleNotLoaded { module: String },
    #[error("failed to execute `{cmd}`: {reason}")]
    ExecFailed { cmd: String, reason: String },
}

#[derive(Debug)]
pub struct DepCheck {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

/// Parse a semver-like version string (e.g. "9.2.1") into (major, minor, patch).
pub fn parse_version(s: &str) -> Option<(u32, u32, u32)> {
    // Extract first token that looks like a version (digits and dots)
    let token = s.split_whitespace().find(|t| {
        let first = t.chars().next().unwrap_or(' ');
        first.is_ascii_digit()
    })?;
    let parts: Vec<&str> = token.split('.').collect();
    let major = parts.first()?.parse().ok()?;
    let minor = parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|p| p.parse().ok()).unwrap_or(0);
    Some((major, minor, patch))
}

fn version_at_least(found: (u32, u32, u32), required: (u32, u32, u32)) -> bool {
    found >= required
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<String, DepError> {
    let output = Command::new(cmd).args(args).output().map_err(|_| DepError::NotFound {
        cmd: cmd.to_string(),
    })?;
    if !output.status.success() {
        return Err(DepError::ExecFailed {
            cmd: cmd.to_string(),
            reason: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn check_qemu() -> Result<DepCheck, DepError> {
    let out = run_cmd("qemu-system-x86_64", &["--version"])?;
    let ver = parse_version(&out).ok_or_else(|| DepError::ExecFailed {
        cmd: "qemu-system-x86_64".into(),
        reason: format!("cannot parse version from: {out}"),
    })?;
    let required = (9, 2, 0);
    if !version_at_least(ver, required) {
        return Err(DepError::VersionTooLow {
            cmd: "qemu-system-x86_64".into(),
            found: format!("{}.{}.{}", ver.0, ver.1, ver.2),
            required: format!("{}.{}.{}", required.0, required.1, required.2),
        });
    }
    Ok(DepCheck {
        name: "qemu".into(),
        ok: true,
        detail: format!("QEMU {}.{}.{}", ver.0, ver.1, ver.2),
    })
}

fn check_qemu_img() -> Result<DepCheck, DepError> {
    let out = run_cmd("qemu-img", &["--version"])?;
    let ver = parse_version(&out).ok_or_else(|| DepError::ExecFailed {
        cmd: "qemu-img".into(),
        reason: format!("cannot parse version from: {out}"),
    })?;
    Ok(DepCheck {
        name: "qemu-img".into(),
        ok: true,
        detail: format!("qemu-img {}.{}.{}", ver.0, ver.1, ver.2),
    })
}

fn check_virsh() -> Result<DepCheck, DepError> {
    let out = run_cmd("virsh", &["--version"])?;
    let ver = parse_version(out.trim()).ok_or_else(|| DepError::ExecFailed {
        cmd: "virsh".into(),
        reason: format!("cannot parse version from: {out}"),
    })?;
    Ok(DepCheck {
        name: "virsh (libvirt)".into(),
        ok: true,
        detail: format!("libvirt {}.{}.{}", ver.0, ver.1, ver.2),
    })
}

fn check_kvm_module() -> Result<DepCheck, DepError> {
    let modules = std::fs::read_to_string("/proc/modules").map_err(|e| DepError::ExecFailed {
        cmd: "read /proc/modules".into(),
        reason: e.to_string(),
    })?;
    let kvm_loaded = modules.lines().any(|l| {
        let name = l.split_whitespace().next().unwrap_or("");
        name == "kvm" || name == "kvm_intel" || name == "kvm_amd"
    });
    if !kvm_loaded {
        return Err(DepError::ModuleNotLoaded {
            module: "kvm / kvm_intel / kvm_amd".into(),
        });
    }
    Ok(DepCheck {
        name: "KVM".into(),
        ok: true,
        detail: "kvm module loaded".into(),
    })
}

fn check_virtiofsd() -> Result<DepCheck, DepError> {
    // virtiofsd may be at different paths
    let out = run_cmd("virtiofsd", &["--version"]);
    match out {
        Ok(s) => Ok(DepCheck {
            name: "virtiofsd".into(),
            ok: true,
            detail: s.trim().to_string(),
        }),
        Err(DepError::NotFound { .. }) => {
            // Try /usr/libexec path
            let out2 = run_cmd("/usr/libexec/virtiofsd", &["--version"]);
            match out2 {
                Ok(s) => Ok(DepCheck {
                    name: "virtiofsd".into(),
                    ok: true,
                    detail: s.trim().to_string(),
                }),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

/// Run all dependency checks, returning results and overall pass/fail.
pub fn check_all() -> (Vec<DepCheck>, Vec<DepError>) {
    let checks: Vec<Box<dyn Fn() -> Result<DepCheck, DepError>>> = vec![
        Box::new(check_qemu),
        Box::new(check_qemu_img),
        Box::new(check_virsh),
        Box::new(check_kvm_module),
        Box::new(check_virtiofsd),
    ];

    let mut ok = Vec::new();
    let mut errors = Vec::new();
    for check in checks {
        match check() {
            Ok(c) => ok.push(c),
            Err(e) => errors.push(e),
        }
    }
    (ok, errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version_qemu() {
        let s = "QEMU emulator version 9.2.1 (Debian 1:9.2.1+ds-2)";
        assert_eq!(parse_version(s), Some((9, 2, 1)));
    }

    #[test]
    fn test_parse_version_simple() {
        assert_eq!(parse_version("10.0.0"), Some((10, 0, 0)));
    }

    #[test]
    fn test_parse_version_two_parts() {
        assert_eq!(parse_version("8.2"), Some((8, 2, 0)));
    }

    #[test]
    fn test_parse_version_garbage() {
        assert_eq!(parse_version("no version here"), None);
    }

    #[test]
    fn test_version_at_least() {
        assert!(version_at_least((9, 2, 0), (9, 2, 0)));
        assert!(version_at_least((10, 0, 0), (9, 2, 0)));
        assert!(!version_at_least((9, 1, 9), (9, 2, 0)));
    }
}
