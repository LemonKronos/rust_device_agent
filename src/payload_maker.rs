///
/// Make make payload by calling info_gatherer, serialize to json
/// 

use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value as Json};
use rustc_hash::FxHashMap;

use crate::utils::*;
use crate::info_gatherer::Info;
use crate::scheduler::{ScheduledTask, TaskID, {TaskID::*}, AgentValue};

mod serialize_type;
use serialize_type::*;


pub struct Payload {
    info: Info,
}

impl Payload {
    pub fn new() -> Self {
        Self { info:Info::new() }
    }

    pub fn prepare(&mut self) {
        self.info.prepare()
    }

    pub fn timestamp(&self) -> u64 {
        self.info.get_timestamp()
    }

    pub fn get_json_v3_fullscan(&self) -> Json {

        let rams_json = self.info.get_ram_list().map(|ram| {
            ram.iter().map(|ram| json!({
                "name": ram.get_name(),
                "serial": ram.get_serial(),
                "type": ram.get_type(),
                "config_speed": ram.get_speed(),
                "size": ram.get_size(),
                "bank": ram.get_bank(),
                "form_factor": ram.get_form_factor(),
            })).collect::<Vec<_>>()
        });

        let gpus_json: Vec<_> = self.info.get_gpu_list().iter().map(|gpu| {
            json!({
                "name": gpu.get_name(),
                "driver": gpu.get_driver_version(),
                "utilization": gpu.get_util(),
                "temperature": gpu.get_tempe(),
                "frequency": gpu.get_freq(),
                "vram_total": gpu.get_vram_total(),
                "vram_usage": gpu.get_vram_usage(),
                "max_clock": gpu.get_max_clock(),
                "serial": gpu.get_serial(),
            })
        }).collect();

        let logical_disks_json: Vec<_> = self.info.get_logical_disk_list().iter().map(|disk| {
            json!({
                "name": disk.get_name(),
                "file_system": disk.get_file_system(),
                "mount_point": disk.get_mount_point(),
                "removable": disk.get_removable(),
                "total": disk.get_total(),
                "used": disk.get_used(),
            })
        }).collect();

        let physical_disks_json = self.info.get_physical_disk_list().map(|hd| {
            hd.iter().map(|hd| {
                let partition_list = hd.get_partition().map(|part| {
                    part.iter().map(|part| json!({
                        "name": part.get_name(),
                        "size": part.get_size(),
                    })).collect::<Vec<_>>()
                });

                json!({
                    "drive": hd.get_drive(),
                    "index": hd.get_index(),
                    "model": hd.get_model(),
                    "serial": hd.get_serial(),
                    "firmware": hd.get_firmware(),
                    "size": hd.get_size(),
                    "status": hd.get_status(),
                    "media": hd.get_media(),
                    "interface": hd.get_interface(),
                    "parted": hd.get_partition_number(),
                    "partition": partition_list,
                })
            }).collect::<Vec<_>>()
        });

        let networks_json: Vec<_> = self.info.get_network_list().iter().map(|network| {
            json!({
                "name": network.get_name(),
                "ipv4": network.get_ipv4(),
                "ipv6": network.get_ipv6(),
                "mac": network.get_mac(),
                "mtu": network.get_mtu(),
                "upload": network.get_upload(),
                "download": network.get_download(),
                "card": network.get_card(),
                "config_speed": network.get_config_speed(),
                "ssid": network.get_ssid(),
            })
        }).collect();

        let top_processes_json: Vec<_> = self.info.get_top_process_list().iter().map(|process| {
            json!({
                "name": process.get_name(),
                "cpu": process.get_cpu_usage(),
                "memory": process.get_memory(),
                "runtime": process.get_runtime(),
            })
        }).collect();

        // let all_processes_json: Vec<_> = self.info.get_full_process_list().iter().map(|process| {
        //     json!({
        //         "name": process.get_name(),
        //         "cpu": process.get_cpu_usage(),
        //         "memory": process.get_memory(),
        //         "runtime": process.get_runtime(),
        //     })
        // }).collect();

        //_ this have take(5) for debug
        let softwares_json = self.info.get_software_list().map(|app| {
            app.iter().take(5).map(|app| json!({
                "name": app.get_name(),
                "version": app.get_version(),
                "source": app.get_source(),
                "size": app.get_size(),
                "install_date": app.get_install_date().map(|systime| systime.to_time_sec()),
            })).collect::<Vec<_>>()
        });

        //TODO peripheral
        // let peripheral_json: Vec<_> = 

        let payload = json!({
            "agent_version": "0.3.0",
            "time_stamp": self.info.get_timestamp(), // second
            "general": {
                "host": self.info.get_host(),
                "boot_time": self.info.get_boot_time(), // second
                "run_time": self.info.get_up_time(), // second
            },
            "machine": {   
                "serial": self.info.get_product_serial(),
                "architecture": self.info.get_architecture(),
                "producer": self.info.get_producer(),
                "model": self.info.get_system_model(),
                "type": self.info.get_machine_type(),
            },
            "motherboard": {
                "name": self.info.get_motherboard(),
                "serial": self.info.get_motherboard_serial(),
                "tempe": self.info.get_tempe_mobo(),
                "cpu_slot": self.info.get_cpu_slot(),
                "ram_slot": self.info.get_ram_slot(),
                "gpu_slot": self.info.get_gpu_slot(),
            },
            "bios": {
                "version": self.info.get_os_version(),
                "vendor": self.info.get_bios_vendor(),
                "secure_boot": self.info.get_is_secure_boot(),
            },
            "os": {
                "distro": self.info.get_os_distro(),
                "name": self.info.get_os_name(),
                "version": self.info.get_os_version(),
                "kernel": self.info.get_kernel(),
            },
            "cpu": {
                "name": self.info.get_cpu_name(),
                "core": self.info.get_cpu_core(),
                "usage": self.info.get_cpu_usage(), // %
                "frequency": self.info.get_cpu_freq(), // MHz
                "temperature": self.info.get_cpu_tempe(), // ℃
            },
            "ram": {
                "total": self.info.get_ram_total(), // byte
                "usage": self.info.get_ram_usage(), // byte
                "physical": rams_json, // list
            },
            "swap": {
                "total": self.info.get_swap_total(), // byte
                "usage": &self.info.get_swap_usage(), // byte
            },
            "gpu": gpus_json, // list
            "disk": {
                "logical": logical_disks_json, // list
                "hardware": physical_disks_json, // list
            },
            "network": networks_json, // list
            "process_count": self.info.get_process_count(),
            "top_process": top_processes_json, // list
            // "all_process": all_processes_json, // list
            "battery": {
                "percentage": self.info.get_battery_percentage(),
                "is_plugged_in": self.info.get_battery_is_plugged_in(),
            },
            "software": softwares_json,
        });

        let Ok(pretty_json) = serde_json::to_string_pretty(&payload) else {
            log::error!("Failed to serialize payload to JSON");
            return payload;
        };

        let md_content = format!("```json\n{}\n```", pretty_json);

        if let Err(e) = fs::create_dir_all("doc/sample/") {
            log::error!("Failed to create directory 'doc/sample/': {}", e);
        }

        if let Err(e) = fs::write("doc/sample/json_v3_fullscan.md", md_content) {
            log::error!("Failed to write file 'json_v3_fullscan.md': {}", e);
        } else {
            log::info!("Successfully saved JSON V3 sample.");
        }

        payload
    }

