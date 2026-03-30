use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Real OUI prefixes from major NIC vendors (Intel, Realtek, Broadcom).
const OUI_PREFIXES: &[[u8; 3]] = &[
    [0x3C, 0x97, 0x0E], // Intel
    [0xA4, 0xBB, 0x6D], // Intel
    [0x00, 0x1B, 0x21], // Intel
    [0x68, 0x05, 0xCA], // Intel
    [0xE8, 0x6A, 0x64], // Intel
    [0x00, 0xE0, 0x4C], // Realtek
    [0x52, 0x54, 0xAB], // Realtek
    [0x28, 0x6E, 0xD4], // Realtek
    [0xB0, 0x25, 0xAA], // Realtek
    [0x00, 0x10, 0x18], // Broadcom
    [0x00, 0x24, 0xD7], // Broadcom
];

/// Realistic SMBIOS manufacturer/product pairs.
const SMBIOS_SYSTEMS: &[(&str, &str)] = &[
    ("Dell Inc.", "OptiPlex 7080"),
    ("Dell Inc.", "Latitude 5520"),
    ("HP", "EliteDesk 800 G6"),
    ("HP", "ProDesk 400 G7"),
    ("Lenovo", "ThinkCentre M920q"),
    ("Lenovo", "ThinkPad T14 Gen 2"),
    ("ASUS", "PRIME B560M-A"),
    ("Gigabyte Technology Co., Ltd.", "B560M DS3H"),
    ("MSI", "MAG B560 TOMAHAWK WIFI"),
    ("ASRock", "B560M Steel Legend"),
];

/// Realistic disk model names.
const DISK_MODELS: &[&str] = &[
    "WDC WD10EZEX-08WN4A0",
    "Samsung SSD 870 EVO 500GB",
    "Seagate ST2000DM008-2UB102",
    "KINGSTON SA400S37480G",
    "Crucial CT500MX500SSD1",
    "Toshiba HDWD110",
    "WDC WDS500G2B0A-00SM50",
    "Intel SSDSC2BB480G7",
    "Samsung SSD 980 PRO 1TB",
    "SK hynix PC711 NVMe 512GB",
];

/// Create a deterministic RNG from a VM name.
fn rng_from_seed(vm_name: &str) -> StdRng {
    let mut hasher = DefaultHasher::new();
    vm_name.hash(&mut hasher);
    let seed = hasher.finish();
    StdRng::seed_from_u64(seed)
}

/// Spoofed hardware identity for a single VM.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HwIdentity {
    pub mac_address: String,
    pub smbios_manufacturer: String,
    pub smbios_product: String,
    pub smbios_serial: String,
    pub disk_serial: String,
    pub disk_model: String,
}

/// Generate a deterministic spoofed identity for a given VM name.
pub fn generate_identity(vm_name: &str) -> HwIdentity {
    let mut rng = rng_from_seed(vm_name);

    // MAC address
    let oui = OUI_PREFIXES[rng.random_range(0..OUI_PREFIXES.len())];
    let mac = format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        oui[0],
        oui[1],
        oui[2],
        rng.random_range(0u8..=255),
        rng.random_range(0u8..=255),
        rng.random_range(0u8..=255),
    );

    // SMBIOS
    let sys = SMBIOS_SYSTEMS[rng.random_range(0..SMBIOS_SYSTEMS.len())];
    let serial = format!(
        "{}{:07}",
        if sys.0.starts_with("Dell") {
            "SVC"
        } else if sys.0.starts_with("HP") {
            "CZC"
        } else if sys.0.starts_with("Lenovo") {
            "PF"
        } else {
            "SN"
        },
        rng.random_range(1000000u32..9999999),
    );

    // Disk
    let model = DISK_MODELS[rng.random_range(0..DISK_MODELS.len())];
    let disk_serial = format!(
        "{}{:08X}",
        &model[..3].to_uppercase().replace(' ', ""),
        rng.random::<u32>(),
    );

    HwIdentity {
        mac_address: mac,
        smbios_manufacturer: sys.0.to_string(),
        smbios_product: sys.1.to_string(),
        smbios_serial: serial,
        disk_serial,
        disk_model: model.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_identity() {
        let id1 = generate_identity("test-vm-1");
        let id2 = generate_identity("test-vm-1");
        assert_eq!(id1.mac_address, id2.mac_address);
        assert_eq!(id1.smbios_serial, id2.smbios_serial);
        assert_eq!(id1.disk_serial, id2.disk_serial);
    }

    #[test]
    fn test_different_vms_different_ids() {
        let id1 = generate_identity("vm-alpha");
        let id2 = generate_identity("vm-beta");
        assert_ne!(id1.mac_address, id2.mac_address);
    }

    #[test]
    fn test_mac_format() {
        let id = generate_identity("some-vm");
        let parts: Vec<&str> = id.mac_address.split(':').collect();
        assert_eq!(parts.len(), 6);
        for part in &parts {
            assert_eq!(part.len(), 2);
            assert!(u8::from_str_radix(part, 16).is_ok());
        }
    }

    #[test]
    fn test_mac_uses_real_oui() {
        let id = generate_identity("oui-check");
        let bytes: Vec<u8> = id
            .mac_address
            .split(':')
            .map(|h| u8::from_str_radix(h, 16).unwrap())
            .collect();
        let oui = [bytes[0], bytes[1], bytes[2]];
        assert!(
            OUI_PREFIXES.contains(&oui),
            "MAC OUI {:02x}:{:02x}:{:02x} not in known vendor list",
            oui[0],
            oui[1],
            oui[2]
        );
    }
}
