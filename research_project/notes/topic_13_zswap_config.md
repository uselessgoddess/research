# Topic 13: zswap Configuration Best Practices for KVM/QEMU VM Hosts

## 1. What zswap Is

zswap is a kernel feature providing a compressed RAM cache that sits **in front of** a traditional swap device (disk/NVMe). Pages being swapped out are first compressed and stored in a dynamically allocated RAM pool. When the pool fills up, the least recently used (LRU) pages are evicted to the backing swap device. This is fundamentally different from zram, which IS the swap device itself.

Key benefit for VM hosts (from kernel docs): "Overcommitted guests that share a common I/O resource can dramatically reduce their swap I/O pressure, avoiding heavy handed I/O throttling by the hypervisor."

## 2. zswap Kernel Parameters

All parameters are exposed via `/sys/module/zswap/parameters/` and can be set at boot via kernel command line or at runtime via sysfs.

| Parameter | Default | Description |
|---|---|---|
| `enabled` | Y (kernel-dependent) | Enable/disable zswap |
| `compressor` | zstd | Compression algorithm (zstd, lz4, lzo, lz4hc, deflate, 842) |
| `max_pool_percent` | 20 | Max percentage of total RAM for the compressed pool |
| `shrinker_enabled` | Y | Proactively evict cold pages to swap under memory pressure |
| `accept_threshold_percent` | 90 | After pool hits limit, only accept new pages when pool shrinks below this % of max |

View current parameters:
```bash
grep -r . /sys/module/zswap/parameters/
```

View statistics:
```bash
grep -r . /sys/kernel/debug/zswap/
```

Key statistics to monitor:
- `stored_pages` — number of pages currently in zswap pool
- `pool_total_size` — actual compressed pool size in bytes
- `written_back_pages` — pages evicted to disk (high count = pool too small or wrong workload)
- `reject_compress_poor` — pages rejected due to poor compression (incompressible data)
- `pool_limit_hit` — number of times the pool hit its maximum size

## 3. zswap vs zram — Updated Comparison

| Feature | zswap | zram |
|---|---|---|
| Architecture | Compressed cache in front of disk swap | Compressed block device used as swap |
| Requires backing swap device | **Yes** (disk/NVMe) | No |
| LRU eviction to disk | Yes, automatic | Only with explicit backing_dev config |
| Page eviction correctness | Maintained via shrinker | Can invert LRU — full zram leads to OOM or long hangs |
| zswap + zram interaction | **Conflicts** — zswap intercepts pages before they reach zram, making zram ineffective | Must disable zswap to use zram properly |
| Default in modern distros | Ubuntu 22.04+, Fedora 33+ | ChromeOS, Pop!_OS (with systemd-zram-generator) |
| Best for VM hosts with NVMe | **Yes** — graceful overflow to fast disk | Less ideal — no automatic disk overflow |
| Hibernation support | Compatible (works with disk swap) | Not supported for hibernation |

**Recommendation for KVM hosts:** Use **zswap** (not zram) when the host has NVMe/SSD swap. zswap provides a safety net: compressed hot pages stay in RAM, cold pages gracefully overflow to disk. This avoids the catastrophic "zram full" scenario where VMs hang waiting for OOM resolution.

Use zram only on diskless/embedded hosts or when disk swap is intentionally avoided.

## 4. Enabling zswap on Ubuntu 22.04+

### Ubuntu 22.04 (Jammy)
zswap is compiled into the kernel but may not be enabled by default. Enable it:

```bash
# Check current status
cat /sys/module/zswap/parameters/enabled

# Enable at runtime (temporary)
echo 1 | sudo tee /sys/module/zswap/parameters/enabled

# Enable permanently via GRUB
sudo sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="\(.*\)"/GRUB_CMDLINE_LINUX_DEFAULT="\1 zswap.enabled=1 zswap.compressor=zstd zswap.max_pool_percent=25 zswap.shrinker_enabled=1"/' /etc/default/grub
sudo update-grub
sudo reboot
```

### Ubuntu 23.10+ / 24.04 LTS
zswap is enabled by default (`CONFIG_ZSWAP_DEFAULT_ON=y`). Ubuntu 24.04 also ships with zswap enabled by default. Verify:

```bash
zgrep CONFIG_ZSWAP_DEFAULT_ON /proc/config.gz 2>/dev/null || grep CONFIG_ZSWAP_DEFAULT_ON /boot/config-$(uname -r)
cat /sys/module/zswap/parameters/enabled
dmesg | grep zswap
```

