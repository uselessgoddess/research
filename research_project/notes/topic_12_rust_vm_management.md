# Topic 12: Rust VM Management — Libvirt XML Configuration Approaches

## Available Rust Crates for Libvirt / VM Management

### Primary: `virt` crate (official)
- **Crate:** [virt](https://crates.io/crates/virt) v0.4.3
- **Downloads:** ~185K all-time, ~41K recent — by far the most popular
- **Repository:** https://gitlab.com/libvirt/libvirt-rust (official libvirt project)
- **License:** LGPL-2.1
- **Maintainers:** libvirt core team (berrange / Red Hat, 20 contributors, 557 commits)
- **Requires:** `libvirt-dev` / `libvirt-devel` system package (FFI bindings to C library)
- **Features:**
  - `qemu` — enables `libvirt-qemu` functions like `qemu_monitor_command`
  - `bindgen_regenerate` — regenerate FFI bindings (maintainers only)
- **API style:** Direct mapping of the C API to Rust idioms. `virDomainCreate` becomes `dom.create()`, `virDomainPinVcpu` becomes `dom.pin_vcpu()`.
- **Modules:** connect, domain, domain_snapshot, error, event, interface, network, nodedev, nwfilter, secret, storage_pool, storage_vol, stream
- **Maturity:** Stable, actively maintained (last commit March 2026), CI on multiple platforms

### Alternative libvirt crates (not recommended)
| Crate | Version | Downloads | Notes |
|---|---|---|---|
| `libvirt` | 0.1.0 | 3,288 | Abandoned (10+ years old) |
| `libvirt-pure` | 0.1.1 | 26 | Pure Rust client, brand new, unproven |
| `libvirt-rpc` | 0.1.12 | 20,368 | Protocol-level implementation, 8 years stale |
| `libvirt-sys` | 1.2.18 | 4,122 | Raw bindings, 10+ years old |
| `vmadm` | 0.5.0 | 5,141 | Higher-level CLI tool for local libvirt VMs |

### XML handling crates
| Crate | Purpose | Key feature |
|---|---|---|
| `quick-xml` | XML reader/writer | ~50x faster than xml-rs, serde support via `serialize` feature |
| `serde-xml-rs` | Serde XML de/serialization | Convention-based, quick-xml follows its conventions |
| `roxmltree` | Read-only XML tree | Very fast parsing, no writing support |

## Approaches to Generating/Managing Libvirt XML from Rust

### Approach 1: `virt` crate API (recommended)

Use the official `virt` crate to interact with libvirt. Domain creation accepts XML strings, but the lifecycle management (start, stop, migrate, snapshot, monitor) is all handled through the typed API.

```rust
use virt::connect::Connect;
use virt::domain::Domain;

let conn = Connect::open(Some("qemu:///system"))?;

// Define a domain from XML
let xml = include_str!("domain_template.xml");
let dom = Domain::define_xml(&conn, xml)?;
dom.create()?; // start the VM

// Query state
let info = dom.get_info()?;
println!("State: {}, Memory: {} KB", info.state, info.memory);

// Get the current XML (libvirt fills in defaults)
let live_xml = dom.get_xml_desc(0)?;
```

**XML generation sub-options within this approach:**

#### 1a. Hardcoded XML templates with string interpolation
```rust
let xml = format!(r#"
<domain type='kvm'>
  <name>{name}</name>
  <memory unit='MiB'>{memory}</memory>
  <vcpu>{vcpus}</vcpu>
  <os><type arch='x86_64'>hvm</type></os>
  <devices>
    <disk type='file' device='disk'>
      <source file='{disk_path}'/>
      <target dev='vda' bus='virtio'/>
    </disk>
  </devices>
</domain>
"#, name=name, memory=memory, vcpus=vcpus, disk_path=disk_path);
```
- Simplest approach
- Fragile: no validation, easy to produce invalid XML
- Fine for a fixed VM config that rarely changes

#### 1b. Typed Rust structs with serde + quick-xml
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "domain")]
struct DomainXml {
    #[serde(rename = "@type")]
    domain_type: String,
    name: String,
    memory: Memory,
    vcpu: u32,
    os: Os,
    devices: Devices,
}

