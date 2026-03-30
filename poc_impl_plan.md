# PoC Implementation Plan — VM Worker CLI

## Architecture Overview

The worker CLI (`vmctl`) manages KVM/QEMU virtual machines for CS2 case farming.
It runs on any Linux host with KVM support and manages VM lifecycle, hardware spoofing,
and dependency validation.

### Module Breakdown

- [x] **1. Dependency Checker** (`deps.rs`) — Validate host: QEMU version ≥ 9.2, libvirt, KVM module, virtiofs, qemu-img, required kernel modules.
- [x] **2. Hardware Spoofing** (`spoof.rs`) — Generate realistic MAC (real OUI), SMBIOS (manufacturer/product/serial), disk serials. Deterministic per-VM seed.
- [x] **3. VM Config / XML Generation** (`config.rs`) — Produce libvirt-compatible XML from structured Rust types. CPU/RAM, VirtIO-GPU Venus, spoofed identifiers, virtiofs mounts, VNC display.
- [x] **4. Disk Management** (`disk.rs`) — qcow2 backing store + per-VM overlay creation via `qemu-img`.
- [x] **5. VM Lifecycle** (`vm.rs`) — Create, start, stop, destroy, list VMs. Wraps `virsh` commands (no libvirt C bindings needed for PoC).
- [x] **6. CLI Interface** (`main.rs` / `cli.rs`) — Subcommands: `check-deps`, `create`, `start`, `stop`, `destroy`, `list`, `setup`.
- [x] **7. Unit Tests** — Tests for spoofing determinism, XML generation correctness, dependency version parsing.

### Non-goals for this PoC
- No RPC/server integration (Phase 3)
- No VNC/QMP automation (Phase 2)
- No AI inference (Phase 4)
- No mini-worker inside VM (future: systemd service approach)

### Design Decisions
- **No libvirt C bindings**: Use `virsh` CLI wrapper for PoC simplicity and portability.
- **No mini-worker inside VM**: For PoC, manual SSH/VNC. Later: cloud-init + systemd service.
- **Deterministic spoofing**: Each VM gets reproducible hardware IDs from a seed (VM name hash).
- **OS-agnostic**: No hard dependency on specific distro; checks required tools at runtime.