### Important: Disable zram if using zswap
If the system has zram enabled (some Ubuntu flavors), disable it:
```bash
sudo systemctl disable --now systemd-zram-setup@zram0.service 2>/dev/null
sudo swapoff /dev/zram0 2>/dev/null
```

zswap sits in front of swap and will intercept pages before they reach zram, making zram a wasted memory allocation. Always use one or the other, never both.

## 5. Recommended Compressor: lz4 vs zstd

| Compressor | Compression Ratio | Compression Speed | Decompression Speed | CPU Cost |
|---|---|---|---|---|
| lz4 | ~2.6:1 | Fastest | Fastest | Lowest |
| lzo-rle | ~2.7:1 | Fast | Fast | Low |
| zstd | ~3.4–5.0:1 | Moderate | Fast | Moderate |
| lz4hc | ~3.0:1 | Slow | Fastest | High (compress only) |

### Recommendation for KVM VM hosts: **zstd**

Rationale:
- **VM workloads are memory-bound, not CPU-bound** on the host. The host CPUs are mostly idle while VMs run their own workloads. The extra CPU for zstd compression is available.
- **Better compression ratio means more pages fit in the pool.** With zstd at ~3.5:1, a 25% pool on a 32 GB host (8 GB pool) effectively caches ~28 GB of swap pages. With lz4 at 2.6:1, the same pool caches ~21 GB.
- **Decompression speed is what matters for latency** (page faults). zstd decompression is nearly as fast as lz4 decompression.
- zstd is the default in modern kernels (6.x) for good reason.

When to use lz4 instead:
- Host CPU is saturated (all cores assigned to VMs with pinning)
- Extremely latency-sensitive workloads where every microsecond matters
- Older/weaker CPUs without hardware acceleration

To change compressor at boot:
```bash
# In GRUB_CMDLINE_LINUX_DEFAULT:
zswap.compressor=zstd

# Or at runtime:
echo zstd | sudo tee /sys/module/zswap/parameters/compressor
```

Note: When changing compressor at runtime, existing compressed pages remain in their old pool with the old compressor. They are decompressed using the original algorithm when faulted back. New pages use the new compressor. Old pool is freed only when all its pages are evicted.

## 6. Pool Size Recommendations for 32 GB+ Hosts

The `max_pool_percent` parameter controls the maximum percentage of **total system RAM** that zswap's compressed pool can use.

### Sizing Guidelines

| Host RAM | VM Count | Recommended max_pool_percent | Effective Pool | Estimated Cached Swap (zstd ~3.5:1) |
|---|---|---|---|---|
| 32 GB | 4–8 VMs | 25% | 8 GB | ~28 GB |
| 32 GB | 8–12 VMs | 30% | ~10 GB | ~35 GB |
| 64 GB | 8–16 VMs | 20–25% | 13–16 GB | ~45–56 GB |
| 128 GB | 16–32 VMs | 15–20% | 19–26 GB | ~67–90 GB |

### Key considerations:

1. **Do not set too high.** If `max_pool_percent` is 50%, zswap itself could consume half your RAM, leaving insufficient memory for VMs and causing more swapping in a vicious cycle.

2. **The pool is NOT pre-allocated.** It grows on demand. Setting 25% means "allow up to 25%", not "reserve 25%".

3. **Monitor `pool_limit_hit`.** If this counter is increasing rapidly, the pool is too small and pages are being evicted to disk frequently. Increase the percentage or add more RAM.

4. **accept_threshold_percent tuning.** Default 90% means: after pool fills up, stop accepting new pages until pool drops below 90% of max. For VM hosts under sustained pressure, lowering to 80% can prevent thrashing:
   ```bash
   echo 80 | sudo tee /sys/module/zswap/parameters/accept_threshold_percent
   ```

5. **Shrinker should be enabled** for VM hosts. It proactively evicts cold pages to disk, keeping hot pages in the compressed pool:
   ```bash
   echo Y | sudo tee /sys/module/zswap/parameters/shrinker_enabled
   ```

### Recommended for 32 GB host running 8 VMs:
```
zswap.enabled=1 zswap.compressor=zstd zswap.max_pool_percent=25 zswap.shrinker_enabled=1
```

## 7. Integration with KSM for VM Workloads

KSM (Kernel Same-page Merging) and zswap operate on different layers and are **complementary**:

| Layer | Technology | What It Does |
|---|---|---|
| Active RAM | KSM | Deduplicates identical pages across VMs (CoW) |
| Swap cache | zswap | Compresses pages being swapped out |
| Swap device | NVMe/SSD | Stores overflow from zswap pool |

