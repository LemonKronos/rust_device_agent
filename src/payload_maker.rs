///
/// Make make payload by calling info_gatherer, serialize to json
/// 

use std::fs;
use serde_json::{json, Value as Json};

use crate::utils::*;
use crate::info_gatherer::Info;
use crate::scheduler::{ScheduledTask, TaskID, {TaskID::*}, AgentValue};
use crate::types;

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
                "tempe": self.info.get_temp_mobo(),
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
                "temperature": self.info.get_cpu_temp(), // ℃
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
                    "TEMPERATURE": self.info.get_cpu_temp().unwrap_or_default().to_string(),
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
                    "TEMPERATURE": self.info.get_cpu_temp().unwrap_or_default().to_string(),
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
}

impl PayloadMaker {
    pub fn new() -> Self {
        Self { info: Info::new() }
    }

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

                | DiskPhysicalPartitionName
                | DiskPhysicalPartitionSize => { disks_physical = true; disks_physical_partition = true },

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
                    general.host = check_diff_update(task, self.info.get_host());
                },
                GeneralBootTime => {
                    let general = info_payload.general.get_or_insert_with(GeneralPayload::default);
                    general.boot_time = check_diff_update(task, self.info.get_boot_time());
                }
                
                DiskPhysicalFirmware => {
                    let disk_payload = info_payload.disk.get_or_insert_with(DiskPayload::default);
                    let disk_physical_payload_list = disk_payload.physical.get_or_insert_with(Vec::new);

                    if disk_physical_list.is_none() {
                        disk_physical_list = self.info.get_physical_disk_list();
                    }

                    if disk_physical_payload_list.is_empty() {
                        disk_physical_list.map(|pd| {
                            pd.iter().map(|pd| {
                                let new_pd_payload = PhysicalDiskPayload::default();
                                new_pd_payload.firmware = pd.firmware;
                                disk_physical_payload_list.push(new_pd_payload);
                            })
                        })
                    } else {
                        
                    };

                    disk_physical_payload_list.push(value);


                }
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
}

//: Helper fn
fn check_diff_update<T: CheckDiffUpdate>(task: &mut ScheduledTask, new_value: T) -> Option<AgentValue> {
    new_value.check_diff_update(task)
}

pub trait CheckDiffUpdate {
    fn check_diff_update(self, task: &mut ScheduledTask) -> Option<AgentValue>;
}

impl CheckDiffUpdate for u64 {
    fn check_diff_update(self, task: &mut ScheduledTask) -> Option<AgentValue> {
        match (&task.last_value, &task.limit) {
            //: Have a thresshold => compare
            (Some(AgentValue::Int(last)), Some(AgentValue::Int(limit))) => {
                if self.abs_diff(*last) >= *limit {
                    task.last_value = Some(AgentValue::Int(self));
                    Some(AgentValue::Int(self))
                } else {
                    None
                }
            },
            //: Don't have a thresshold => update if changed
            (Some(AgentValue::Int(last)), None) => {
                if self != *last {
                    task.last_value = Some(AgentValue::Int(self));
                    Some(AgentValue::Int(self))
                } else {
                    None
                }
            },
            //: Init or else => fill
            _ => {
                task.last_value = Some(AgentValue::Int(self));
                Some(AgentValue::Int(self))
            },
        }
    }
}

impl CheckDiffUpdate for f64 {
    fn check_diff_update(self, task: &mut ScheduledTask) -> Option<AgentValue> {
        match (&task.last_value, &task.limit) {
            //: Have a thresshold => compare
            (Some(AgentValue::Float(last)), Some(AgentValue::Float(limit))) => {
                if (self - last).abs() >= *limit {
                    task.last_value = Some(AgentValue::Float(self));
                    Some(AgentValue::Float(self))
                } else {
                    None
                }
            },
            //: Don't have a thresshold => update if changed
            (Some(AgentValue::Float(last)), None) => {
                if self != *last {
                    task.last_value = Some(AgentValue::Float(self));
                    Some(AgentValue::Float(self))
                } else {
                    None
                }
            },
            //: Init or else => fill
            _ => {
                task.last_value = Some(AgentValue::Float(self));
                Some(AgentValue::Float(self))
            },
        }
    }
}

impl CheckDiffUpdate for String {
    fn check_diff_update(self, task: &mut ScheduledTask) -> Option<AgentValue> {
        match &task.last_value {
            Some(AgentValue::Text(last)) => {
                if self != *last {
                    task.last_value = Some(AgentValue::Text(self.clone()));
                    Some(AgentValue::Text(self))
                } else {
                    None
                }
            },
            _ => {
                task.last_value = Some(AgentValue::Text(self.clone()));
                Some(AgentValue::Text(self))
            },
        }
    }
}