// Serialize to XML
let xml = quick_xml::se::to_string(&domain)?;
```
- Type-safe, compile-time guarantees
- Can deserialize XML returned by libvirt back into structs
- Significant upfront work to model the full libvirt domain schema
- quick-xml serde support handles attributes via `@` prefix, text via `$text`

#### 1c. Programmatic XML building with quick-xml Writer
```rust
use quick_xml::Writer;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};

let mut writer = Writer::new(Cursor::new(Vec::new()));
let mut domain = BytesStart::new("domain");
domain.push_attribute(("type", "kvm"));
writer.write_event(Event::Start(domain))?;
// ... build rest of tree
```
- Full control, no template issues
- Verbose but correct
- Good for dynamic configs with many conditional sections

### Approach 2: Shell out to `virt-install`

```rust
use std::process::Command;

Command::new("virt-install")
    .args(&[
        "--name", &name,
        "--memory", &memory.to_string(),
        "--vcpus", &vcpus.to_string(),
        "--disk", &format!("path={},bus=virtio", disk_path),
        "--import",
        "--os-variant", "ubuntu22.04",
        "--noautoconsole",
    ])
    .status()?;
```

### Approach 3: Pure Rust libvirt client (`libvirt-pure` / `libvirt-rpc`)
- Avoids C library dependency
- Too immature / abandoned for production use

## Comparison Table

| Criterion | virt crate + XML templates | virt crate + serde structs | virt crate + quick-xml Writer | Shell to virt-install | Direct XML files + virsh |
|---|---|---|---|---|---|
| **Type safety** | Low (string interpolation) | High (compile-time) | Medium (runtime) | None | None |
| **Libvirt C dep** | Yes | Yes | Yes | No (CLI only) | No (CLI only) |
| **Setup complexity** | Low | High (model schema) | Medium | Very low | Very low |
| **Runtime flexibility** | Medium | High | High | Low (flag-based) | Low |
| **Error handling** | Poor (invalid XML at runtime) | Good (serde errors) | Good (writer errors) | Poor (parse stderr) | Poor |
| **Lifecycle mgmt** | Full (start/stop/migrate/snap) | Full | Full | Limited | Manual |
| **Monitoring** | Full (events, stats, info) | Full | Full | None | Manual polling |
| **Parallel VM mgmt** | Excellent (threaded connections) | Excellent | Excellent | Process overhead | Script overhead |
| **Deployment** | Needs libvirt-dev at build | Same | Same | Needs virt-install pkg | Needs virsh pkg |
| **Best for** | Fixed configs, fast iteration | Complex dynamic configs | Dynamic configs, medium complexity | Prototyping, one-off VMs | Manual sysadmin |

## Direct XML Management vs Libvirt API

### Why the libvirt API (`virt` crate) is better than raw XML + virsh:

1. **Atomic operations:** The API handles locking, state transitions, and error reporting properly. Shelling out to `virsh` requires parsing text output.

2. **Event-driven monitoring:** The `virt` crate exposes libvirt's event loop — you can register callbacks for domain lifecycle events (started, stopped, crashed) instead of polling.

3. **Resource management:** Connection pooling, automatic cleanup via Rust's `Drop` trait, proper error types.

4. **Concurrency:** A single `Connect` object can be shared across threads. Running 10+ VMs in parallel is straightforward with the API; with virsh you'd need to manage subprocess pools.

5. **Live state queries:** `dom.get_info()`, `dom.get_xml_desc()`, `dom.memory_stats()` etc. give structured data directly — no XML parsing needed for monitoring.

### When raw XML management might be acceptable:
- If VMs are set-and-forget (define once, rarely change)
- If you want zero Rust build dependencies (no libvirt-dev)
- If the orchestrator is a simple shell script, not a Rust service

## Recommendation for This Project

**Use the `virt` crate (v0.4.3) with template-based XML generation (Approach 1a), upgrading to serde structs (1b) if complexity grows.**

Rationale:

1. **The `virt` crate is the clear choice** for libvirt interaction from Rust. It is the official binding maintained by the libvirt project itself, has 185K downloads, active maintenance, and covers the full domain lifecycle API.

2. **Start with string-template XML.** For a CS2 case farming system, the VM configuration is largely identical across workers — same OS, same resource allocation, same disk layout. A simple `format!()` template with a handful of variables (VM name, MAC address, disk path) is sufficient and avoids overengineering.

3. **Keep the option to upgrade to serde structs.** If the project later needs per-worker hardware variation (different GPUs, different memory sizes, different network configs), define Rust structs with `#[derive(Serialize)]` and use `quick-xml` serde serialization. The `quick-xml` crate is fast, well-maintained, and its serde support is mature.

