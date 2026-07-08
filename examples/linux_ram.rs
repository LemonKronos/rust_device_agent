use smbioslib::{SMBiosMemoryDevice, table_load_from_device};

#[derive(Debug, Default)]
pub struct Ram {
    name: String,
    serial: String,
    type_: String,
    speed: String,
    size: String,
}

fn main() {
    println!("--- Initializing RAW RAM Hardware Dump ---\n");

    let data = match table_load_from_device() {
        Ok(d) => d,
        Err(e) => {
            println!("Failed to read SMBIOS. Are you root? Error: {}", e);
            return;
        }
    };

    let mut populated_ram = Vec::new();

    for mem_device in data.defined_struct_iter::<SMBiosMemoryDevice>() {
        // Leave size as the raw debug string
        let size_raw = format!("{:?}", mem_device.size());
        
        // Skip empty motherboard slots
        if size_raw.contains("0") || size_raw.contains("Unknown") || size_raw.contains("None") {
            continue;
        }

        let name = format!("{} {}", mem_device.manufacturer(), mem_device.part_number()).trim().to_string();
        let serial = mem_device.serial_number().to_string();

        // Safely extract just the 'value' enum from the MemoryDeviceTypeData
        let type_ = match mem_device.memory_type() {
            Some(mem_type_data) => format!("{:?}", mem_type_data.value).to_uppercase(),
            None => "UNKNOWN".to_string(),
        };

        // Parse Size (Clean up the enum wrapper, e.g., "Some(Gigabytes(16))" -> "16 GB")
        let size = if size_raw.contains("Gigabytes") {
            size_raw.replace("Some(Gigabytes(", "").replace("))", " GB")
                      .replace("Gigabytes(", "").replace(")", " GB")
        } else if size_raw.contains("Megabytes") {
            size_raw.replace("Some(Megabytes(", "").replace("))", " MB")
                      .replace("Megabytes(", "").replace(")", " MB")
        } else {
            size_raw
        };

        let speed_debug = format!("{:?}", mem_device.configured_memory_speed().or_else(|| mem_device.speed()));
        let speed = speed_debug.replace("Some(MTs(", "").replace("))", " MT/s")
                               .replace("MTs(", "").replace(")", " MT/s")
                               .replace("Some(Unknown)", "Unknown");

        populated_ram.push(Ram {
            name,
            serial,
            type_,
            speed,
            size: size,
        });
    }

    // --- Final Output ---
    for (i, ram) in populated_ram.iter().enumerate() {
        println!("Slot {} [Populated]:", i + 1);
        println!("  Name:   {}", ram.name);
        println!("  Serial: {}", ram.serial);
        println!("  Type:   {}", ram.type_);
        println!("  Speed:  {}", ram.speed);
        println!("  Size:   {}", ram.size);
        println!("--------------------------------------");
    }
}