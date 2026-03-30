use std::path::Path;
use std::process::Command;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiskError {
    #[error("qemu-img failed: {0}")]
    QemuImg(String),
    #[error("backing image not found: {0}")]
    BackingNotFound(String),
    #[error("overlay already exists: {0}")]
    OverlayExists(String),
}

/// Create a new qcow2 base image of the given size (e.g. "20G").
pub fn create_base_image(path: &str, size: &str) -> Result<(), DiskError> {
    let output = Command::new("qemu-img")
        .args(["create", "-f", "qcow2", path, size])
        .output()
        .map_err(|e| DiskError::QemuImg(e.to_string()))?;

    if !output.status.success() {
        return Err(DiskError::QemuImg(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    Ok(())
}

/// Create a thin-provisioned qcow2 overlay backed by a base image.
pub fn create_overlay(overlay_path: &str, backing_path: &str) -> Result<(), DiskError> {
    if !Path::new(backing_path).exists() {
        return Err(DiskError::BackingNotFound(backing_path.to_string()));
    }
    if Path::new(overlay_path).exists() {
        return Err(DiskError::OverlayExists(overlay_path.to_string()));
    }

    let output = Command::new("qemu-img")
        .args([
            "create",
            "-f",
            "qcow2",
            "-F",
            "qcow2",
            "-b",
            backing_path,
            overlay_path,
        ])
        .output()
        .map_err(|e| DiskError::QemuImg(e.to_string()))?;

    if !output.status.success() {
        return Err(DiskError::QemuImg(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    Ok(())
}

/// Get info about a qcow2 image (JSON output from qemu-img info).
pub fn image_info(path: &str) -> Result<String, DiskError> {
    let output = Command::new("qemu-img")
        .args(["info", "--output=json", path])
        .output()
        .map_err(|e| DiskError::QemuImg(e.to_string()))?;

    if !output.status.success() {
        return Err(DiskError::QemuImg(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backing_not_found() {
        let err = create_overlay("/tmp/test-overlay.qcow2", "/nonexistent/base.qcow2");
        assert!(matches!(err, Err(DiskError::BackingNotFound(_))));
    }
}
