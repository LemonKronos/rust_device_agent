///
/// Load, store, update config.json file
/// 
/// Define cycle_time == 0 mean only scan at init, ex: architecture
/// There're also info that must be include with each send, ex: agent_version
/// Note that this is WHEN TO SCAN, not when to send. Agent only send when info change
/// 

use std::fs;
use std::path::Path;
use tokio::time::Instant;

use crate::scheduler::{ScheduledTask,TimerWheel};

const CONFIG_PATH: &str = "doc/config.json";

pub fn load_config() -> TimerWheel {
    let config_path = Path::new(CONFIG_PATH);

    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(config_path) {
            match serde_json::from_str::<TimerWheel>(&content) {
                Ok(queue) => {
                    log::info!("Successfully loaded config from {}", CONFIG_PATH);
                    return queue;
                }
                Err(e) => log::warn!("Config corrupted or invalid JSON: {}. Rebuilding defaults.", e),
            }
        }
    } else {
        log::warn!("Config file not found at {}. Initializing defaults.", CONFIG_PATH);
    }

    // Can't find config file, generate default
    init_config()
}

pub fn save_config() {
    todo!()
}

/// Make default config, load to TimerWheel and store it
pub fn init_config() -> TimerWheel {
    let config_path = Path::new(CONFIG_PATH);

    let mut default_queue = TimerWheel::new();
    
    let now = Instant::now();
    let init = 0;
    let min5 = 300;
    let min30 = 1800;
    let hourly = 3600;

    //_ General
    default_queue.push(ScheduledTask {
        id: "general.host".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "general.boot_time".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "general.run_time".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    //_ Machine
    default_queue.push(ScheduledTask {
        id: "machine.architecture".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "machine.producer".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "machine.model".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "machine.type".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    //_ Motherboard
    default_queue.push(ScheduledTask {
        id: "motherboard.name".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "motherboard.serial".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "motherboard.tempe".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "motherboard.cpu_slot".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "motherboard.ram_slot".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "motherboard.gpu_slot".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    //_ Bios
    default_queue.push(ScheduledTask {
        id: "bios.version".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "bios.vendor".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "bios.sercure_boot".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    //_ OS
    default_queue.push(ScheduledTask {
        id: "os.distro".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "os.name".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "os.version".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "os.kernel".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    //_ CPU
    default_queue.push(ScheduledTask {
        id: "cpu.name".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "cpu.core".to_string(),
        cycle_time: init,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "cpu.usage".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "cpu.frequency".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "cpu.temperature".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    //_ RAM
    default_queue.push(ScheduledTask {
        id: "ram.total".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "ram.usage".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
        //_Physical
    default_queue.push(ScheduledTask {
        id: "ram.physical.name".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "ram.physical.serial".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "ram.physical.type".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "ram.physical.config_speed".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "ram.physical.size".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "ram.physical.bank".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "ram.physical.form_factor".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    //_ SWAP
    default_queue.push(ScheduledTask {
        id: "swap.total".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "swap.usage".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    //_ GPU
    default_queue.push(ScheduledTask {
        id: "gpu.name".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "gpu.driver".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "gpu.utilization".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "gpu.temperature".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "gpu.frequency".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "gpu.vram_total".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "gpu.vram_usage".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "gpu.max_clock".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "gpu.serial".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    //_ Disk
        //_ Logical
    default_queue.push(ScheduledTask {
        id: "disk.logical.name".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.logical.file_name".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.logical.mount_point".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.logical.removable".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.logical.total".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.logical.used".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
        //_Physical
    default_queue.push(ScheduledTask {
        id: "disk.physical.drive".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.physical.index".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.physical.serial".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.physical.firmware".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.physical.size".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.physical.status".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.physical.media".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.physical.interface".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.physical.parted".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.physical.partition.name".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "disk.physical.partition.size".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    //_Network
    default_queue.push(ScheduledTask {
        id: "network.name".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "network.ipv4".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "network.ipv6".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "network.mac".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "network.mtu".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "network.upload".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "network.download".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "network.card".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "network.config_speed".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "network.ssid".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    //_ Process
    default_queue.push(ScheduledTask {
        id: "process_count".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "top_process.name".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "top_process.cpu".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "top_process.memory".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "top_process.runtime".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    //TODO All Process
    //_Battery
    default_queue.push(ScheduledTask {
        id: "battery.percentage".to_string(),
        cycle_time: min5,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "battery.is_plugged_in".to_string(),
        cycle_time: min30,
        execute_at: now,
    });
    //_Software
    default_queue.push(ScheduledTask {
        id: "software.name".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "software.version".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "sofware.source".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "software.size".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });
    default_queue.push(ScheduledTask {
        id: "software.install_date".to_string(),
        cycle_time: hourly,
        execute_at: now,
    });

    // Ensure the parent directory ("doc/") exists
    if let Some(parent) = config_path.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = fs::create_dir_all(parent);
        }
    }

    match serde_json::to_string_pretty(&default_queue) {
        Ok(json) => {
            let re = regex::Regex::new(r"\[\s+(\d+),\s+(\d+)\s+\]").unwrap();
            let flat_json = re.replace_all(&json, "[$1, $2]").to_string();

            if let Err(e) = std::fs::write(config_path, flat_json) {
                log::error!("Failed to write default config to disk: {}", e);
            } else {
                log::info!("Default config saved to {}", CONFIG_PATH);
            }
        }
        Err(e) => log::error!("Failed to serialize default config: {}", e),
    }

    default_queue
}
