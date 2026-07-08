use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
pub struct Software {
    name: String,
    version: String,
    source: String, // Mapped to the dpkg Maintainer
    // license: String,
    // expiration_date: String,
}

fn get_linux_software() -> Vec<Software> {
    let mut software_list = Vec::new();
    
    // Open the dpkg status file natively
    let file = match File::open("/var/lib/dpkg/status") {
        Ok(f) => f,
        Err(_) => return software_list,
    };
    
    let reader = BufReader::new(file);

    // Initialize our first struct with the defaults we know
    let mut current = Software {
        // license: "Unknown (Debian Policy)".to_string(),
        // expiration_date: "N/A".to_string(),
        ..Default::default()
    };

    // The status file separates packages by a blank line
    for line in reader.lines().flatten() {
        if line.trim().is_empty() {
            // End of a package block: push the struct and reset
            if !current.name.is_empty() {
                software_list.push(current);
                current = Software {
                    // license: "Unknown (Debian Policy)".to_string(),
                    // expiration_date: "N/A".to_string(),
                    ..Default::default()
                };
            }
            continue;
        }

        // Parse the exact fields for the updated struct
        if let Some(name) = line.strip_prefix("Package: ") {
            current.name = name.to_string();
        } else if let Some(version) = line.strip_prefix("Version: ") {
            current.version = version.to_string();
        } else if let Some(maintainer) = line.strip_prefix("Maintainer: ") {
            current.source = maintainer.to_string();
        }
    }
    
    software_list
}

fn main() {
    println!("--- Scanning Installed Linux Software ---\n");
    
    let apps = get_linux_software();
    println!("Total Installed Packages (dpkg): {}\n", apps.len());
    
    // Dump the first 3 just to verify the parse worked with the new struct
    for app in apps.iter().take(3) {
        println!("{:#?}", app);
    }
}