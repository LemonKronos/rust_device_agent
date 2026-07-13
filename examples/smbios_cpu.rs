use smbioslib::{SMBiosProcessorInformation, table_load_from_device};

#[derive(Debug, Default)]
pub struct CpuBlueprint {
    pub socket: String,
    pub description: String,
    pub manufacturer: String,
}

fn get_cpu_blueprint() -> Option<CpuBlueprint> {
    let data = match table_load_from_device() {
        Ok(d) => d,
        Err(_) => return None,
    };

    // Grab the first processor found in Type 4 (SMBiosProcessorInformation)
    if let Some(processor) = data.defined_struct_iter::<SMBiosProcessorInformation>().next() {
        return Some(CpuBlueprint {
            // socket_designation gives you things like "LGA1151" or "AM4"
            socket: processor.socket_designation().to_string(),
            // processor_version gives the factory description string
            description: processor.processor_version().to_string(),
            manufacturer: processor.processor_manufacturer().to_string(),
        });
    }

    None
}

fn main() {
    println!("--- Grabbing CPU Blueprint ---");
    if let Some(cpu_hw) = get_cpu_blueprint() {
        println!("Socket:       {}", cpu_hw.socket);
        println!("Description:  {}", cpu_hw.description);
        println!("Manufacturer: {}", cpu_hw.manufacturer);
    } else {
        println!("Failed to read SMBIOS. Are you running as Admin/Root?");
    }
}