    pub fn get_json_v2_fullscan(&self) -> Json {
        
        let rams_json = self.info.get_ram_list().map(|ram| {
            ram.iter().map(|ram| json!({
                "BANK_LABEL": ram.get_bank(),
                "CAPACITY": ram.get_size(),
                "SPEED": ram.get_speed(),
                "RAM_TYPE": ram.get_type(),
                "FORM_FACTOR": ram.get_form_factor(),
                "MANUFACTURER": ram.get_name(),
                "SERIAL_NUMBER": ram.get_serial(),
            })).collect::<Vec<_>>()
        });

        let networks_json: Vec<_> = self.info.get_network_list().iter().map(|network| {
            json!({
                "IP": network.get_ipv4(),
                "MAC_ADDRESS": network.get_mac(),
                "CONNECTION_TYPE": network.get_name(),
                "SSID": network.get_ssid(),
                "INET_CARD": network.get_card(),
                "INET_CARD_SPEED": network.get_config_speed(),
            })
        }).collect();

        let gpus_json: Vec<_> = self.info.get_gpu_list().iter().map(|gpu| {
            json!({
                "GPU_NAME": gpu.get_name(),
                "GLOBAL_MEMORY": gpu.get_vram_total().b_to_gb().to_string(),
                // "VERSION": "", // ! Agent can not get this info
                "DRIVER_VERSION": gpu.get_driver_version(),
                // "COMPUTE_UNITS": "",  // ! Agent can not get this info
                "MAX_CLOCK_SPEED": gpu.get_max_clock().unwrap_or(0).to_string(),
                // "VENDOR": "NVIDIA", // ! Will NOT be able to parse correctly by Agent
                "TEMPERATURE": gpu.get_tempe().to_string(),
            })
        }).collect();

        let logical_disks_json: Vec<_> = self.info.get_logical_disk_list().iter().map(|disk| {
            json!({
                "HARD_DISK_NAME": disk.get_name(),
                "TOTAL": disk.get_total().b_to_gb(),
                "USAGE": disk.get_used().b_to_gb(),
                // "FREE": 126.9, // ? Agent will not calculate and send this info
                // "PERCENT": 72.19, // ? Agent will not calculate and send this info
                "FILE_SYSTEM_TYPE": disk.get_file_system(),
                "MOUNT_POINT": disk.get_mount_point(),
            })
        }).collect();

        let physical_disks_json = self.info.get_physical_disk_list().map(|hd| {
            hd.iter().map(|hd| {
                let partition_list = hd.get_partition().map(|part| {
                    part.iter().map(|part| json!({
                        "PARTITION_INDEX": part.get_name(),
                        "SIZE": part.get_size(),
                    })).collect::<Vec<_>>()
                });
                
                json!({
                    "DISK_NAME": hd.get_drive(),
                    "MODEL": hd.get_model(),
                    "SIZE": hd.get_size(),
                    "SERIAL_NUMBER": hd.get_serial(),
                    "INTERFACE_TYPE": hd.get_interface(),
                    "MEDIA_TYPE": hd.get_media(),
                    "NUMBER_PARTITIONS": hd.get_partition_number(),
                    "PHYSICAL_DISK_STATUS": hd.get_status(),
                    "FIRMWARE": hd.get_firmware(),
                    "PHYSICAL_DISK_INDEX": hd.get_index().to_string(),
                    "PARTITION_Lst": partition_list,
                })
            }).collect::<Vec<_>>()
        });

        //_ this have take(5) for debug
        let softwares_json = self.info.get_software_list().map(|app| {
            app.iter().take(5).map(|app| json!({
                "SOFTWARE_NAME": app.get_name(),
                "VERSION": app.get_version(),
                "PUBLISHER": app.get_source(),
                "INSTALL_DATE": app.get_install_date().map(|time| time.to_datetime_string()),
                "SIZE": app.get_size().to_string(),
            })).collect::<Vec<_>>()
        });

        let payload = json!({
            "SCAN_DATA": {
                "FULL_COMPUTER_NAME": self.info.get_host(),
                "COMPUTER_NAME": self.info.get_host(), 
                "OPERATING_SYSTEM": format!("{} (kernel {})", self.info.get_os_distro(), self.info.get_kernel()),
                "BOOT_TIME_EPOCH": self.info.get_boot_time(), 
                "SERIAL_NUMBER": self.info.get_product_serial(),
                "PRODUCER": self.info.get_producer(),
                "SYSTEM_MODEL": self.info.get_system_model(),
                "BIOS_VERSION": format!("{} {}", self.info.get_bios_vendor(), self.info.get_bios_version()),
                "SYSTEM_TYPE": self.info.get_architecture(),
                "MOTHER_BOARD": self.info.get_motherboard(),
                "COMPUTER_TYPE": self.info.get_machine_type(),
                "Type": "COMPUTER", // Hardcoded for server
                "ASSET_CODE": null, // Default
                "BATTERY": {
                    "PERCENT": self.info.get_battery_percentage().unwrap_or_default().to_string(),
                    "POWER_PLUGGED": self.info.get_battery_is_plugged_in(),
                },
                "CPU": {
                    "CORES": self.info.get_cpu_core() / 2, // ? Mayber not just divided by 2
                    "LOGICAL_PROCESSORS": self.info.get_cpu_core(), 
                    "CPU_NAME": self.info.get_cpu_name(),
                    "CPU_DESCRIPTION": "Comming Soon",  // ? What is this field
                    "CPU_USAGE_RATE": self.info.get_cpu_usage(),
                    "SPEED": self.info.get_cpu_freq(),
                    "PROCESSES": self.info.get_process_count(),
                    "SOCKETS": self.info.get_cpu_slot().unwrap_or(0), // ? This supposed to be "in use", that doesn't make sense
                    "UPTIME": self.info.get_up_time().to_uptime_string(), 
                    "MACHINE": self.info.get_architecture(), // ? Why this being the same as "SYSTEM TYPE"
                    "TEMPERATURE": self.info.get_cpu_tempe().unwrap_or_default().to_string(),
                },
                "RAM": {
                "TOTAL": self.info.get_ram_total().b_to_gb(),
                "USAGE": self.info.get_ram_usage().b_to_gb(),
                // "PERCENTAGE": 69.96, // ? This is just for display, so Agent will not send this
                "DETAILS": rams_json,
                },
                "NETWORK_Lst": networks_json,
                "GPU_Lst": gpus_json,
                "HARD_DISK_Lst": logical_disks_json,
                "PHYSICAL_DISK_Lst": physical_disks_json,
                "SOFTWARE_Lst": softwares_json,
            },
            "AGENT_VERSION": "2.0.0"
            });

        let Ok(pretty_json) = serde_json::to_string_pretty(&payload) else {
            log::error!("Failed to serialize payload to JSON");
            return payload;
        };

        let md_content = format!("```json\n{}\n```", pretty_json);

        if let Err(e) = fs::create_dir_all("doc/sample/") {
            log::error!("Failed to create directory 'doc/sample/': {}", e);
        }

        if let Err(e) = fs::write("doc/sample/json_v2_fullscan.md", md_content) {
            log::error!("Failed to write file 'json_v2_fullscan.md': {}", e);
        } else {
            log::info!("Successfully saved JSON sample.");
        }

        payload
    }

    pub fn get_json_v2_telemetry(&self) -> Json {
        let payload = json!({
            "SCAN_DATA": {
                "COMPUTER_NAME": self.info.get_host(), 
                "BOOT_TIME_EPOCH": self.info.get_boot_time(), 
                "SERIAL_NUMBER": self.info.get_product_serial(),
                "CPU": {
                    "CPU_USAGE_RATE": self.info.get_cpu_usage(),
                    "SPEED": self.info.get_cpu_freq(),
                    "PROCESSES": self.info.get_process_count(),
                    "UPTIME": self.info.get_up_time().to_uptime_string(), 
                    "TEMPERATURE": self.info.get_cpu_tempe().unwrap_or_default().to_string(),
                },
            },
            "AGENT_VERSION": "2.0.0",
            });

        let Ok(pretty_json) = serde_json::to_string_pretty(&payload) else {
            log::error!("Failed to serialize payload to JSON");
            return payload;
        };

        let md_content = format!("```json\n{}\n```", pretty_json);

        if let Err(e) = fs::create_dir_all("doc/sample/") {
            log::error!("Failed to create directory 'doc/sample/': {}", e);
        }

        if let Err(e) = fs::write("doc/sample/json_v2_telemetry.md", md_content) {
            log::error!("Failed to write file 'json_v2_telemetry.md': {}", e);
        } else {
            log::info!("Successfully saved JSON sample.");
        }

        payload
    }

}

pub struct PayloadMaker {
    info: Info,
    last_info_payload: InfoPayload,
}

