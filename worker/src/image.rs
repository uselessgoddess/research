//! Base image preparation and setup for CS2 farming VMs.
//!
//! Handles qcow2 base image management including:
//! - Cloud image download tracking
//! - NBD-based offline image customization
//! - cloud-init configuration generation
//! - Base image metadata

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("command failed ({cmd}): {reason}")]
    Command { cmd: String, reason: String },
    #[error("path not found: {0}")]
    NotFound(String),
    #[error("I/O error: {0}")]
    Io(String),
}

/// Describes the base image and its configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseImageConfig {
    /// Path to the qcow2 base image.
    pub image_path: String,
    /// OS type (e.g., "debian-14", "ubuntu-26.04").
    pub os_type: String,
    /// Packages to install during image preparation.
    pub packages: Vec<String>,
    /// Whether the image has been prepared (packages installed, services configured).
    pub prepared: bool,
}

impl Default for BaseImageConfig {
    fn default() -> Self {
        Self {
            image_path: "/var/lib/vmctl/base/ubuntu-26.04.qcow2".into(),
            os_type: "ubuntu-26.04".into(),
            packages: default_packages(),
            prepared: false,
        }
    }
}

/// Default packages required for CS2 farming VM.
fn default_packages() -> Vec<String> {
    [
        "qemu-guest-agent",
        "steam-installer",
        "mesa-vulkan-drivers",
        "vulkan-tools",      // Добавлено: утилиты для дебага (vulkaninfo)
        "mesa-utils",  
        "gamescope", // Wayland микро-композитор
        "xwayland",  // Прослойка для Steam
        "dmidecode",
        "lsblk",
        "curl",
        "ca-certificates",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

/// Generate cloud-init user-data YAML for automatic VM provisioning.
pub fn generate_cloud_init_userdata(farm_user: &str, farm_password: &str) -> String {
    format!(
        r#"#cloud-config
hostname: cs2-farm-vm
manage_etc_hosts: true

growpart:
  mode: auto

users:
  - name: {user}
    groups: [sudo, video, render]
    shell: /bin/bash
    lock_passwd: false
    plain_text_passwd: "{password}"
    sudo: ALL=(ALL) NOPASSWD:ALL

apt:
  sources:
    multiverse:
      source: "deb http://archive.ubuntu.com/ubuntu/ $RELEASE multiverse"
    multiverse-updates:
      source: "deb http://archive.ubuntu.com/ubuntu/ $RELEASE-updates multiverse"
    multiverse-security:
      source: "deb http://security.ubuntu.com/ubuntu/ $RELEASE-security multiverse"

bootcmd:
  - sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="[^"]*/& console=ttyS0/' /etc/default/grub
  - update-grub
  - dpkg --add-architecture i386
  - apt-get update

package_update: true
packages:
  - qemu-guest-agent
  - mesa-vulkan-drivers
  - vulkan-tools
  - mesa-utils
  - gamescope
  - xwayland
  - dmidecode

runcmd:
  # Включаем guest-agent
  - systemctl enable --now qemu-guest-agent

  - DEBIAN_FRONTEND=noninteractive apt-get install -y steam-installer

  # Создаем скрипт запуска Steam
  - mkdir -p /opt/farm
  - |
    cat > /opt/farm/steam-launcher.sh << 'LAUNCHER'
    #!/bin/bash
    while [ ! -f /home/{user}/.steam/steam/config/.ready ]; do
        sleep 2
    done
    rm -f /home/{user}/.steam/steam/config/.ready

    # Запускаем Steam и сразу стартуем CS2 (app 730)
    # -nosound: отключаем звук для экономии ресурсов
    # -novid: пропускаем заставку
    exec steam -silent -no-browser -console \
        -applaunch 730 -nosound -novid -windowed -w 384 -h 288 +connect_lobby default
    LAUNCHER
  - chmod +x /opt/farm/steam-launcher.sh
  - chown {user}:{user} /opt/farm/steam-launcher.sh

  # Создаем systemd сервис для Gamescope
  - |
    cat > /etc/systemd/system/steam-farm.service << 'SERVICE'
    [Unit]
    Description=Steam CS2 Farming Session (Gamescope)
    After=network.target

    [Service]
    Type=simple
    User={user}
    Environment=XDG_RUNTIME_DIR=/run/user/1000
    
    # Создаем директорию для Wayland-сокетов (т.к. мы запускаем сервис вне сессии пользователя)
    ExecStartPre=/bin/mkdir -p /run/user/1000
    ExecStartPre=/bin/chown {user}:{user} /run/user/1000
    ExecStartPre=/bin/chmod 0700 /run/user/1000

    # Запускаем Gamescope с лимитом 20 FPS
    ExecStart=/usr/bin/gamescope -w 384 -h 288 -W 384 -H 288 -r 20 -e -- /opt/farm/steam-launcher.sh
    Restart=on-failure
    RestartSec=10

    [Install]
    WantedBy=multi-user.target
    SERVICE
  - systemctl daemon-reload
  - systemctl enable steam-farm.service

  # Монтируем общую папку CS2
  - mkdir -p /opt/cs2
  - |
    echo 'cs2 /opt/cs2 virtiofs defaults,nofail 0 0' >> /etc/fstab

final_message: "CS2 farm VM provisioning complete (Gamescope No-Sound Mode)"
"#,
        user = farm_user,
        password = farm_password,
    )
}

/// Generate cloud-init meta-data for a specific VM instance.
pub fn generate_cloud_init_metadata(vm_name: &str) -> String {
    format!(
        r#"instance-id: {name}
local-hostname: {name}
"#,
        name = vm_name
    )
}

/// Create a cloud-init ISO (seed image) for VM provisioning.
///
/// Generates user-data and meta-data files and packs them into an ISO
/// using `genisoimage` or `mkisofs`.
pub fn create_cloud_init_iso(
    output_path: &str,
    vm_name: &str,
    farm_user: &str,
    farm_password: &str,
) -> Result<(), ImageError> {
    let tmp_dir = format!("/tmp/vmctl-cloud-init-{vm_name}");
    std::fs::create_dir_all(&tmp_dir).map_err(|e| ImageError::Io(e.to_string()))?;

    let userdata = generate_cloud_init_userdata(farm_user, farm_password);
    let metadata = generate_cloud_init_metadata(vm_name);

    std::fs::write(format!("{tmp_dir}/user-data"), userdata)
        .map_err(|e| ImageError::Io(e.to_string()))?;
    std::fs::write(format!("{tmp_dir}/meta-data"), metadata)
        .map_err(|e| ImageError::Io(e.to_string()))?;

    // Try genisoimage first, fall back to mkisofs
    let iso_cmd = if which_exists("genisoimage") {
        "genisoimage"
    } else {
        "mkisofs"
    };

    let output = Command::new(iso_cmd)
        .args([
            "-output",
            output_path,
            "-volid",
            "cidata",
            "-joliet",
            "-rock",
            &format!("{tmp_dir}/user-data"),
            &format!("{tmp_dir}/meta-data"),
        ])
        .output()
        .map_err(|e| ImageError::Command {
            cmd: iso_cmd.to_string(),
            reason: e.to_string(),
        })?;

    // Cleanup temp files
    let _ = std::fs::remove_dir_all(&tmp_dir);

    if !output.status.success() {
        return Err(ImageError::Command {
            cmd: iso_cmd.to_string(),
            reason: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }
    Ok(())
}

/// Resize a qcow2 image.
pub fn resize_image(image_path: &str, size: &str) -> Result<(), ImageError> {
    if !Path::new(image_path).exists() {
        return Err(ImageError::NotFound(image_path.to_string()));
    }

    let output = Command::new("qemu-img")
        .args(["resize", image_path, size])
        .output()
        .map_err(|e| ImageError::Command {
            cmd: "qemu-img resize".into(),
            reason: e.to_string(),
        })?;

    if !output.status.success() {
        return Err(ImageError::Command {
            cmd: "qemu-img resize".into(),
            reason: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }
    Ok(())
}

/// Convert an image to compressed qcow2 format.
pub fn compress_image(input_path: &str, output_path: &str) -> Result<(), ImageError> {
    if !Path::new(input_path).exists() {
        return Err(ImageError::NotFound(input_path.to_string()));
    }

    let output = Command::new("qemu-img")
        .args(["convert", "-c", "-O", "qcow2", input_path, output_path])
        .output()
        .map_err(|e| ImageError::Command {
            cmd: "qemu-img convert".into(),
            reason: e.to_string(),
        })?;

    if !output.status.success() {
        return Err(ImageError::Command {
            cmd: "qemu-img convert".into(),
            reason: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }
    Ok(())
}

fn which_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_packages() {
        let pkgs = default_packages();
        assert!(pkgs.contains(&"qemu-guest-agent".to_string()));
        assert!(pkgs.contains(&"steam-installer".to_string()));
        assert!(pkgs.contains(&"mesa-vulkan-drivers".to_string()));
        assert!(pkgs.contains(&"gamescope".to_string())); // Изменено с openbox
        assert!(pkgs.contains(&"dmidecode".to_string()));
    }

    #[test]
    fn test_cloud_init_userdata_contains_user() {
        let ud = generate_cloud_init_userdata("farmuser", "secret123");
        assert!(ud.contains("name: farmuser"));
        assert!(ud.contains("plain_text_passwd: \"secret123\""));
        assert!(ud.contains("qemu-guest-agent"));
        assert!(ud.contains("steam-farm.service"));
        assert!(ud.contains("steam-launcher.sh"));
    }

    #[test]
    fn test_cloud_init_userdata_contains_virtiofs_mount() {
        let ud = generate_cloud_init_userdata("farmuser", "pass");
        assert!(ud.contains("/opt/cs2"));
        assert!(ud.contains("virtiofs"));
    }

    #[test]
    fn test_cloud_init_metadata() {
        let md = generate_cloud_init_metadata("cs2-farm-0");
        assert!(md.contains("instance-id: cs2-farm-0"));
        assert!(md.contains("local-hostname: cs2-farm-0"));
    }

    #[test]
    fn test_base_image_config_default() {
        let cfg = BaseImageConfig::default();
        assert!(!cfg.prepared);
        assert_eq!(cfg.os_type, "debian-14");
        assert!(!cfg.packages.is_empty());
    }

    #[test]
    fn test_base_image_config_serialization() {
        let cfg = BaseImageConfig::default();
        let json = serde_json::to_string(&cfg).unwrap();
        let parsed: BaseImageConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.os_type, cfg.os_type);
        assert_eq!(parsed.packages.len(), cfg.packages.len());
    }

    #[test]
    fn test_resize_not_found() {
        let err = resize_image("/nonexistent/image.qcow2", "30G");
        assert!(matches!(err, Err(ImageError::NotFound(_))));
    }

    #[test]
    fn test_compress_not_found() {
        let err = compress_image("/nonexistent/input.qcow2", "/tmp/output.qcow2");
        assert!(matches!(err, Err(ImageError::NotFound(_))));
    }

    #[test]
    fn test_cloud_init_userdata_is_valid_yaml_header() {
        let ud = generate_cloud_init_userdata("user", "pass");
        assert!(ud.starts_with("#cloud-config\n"));
    }
}
