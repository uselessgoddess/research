# RUN_INSTRUCTIONS — vmctl Worker PoC

## Prerequisites

### Hardware
- x86_64 CPU with Intel VT-x or AMD-V
- Recommended: 32 GB RAM for 8+ VMs (each uses ~2 GB)
- GPU for VirtIO-GPU Venus (Mesa 24.2+, kernel 6.13+)

### Software (host)
| Package | Min Version | Purpose |
|---------|-------------|---------|
| QEMU | 9.2+ | VM hypervisor with Venus GPU support |
| libvirt + virsh | any recent | VM lifecycle management |
| qemu-img | (matches QEMU) | Disk overlay creation |
| virtiofsd | any | Shared filesystem (CS2 binaries) |
| KVM kernel module | loaded | Hardware virtualization |

### Install dependencies

**Ubuntu / Debian:**
```bash
sudo apt-get install -y qemu-system-x86 qemu-utils libvirt-daemon-system virtinst virtiofsd
sudo systemctl enable --now libvirtd
sudo modprobe kvm kvm_intel  # or kvm_amd
```

**Gentoo:**
```bash
sudo emerge -av app-emulation/qemu app-emulation/libvirt app-emulation/virtiofsd
sudo rc-update add libvirtd default
sudo rc-service libvirtd start
sudo modprobe kvm kvm_intel  # or kvm_amd
```

**Arch Linux:**
```bash
sudo pacman -S qemu-full libvirt virtiofsd
sudo systemctl enable --now libvirtd
```

### User permissions
```bash
sudo usermod -aG libvirt,kvm $(whoami)
# Log out and back in for group changes to take effect
```

## Build

```bash
cd worker
cargo build --release
# Binary: target/release/worker
```

## Usage

### 1. Check host dependencies
```bash
./target/release/worker check-deps
```

### 2. Prepare a base disk image

Download or create a Linux base image (e.g., Ubuntu Server or Arch with OpenBox):
```bash
# Example: download a cloud image
wget https://cloud-images.ubuntu.com/jammy/current/jammy-server-cloudimg-amd64.img \
  -O /var/lib/vmctl/base/ubuntu-22.04.qcow2
```

### 3. Create a single VM
```bash
./target/release/worker create \
  --name cs2-test-1 \
  --ram 2048 \
  --cpus 2 \
  --vnc-port 5901 \
  --base-disk /var/lib/vmctl/base/ubuntu-22.04.qcow2 \
  --disk-dir /var/lib/vmctl/disks
```

### 4. Batch-create multiple VMs
```bash
./target/release/worker setup \
  --base-disk /var/lib/vmctl/base/ubuntu-22.04.qcow2 \
  --count 4 \
  --prefix cs2-farm \
  --ram 2048 \
  --cpus 2 \
  --vnc-start 5901 \
  --disk-dir /var/lib/vmctl/disks
```

### 5. VM lifecycle
```bash
./target/release/worker list
./target/release/worker start cs2-farm-0
./target/release/worker stop cs2-farm-0
./target/release/worker destroy cs2-farm-0    # force-stop
./target/release/worker undefine cs2-farm-0   # remove definition
```

### 6. Inspect spoofed identity
```bash
./target/release/worker show-identity cs2-farm-0
```
Output (JSON):
```json
{
  "mac_address": "a4:bb:6d:xx:xx:xx",
  "smbios_manufacturer": "Dell Inc.",
  "smbios_product": "OptiPlex 7080",
  "smbios_serial": "SVC1234567",
  "disk_serial": "SAM12345678",
  "disk_model": "Samsung SSD 870 EVO 500GB"
}
```

### 7. With virtiofs (shared CS2 directory)
```bash
# On the host, share /opt/cs2 to all VMs:
./target/release/worker create \
  --name cs2-vm \
  --base-disk /var/lib/vmctl/base/ubuntu-22.04.qcow2 \
  --virtiofs-source /opt/cs2

# Inside the guest:
sudo mount -t virtiofs cs2 /opt/cs2
```

## Architecture

```
vmctl (this binary)
├── check-deps     → validates QEMU, libvirt, KVM, virtiofsd
├── create         → generates spoofed HW identity + qcow2 overlay + libvirt XML
├── setup          → batch-creates N VMs
├── start/stop     → virsh start/shutdown
├── destroy        → virsh destroy (force)
├── undefine       → virsh undefine
├── list           → virsh list --all
└── show-identity  → deterministic HW spoofing preview
```

## Next Steps (beyond this PoC)

1. **Inside-VM automation**: cloud-init + systemd service for Steam login, CS2 launch
2. **VNC/QMP controller**: read VM display, inject keyboard/mouse from the host worker
3. **shai RPC integration**: connect to central server for account queue, heartbeat
4. **KSM + memory tuning**: enable kernel same-page merging for multi-VM density
5. **Venus GPU validation**: test VirtIO-GPU Venus with CS2 at 384×288
