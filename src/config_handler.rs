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

use crate::scheduler::TaskID::*;
use crate::scheduler::{ScheduledTask,TimerWheel};
use crate::utils::flatten_config;
use super::SCAN_SOFTWARE;

const CONFIG_PATH: &str = "doc/config.json";

/// Try to load config, if not found generate default config
pub fn load_config() -> TimerWheel {
    let config_path = Path::new(CONFIG_PATH);

    if !config_path.exists() {
        log::warn!("Config file not found at {}. Initializing defaults.", CONFIG_PATH);
    } else {
        match fs::read_to_string(config_path) {
            Err(e) => log::error!("Failed to read config file from disk: {}. Rebuilding defaults.", e),
            Ok(content) if content.trim().is_empty() => {
                log::warn!("Config file is completely empty. Rebuilding defaults.");
            }
            Ok(content) => match serde_json::from_str::<TimerWheel>(&content) {
                Err(e) => log::warn!("Config corrupted or invalid JSON: {}. Rebuilding defaults.", e),
                Ok(queue) if queue.heap.is_empty() => {
                    log::warn!("Config parsed successfully, but contained zero valid tasks! Rebuilding defaults.");
                }
                Ok(queue) => {
                    log::info!("Successfully loaded config from {}", CONFIG_PATH);
                    return queue;
                }
            }
        }
    }

    // Fallback to default config
    log::info!("Firing init_config()...");
    init_config()
}

pub fn save_config(timer_wheel: &TimerWheel) {
    let config_path = Path::new(CONFIG_PATH);

    match serde_json::to_string_pretty(timer_wheel) {
        Ok(json) => {
            let flat_json = flatten_config(&json);

            if let Err(e) = fs::write(config_path, flat_json) {
                log::error!("Failed to write config to disk: {}",  e);
            } else {
                log::info!("Config saved to {}", CONFIG_PATH);
            }
        },
        Err(e) => log::error!("Failed to serialize config: {}", e),
    }
}