4. **Do not shell out to virt-install or virsh.** The orchestrator needs to manage many VMs in parallel, monitor their state, and react to events. This requires the programmatic API, not CLI tools.

5. **Dependencies to add:**
   ```toml
   [dependencies]
   virt = { version = "0.4", features = ["qemu"] }
   quick-xml = { version = "0.36", features = ["serialize"] }  # for future XML struct work
   serde = { version = "1", features = ["derive"] }
   ```

6. **System requirement:** `libvirt-dev` package must be installed on build machines and `libvirtd` must be running on worker nodes. This is already assumed since we are using KVM/QEMU.

## Minimal Example: VM Lifecycle with `virt` Crate

```rust
use virt::connect::Connect;
use virt::domain::Domain;
use virt::sys;

fn create_worker_vm(
    conn: &Connect,
    name: &str,
    memory_mb: u64,
    vcpus: u32,
    disk_path: &str,
    mac_addr: &str,
) -> Result<Domain, virt::error::Error> {
    let xml = format!(r#"
<domain type='kvm'>
  <name>{name}</name>
  <memory unit='MiB'>{memory_mb}</memory>
  <vcpu placement='static'>{vcpus}</vcpu>
  <os>
    <type arch='x86_64' machine='pc-q35-8.2'>hvm</type>
    <boot dev='hd'/>
  </os>
  <features>
    <acpi/><apic/><kvm><hidden state='on'/></kvm>
  </features>
  <cpu mode='host-passthrough'/>
  <devices>
    <disk type='file' device='disk'>
      <driver name='qemu' type='qcow2' cache='writeback'/>
      <source file='{disk_path}'/>
      <target dev='vda' bus='virtio'/>
    </disk>
    <interface type='network'>
      <mac address='{mac_addr}'/>
      <source network='default'/>
      <model type='virtio'/>
    </interface>
    <graphics type='vnc' port='-1' autoport='yes'/>
    <memballoon model='virtio'/>
  </devices>
</domain>
"#);

    let dom = Domain::define_xml(conn, &xml)?;
    dom.create()?;
    Ok(dom)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connect::open(Some("qemu:///system"))?;

    let dom = create_worker_vm(
        &conn, "cs2-worker-01", 4096, 4,
        "/var/lib/libvirt/images/cs2-worker-01.qcow2",
        "52:54:00:12:34:01",
    )?;

    let info = dom.get_info()?;
    println!("VM state: {}, memory: {} KB", info.state, info.memory);

    // Graceful shutdown
    dom.shutdown()?;

    Ok(())
}
```

## Sources
- https://crates.io/crates/virt (official Rust bindings, 185K downloads)
- https://gitlab.com/libvirt/libvirt-rust (source repository, 557 commits)
- https://docs.rs/virt/latest/virt/ (API documentation)
- https://libvirt.org/formatdomain.html (domain XML format reference)
- https://crates.io/crates/quick-xml (XML handling, 50x faster than xml-rs)
- https://crates.io/search?q=libvirt (27 libvirt-related crates)