### How they work together for VM hosts:

1. **KSM reduces active memory first.** For identical VMs (same OS, same application), KSM can deduplicate 30–50% of shared pages. This means fewer pages need to be swapped at all.

2. **zswap catches overflow.** When VMs collectively exceed physical RAM even after KSM deduplication, zswap compresses the evicted pages and keeps them in RAM.

3. **Disk swap is the last resort.** Only when zswap pool is full do pages hit disk.

### Combined configuration for KVM host:

```bash
# --- KSM ---
echo 1 | sudo tee /sys/kernel/mm/ksm/run
echo 1000 | sudo tee /sys/kernel/mm/ksm/pages_to_scan    # Aggressive scanning
echo 50 | sudo tee /sys/kernel/mm/ksm/sleep_millisecs     # Scan frequently
echo 512 | sudo tee /sys/kernel/mm/ksm/max_page_sharing

# --- zswap ---
echo 1 | sudo tee /sys/module/zswap/parameters/enabled
echo zstd | sudo tee /sys/module/zswap/parameters/compressor
echo 25 | sudo tee /sys/module/zswap/parameters/max_pool_percent
echo Y | sudo tee /sys/module/zswap/parameters/shrinker_enabled

# --- THP (compatible with KSM on Linux 6.1+) ---
echo madvise | sudo tee /sys/kernel/mm/transparent_hugepage/enabled
echo defer+madvise | sudo tee /sys/kernel/mm/transparent_hugepage/defrag
```

### Important notes:
- **KSM is incompatible with static huge pages** but works with THP in madvise mode on newer kernels.
- **KSM + zswap order matters:** KSM deduplicates first (reducing memory footprint), then zswap compresses whatever is swapped out. They do not interfere with each other.
- QEMU/KVM automatically marks guest RAM as `MADV_MERGEABLE`, so KSM works without guest-side configuration.

### Expected memory savings stack (32 GB host, 8 identical VMs at 2 GB each):
| Layer | Before | After | Savings |
|---|---|---|---|
| Raw provisioned | 16 GB | 16 GB | — |
| After KSM (~40% dedup) | 16 GB | ~10 GB active | ~6 GB |
| zswap pool (25% = 8 GB) | — | Caches ~28 GB overflow | Avoids disk I/O |
| Host + QEMU overhead | — | ~3 GB | — |
| **Effective RAM needed** | 16 GB | **~13 GB** | Fits in 32 GB with headroom |

## 8. Practical Configuration Steps

### Step-by-step for Ubuntu 22.04+ KVM Host (32 GB+ RAM)

#### A. Ensure NVMe swap exists
```bash
# Check existing swap
swapon --show

# If no swap, create a swap file on NVMe (zswap needs a backing device)
sudo fallocate -l 16G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Add to /etc/fstab for persistence
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
```

#### B. Configure kernel boot parameters
```bash
# Edit GRUB config
sudo nano /etc/default/grub

# Add to GRUB_CMDLINE_LINUX_DEFAULT:
# zswap.enabled=1 zswap.compressor=zstd zswap.max_pool_percent=25 zswap.shrinker_enabled=1

sudo update-grub
```

#### C. Configure sysctl for VM host workload
```bash
cat <<'EOF' | sudo tee /etc/sysctl.d/99-vm-host-zswap.conf
# Swap tuning for zswap on KVM host
vm.swappiness = 120
# Higher than default (60) to prefer swapping to zswap (compressed RAM)
# over dropping file caches. Not as high as 180 (zram recommendation)
# because zswap has disk writeback overhead.

vm.vfs_cache_pressure = 200
# More aggressive reclaim of dentry/inode caches

vm.dirty_background_ratio = 5
vm.dirty_ratio = 20
# Reasonable writeback thresholds

vm.page-cluster = 0
# Read single pages from swap (no readahead). Compressed swap
# does not benefit from sequential readahead.
EOF

sudo sysctl --system
```