/// Make default config, load to TimerWheel and store it
pub fn init_config() -> TimerWheel {
    let config_path = Path::new(CONFIG_PATH);

    let mut default_queue = TimerWheel::new();
    
    let now = Instant::now();
    let init = 0;
    
    //TODO There are more that 80 push here, is that OK

    //: Release default config
    #[cfg(not(debug_assertions))] {
        let min5 = 300;
        let min30 = 1800;
        let hourly = 3600;

        //_ General
        default_queue.push(ScheduledTask {
            id: GeneralHost,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GeneralBootTime,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GeneralRunTime,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        //_ Machine
        default_queue.push(ScheduledTask {
            id: MachineArchitecture,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MachineProducer,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MachineModel,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MachineType,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        //_ Motherboard
        default_queue.push(ScheduledTask {
            id: MotherboardName,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MotherboardSerial,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MotherboardTempe,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MotherboardCpuSlot,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MotherboardRamSlot,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MotherboardGpuSlot,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        //_ Bios
        default_queue.push(ScheduledTask {
            id: BiosVersion,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: BiosVendor,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: BiosSecureBoot,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        //_ OS
        default_queue.push(ScheduledTask {
            id: OsDistro,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: OsName,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: OsVersion,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: OsKernel,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        //_ CPU
        default_queue.push(ScheduledTask {
            id: CpuName,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: CpuCore,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: CpuUsage,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: CpuFreq,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: CpuTempe,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        //_ RAM
        default_queue.push(ScheduledTask {
            id: RamTotal,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamUsage,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
            //_Physical
        default_queue.push(ScheduledTask {
            id: RamPhysicalName,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalSerial,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalType,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalConfigSpeed,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalSize,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalBank,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalFormFactor,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        //_ SWAP
        default_queue.push(ScheduledTask {
            id: SwapTotal,
            cycle_time: min30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: SwapUsage,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        //_ GPU
        default_queue.push(ScheduledTask {
            id: GpuName,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuDriver,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuUtilization,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuTempe,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuFreq,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuVramTotal,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuVramUsage,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuMaxClock,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuSerial,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        //_ Disk
            //_ Logical
        default_queue.push(ScheduledTask {
            id: DiskLogicalName,
            cycle_time: min30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskLogicalFileName,
            cycle_time: min30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskLogicalMountPoint,
            cycle_time: min30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskLogicalRemovable,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskLogicalTotal,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskLogicalUsed,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
            //_Physical
        default_queue.push(ScheduledTask {
            id: DiskPhysicalDrive,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalModel,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalIndex,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalSerial,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalFirmware,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalSize,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalStatus,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalMedia,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalInterface,
            cycle_time: hourly,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalNumPartition,
            cycle_time: min30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalPartition,
            cycle_time: min30,
            execute_at: now,
            limit: None,
        });
        //_Network
        default_queue.push(ScheduledTask {
            id: NetworkName,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkIpv4,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkIpv6,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkMac,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkMtu,
            cycle_time: min30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkUpload,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkDownload,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkCard,
            cycle_time: min30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkConfigSpeed,
            cycle_time: min30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkSsid,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        //_ Process
        default_queue.push(ScheduledTask {
            id: ProcessCount,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: TopProcessName,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: TopProcessCpu,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: TopProcessMemory,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id : TopProcessRuntime,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        //_Battery
        default_queue.push(ScheduledTask {
            id: BatteryPercentage,
            cycle_time: min5,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: BatteryIsPluggedIn,
            cycle_time: min30,
            execute_at: now,
            limit: None,
        });
        //_Software, skip in debug
        if SCAN_SOFTWARE {
            default_queue.push(ScheduledTask {
                id: Software,
                cycle_time: hourly,
                execute_at: now,
                limit: None,
            });
        }
    }

    //: Debug default config, info fire more frequently
    #[cfg(debug_assertions)]{
        //_ General
        default_queue.push(ScheduledTask {
            id: GeneralHost,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GeneralBootTime,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GeneralRunTime,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        //_ Machine
        default_queue.push(ScheduledTask {
            id: MachineArchitecture,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MachineProducer,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MachineModel,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MachineType,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        //_ Motherboard
        default_queue.push(ScheduledTask {
            id: MotherboardName,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MotherboardSerial,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MotherboardTempe,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MotherboardCpuSlot,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MotherboardRamSlot,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: MotherboardGpuSlot,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        //_ Bios
        default_queue.push(ScheduledTask {
            id: BiosVersion,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: BiosVendor,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: BiosSecureBoot,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        //_ OS
        default_queue.push(ScheduledTask {
            id: OsDistro,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: OsName,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: OsVersion,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: OsKernel,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        //_ CPU
        default_queue.push(ScheduledTask {
            id: CpuName,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: CpuCore,
            cycle_time: init,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: CpuUsage,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: CpuFreq,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: CpuTempe,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        //_ RAM
        default_queue.push(ScheduledTask {
            id: RamTotal,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamUsage,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
            //_Physical
        default_queue.push(ScheduledTask {
            id: RamPhysicalName,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalSerial,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalType,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalConfigSpeed,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalSize,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalBank,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: RamPhysicalFormFactor,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        //_ SWAP
        default_queue.push(ScheduledTask {
            id: SwapTotal,
            cycle_time: 30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: SwapUsage,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        //_ GPU
        default_queue.push(ScheduledTask {
            id: GpuName,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuDriver,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuUtilization,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuTempe,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuFreq,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuVramTotal,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuVramUsage,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuMaxClock,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: GpuSerial,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        //_ Disk
            //_ Logical
        default_queue.push(ScheduledTask {
            id: DiskLogicalName,
            cycle_time: 30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskLogicalFileName,
            cycle_time: 30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskLogicalMountPoint,
            cycle_time: 30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskLogicalRemovable,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskLogicalTotal,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskLogicalUsed,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
            //_Physical
        default_queue.push(ScheduledTask {
            id: DiskPhysicalDrive,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalModel,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalIndex,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalSerial,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalFirmware,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalSize,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalStatus,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalMedia,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalInterface,
            cycle_time: 60,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalNumPartition,
            cycle_time: 30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: DiskPhysicalPartition,
            cycle_time: 30,
            execute_at: now,
            limit: None,
        });
        //_Network
        default_queue.push(ScheduledTask {
            id: NetworkName,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkIpv4,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkIpv6,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkMac,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkMtu,
            cycle_time: 30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkUpload,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkDownload,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkCard,
            cycle_time: 30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkConfigSpeed,
            cycle_time: 30,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: NetworkSsid,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        //_ Process
        default_queue.push(ScheduledTask {
            id: ProcessCount,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: TopProcessName,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: TopProcessCpu,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: TopProcessMemory,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id : TopProcessRuntime,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        //TODO All Process
        //_Battery
        default_queue.push(ScheduledTask {
            id: BatteryPercentage,
            cycle_time: 10,
            execute_at: now,
            limit: None,
        });
        default_queue.push(ScheduledTask {
            id: BatteryIsPluggedIn,
            cycle_time: 30,
            execute_at: now,
            limit: None,
        });
        //_Software, skip in debug
        if SCAN_SOFTWARE {
            default_queue.push(ScheduledTask {
                id: Software,
                cycle_time: 60,
                execute_at: now,
                limit: None,
            });
        }
    }

    // Ensure the parent directory ("doc/") exists
    if let Some(parent) = config_path.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = fs::create_dir_all(parent);
        }
    }

    save_config(&default_queue);

    default_queue
}