impl PayloadMaker {
    pub fn new() -> Self {
        Self { 
            info: Info::new(),
            last_info_payload: InfoPayload::default(),
        }
    }

/*
    pub fn process_batch(&mut self, mut batch: Vec<ScheduledTask>) -> (Vec<ScheduledTask>, Json) {
        //: Flags list
        let mut rams_physical = false;
        let mut gpus = false;
        let mut disks_logical = false;
        let mut disks_physical = false;
        let mut disks_physical_partition = false;
        let mut networks = false;
        let mut processes = false;
        let mut software = false;

        //: Check to raise flags
        for task in &batch {
            match task.id {
                | RamPhysicalBank
                | RamPhysicalConfigSpeed
                | RamPhysicalFormFactor
                | RamPhysicalName
                | RamPhysicalSerial
                | RamPhysicalSize
                | RamPhysicalType => rams_physical = true,

                | GpuDriver
                | GpuFreq
                | GpuMaxClock
                | GpuName
                | GpuSerial
                | GpuTempe
                | GpuUtilization
                | GpuVramTotal
                | GpuVramUsage => gpus = true,

                | DiskLogicalFileName
                | DiskLogicalMountPoint
                | DiskLogicalName
                | DiskLogicalRemovable
                | DiskLogicalTotal
                | DiskLogicalUsed => disks_logical = true,

                | DiskPhysicalDrive
                | DiskPhysicalFirmware
                | DiskPhysicalIndex
                | DiskPhysicalInterface
                | DiskPhysicalMedia
                | DiskPhysicalParted
                | DiskPhysicalSerial
                | DiskPhysicalSize
                | DiskPhysicalStatus => disks_physical = true,

                | DiskPhysicalPartition => { disks_physical = true; disks_physical_partition = true },

                | NetworkCard
                | NetworkConfigSpeed
                | NetworkDownload
                | NetworkIpv4
                | NetworkIpv6
                | NetworkMac
                | NetworkMtu
                | NetworkName
                | NetworkSsid
                | NetworkUpload => networks = true,

                | TopProcessCpu
                | TopProcessMemory
                | TopProcessName
                | TopProcessRuntime
                | AllProcessCpu
                | AllProcessMemory
                | AllProcessName
                | AllProcessRuntime => processes = true,

                | SoftwareInstallDate
                | SoftwareName
                | SoftwareSize
                | SoftwareSource
                | SoftwareVersion => software = true,

                _ => continue,
            }
        }
        
        //: Define parent field
        let mut ram_physical_list: Option<&Vec<types::Ram>> = None;
        let mut gpu_list: Option<&Vec<types::Gpu>> = None;
        let mut disk_logical_list: Option<&Vec<types::LogicalDisk>> = None;
        let mut disk_physical_list: Option<&Vec<types::PhysicalDisk>> = None;

        //: Build payload
        let mut info_payload = InfoPayload::default();
        for task in &mut batch {
            match task.id {
                GeneralHost => {
                    let general = info_payload.general.get_or_insert_with(GeneralPayload::default);
                    let last_general = self.last_info_payload.general.get_or_insert_with(GeneralPayload::default);
                    general.host = check_diff_update(self.info.get_host(), &mut last_general.host, &task.limit);
                },
                GeneralBootTime => {
                    let general = info_payload.general.get_or_insert_with(GeneralPayload::default);
                    let last_general = self.last_info_payload.general.get_or_insert_with(GeneralPayload::default);
                    general.boot_time = check_diff_update(self.info.get_boot_time(), &mut last_general.boot_time, &task.limit);
                }
                GeneralRunTime => {
                    let general = info_payload.general.get_or_insert_with(GeneralPayload::default);
                    let last_general = self.last_info_payload.general.get_or_insert_with(GeneralPayload::default);
                    general.run_time = check_diff_update(self.info.get_up_time(), &mut last_general.run_time, &task.limit);
                },

                MachineArchitecture => {
                    let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
                    let last_machine = self.last_info_payload.machine.get_or_insert_with(MachinePayload::default);
                    machine.architecture = check_diff_update(self.info.get_architecture().to_string(), &mut last_machine.architecture, &task.limit);
                },
                MachineModel => {
                    let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
                    let last_machine = self.last_info_payload.machine.get_or_insert_with(MachinePayload::default);
                    machine.model = check_diff_update(self.info.get_system_model().to_string(), &mut last_machine.model, &task.limit);
                },
                MachineProducer => {
                    let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
                    let last_machine = self.last_info_payload.machine.get_or_insert_with(MachinePayload::default);
                    machine.producer = check_diff_update(self.info.get_producer().to_string(), &mut last_machine.producer, &task.limit);
                },
                MachineType => {
                    let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
                    let last_machine = self.last_info_payload.machine.get_or_insert_with(MachinePayload::default);
                    machine.r#type = check_diff_update(self.info.get_machine_type().to_string(), &mut last_machine.r#type, &task.limit);
                },
                MachineSerial => {
                    let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
                    let last_machine = self.last_info_payload.machine.get_or_insert_with(MachinePayload::default);
                    machine.serial = check_diff_update(self.info.get_product_serial().to_string(), &mut last_machine.serial, &task.limit);
                },

                MotherboardCpuSlot => {
                    let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    if let Some(cpu_slot) = self.info.get_cpu_slot() {
                        mobo.cpu_slot = check_diff_update(cpu_slot, &mut last_mobo.cpu_slot, &task.limit);
                    }
                },
                MotherboardGpuSlot => {
                    let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    if let Some(gpu_slot) = self.info.get_gpu_slot() {
                        mobo.gpu_slot = check_diff_update(gpu_slot, &mut last_mobo.gpu_slot, &task.limit)
                    }
                },
                MotherboardName => {
                    let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    mobo.name = check_diff_update(self.info.get_motherboard().to_string(), &mut last_mobo.name, &task.limit)
                },
                MotherboardRamSlot => {
                    let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    if let Some(ram_slot) = self.info.get_ram_slot() {
                        mobo.ram_slot = check_diff_update(ram_slot, &mut last_mobo.ram_slot, &task.limit)
                    }
                },
                MotherboardSerial => {
                    let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    mobo.serial = check_diff_update(self.info.get_motherboard_serial().to_string(), &mut last_mobo.serial, &task.limit)
                },
                MotherboardTempe => {
                    let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    if let Some(tempe) = self.info.get_tempe_mobo() {
                        mobo.tempe = check_diff_update(tempe, &mut last_mobo.tempe, &task.limit)
                    }
                },

                BiosSecureBoot => { // ! This cast true to 1, false to 0
                    let bios = info_payload.bios.get_or_insert_with(BiosPayload::default);
                    let last_bios = self.last_info_payload.bios.get_or_insert_with(BiosPayload::default);
                    if let Some(sercure_boot) = self.info.get_is_secure_boot() {
                        bios.secure_boot = check_diff_update(sercure_boot as u64, &mut last_bios.secure_boot , &task.limit)
                    }
                },
                BiosVendor => {
                    let bios = info_payload.bios.get_or_insert_with(BiosPayload::default);
                    let last_bios = self.last_info_payload.bios.get_or_insert_with(BiosPayload::default);
                    bios.vendor = check_diff_update(self.info.get_bios_vendor().to_string(), &mut last_bios.vendor, &task.limit)
                },
                BiosVersion => {
                    let bios = info_payload.bios.get_or_insert_with(BiosPayload::default);
                    let last_bios = self.last_info_payload.bios.get_or_insert_with(BiosPayload::default);
                    bios.version = check_diff_update(self.info.get_bios_version().to_string(), &mut last_bios.version, &task.limit)
                },

                OsDistro => {
                    let os = info_payload.os.get_or_insert_with(OsPayload::default);
                    let last_os = self.last_info_payload.os.get_or_insert_with(OsPayload::default);
                    os.distro = check_diff_update(self.info.get_os_distro().to_string(), &mut last_os.distro, &task.limit)
                },
                OsKernel => {
                    let os = info_payload.os.get_or_insert_with(OsPayload::default);
                    let last_os = self.last_info_payload.os.get_or_insert_with(OsPayload::default);
                    os.kernel = check_diff_update(self.info.get_kernel().to_string(), &mut last_os.kernel, &task.limit)
                },
                OsName => {
                    let os = info_payload.os.get_or_insert_with(OsPayload::default);
                    let last_os = self.last_info_payload.os.get_or_insert_with(OsPayload::default);
                    os.name = check_diff_update(self.info.get_os_name().to_string(), &mut last_os.name, &task.limit)
                },
                OsVersion => {
                    let os = info_payload.os.get_or_insert_with(OsPayload::default);
                    let last_os = self.last_info_payload.os.get_or_insert_with(OsPayload::default);
                    os.version = check_diff_update(self.info.get_os_version(), &mut last_os.version, &task.limit)
                },

                


                _ => log::warn!("Unknown task ID: {:?}", task.id),
            }
        }


        //: Finalize json
        let info_json = match serde_json::to_value(info_payload) {
            Ok(json) => json,
            Err(e) => {
                let err_msg = e.to_string();
                serde_json::to_value(err_msg).expect("Can not error here")
            },
        };

        let payload_json = json!({
            "agent_version": "0.3.0.scheduler_run",
            "in_test": true,
            "info": info_json,
        });

        (batch, payload_json)
    }
*/

