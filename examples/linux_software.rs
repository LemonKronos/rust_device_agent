use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::time::UNIX_EPOCH;

#[cfg_attr(debug_assertions, allow(dead_code))]
#[derive(Debug, Default)]
pub struct Software {
    name: String,
    version: String,
    source: String,
    size: u64,
    license: String,
    install_date: String, // Added this field
    expiration_date: String,
}

fn get_linux_software() -> Vec<Software> {
    let mut software_list = Vec::new();
    
    let file = match File::open("/var/lib/dpkg/status") {
        Ok(f) => f,
        Err(_) => return software_list,
    };
    
    let reader = BufReader::new(file);

    let mut current = Software {
        license: "Unknown".to_string(),
        expiration_date: "N/A".to_string(),
        install_date: "Unknown".to_string(),
        ..Default::default()
    };

    for line in reader.lines().flatten() {
        if line.trim().is_empty() {
            if !current.name.is_empty() {
                let info_path = format!("/var/lib/dpkg/info/{}.list", current.name);
                if let Ok(meta) = fs::metadata(&info_path) {
                    if let Ok(time) = meta.modified() {
                        if let Ok(duration) = time.duration_since(UNIX_EPOCH) {
                            current.install_date = duration.as_secs().to_string();
                        }
                    }
                }
                
                software_list.push(current);
                
                // Reset for the next package
                current = Software {
                    license: "Unknown".to_string(),
                    expiration_date: "N/A".to_string(),
                    install_date: "Unknown".to_string(),
                    ..Default::default()
                };
            }
            continue;
        }

        if let Some(name) = line.strip_prefix("Package: ") {
            current.name = name.to_string();
        } else if let Some(version) = line.strip_prefix("Version: ") {
            current.version = version.to_string();
        } else if let Some(maintainer) = line.strip_prefix("Maintainer: ") {
            current.source = maintainer.to_string();
        } else if let Some(size) = line.strip_prefix("Installed-Size: ") {
            current.size = size.parse::<u64>().unwrap_or(0);
        }
    }
    
    software_list
}

fn main() {
    let apps = get_linux_software();
    for app in apps.iter().take(3) {
        println!("{:#?}", app);
    }
}