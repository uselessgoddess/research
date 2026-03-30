Title: zram - ArchWiki

URL Source: https://wiki.archlinux.org/title/Zram

Published Time: Sun, 29 Mar 2026 23:21:18 GMT

Markdown Content:
[zram](https://docs.kernel.org/admin-guide/blockdev/zram.html), formerly called compcache, is a Linux kernel module for creating a compressed block device in RAM, i.e. a RAM disk with on-the-fly disk compression. The block device created with zram can then be used for swap or as a general-purpose RAM disk. The two most common uses for zram are for the storage of temporary files (`/tmp`) and as a swap device. Initially, zram had only the latter function, hence the original name "compcache" ("compressed cache").

## Usage as swap

Initially the created zram block device does not reserve or use any RAM. Only as files need or want to be swapped out, they will be compressed and moved into the zram block device. The zram block device will then dynamically grow or shrink as required.

Even when assuming that zstd only achieves a conservative 1:2 compression ratio (real world data shows a common ratio of 1:3), zram will offer the advantage of being able to store more content in RAM than without memory compression.

**Note**

*   When configuring zram, the size of the zram device controls the maximum uncompressed amount of data it can store, _not_ the maximum compressed size. You can configure the zram's size to be equal to or even greater than your system's physical RAM capacity, as long as the compressed size on physical RAM will not exceed your system's physical RAM capacity.
*   If the related [zswap](https://wiki.archlinux.org/title/Zswap "Zswap") kernel feature remains enabled, it will prevent zram from being used effectively. This is because zswap functions as a swap cache in front of zram, intercepting and compressing evicted memory pages before they can reach zram. Despite the output of [zramctl(8)](https://man.archlinux.org/man/zramctl.8), most of zswap is unused in this circumstance. Therefore, it's recommended to permanently [disable zswap](https://wiki.archlinux.org/title/Zswap#Toggling_zswap "Zswap") using the kernel parameter or sysfs setting before starting.
*   Hibernating to swap on zram is not supported, even when zram is configured with a backing device on permanent storage. _logind_ will protect against trying to hibernate to a swap space on zram.

**Tip** You can specify the maximum amount of memory zram can use to store the compressed data with the `mem_limit` parameter.

A simple size to start with is half of the total system memory.

### Manually

To set up one zstd compressed zram device with half the system memory capacity and a higher-than-normal priority (only for the current session):

# modprobe zram
# zramctl /dev/zram0 --algorithm zstd --size "$(($(grep -Po 'MemTotal:\s*\K\d+' /proc/meminfo)/2))KiB"
# mkswap -U clear /dev/zram0
# swapon --discard --priority 100 /dev/zram0

To disable it again, either reboot or run:

# swapoff /dev/zram0
# modprobe -r zram

A detailed explanation of all steps, options and potential problems is provided in the [official documentation of the zram module](https://docs.kernel.org/admin-guide/blockdev/zram.html).

For a permanent solution, use a method from one of the following sections.

### Using a udev rule

The example below describes how to set up swap on zram automatically at boot with a single udev rule and an fstab entry. No additional packages are needed to make this work.

Explicitly [load the module at boot](https://wiki.archlinux.org/title/Load_the_module_at_boot "Load the module at boot"):

/etc/modules-load.d/zram.conf zram

Create the following [udev rule](https://wiki.archlinux.org/title/Udev_rule "Udev rule") adjusting the `disksize` attribute as necessary:

/etc/udev/rules.d/99-zram.rules ACTION=="add", KERNEL=="zram0", ATTR{initstate}=="0", ATTR{comp_algorithm}="zstd", ATTR{disksize}="4G", TAG+="systemd"
Add `/dev/zram` to your [fstab](https://wiki.archlinux.org/title/Fstab "Fstab") with a higher than default priority and the `x-systemd.makefs` option:

/etc/fstab/dev/zram0 none swap defaults,discard,pri=100,x-systemd.makefs 0 0
### Using zram-generator

[zram-generator](https://archlinux.org/packages/?name=zram-generator) is a systemd-related project hosted under its [GitHub organization](https://github.com/systemd/zram-generator) that makes use of its conventions and mechanisms.

It provides `systemd-zram-setup@zramN.service` units to automatically initialize zram devices without users needing to [enable/start](https://wiki.archlinux.org/title/Enable/start "Enable/start") the template or its instances. See [zram-generator(8)](https://man.archlinux.org/man/zram-generator.8).

To use it, [install](https://wiki.archlinux.org/title/Install "Install")[zram-generator](https://archlinux.org/packages/?name=zram-generator), and create `/etc/systemd/zram-generator.conf` with the following:

/etc/systemd/zram-generator.conf[zram0]
This is sufficient to get a zram device called `zram0` created with the default options (see [zram-generator.conf(5)](https://man.archlinux.org/man/zram-generator.conf.5) for their current values).

By default the zram device will use **half the RAM size** but **no more than 4 GiB**: _e.g._ it will create a 2 GiB zram device if you have 4 GiB of RAM, but only a 4 GiB zram device if you have 8 GiB of memory or more.

If you wish to change this, use the `zram-size` parameter:

/etc/systemd/zram-generator.conf[zram0]
zram-size = min(ram / 2, 16384)
This will allow the zram device to use half the total amount of memory up to 16 GiB (`min(x,y)`compares the two values and picks the smallest).

*   if you want to unconditionally allow the zram device to use half the memory, use `zram-size = ram / 2`
*   for a fixed size, use _e.g._`zram-size = 8192`.

See [zram-generator.conf(5) § EXAMPLES](https://man.archlinux.org/man/zram-generator.conf.5#EXAMPLES) for graphs and a more complex example.

To use a different [compression algorithm](https://wiki.archlinux.org/title/Zram#Compression_algorithm) than the kernel's current default, use `compression-algorithm`:

/etc/systemd/zram-generator.conf[zram0]
compression-algorithm = lzo-rle zstd(level=3) (type=idle)
Subsequent algorithms are used for recompression.

See [zram-generator.conf(5)](https://man.archlinux.org/man/zram-generator.conf.5) for a list of all available options.

Once a configuration has been defined, run [daemon-reload](https://wiki.archlinux.org/title/Daemon-reload "Daemon-reload") and [start](https://wiki.archlinux.org/title/Start "Start") your configured `systemd-zram-setup@zramN.service` instance (`N` matching the numerical instance-ID, in the example it is `systemd-zram-setup@zram0.service`).

You can [check the swap status](https://wiki.archlinux.org/title/Swap#Swap_space "Swap") of your configured `/dev/zramN` device(s) by reading the [unit status](https://wiki.archlinux.org/title/Unit_status "Unit status") of your `systemd-zram-setup@zramN.service` instance(s), by using [zramctl(8)](https://man.archlinux.org/man/zramctl.8), or by using [swapon(8)](https://man.archlinux.org/man/swapon.8).

### Using zramswap

[zramswap](https://aur.archlinux.org/packages/zramswap/)AUR provides an automated script for setting up a swap with a higher priority and a default size of 20% of the RAM size of your system. To do this automatically on every boot, [enable](https://wiki.archlinux.org/title/Enable "Enable")`zramswap.service`.

## Tips and tricks

### Checking zram statistics

Use [zramctl(8)](https://man.archlinux.org/man/zramctl.8). Example:

$ zramctl NAME       ALGORITHM DISKSIZE  DATA  COMPR  TOTAL STREAMS MOUNTPOINT
/dev/zram0 zstd           32G  1.9G 318.6M 424.9M         [SWAP]

*   DISKSIZE = 32G: this zram device will store up to 32 GiB of uncompressed data.
*   DATA = 1.9G: currently, 1.9 GiB (uncompressed) of data is being stored in this zram device
*   COMPR = 318.6M: the 1.9 GiB uncompressed data was compressed to 318.6 MiB
*   TOTAL = 424.9M: including metadata, the 1.9 GiB of uncompressed data is using up 424.9 MiB of physical RAM

### Compression algorithm

Use `cat /sys/block/zramN/comp_algorithm` after creating the zram device _N_ to get the available compression algorithms, as well as the currently selected one (included in brackets).

Results with [linux](https://archlinux.org/packages/?name=linux) 6.17.9.arch1-1 using default options:

lzo-rle lzo lz4 lz4hc [zstd] deflate 842

### Multiple zram devices

By default, loading the `zram` module creates a single `/dev/zram0` device.

If you need more than one `/dev/zram` device, specify the amount using the `num_devices`[kernel module parameter](https://wiki.archlinux.org/title/Kernel_module_parameter "Kernel module parameter") or [add them as needed afterwards](https://docs.kernel.org/admin-guide/blockdev/zram.html#add-remove-zram-devices).

### Optimizing swap on zram

Since zram behaves differently than disk swap, we can configure the system's swap to take full potential of the zram advantages:

/etc/sysctl.d/99-vm-zram-parameters.conf vm.swappiness = 180
vm.watermark_boost_factor = 0
vm.watermark_scale_factor = 125
vm.page-cluster = 0
Explanation of the configuration:

These values are [what Pop!_OS uses](https://github.com/pop-os/default-settings/pull/163). That Pop!_OS GitHub pull request also links to [some testing done by users on r/Fedora](https://old.reddit.com/r/Fedora/comments/mzun99/new_zram_tuning_benchmarks/), which determined that `vm.page-cluster = 0` is ideal. They also found a high swappiness value to be ideal, which matches what is suggested by [the kernel docs](https://docs.kernel.org/admin-guide/sysctl/vm.html):

The default value is 60.For in-memory swap, like zram or zswap, as well as hybrid setups that have swap on faster devices than the filesystem, values beyond 100 can be considered. For example, if the random IO against the swap device is on average 2x faster than IO from the filesystem, swappiness should be 133 (x + 2x = 200, 2x = 133.33).
On a system with a hard drive, random I/O against the in-memory device would be orders of magnitude faster than I/O against the filesystem, so swappiness should be ~200. Even on a system with a fast SSD, a high swappiness value may be ideal.

### Enabling a backing device for a zram block

zram can be configured to push incompressible pages to a specified block device:

To add a backing device manually:

# echo /dev/_sdX_ > /sys/block/zram0/backing_dev

To add a backing device to your zram block device using _zram-generator_, update `/etc/systemd/zram-generator.conf` with the following under your `[zramX]` device you want the backing device added to:

/etc/systemd/zram-generator.conf writeback-device=/dev/disk/by-partuuid/_XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX_
Incompressible pages can then be pushed to the block device by executing:

# echo huge > /sys/block/zramX/writeback

### Using zram for non-swap purposes

zram can also be used as a generic RAM-backed block device, e.g. a `/dev/ram` with less physical memory usage, but slightly lower performance. However there are some caveats:

*   There is no partition table support (no automatic creation of `/dev/zramxpy`).
*   The block size is fixed to 4 kiB.

The obvious way around this is to stack a loop device on-top the zram, using _losetup_, specifying the desired block size using the `-b` option and the `-P` option to process partition tables and automatic creation of the partition loop devices.

# zramctl -f -s _N_G/dev/zram_x_
Copy the disk image to the new `/dev/zramx`, then create a loop device. If the disk image has a partition table, the block size of the loop device must match the block size used by the partition table, which is typically 512 or 4096 bytes.

# losetup -f -b 512 -P /dev/zram_x_
# ls /dev/loop*/dev/loop0 /dev/loop0p1 /dev/loop0p2# mount /dev/loop0p1 /mnt/boot
# mount /dev/loop0p2 /mnt/root

**Note**

*   The zram device numbering depends on pre-existing zram devices and its size should be enough to hold the disk image.
*   The output from `ls /dev/loop*` depends on the contents of the disk image.

## See also

*   [Wikipedia:zram](https://en.wikipedia.org/wiki/zram "wikipedia:zram")
*   [https://github.com/pop-os/default-settings/pull/163](https://github.com/pop-os/default-settings/pull/163)
*   [https://www.reddit.com/r/pop_os/comments/znh9n6/help_test_a_zram_optimization_for_pop_os/](https://www.reddit.com/r/pop_os/comments/znh9n6/help_test_a_zram_optimization_for_pop_os/)
