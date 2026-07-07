
use std::fs;
use std::path::Path;

fn main() {
    // ==========================================
    // 4. Physical Disks (sysfs Block tree)
    // ==========================================
    println!(">>> PHYSICAL DISK HARDWARE DUMP <<<");
    let block_dir = Path::new("/sys/block");

    if let Ok(entries) = fs::read_dir(block_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            
            // Filter virtual mounts and ram disks
            if name.starts_with("loop") || name.starts_with("ram") || name.starts_with("sr") {
                continue;
            }

            let device_path = entry.path().join("device");

            // Hardware Identifiers
            let model = fs::read_to_string(device_path.join("model"))
                .unwrap_or_else(|_| "Unknown".to_string());
            let serial = fs::read_to_string(device_path.join("serial"))
                .unwrap_or_else(|_| "Unknown".to_string());

            // Firmware: Check 'firmware_rev' (NVMe) first, fallback to 'rev' (SATA)
            let firmware = fs::read_to_string(device_path.join("firmware_rev"))
                .or_else(|_| fs::read_to_string(device_path.join("rev")))
                .unwrap_or_else(|_| "Unknown".to_string());

            // Sale Size Calculation (Base-10 GB)
            let size_path = entry.path().join("size");
            let sectors: u64 = fs::read_to_string(size_path)
                .unwrap_or_else(|_| "0".to_string())
                .trim()
                .parse()
                .unwrap_or(0);
            
            let sale_size_gb = (sectors * 512) as f64 / 1_000_000_000.0;

            println!("Drive: /dev/{}", name);
            println!("  Model:    {}", model.trim());
            println!("  Serial:   {}", serial.trim());
            println!("  Firmware: {}", firmware.trim());
            println!("  Size:     {:.0} GB", sale_size_gb.round());
            println!("--------------------------------------");
        }
    }
}