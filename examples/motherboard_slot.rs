use smbioslib::{SMBiosPhysicalMemoryArray, SMBiosProcessorInformation, table_load_from_device};
use std::fs;

fn main() {
    println!("--- Initializing Hardware Blueprint Dump ---\n");

    let mut cpu_sockets = 0;
    let mut ram_slots = 0;

    // 1 & 2. SMBIOS Data (CPU & RAM Blueprint)
    match table_load_from_device() {
        Ok(data) => {
            println!(">>> DUMPING CPU SOCKETS (Type 4) <<<");
            for cpu in data.defined_struct_iter::<SMBiosProcessorInformation>() {
                println!("{:#?}", cpu);
                cpu_sockets += 1;
            }

            println!("\n>>> DUMPING RAM ARRAYS (Type 16) <<<");
            for mem_array in data.defined_struct_iter::<SMBiosPhysicalMemoryArray>() {
                println!("{:#?}", mem_array);
                if let Some(count) = mem_array.number_of_memory_devices() {
                    ram_slots += count;
                }
            }
        }
        Err(e) => {
            println!("Failed to read SMBIOS. Are you running as root? Error: {}", e);
        }
    }

    // 3. GPU Data (sysfs PCI tree - captures both iGPU and dGPU)
    println!("\n>>> DUMPING DISPLAY CONTROLLERS (sysfs PCI) <<<");
    let mut gpu_count = 0;
    
    if let Ok(entries) = fs::read_dir("/sys/bus/pci/devices") {
        for entry in entries.flatten() {
            let path = entry.path();
            let class_path = path.join("class");
            
            // PCI Class 0x03xxxx is reserved for Display Controllers
            if let Ok(class_str) = fs::read_to_string(&class_path) {
                if class_str.trim().starts_with("0x03") {
                    // Dump the PCI address string (e.g., "0000:00:02.0")
                    println!("Found GPU at PCI address: {:?}", path.file_name().unwrap_or_default());
                    gpu_count += 1;
                }
            }
        }
    } else {
        println!("Warning: Could not read /sys/bus/pci/devices");
    }

    // --- Final Summary ---
    println!("\n======================================");
    println!("       MOTHERBOARD SLOT SUMMARY       ");
    println!("======================================");
    println!(" CPU Sockets (Blueprint): {}", cpu_sockets);
    println!(" RAM Slots (Blueprint):   {}", ram_slots);
    println!(" GPUs Found (Total):      {}", gpu_count);
    println!("======================================");
}