#### D. Configure KSM
```bash
cat <<'EOF' | sudo tee /etc/sysctl.d/99-ksm.conf
# KSM will be enabled via a separate script since it uses /sys not /proc
EOF

# Create systemd service for KSM + zswap runtime settings
cat <<'EOF' | sudo tee /etc/systemd/system/vm-memory-optimize.service
[Unit]
Description=VM Host Memory Optimization (KSM + zswap tuning)
After=multi-user.target

[Service]
Type=oneshot
RemainAfterExit=yes

# KSM
ExecStart=/bin/sh -c 'echo 1 > /sys/kernel/mm/ksm/run'
ExecStart=/bin/sh -c 'echo 1000 > /sys/kernel/mm/ksm/pages_to_scan'
ExecStart=/bin/sh -c 'echo 50 > /sys/kernel/mm/ksm/sleep_millisecs'

# THP
ExecStart=/bin/sh -c 'echo madvise > /sys/kernel/mm/transparent_hugepage/enabled'
ExecStart=/bin/sh -c 'echo defer+madvise > /sys/kernel/mm/transparent_hugepage/defrag'

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable --now vm-memory-optimize.service
```

#### E. Verify everything is working
```bash
# Check zswap
echo "=== zswap status ==="
grep -r . /sys/module/zswap/parameters/
echo ""
echo "=== zswap stats ==="
sudo grep -r . /sys/kernel/debug/zswap/
echo ""

# Check KSM
echo "=== KSM status ==="
cat /sys/kernel/mm/ksm/run
pages_sharing=$(cat /sys/kernel/mm/ksm/pages_sharing)
pages_shared=$(cat /sys/kernel/mm/ksm/pages_shared)
echo "KSM saving: $(( (pages_sharing - pages_shared) * 4 / 1024 )) MB"
echo ""

# Check swap
echo "=== Swap ==="
swapon --show
echo ""

# Check THP
echo "=== THP ==="
cat /sys/kernel/mm/transparent_hugepage/enabled
```

#### F. Monitoring script (optional)
```bash
#!/bin/bash
# /usr/local/bin/vm-memory-monitor.sh
while true; do
    echo "--- $(date) ---"

    # zswap
    stored=$(cat /sys/kernel/debug/zswap/stored_pages 2>/dev/null)
    pool_size=$(cat /sys/kernel/debug/zswap/pool_total_size 2>/dev/null)
    written_back=$(cat /sys/kernel/debug/zswap/written_back_pages 2>/dev/null)
    pool_limit=$(cat /sys/kernel/debug/zswap/pool_limit_hit 2>/dev/null)
    if [ -n "$stored" ]; then
        pool_mb=$((pool_size / 1024 / 1024))
        stored_mb=$((stored * 4 / 1024))
        if [ "$pool_size" -gt 0 ] && [ "$stored" -gt 0 ]; then
            ratio=$(echo "scale=1; $stored_mb / $pool_mb" | bc)
        else
            ratio="N/A"
        fi
        echo "zswap: pool=${pool_mb}MB stored=${stored_mb}MB ratio=${ratio}:1 writeback=${written_back} limit_hits=${pool_limit}"
    fi

    # KSM
    sharing=$(cat /sys/kernel/mm/ksm/pages_sharing)
    shared=$(cat /sys/kernel/mm/ksm/pages_shared)
    ksm_mb=$(( (sharing - shared) * 4 / 1024 ))
    echo "KSM: saving ${ksm_mb}MB (${sharing} pages sharing, ${shared} pages shared)"

    # Overall
    free -h | head -2
    echo ""
    sleep 30
done
```

## 9. Summary: Recommended Configuration for KVM Host

| Setting | Value | Rationale |
|---|---|---|
| zswap.enabled | 1 | Enable compressed swap cache |
| zswap.compressor | zstd | Best ratio for VM host (CPU usually available) |
| zswap.max_pool_percent | 25 | Good balance for 32 GB+ (8 GB pool caches ~28 GB) |
| zswap.shrinker_enabled | Y | Proactively evict cold pages |
| vm.swappiness | 120 | Prefer zswap over dropping caches |
| vm.page-cluster | 0 | No swap readahead for compressed swap |
| KSM | Enabled, aggressive | 30–50% dedup across identical VMs |
| THP | madvise | Compatible with KSM, moderate TLB improvement |
| Backing swap | NVMe, 16–32 GB | Fast overflow target for zswap evictions |
| zram | **Disabled** | Conflicts with zswap; do not use both |

## Sources
- https://www.kernel.org/doc/html/latest/admin-guide/mm/zswap.html (fetched via Jina Reader)
- https://wiki.archlinux.org/title/Zswap (fetched via Jina Reader)
- https://wiki.archlinux.org/title/Zram (fetched via Jina Reader)
- https://chrisdown.name/2026/03/24/zswap-vs-zram-when-to-use-what.html
- https://docs.kernel.org/admin-guide/sysctl/vm.html