    /// Use to save json as markdown in disk
    pub fn store_payload_md(&self, payload: &Json, name: &str) {
        let Ok(pretty_json) = serde_json::to_string_pretty(&payload) else {
            log::error!("Failed to serialize payload to JSON");
            return;
        };

        let md_content = format!("```json\n{}\n```", pretty_json);

        if let Err(e) = fs::create_dir_all("doc/sample/") {
            log::error!("Failed to create directory 'doc/sample/': {}", e);
            return;
        }

        let path = PathBuf::from(format!("doc/sample/{}.md", name));
        if let Err(e) = fs::write(path, md_content) {
            log::error!("Failed to write file '{}.md': {}", name, e);
        } else {
            log::info!("Successfully saved {}.", name);
        }
    }

    ///TODO this was too much handling, needed fix
    pub fn process_batch_mock_up(&mut self, batch: &Vec<ScheduledTask>) -> Json {
        //: Vec batch to hash map
        let task_map: FxHashMap<TaskID, ScheduledTask> = batch
            .iter()
            .map(|task| (task.id.clone(), task.clone()))
            .collect();

        //: Refresh
        self.info.prepare();

        //: Build payload
        let mut info_payload = InfoPayload::default();

            //: General
        {
            if let Some(task) = task_map.get(&GeneralHost) {
                let last_general = self.last_info_payload.general.get_or_insert_with(GeneralPayload::default);
                if let Some(update_host) = check_diff_update(self.info.get_host(), &mut last_general.host, &task.limit) {
                    let general = info_payload.general.get_or_insert_with(GeneralPayload::default);
                    general.host = Some(update_host)
                }
            }

            if let  Some(task) = task_map.get(&GeneralBootTime) {
                let last_general = self.last_info_payload.general.get_or_insert_with(GeneralPayload::default);
                if let Some(update_boot_time) = check_diff_update(self.info.get_boot_time(), &mut last_general.boot_time, &task.limit) {
                    let general = info_payload.general.get_or_insert_with(GeneralPayload::default);
                    general.boot_time = Some(update_boot_time);
                }
            }

            if let  Some(task) = task_map.get(&GeneralRunTime) {
                let last_general = self.last_info_payload.general.get_or_insert_with(GeneralPayload::default);
                if let Some(update_run_time) = check_diff_update(self.info.get_up_time(), &mut last_general.run_time, &task.limit) {
                    let general = info_payload.general.get_or_insert_with(GeneralPayload::default);
                    general.run_time = Some(update_run_time);
                }
            }
        }

            //: Machine
        {
        // Machine serial will always be include in the payload, so check for it at final step
            if let  Some(task) = task_map.get(&MachineArchitecture) {
                let last_machine = self.last_info_payload.machine.get_or_insert_with(MachinePayload::default);
                if let Some(update_arch) = check_diff_update(self.info.get_architecture(), &mut last_machine.architecture, &task.limit) {
                    let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
                    machine.architecture = Some(update_arch);
                }
            }

            if let  Some(task) = task_map.get(&MachineModel) {
                let last_machine = self.last_info_payload.machine.get_or_insert_with(MachinePayload::default);
                if let Some(update_model) = check_diff_update(self.info.get_system_model(), &mut last_machine.model, &task.limit) {
                    let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
                    machine.model = Some(update_model);
                }
            }

            if let  Some(task) = task_map.get(&MachineProducer) {
                let last_machine = self.last_info_payload.machine.get_or_insert_with(MachinePayload::default);
                if let Some(update_producer) = check_diff_update(self.info.get_producer(), &mut last_machine.producer, &task.limit) {
                    let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
                    machine.producer = Some(update_producer);
                }
            }

            if let  Some(task) = task_map.get(&MachineType) {
                let last_machine = self.last_info_payload.machine.get_or_insert_with(MachinePayload::default);
                if let Some(update_type) = check_diff_update(self.info.get_machine_type(), &mut last_machine.r#type, &task.limit) {
                    let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
                    machine.r#type = Some(update_type)
                }
            }
        }

            //: Motherboard
        {
            if let  Some(task) = task_map.get(&MotherboardName) {
                let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                if let Some(update_name) = check_diff_update(self.info.get_motherboard(), &mut last_mobo.name, &task.limit) {
                    let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    mobo.name = Some(update_name);
                }
            }

            if let  Some(task) = task_map.get(&MotherboardSerial) {
                if let Some(serial) = self.info.get_motherboard_serial() {
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    if let Some(update_serial) = check_diff_update(serial, &mut last_mobo.serial, &task.limit) {
                        let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                        mobo.serial = Some(update_serial);
                    }
                }
            }

            if let  Some(task) = task_map.get(&MotherboardTempe) {
                if let Some(tempe) = self.info.get_tempe_mobo() {
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    if let Some(update_tempe) = check_diff_update(tempe, &mut last_mobo.tempe, &task.limit) {
                        let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                        mobo.tempe = Some(update_tempe);
                    }
                }
            }

            if let  Some(task) = task_map.get(&MotherboardCpuSlot) {
                if let Some(slot) = self.info.get_cpu_slot() {
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    if let Some(update_cpu_slot) = check_diff_update(slot, &mut last_mobo.cpu_slot, &task.limit) {
                        let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                        mobo.cpu_slot = Some(update_cpu_slot);
                    }
                }
            }
        
            if let  Some(task) = task_map.get(&MotherboardGpuSlot) {
                if let Some(slot) = self.info.get_gpu_slot() {
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    if let Some(update_gpu_slot) = check_diff_update(slot, &mut last_mobo.gpu_slot, &task.limit) {
                        let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                        mobo.gpu_slot = Some(update_gpu_slot);
                    }
                }
            }

            if let  Some(task) = task_map.get(&MotherboardRamSlot) {
                if let Some(slot) = self.info.get_ram_slot() {
                    let last_mobo = self.last_info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                    if let Some(update_ram_slot) = check_diff_update(slot, &mut last_mobo.ram_slot, &task.limit) {
                        let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                        mobo.ram_slot = Some(update_ram_slot);
                    }
                }
            }
        }

           //: Bios
        {
            if let  Some(task) = task_map.get(&BiosVersion) {
                let last_bios = self.last_info_payload.bios.get_or_insert_with(BiosPayload::default);
                if let Some(update_version) = check_diff_update(self.info.get_bios_version(), &mut last_bios.version, &task.limit) {
                    let bios = info_payload.bios.get_or_insert_with(BiosPayload::default);
                    bios.version = Some(update_version);
                }
            }

            if let  Some(task) = task_map.get(&BiosVendor) {
                let last_bios = self.last_info_payload.bios.get_or_insert_with(BiosPayload::default);
                if let Some(update_vendor) = check_diff_update(self.info.get_bios_vendor(), &mut last_bios.vendor, &task.limit) {
                    let bios = info_payload.bios.get_or_insert_with(BiosPayload::default);
                    bios.vendor = Some(update_vendor);
                }
            }

            if let  Some(task) = task_map.get(&BiosSecureBoot) {
                if let Some(secure) = self.info.get_is_secure_boot() {
                    let last_bios = self.last_info_payload.bios.get_or_insert_with(BiosPayload::default);
                    if let Some(update_sercure) = check_diff_update(secure as u64, &mut last_bios.version, &task.limit) {
                        let bios = info_payload.bios.get_or_insert_with(BiosPayload::default);
                        bios.secure_boot = Some(update_sercure);
                    }
                }
            }
        }
        
            //: OS
        {
            if let  Some(task) = task_map.get(&OsDistro) {
                let last_os = self.last_info_payload.os.get_or_insert_with(OsPayload::default);
                if let Some(update_distro) = check_diff_update(self.info.get_os_distro(), &mut last_os.distro, &task.limit) {
                    let os = info_payload.os.get_or_insert_with(OsPayload::default);
                    os.distro = Some(update_distro);
                }
            }

            if let  Some(task) = task_map.get(&OsName) {
                let last_os = self.last_info_payload.os.get_or_insert_with(OsPayload::default);
                if let Some(update_name) = check_diff_update(self.info.get_os_name(), &mut last_os.name, &task.limit) {
                    let os = info_payload.os.get_or_insert_with(OsPayload::default);
                    os.name = Some(update_name);
                }
            }

            if let  Some(task) = task_map.get(&OsKernel) {
                let last_os = self.last_info_payload.os.get_or_insert_with(OsPayload::default);
                if let Some(update_kernel) = check_diff_update(self.info.get_kernel(), &mut last_os.kernel, &task.limit) {
                    let os = info_payload.os.get_or_insert_with(OsPayload::default);
                    os.kernel = Some(update_kernel);
                }
            }

            if let  Some(task) = task_map.get(&OsVersion) {
                let last_os = self.last_info_payload.os.get_or_insert_with(OsPayload::default);
                if let Some(update_version) = check_diff_update(self.info.get_os_version(), &mut last_os.version, &task.limit) {
                    let os = info_payload.os.get_or_insert_with(OsPayload::default);
                    os.version = Some(update_version);
                }
            }
        }

            //: Cpu
        {
            if let Some(task) = task_map.get(&CpuName) {
                let last_cpu = self.last_info_payload.cpu.get_or_insert_with(CpuPayload::default);
                if let Some(update_name) = check_diff_update(self.info.get_cpu_name(), &mut last_cpu.name, &task.limit) {
                    let cpu = info_payload.cpu.get_or_insert_with(CpuPayload::default);
                    cpu.name = Some(update_name);
                }
            }

            if let Some(task) = task_map.get(&CpuCore) {
                let last_cpu = self.last_info_payload.cpu.get_or_insert_with(CpuPayload::default);
                if let Some(update_core) = check_diff_update(self.info.get_cpu_core(), &mut last_cpu.core, &task.limit) {
                    let cpu = info_payload.cpu.get_or_insert_with(CpuPayload::default);
                    cpu.core = Some(update_core);
                }
            }

            if let Some(task) = task_map.get(&CpuUsage) {
                let last_cpu = self.last_info_payload.cpu.get_or_insert_with(CpuPayload::default);
                if let Some(update_usage) = check_diff_update(self.info.get_cpu_usage(), &mut last_cpu.usage, &task.limit) {
                    let cpu = info_payload.cpu.get_or_insert_with(CpuPayload::default);
                    cpu.usage = Some(update_usage);
                }
            }

            if let Some(task) = task_map.get(&CpuFreq) {
                let last_cpu = self.last_info_payload.cpu.get_or_insert_with(CpuPayload::default);
                if let Some(update_freq) = check_diff_update(self.info.get_cpu_freq(), &mut last_cpu.frequency, &task.limit) {
                    let cpu = info_payload.cpu.get_or_insert_with(CpuPayload::default);
                    cpu.frequency = Some(update_freq);
                }
            }

            if let Some(task) = task_map.get(&CpuTempe) {
                if let Some(tempe) = self.info.get_cpu_tempe() {
                    let last_cpu = self.last_info_payload.cpu.get_or_insert_with(CpuPayload::default);
                    if let Some(update_tempe) = check_diff_update(tempe, &mut last_cpu.temperature, &task.limit) {
                        let cpu = info_payload.cpu.get_or_insert_with(CpuPayload::default);
                        cpu.temperature = Some(update_tempe);
                    }
                }
            }
        }

            //: Swap
        {
            if let Some(task) = task_map.get(&SwapTotal) {
                let last_swap = self.last_info_payload.swap.get_or_insert_with(SwapPayload::default);
                if let Some(update_total) = check_diff_update(self.info.get_swap_total(), &mut last_swap.total, &task.limit) {
                    let swap = info_payload.swap.get_or_insert_with(SwapPayload::default);
                    swap.total = Some(update_total);
                }
            }

            if let Some(task) = task_map.get(&SwapUsage) {
                let last_swap = self.last_info_payload.swap.get_or_insert_with(SwapPayload::default);
                if let Some(update_usage) = check_diff_update(self.info.get_swap_usage(), &mut last_swap.usage, &task.limit) {
                    let swap = info_payload.swap.get_or_insert_with(SwapPayload::default);
                    swap.usage = Some(update_usage);
                }
            }
        }

            //: Process count
        if let Some(task) = task_map.get(&ProcessCount) {
            info_payload.process_count = check_diff_update(
                self.info.get_process_count(), 
                &mut self.last_info_payload.process_count, 
                &task.limit
            );
        }

            //: Battery
        {
            if let Some(task) = task_map.get(&BatteryPercentage) {
                if let Some(percent) = self.info.get_battery_percentage() {
                    let last_batt = self.last_info_payload.battery.get_or_insert_with(BatteryPayload::default);
                    if let  Some(update_percent) = check_diff_update(percent, &mut last_batt.percentage, &task.limit) {
                        let batt = info_payload.battery.get_or_insert_with(BatteryPayload::default);
                        batt.percentage = Some(update_percent)
                    }
                }
            }

            if let Some(task) = task_map.get(&BatteryPercentage) {
                if let Some(plugged) = self.info.get_battery_is_plugged_in() {
                    let last_batt = self.last_info_payload.battery.get_or_insert_with(BatteryPayload::default);
                    if let Some(update_plugged) = check_diff_update(plugged as u64, &mut last_batt.power_plugged, &task.limit) {
                        let batt = info_payload.battery.get_or_insert_with(BatteryPayload::default);
                        batt.power_plugged = Some(update_plugged)
                    }
                }
            }
        }
        
            //: Ram
        {
            if let Some(task) = task_map.get(&RamTotal) {
                let last_ram = self.last_info_payload.ram.get_or_insert_with(RamPayload::default);
                if let Some(update_total) = check_diff_update(self.info.get_ram_total(), &mut last_ram.total, &task.limit) {
                    let ram = info_payload.ram.get_or_insert_with(RamPayload::default);
                    ram.total = Some(update_total);
                }
            }

            if let Some(task) = task_map.get(&RamUsage) {
                let last_ram = self.last_info_payload.ram.get_or_insert_with(RamPayload::default);
                if let Some(update_usage) = check_diff_update(self.info.get_ram_usage(), &mut last_ram.usage, &task.limit) {
                    let ram = info_payload.ram.get_or_insert_with(RamPayload::default);
                    ram.usage = Some(update_usage);
                }
            }
        }

            //: Ram Physical
        {
        let mut diff_ram_physical_payloads = Vec::new();

        if let Some(rams) = self.info.get_ram_list() {
            // 1. Unpack or initialize the top-level cache safely
            let last_ram_cache = self.last_info_payload.ram.get_or_insert_with(RamPayload::default);
            let last_phys_vec = last_ram_cache.physical.get_or_insert_with(Vec::new);

            // 2. Iterate using enumerate to match the cache index
            for (i, ram) in rams.iter().enumerate() {
                // Grow the cache vector if we found a new RAM stick
                if i >= last_phys_vec.len() {
                    last_phys_vec.push(RamPhysicalPayload::default());
                }
                let last_phys = &mut last_phys_vec[i];

                let mut diff_payload = RamPhysicalPayload::default();
                let mut has_diff = false;

                // Use .get() instead of .remove() so it applies to all RAM sticks!
                if let Some(task) = task_map.get(&RamPhysicalSerial) {
                    
                    // Pass in the cloned value, the mutable cache ref, and the limit
                    if let Some(updated_serial) = check_diff_update(
                        ram.serial.clone(), 
                        &mut last_phys.serial, 
                        &task.limit
                    ) {
                        diff_payload.serial = Some(updated_serial);
                        has_diff = true;
                    }
                    
                    // You can repeat the block above for ram.name, ram.type_, etc.
                }

                if let Some(task) = task_map.get(&RamPhysicalType) {
                    if let Some(update_type) = check_diff_update(
                        ram.type_.clone(),
                        &mut last_phys.r#type,
                        &task.limit
                    ) {
                        diff_payload.r#type = Some(update_type);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&RamPhysicalConfigSpeed) {
                    if let Some(update_speed) = check_diff_update(
                        ram.speed.clone(),
                        &mut last_phys.config_speed,
                        &task.limit
                    ) {
                        diff_payload.config_speed = Some(update_speed);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&RamPhysicalSize) {
                    if let Some(update_size) = check_diff_update(
                        ram.size,
                        &mut last_phys.size,
                        &task.limit
                    ) {
                        diff_payload.size = Some(update_size);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&RamPhysicalName) {
                    if let Some(update_name) = check_diff_update(
                        ram.name.clone(),
                        &mut last_phys.name,
                        &task.limit
                    ) {
                        diff_payload.name = Some(update_name);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&RamPhysicalFormFactor) {
                    if let Some(update_form) = check_diff_update(
                        ram.form_factor.clone(),
                        &mut last_phys.form_factor,
                        &task.limit
                    ) {
                        diff_payload.form_factor = Some(update_form);
                        has_diff = true;
                    }
                }

                // Only push to the final vector if something actually changed
                if has_diff {
                    if diff_payload.bank.is_none() {
                        diff_payload.bank = Some(AgentValue::Text(ram.get_bank().to_string()));
                    }
                    diff_ram_physical_payloads.push(diff_payload);
                }
            }
        }

        // Now you can wrap diff_physical_payloads in a new RamPayload if it's not empty
        if !diff_ram_physical_payloads.is_empty() {
            if let Some(ram_payload) = info_payload.ram.as_mut() {
                ram_payload.physical = Some(diff_ram_physical_payloads);
            }
        }
        }

            //: Gpu
        {
        let mut diff_gpu_payloads: Vec<GpuPayload> = Vec::new();

        let gpu_vec = self.info.get_gpu_list();
        let last_gpu_vec = self.last_info_payload.gpu.get_or_insert_with(Vec::new);
        
        for (i, gpu) in gpu_vec.iter().enumerate() {
            if i >= last_gpu_vec.len() {
                last_gpu_vec.push(GpuPayload::default());
            }
            let last_gpu = &mut last_gpu_vec[i];

            let mut diff_payload = GpuPayload::default();
            let mut has_diff = false;

            if let Some(task) = task_map.get(&GpuDriver) {
                if let Some(update_driver) = check_diff_update(
                    gpu.get_driver_version(),
                    &mut last_gpu.driver,
                    &task.limit
                ) {
                    diff_payload.driver = Some(update_driver);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&GpuFreq) {
                if let Some(update_freq) = check_diff_update(
                    gpu.get_freq(),
                    &mut last_gpu.driver,
                    &task.limit
                ) {
                    diff_payload.frequency = Some(update_freq);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&GpuMaxClock) {
                if let Some(clock) = gpu.get_max_clock() {
                    if let Some(update_clock) = check_diff_update(
                        clock,
                        &mut last_gpu.max_clock,
                        &task.limit
                    ) {
                        diff_payload.max_clock = Some(update_clock);
                        has_diff = true;
                    }
                }
            }

            if let Some(task) = task_map.get(&GpuSerial) {
                if let Some(update_serial) = check_diff_update(
                    gpu.get_serial(),
                    &mut last_gpu.serial,
                    &task.limit
                ) {
                    diff_payload.serial = Some(update_serial);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&GpuTempe) {
                if let Some(update_tempe) = check_diff_update(
                    gpu.get_tempe(),
                    &mut last_gpu.serial,
                    &task.limit
                ) {
                    diff_payload.temperature = Some(update_tempe);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&GpuUtilization) {
                if let Some(update_util) = check_diff_update(
                    gpu.get_util(),
                    &mut last_gpu.utilization,
                    &task.limit
                ) {
                    diff_payload.utilization = Some(update_util);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&GpuVramTotal) {
                if let Some(update_vram_total) = check_diff_update(
                    gpu.get_vram_total(),
                    &mut last_gpu.vram_total,
                    &task.limit
                ) {
                    diff_payload.vram_total = Some(update_vram_total);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&GpuVramUsage) {
                if let Some(update_vram_usage) = check_diff_update(
                    gpu.get_vram_usage(),
                    &mut last_gpu.vram_usage,
                    &task.limit
                ) {
                    diff_payload.vram_usage = Some(update_vram_usage);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&GpuName) {
                if let Some(update_name) = check_diff_update(
                    gpu.get_name(),
                    &mut last_gpu.name,
                    &task.limit
                ) {
                    diff_payload.name = Some(update_name);
                    has_diff = true;
                }
            }

            if has_diff {
                if diff_payload.name.is_none() {
                    diff_payload.name = Some(AgentValue::Text(gpu.get_name()));
                }
                diff_gpu_payloads.push(diff_payload);
            }
        }

        if !diff_gpu_payloads.is_empty() {
            info_payload.gpu = Some(diff_gpu_payloads);
        }
        }

            //: Logical Disk
        {
        let mut diff_logical_disk_payload: Vec<LogicalDiskPayload> = Vec::new();

        let logical_disks = self.info.get_logical_disk_list();
        let last_disk_cache = self.last_info_payload.disk.get_or_insert_with(DiskPayload::default);
        let last_logical_disk_vec = last_disk_cache.logical.get_or_insert_with(Vec::new);

        for (i, logical_disk) in logical_disks.iter().enumerate() {
            if i >= last_logical_disk_vec.len() {
                last_logical_disk_vec.push(LogicalDiskPayload::default());
            }
            let last_logical_disk = &mut last_logical_disk_vec[i];

            let mut diff_payload = LogicalDiskPayload::default();
            let mut has_diff = false;

            if let Some(task) = task_map.get(&DiskLogicalName) {
                if let Some(update_name) = check_diff_update(
                    logical_disk.get_name(),
                    &mut last_logical_disk.name,
                    &task.limit
                ) {
                    diff_payload.name = Some(update_name);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&DiskLogicalFileName) {
                if let Some(update_file_name) = check_diff_update(
                    logical_disk.get_file_system(),
                    &mut last_logical_disk.file_system,
                    &task.limit
                ) {
                    diff_payload.file_system = Some(update_file_name);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&DiskLogicalMountPoint) {
                if let Some(update_mount) = check_diff_update(
                    logical_disk.get_mount_point(),
                    &mut last_logical_disk.mount_point,
                    &task.limit
                ) {
                    diff_payload.mount_point = Some(update_mount);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&DiskLogicalRemovable) {
                if let Some(update_removable) = check_diff_update(
                    logical_disk.get_removable() as u64,
                    &mut last_logical_disk.removable,
                    &task.limit
                ) {
                    diff_payload.removable = Some(update_removable);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&DiskLogicalTotal) {
                if let Some(update_total) = check_diff_update(
                    logical_disk.get_total(),
                    &mut last_logical_disk.total,
                    &task.limit
                ) {
                    diff_payload.total = Some(update_total);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&DiskLogicalUsed) {
                if let Some(update_used) = check_diff_update(
                    logical_disk.get_used(),
                    &mut last_logical_disk.used,
                    &task.limit
                ) {
                    diff_payload.used = Some(update_used);
                    has_diff = true;
                }
            }

            if has_diff {
                if diff_payload.mount_point.is_none() {
                    diff_payload.mount_point = Some(AgentValue::Text(logical_disk.get_mount_point()));
                }
                diff_logical_disk_payload.push(diff_payload);
            }
        }

        if !diff_logical_disk_payload.is_empty() {
            let logical_disk_payload = info_payload.disk.get_or_insert_default();
            logical_disk_payload.logical = Some(diff_logical_disk_payload);
        }
        }

            //: Physical Disk
        {
        let mut diff_physical_disk_payloads: Vec<PhysicalDiskPayload> = Vec::new();

        let physical_disks_wrapper = self.info.get_physical_disk_list();
        if let Some(physical_disks) = physical_disks_wrapper {
            let last_disk_cache = self.last_info_payload.disk.get_or_insert_with(DiskPayload::default);
            let last_physical_disk_vec = last_disk_cache.physical.get_or_insert_with(Vec::new);
            
            for (i, physical_disk) in physical_disks.iter().enumerate() {
                if i >= last_physical_disk_vec.len() {
                    last_physical_disk_vec.push(PhysicalDiskPayload::default());
                }
                let last_physical_disk = &mut last_physical_disk_vec[i];
                
                let mut diff_payload = PhysicalDiskPayload::default();
                let mut has_diff = false;

                if let Some(task) = task_map.get(&DiskPhysicalDrive) {
                    if let Some(update_drive) = check_diff_update(
                        physical_disk.get_drive(),
                        &mut last_physical_disk.drive,
                        &task.limit
                    ) {
                        diff_payload.drive = Some(update_drive);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&DiskPhysicalFirmware) {
                    if let Some(update_firmware) = check_diff_update(
                        physical_disk.get_firmware(),
                        &mut last_physical_disk.firmware,
                        &task.limit
                    ) {
                        diff_payload.firmware = Some(update_firmware);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&DiskPhysicalIndex) {
                    if let Some(update_index) = check_diff_update(
                        physical_disk.get_index(),
                        &mut last_physical_disk.index,
                        &task.limit
                    ) {
                        diff_payload.index = Some(update_index);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&DiskPhysicalInterface) {
                    if let Some(update_interface) = check_diff_update(
                        physical_disk.get_interface(),
                        &mut last_physical_disk.interface,
                        &task.limit
                    ) {
                        diff_payload.interface = Some(update_interface);
                        has_diff = true
                    }
                }

                if let Some(task) = task_map.get(&DiskPhysicalMedia) {
                    if let Some(update_media) = check_diff_update(
                        physical_disk.get_media(),
                        &mut last_physical_disk.media,
                        &task.limit
                    ) {
                        diff_payload.media = Some(update_media);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&DiskPhysicalModel) {
                    if let Some(update_model) = check_diff_update(
                        physical_disk.get_model(),
                        &mut last_physical_disk.model,
                        &task.limit
                    ) {
                        diff_payload.model = Some(update_model);
                        has_diff = true;
                    }
                }
                
                if let Some(task) = task_map.get(&DiskPhysicalSerial) {
                    if let Some(update_serial) = check_diff_update(
                        physical_disk.get_serial(),
                        &mut last_physical_disk.serial,
                        &task.limit
                    ) {
                        diff_payload.serial = Some(update_serial);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&DiskPhysicalSize) {
                    if let Some(update_size) = check_diff_update(
                        physical_disk.get_size(),
                        &mut last_physical_disk.size,
                        &task.limit
                    ) {
                        diff_payload.size = Some(update_size);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&DiskPhysicalStatus) {
                    if let Some(update_status) = check_diff_update(
                        physical_disk.get_status(),
                        &mut last_physical_disk.status,
                        &task.limit
                    ) {
                        diff_payload.status = Some(update_status);
                        has_diff = true;
                    }
                }

                if let Some(task) = task_map.get(&DiskPhysicalPartition) {
                    let last_part_vec = last_physical_disk.partition.get_or_insert_with(Vec::new);
                    let mut part_vec = Vec::new();

                    if let Some(parts) = physical_disk.get_partition() {
                        for (j, part) in parts.iter().enumerate() {
                            if j >= last_part_vec.len() {
                                last_part_vec.push(PartitionPayload::default());
                            }
                            let last_part = &mut last_part_vec[j];

                            let mut diff_part = PartitionPayload::default();
                            let mut has_diff_part = false;

                            if let Some(update_part_name) = check_diff_update(
                                part.get_name(),
                                &mut last_part.name,
                                &task.limit
                            ) {
                                diff_part.name = Some(update_part_name);
                                has_diff_part = true;
                            }

                            if let Some(update_part_size) = check_diff_update(
                                part.get_size(),
                                &mut last_part.size,
                                &task.limit
                            ) {
                                diff_part.size = Some(update_part_size);
                                has_diff_part = true;
                            }

                            if has_diff_part {
                                part_vec.push(diff_part);
                            }
                        }
                    }

                    if let Some(task) = task_map.get(&DiskPhysicalNumPartition) {
                        if let Some(update_num_part) = check_diff_update(
                            physical_disk.get_partition_number(),
                            &mut last_physical_disk.num_part,
                            &task.limit
                        ) {
                            diff_payload.num_part = Some(update_num_part);
                            has_diff = true;
                        }
                    }
                    
                    if !part_vec.is_empty() {
                        diff_payload.partition = Some(part_vec);
                        has_diff = true;
                    }
                }

                if has_diff {
                    if diff_payload.drive.is_none() {
                        diff_payload.drive = Some(AgentValue::Text(physical_disk.get_drive().to_string()));
                    }
                    diff_physical_disk_payloads.push(diff_payload);
                }
            }
        }

        if !diff_physical_disk_payloads.is_empty() {
            let physical_disk_payload = info_payload.disk.get_or_insert_default();
            physical_disk_payload.physical = Some(diff_physical_disk_payloads);
        }
        }

            //: Network
        {
        let mut diff_net_payload: Vec<NetworkPayload> = Vec::new();

        let network_vec = self.info.get_network_list();
        let last_network_vec = self.last_info_payload.network.get_or_insert_with(Vec::new);

        for (i, network) in network_vec.iter().enumerate() {
            if i >= last_network_vec.len() {
                last_network_vec.push(NetworkPayload::default());
            }
            let last_network = &mut last_network_vec[i];

            let mut diff_payload = NetworkPayload::default();
            let mut has_diff = false;

            if let Some(task) = task_map.get(&NetworkCard) {
                if let Some(card) = network.get_card() {
                    if let Some(update_card) = check_diff_update(
                        card,
                        &mut last_network.card,
                        &task.limit
                    ) {
                        diff_payload.card = Some(update_card);
                        has_diff = true;
                    }
                }
            }

            if let Some(task) = task_map.get(&NetworkConfigSpeed) {
                if let Some(speed) = network.get_config_speed() {
                    if let Some(update_speed) = check_diff_update(
                        speed,
                        &mut last_network.config_speed,
                        &task.limit
                    ) {
                        diff_payload.config_speed = Some(update_speed);
                        has_diff = true;
                    }
                }
            }

            if let Some(task) = task_map.get(&NetworkDownload) {
                if let Some(update_download) = check_diff_update(
                    network.get_download(),
                    &mut last_network.download,
                    &task.limit
                ) {
                    diff_payload.download = Some(update_download);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&NetworkIpv4) {
                if let Some(update_ipv4) = check_diff_update(
                    network.get_ipv4(),
                    &mut last_network.ipv4,
                    &task.limit
                ) {
                    diff_payload.ipv4 = Some(update_ipv4);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&NetworkIpv6) {
                if let Some(update_piv6) = check_diff_update(
                    network.get_ipv6(),
                    &mut last_network.ipv6,
                    &task.limit
                ) {
                    diff_payload.ipv6 = Some(update_piv6);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&NetworkMac) {
                if let Some(update_mac) = check_diff_update(
                    network.get_mac(),
                    &mut last_network.mac,
                    &task.limit
                ) {
                    diff_payload.mac = Some(update_mac);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&NetworkMtu) {
                if let Some(update_mtu) = check_diff_update(
                    network.get_mtu(),
                    &mut last_network.mtu,
                    &task.limit
                ) {
                    diff_payload.mtu = Some(update_mtu);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&NetworkName) {
                if let Some(update_name) = check_diff_update(
                    network.get_name(),
                    &mut last_network.name,
                    &task.limit
                ) {
                    diff_payload.name = Some(update_name);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&NetworkSsid) {
                if let Some(ssid) = network.get_ssid() {
                    if let Some(update_ssid) = check_diff_update(
                        ssid,
                        &mut last_network.ssid,
                        &task.limit
                    ) {
                        diff_payload.ssid = Some(update_ssid);
                        has_diff = true;
                    }
                }
            }

            if let Some(task) = task_map.get(&NetworkUpload) {
                if let Some(update_upload) = check_diff_update(
                    network.get_upload(),
                    &mut last_network.upload,
                    &task.limit
                ) {
                    diff_payload.upload = Some(update_upload);
                    has_diff = true;
                }
            }

            if has_diff {
                if diff_payload.name.is_none() {
                    diff_payload.name = Some(AgentValue::Text(network.get_name()));
                }
                diff_net_payload.push(diff_payload);
            }
        }

        if !diff_net_payload.is_empty() {
            info_payload.network = Some(diff_net_payload);
        }
        }

            //: Top Process
        {
        let mut diff_top_process_payload: Vec<ProcessPayload> = Vec::new();

        let top_processes = self.info.get_top_process_list();
        let last_top_processes = self.last_info_payload.top_process.get_or_insert_with(Vec::new);

        for (i, process) in top_processes.iter().enumerate() {
            if i >= last_top_processes.len() {
                last_top_processes.push(ProcessPayload::default());
            }
            let last_top_process = &mut last_top_processes[i];

            let mut diff_payload = ProcessPayload::default();
            let mut has_diff = false;

            if let Some(task) = task_map.get(&TopProcessCpu) {
                if let Some(update_cpu) = check_diff_update(
                    process.get_cpu_usage(),
                    &mut last_top_process.cpu,
                    &task.limit
                ) {
                    diff_payload.cpu = Some(update_cpu);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&TopProcessMemory) {
                if let Some(update_mem) = check_diff_update(
                    process.get_memory(),
                    &mut last_top_process.memory,
                    &task.limit
                ) {
                    diff_payload.memory = Some(update_mem);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&TopProcessRuntime) {
                if let Some(update_runtime) = check_diff_update(
                    process.get_runtime(),
                    &mut last_top_process.run_time,
                    &task.limit
                ) {
                    diff_payload.run_time = Some(update_runtime);
                    has_diff = true;
                }
            }

            if let Some(task) = task_map.get(&TopProcessName) {
                if let Some(update_name) = check_diff_update(
                    process.get_name(),
                    &mut last_top_process.name,
                    &task.limit
                ) {
                    diff_payload.name = Some(update_name);
                    has_diff = true;
                }
            }

            if has_diff {
                if diff_payload.name.is_none() {
                    diff_payload.name = Some(AgentValue::Text(process.get_name()));
                }
                diff_top_process_payload.push(diff_payload);
            }
        }

        if !diff_top_process_payload.is_empty() {
            info_payload.top_process = Some(diff_top_process_payload);
        }
        }
        
            //: Software
        // No cache checking since this have to many entries
        if let Some(_task) = task_map.get(&Software) {
            info_payload.software = self.info.get_software_list();
        }

        //: Make sure it have ID, either machine serial or motherboard serial
        let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
        machine.serial = match self.info.get_product_serial() {
            Some(serial) => {
                Some(AgentValue::Text(serial.to_string()))
            },
            None => {
                match self.info.get_motherboard_serial() {
                    Some(mobo_serial) => {
                        let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
                        mobo.serial = Some(AgentValue::Text(mobo_serial.to_string()));
                        None
                    },
                    None => Some(AgentValue::Text("UnknownMachine<Hash>".to_string())),
                }
            }
        };

        //: Finalize json
        let info_json = match serde_json::to_value(info_payload) {
        Ok(json) => json,
        Err(e) => {
            let err_msg = e.to_string();
            serde_json::to_value(err_msg).expect("Can not error here")
        },
        };

        // let payload_json = json!({
        //     "AGENT_VERSION": "0.3.0.scheduler_option",
        //     "TIME_STAMP": self.info.get_timestamp(),
        //     "IN_TEST": true,
        //     "INFO": info_json,
        // });

        let mut payload_json = info_json;
        if let Some(map) = payload_json.as_object_mut() {
            map.insert("AGENT_VERSION".into(), json!("0.3.0.scheduler_option"));
            map.insert("TIME_STAMP".into(), json!(self.info.get_timestamp()));
            map.insert("IN_TEST".into(), json!(true));
        }

        //: Save payload debug
        if cfg!(debug_assertions) {
            let full_scan_payload = Payload::new();
            full_scan_payload.get_json_v3_fullscan();

            self.store_payload_md(&payload_json, "v3_scheduler_config");

            if let Ok(last_info) = serde_json::to_value(&self.last_info_payload) {
                self.store_payload_md(&last_info, "last_info");
            } else {
                log::debug!("Can not save last info!");
            }
            
            log::debug!("Save payload debug");
        }

        payload_json
    }
}

//: Helper fn
//TODO maybe not consume the new value here
fn check_diff_update<T: CheckDiffUpdate>(new_value: T, old_value: &mut Option<AgentValue>, limit: &Option<AgentValue>) -> Option<AgentValue> {
    new_value.check_diff_update(old_value, limit)
}

//: Generic trait
pub trait CheckDiffUpdate {
    fn check_diff_update(self, old_value: &mut Option<AgentValue>, limit: &Option<AgentValue>) -> Option<AgentValue>;
}

impl CheckDiffUpdate for u64 {
    fn check_diff_update(self, old_value: &mut Option<AgentValue>, limit: &Option<AgentValue>) -> Option<AgentValue> {
        match (old_value.as_ref(), limit) {
            //: Have a thresshold => compare
            (Some(AgentValue::Int(old)), Some(AgentValue::Int(lim))) => {
                if self.abs_diff(*old) >= *lim {
                    *old_value = Some(AgentValue::Int(self));
                    Some(AgentValue::Int(self))
                } else {
                    None
                }
            },
            //: Don't have a thresshold => update if changed
            (Some(AgentValue::Int(old)), None) => {
                if self != *old {
                    *old_value = Some(AgentValue::Int(self));
                    Some(AgentValue::Int(self))
                } else {
                    None
                }
            },
            //: Init or else => fill
            _ => {
                *old_value = Some(AgentValue::Int(self));
                Some(AgentValue::Int(self))
            },
        }
    }
}

impl CheckDiffUpdate for f64 {
    fn check_diff_update(self, old_value: &mut Option<AgentValue>, limit: &Option<AgentValue>) -> Option<AgentValue> {
        match (old_value.as_ref(), limit) {
            //: Have a thresshold => compare
            (Some(AgentValue::Float(old)), Some(AgentValue::Float(lim))) => {
                if (self - old).abs() >= *lim {
                    *old_value = Some(AgentValue::Float(self));
                    Some(AgentValue::Float(self))
                } else {
                    None
                }
            },
            //: Don't have a thresshold => update if changed
            (Some(AgentValue::Float(old)), None) => {
                if self != *old {
                    *old_value = Some(AgentValue::Float(self));
                    Some(AgentValue::Float(self))
                } else {
                    None
                }
            },
            //: Init or else => fill
            _ => {
                *old_value = Some(AgentValue::Float(self));
                Some(AgentValue::Float(self))
            },
        }
    }
}

impl CheckDiffUpdate for String {
    fn check_diff_update(self, old_value: &mut Option<AgentValue>, _limit: &Option<AgentValue>) -> Option<AgentValue> {
        match old_value.as_ref() {
            Some(AgentValue::Text(old)) => {
                if self != *old {
                    *old_value = Some(AgentValue::Text(self.clone()));
                    Some(AgentValue::Text(self))
                } else {
                    None
                }
            },
            _ => {
                *old_value = Some(AgentValue::Text(self.clone()));
                Some(AgentValue::Text(self))
            },
        }
    }
}

impl CheckDiffUpdate for &str {
    fn check_diff_update(self, old_value: &mut Option<AgentValue>, limit: &Option<AgentValue>) -> Option<AgentValue> {
        let text = self.to_string();
        check_diff_update(text, old_value, limit)
    }
}