use std::process::Command;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VmError {
    #[error("virsh command failed: {0}")]
    Virsh(String),
    #[error("VM `{0}` not found")]
    NotFound(String),
    #[error("failed to write XML: {0}")]
    Io(String),
}

/// VM state as reported by virsh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmState {
    Running,
    ShutOff,
    Paused,
    Other(String),
}

impl std::fmt::Display for VmState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmState::Running => write!(f, "running"),
            VmState::ShutOff => write!(f, "shut off"),
            VmState::Paused => write!(f, "paused"),
            VmState::Other(s) => write!(f, "{s}"),
        }
    }
}

fn parse_state(s: &str) -> VmState {
    match s.trim() {
        "running" => VmState::Running,
        "shut off" => VmState::ShutOff,
        "paused" => VmState::Paused,
        other => VmState::Other(other.to_string()),
    }
}

fn virsh(args: &[&str]) -> Result<String, VmError> {
    let output = Command::new("virsh")
        .args(args)
        .output()
        .map_err(|e| VmError::Virsh(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(VmError::Virsh(stderr));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Define a VM from an XML string (writes temp file, calls virsh define).
pub fn define(xml: &str) -> Result<String, VmError> {
    let tmp = "/tmp/vmctl-define.xml";
    std::fs::write(tmp, xml).map_err(|e| VmError::Io(e.to_string()))?;
    let out = virsh(&["define", tmp])?;
    let _ = std::fs::remove_file(tmp);
    Ok(out.trim().to_string())
}

/// Start a defined VM.
pub fn start(name: &str) -> Result<String, VmError> {
    virsh(&["start", name]).map(|s| s.trim().to_string())
}

/// Gracefully shut down a VM.
pub fn shutdown(name: &str) -> Result<String, VmError> {
    virsh(&["shutdown", name]).map(|s| s.trim().to_string())
}

/// Force-stop a VM.
pub fn destroy(name: &str) -> Result<String, VmError> {
    virsh(&["destroy", name]).map(|s| s.trim().to_string())
}

/// Remove a VM definition (does not delete disk).
pub fn undefine(name: &str) -> Result<String, VmError> {
    virsh(&["undefine", name]).map(|s| s.trim().to_string())
}

/// Get VM state.
pub fn state(name: &str) -> Result<VmState, VmError> {
    let out = virsh(&["domstate", name])?;
    Ok(parse_state(&out))
}

/// Parsed VM info from virsh list.
#[derive(Debug, Clone)]
pub struct VmInfo {
    pub id: Option<u32>,
    pub name: String,
    pub state: VmState,
}

/// List all VMs (both running and defined).
pub fn list_all() -> Result<Vec<VmInfo>, VmError> {
    let out = virsh(&["list", "--all"])?;
    let mut vms = Vec::new();

    for line in out.lines().skip(2) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('-') {
            continue;
        }
        // Format: " Id   Name   State"
        // Running: " 1    vm-1   running"
        // Defined: " -    vm-2   shut off"
        let parts: Vec<&str> = trimmed.splitn(3, char::is_whitespace).collect();
        if parts.len() < 3 {
            continue;
        }

        let id_str = parts[0].trim();
        let id = if id_str == "-" {
            None
        } else {
            id_str.parse().ok()
        };

        // Name and state are trickier — re-parse with better split
        let rest = trimmed[id_str.len()..].trim();
        let (name, state_str) = if let Some(pos) = rest.rfind("  ") {
            let name = rest[..pos].trim();
            let state_s = rest[pos..].trim();
            (name, state_s)
        } else {
            let parts: Vec<&str> = rest.splitn(2, char::is_whitespace).collect();
            if parts.len() == 2 {
                (parts[0].trim(), parts[1].trim())
            } else {
                continue;
            }
        };

        vms.push(VmInfo {
            id,
            name: name.to_string(),
            state: parse_state(state_str),
        });
    }
    Ok(vms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state() {
        assert_eq!(parse_state("running"), VmState::Running);
        assert_eq!(parse_state("shut off"), VmState::ShutOff);
        assert_eq!(parse_state("paused"), VmState::Paused);
        assert_eq!(parse_state("crashed"), VmState::Other("crashed".into()));
    }

    #[test]
    fn test_vm_state_display() {
        assert_eq!(format!("{}", VmState::Running), "running");
        assert_eq!(format!("{}", VmState::ShutOff), "shut off");
    }
}
