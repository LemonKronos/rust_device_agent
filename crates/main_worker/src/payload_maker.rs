//!
//! # Make payload by calling info_gatherer, serialize to json
//! 
//! Contain the logic to process due task batch, compare with cache for valid entry to put in the payload object.
//! 
//! The JSON is not written out, but serialize from an `InfoPayload` object, with have been implemented with derive `serde:Serialize`, and `serde_with::skip_serializing_none` (to not include `None`(null) entry in the final json).
//! 
//! **TODO: Could we use the same type for the real info and payload?**
//! 

use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value as Json};
use rustc_hash::FxHashMap;

use crate::info_gatherer::Info;
use crate::scheduler::{ScheduledTask, TaskID, TaskID::*, AgentValue};

mod serialize_type;
use serialize_type::*;/// Use [`InfoPayload`] to serialize with

use super::SCAN_SOFTWARE;
use super::AGENT_VERSION;

#[cfg(debug_assertions)]
use super::{CUSTOM_SERIAL,USE_CUSTOM_SERIAL};

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

    /// Use to save json as markdown in disk, usefull in dev
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

    /// The main purpose of payload maker, take ref of due tasks to output the JSON
    /// 
    /// **This is still in dev and need to be optimize**
    /// 
    /// Logic:
    /// 1. First convert the batch to `FxHashMap`, with is a lightweight HashMap for enum. 
    /// 2. Go through each topic, check if the batch map have it.
    /// 3. Check if it have valid different, if so, put to the payload.
    /// 4. Add compulsory field like machine serial, agent version, ...
    /// 5. Return payload object.
    /// 
    pub fn process_batch(&mut self, batch: &Vec<ScheduledTask>, full_scan: bool) -> Json {
        let task_map: FxHashMap<TaskID, ScheduledTask> = batch
            .iter()
            .map(|task| (task.id.clone(), task.clone()))
            .collect();

        if full_scan {
            self.last_info_payload = InfoPayload::default();
        }

        //: Refresh
        self.info.prepare();
        let mut info_payload = InfoPayload::default();

        /// Handle standalone scalar fields
        macro_rules! check_scalar {
            ($task_id:ident, $group:ident, $group_type:ident, $field:ident, $getter:expr) => {
                let should_run = full_scan || task_map.contains_key(&$task_id);
                if should_run {
                    let limit = if full_scan { None } else { task_map.get(&$task_id).and_then(|t| t.limit.clone()) };
                    let last = self.last_info_payload.$group.get_or_insert_with($group_type::default);
                    if let Some(updated) = check_diff_update($getter, &mut last.$field, &limit) {
                        let current = info_payload.$group.get_or_insert_with($group_type::default);
                        current.$field = Some(updated);
                    }
                }
            };
        }

        //: General
        check_scalar!(GeneralHost, general, GeneralPayload, host, self.info.get_host());
        check_scalar!(GeneralBootTime, general, GeneralPayload, boot_time, self.info.get_boot_time());
        check_scalar!(GeneralRunTime, general, GeneralPayload, run_time, self.info.get_up_time());

        //: Machine
        check_scalar!(MachineArchitecture, machine, MachinePayload, architecture, self.info.get_architecture());
        check_scalar!(MachineModel, machine, MachinePayload, model, self.info.get_system_model());
        check_scalar!(MachineProducer, machine, MachinePayload, producer, self.info.get_producer());
        check_scalar!(MachineType, machine, MachinePayload, r#type, self.info.get_machine_type());

        //: Motherboard
        check_scalar!(MotherboardName, motherboard, MotherboardPayload, name, self.info.get_motherboard());
        check_scalar!(MotherboardSerial, motherboard, MotherboardPayload, serial, self.info.get_motherboard_serial());
        check_scalar!(MotherboardTempe, motherboard, MotherboardPayload, tempe, self.info.get_tempe_mobo());
        check_scalar!(MotherboardCpuSlot, motherboard, MotherboardPayload, cpu_slot, self.info.get_cpu_slot());
        check_scalar!(MotherboardGpuSlot, motherboard, MotherboardPayload, gpu_slot, self.info.get_gpu_slot());
        check_scalar!(MotherboardRamSlot, motherboard, MotherboardPayload, ram_slot, self.info.get_ram_slot());

        //: Bios
        check_scalar!(BiosVersion, bios, BiosPayload, version, self.info.get_bios_version());
        check_scalar!(BiosVendor, bios, BiosPayload, vendor, self.info.get_bios_vendor());
        check_scalar!(BiosSecureBoot, bios, BiosPayload, secure_boot, self.info.get_is_secure_boot());

        //: OS
        check_scalar!(OsDistro, os, OsPayload, distro, self.info.get_os_distro());
        check_scalar!(OsName, os, OsPayload, name, self.info.get_os_name());
        check_scalar!(OsKernel, os, OsPayload, kernel, self.info.get_kernel());
        check_scalar!(OsVersion, os, OsPayload, version, self.info.get_os_version());

        //: Cpu
        check_scalar!(CpuName, cpu, CpuPayload, name, self.info.get_cpu_name());
        check_scalar!(CpuCore, cpu, CpuPayload, core, self.info.get_cpu_core());
        check_scalar!(CpuUsage, cpu, CpuPayload, usage, self.info.get_cpu_usage());
        check_scalar!(CpuFreq, cpu, CpuPayload, frequency, self.info.get_cpu_freq());
        check_scalar!(CpuTempe, cpu, CpuPayload, temperature, self.info.get_cpu_tempe());

        //: Swap
        check_scalar!(SwapTotal, swap, SwapPayload, total, self.info.get_swap_total());
        check_scalar!(SwapUsage, swap, SwapPayload, usage, self.info.get_swap_usage());

        //: Ram (Total)
        check_scalar!(RamTotal, ram, RamPayload, total, self.info.get_ram_total());
        check_scalar!(RamUsage, ram, RamPayload, usage, self.info.get_ram_usage());

        //: Battery 
        check_scalar!(BatteryPercentage, battery, BatteryPayload, percentage, self.info.get_battery_percentage());
        check_scalar!(BatteryIsPluggedIn, battery, BatteryPayload, power_plugged, self.info.get_battery_is_plugged_in());


        /// Handle Lists inside Groups (e.g. Disks, Ram)
        macro_rules! check_nested_array {
            (
                list: $list:expr,
                group: $group:ident,
                group_type: $group_type:ident,
                list_field: $list_field:ident,
                item_type: $item_type:ident,
                item_ident: $item:ident,
                identity: $id_field:ident = $id_expr:expr,
                fields: [ $( ($task_id:ident, $field:ident, $getter_expr:expr) ),* $(,)? ]
                $(, extra: ($last_ident:ident, $diff_ident:ident, $has_diff_ident:ident) => $extra_block:block )?
            ) => {
                if let Some(items) = $list {
                    let last_group = self.last_info_payload.$group.get_or_insert_with($group_type::default);
                    let last_items = last_group.$list_field.get_or_insert_with(Vec::new);

                    let mut num_item_changed = false;
                    if items.len() != last_items.len() {
                        num_item_changed = true;
                        last_items.clear();
                        last_items.resize_with(items.len(), $item_type::default);
                    }

                    let mut diff_payloads = Vec::new();

                    for (i, $item) in items.iter().enumerate() {
                        let last_item = &mut last_items[i];
                        let mut diff = $item_type::default();
                        let mut has_diff = false;

                        $(
                            let should_run = full_scan || num_item_changed || task_map.contains_key(&$task_id);
                            if should_run {
                                let limit = if full_scan { None } else { task_map.get(&$task_id).and_then(|t| t.limit.clone()) };
                                if let Some(updated) = check_diff_update($getter_expr, &mut last_item.$field, &limit) {
                                    diff.$field = Some(updated);
                                    has_diff = true;
                                }
                            }
                        )*

                        // Inject extra block hygenically 
                        $(
                            {
                                let $last_ident = &mut *last_item;
                                let $diff_ident = &mut diff;
                                let $has_diff_ident = &mut has_diff;
                                $extra_block
                            }
                        )?

                        if has_diff {
                            if diff.$id_field.is_none() {
                                diff.$id_field = Some(AgentValue::Text($id_expr));
                            }
                            diff_payloads.push(diff);
                        }
                    }

                    if !diff_payloads.is_empty() {
                        let current_group = info_payload.$group.get_or_insert_with($group_type::default);
                        current_group.$list_field = Some(diff_payloads);
                    }
                }
            };
        }

        /// Handle Flat Lists (e.g. GPUs, Networks)
        macro_rules! check_flat_array {
            (
                list: $list:expr,
                field: $payload_list:ident,
                item_type: $item_type:ident,
                item_ident: $item:ident,
                identity: $id_field:ident = $id_expr:expr,
                no_cache: $no_cache:ident,
                fields: [ $( ($task_id:ident, $field:ident, $getter_expr:expr) ),* $(,)? ]
            ) => {
                if let Some(items) = $list {
                    let last_items = self.last_info_payload.$payload_list.get_or_insert_with(Vec::new);

                    let mut num_item_changed = false;
                    if items.len() != last_items.len() {
                        num_item_changed = true;
                        last_items.clear();
                        last_items.resize_with(items.len(), $item_type::default);
                    }

                    let mut diff_payloads = Vec::new();

                    for (i, $item) in items.iter().enumerate() {
                        let last_item = &mut last_items[i];
                        let mut diff = $item_type::default();
                        let mut has_diff = false || $no_cache;

                        $(
                            let should_run = full_scan || num_item_changed || task_map.contains_key(&$task_id);
                            if should_run {
                                let limit = if full_scan { None } else { task_map.get(&$task_id).and_then(|t| t.limit.clone()) };
                                if let Some(updated) = check_diff_update($getter_expr, &mut last_item.$field, &limit) {
                                    diff.$field = Some(updated);
                                    has_diff = true;
                                }
                            }
                        )*

                        if has_diff {
                            if diff.$id_field.is_none() {
                                diff.$id_field = Some(AgentValue::Text($id_expr));
                            }
                            diff_payloads.push(diff);
                        }
                    }

                    if !diff_payloads.is_empty() {
                        info_payload.$payload_list = Some(diff_payloads);
                    }
                }
            };
        }

        //: Ram Physical
        check_nested_array! {
            list: self.info.get_ram_list(),
            group: ram,
            group_type: RamPayload,
            list_field: physical,
            item_type: RamPhysicalPayload,
            item_ident: r,
            identity: bank = r.get_bank().to_string(),
            fields: [
                (RamPhysicalSerial, serial, r.serial.clone()),
                (RamPhysicalType, r#type, r.type_.clone()),
                (RamPhysicalConfigSpeed, config_speed, r.speed.clone()),
                (RamPhysicalSize, size, r.size),
                (RamPhysicalName, name, r.name.clone()),
                (RamPhysicalFormFactor, form_factor, r.form_factor.clone())
            ]
        }

        //: Logical Disk
        check_nested_array! {
            list: Some(self.info.get_logical_disk_list()),
            group: disk,
            group_type: DiskPayload,
            list_field: logical,
            item_type: LogicalDiskPayload,
            item_ident: d,
            identity: mount_point = d.get_mount_point(),
            fields: [
                (DiskLogicalName, name, d.get_name()),
                (DiskLogicalFileName, file_system, d.get_file_system()),
                (DiskLogicalMountPoint, mount_point, d.get_mount_point()),
                (DiskLogicalRemovable, removable, d.get_removable()), // Native bool!
                (DiskLogicalTotal, total, d.get_total()),
                (DiskLogicalUsed, used, d.get_used())
            ]
        }

        //: Physical Disk (With nested partition block)
        check_nested_array! {
            list: self.info.get_physical_disk_list(),
            group: disk,
            group_type: DiskPayload,
            list_field: physical,
            item_type: PhysicalDiskPayload,
            item_ident: d,
            identity: drive = d.get_drive().to_string(),
            fields: [
                (DiskPhysicalDrive, drive, d.get_drive()),
                (DiskPhysicalFirmware, firmware, d.get_firmware()),
                (DiskPhysicalIndex, index, d.get_index()),
                (DiskPhysicalInterface, interface, d.get_interface()),
                (DiskPhysicalMedia, media, d.get_media()),
                (DiskPhysicalModel, model, d.get_model()),
                (DiskPhysicalSerial, serial, d.get_serial()),
                (DiskPhysicalSize, size, d.get_size()),
                (DiskPhysicalStatus, status, d.get_status()),
                (DiskPhysicalNumPartition, num_part, d.get_partition_number())
            ],
            extra: (last_disk, diff, has_diff) => {
                let should_run = full_scan || task_map.contains_key(&DiskPhysicalPartition);
                if should_run {
                    let limit = if full_scan { None } else { task_map.get(&DiskPhysicalPartition).and_then(|t| t.limit.clone()) };
                    let last_part_vec = last_disk.partition.get_or_insert_with(Vec::new);
                    let mut part_vec = Vec::new();

                    if let Some(parts) = d.get_partition() {
                        if parts.len() != last_part_vec.len() {
                            last_part_vec.clear();
                            last_part_vec.resize_with(parts.len(), PartitionPayload::default);
                        }

                        for (j, part) in parts.iter().enumerate() {
                            let last_part = &mut last_part_vec[j];
                            let mut diff_part = PartitionPayload::default();
                            let mut has_diff_part = false;

                            if let Some(up) = check_diff_update(part.get_name(), &mut last_part.name, &limit) {
                                diff_part.name = Some(up);
                                has_diff_part = true;
                            }
                            if let Some(up) = check_diff_update(part.get_size(), &mut last_part.size, &limit) {
                                diff_part.size = Some(up);
                                has_diff_part = true;
                            }

                            if has_diff_part {
                                part_vec.push(diff_part);
                            }
                        }
                    }

                    if !part_vec.is_empty() {
                        diff.partition = Some(part_vec);
                        *has_diff = true;
                    }
                }
            }
        }

        //: Gpu
        check_flat_array! {
            list: Some(self.info.get_gpu_list()),
            field: gpu,
            item_type: GpuPayload,
            item_ident: g,
            identity: name = g.get_name(),
            no_cache: false,
            fields: [
                (GpuDriver, driver, g.get_driver_version()),
                (GpuFreq, frequency, g.get_freq()),
                (GpuMaxClock, max_clock, g.get_max_clock()),
                (GpuSerial, serial, g.get_serial()),
                (GpuTempe, temperature, g.get_tempe()),
                (GpuUtilization, utilization, g.get_util()),
                (GpuVramTotal, vram_total, g.get_vram_total()),
                (GpuVramUsage, vram_usage, g.get_vram_usage()),
                (GpuName, name, g.get_name())
            ]
        }

        //: Network
        check_flat_array! {
            list: Some(self.info.get_network_list()),
            field: network,
            item_type: NetworkPayload,
            item_ident: n,
            identity: name = n.get_name(),
            no_cache: false,
            fields: [
                (NetworkCard, card, n.get_card()),
                (NetworkConfigSpeed, config_speed, n.get_config_speed()),
                (NetworkDownload, download, n.get_download()),
                (NetworkIpv4, ipv4, n.get_ipv4()),
                (NetworkIpv6, ipv6, n.get_ipv6()),
                (NetworkMac, mac, n.get_mac()),
                (NetworkMtu, mtu, n.get_mtu()),
                (NetworkName, name, n.get_name()),
                (NetworkSsid, ssid, n.get_ssid()),
                (NetworkUpload, upload, n.get_upload())
            ]
        }

        //: Top Process
        check_flat_array! {
            list: Some(self.info.get_top_process_list()),
            field: top_process,
            item_type: ProcessPayload,
            item_ident: p,
            identity: name = p.get_name(),
            no_cache: true,
            fields: [
                (TopProcessCpu, cpu, p.get_cpu_usage()),
                (TopProcessMemory, memory, p.get_memory()),
                (TopProcessRuntime, run_time, p.get_runtime()),
                (TopProcessName, name, p.get_name())
            ]
        }

        //: Standalone Miscellaneous Checks
        let run_process_count = full_scan || task_map.contains_key(&ProcessCount);
        if run_process_count {
            let limit = if full_scan { None } else { task_map.get(&ProcessCount).and_then(|t| t.limit.clone()) };
            info_payload.process_count = check_diff_update(
                self.info.get_process_count(), 
                &mut self.last_info_payload.process_count, 
                &limit
            );
        }

        if SCAN_SOFTWARE {
            let run_software = full_scan || task_map.contains_key(&Software);
            if run_software {
                info_payload.software = self.info.get_software_list();
            }
        }

        //: Make sure it has ID: Product Serial -> Motherboard Serial -> MAC Addresses -> Fallback
        let mut has_id = false;

        #[cfg(debug_assertions)]
        if USE_CUSTOM_SERIAL {
            let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
            machine.serial = Some(AgentValue::Text(CUSTOM_SERIAL.to_string()));
            has_id = true;
        }

        if !has_id && let Some(serial) = self.info.get_product_serial() {
            let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
            machine.serial = Some(AgentValue::Text(serial.to_string()));
            has_id = true;
        } else if !has_id && let Some(mobo_serial) = self.info.get_motherboard_serial() {
            let mobo = info_payload.motherboard.get_or_insert_with(MotherboardPayload::default);
            mobo.serial.get_or_insert(AgentValue::Text(mobo_serial.to_string()));
            has_id = true;
        } else if !has_id {
            let networks = self.info.get_network_list();
            for net in networks {
                let mac = net.get_mac();
                if !mac.is_empty() {
                    let net_payloads = info_payload.network.get_or_insert_with(Vec::new);
                    let name_val = Some(AgentValue::Text(net.get_name()));
                    
                    if let Some(existing) = net_payloads.iter_mut().find(|p| p.name == name_val) {
                        if existing.mac.is_none() {
                            existing.mac = Some(AgentValue::Text(mac));
                        }
                    } else {
                        let mut diff = NetworkPayload::default();
                        diff.name = name_val;
                        diff.mac = Some(AgentValue::Text(mac));
                        net_payloads.push(diff);
                    }
                    has_id = true;
                }
            }
        }

        if !has_id {
            let machine = info_payload.machine.get_or_insert_with(MachinePayload::default);
            machine.serial = Some(AgentValue::Text("UnknownMachine<Todo_Hash>".to_string()));
        }


        //: Finalize json
        // let payload_json = json!({
        //     "AGENT_VERSION": AGENT_VERSION,
        //     "TIME_STAMP": self.info.get_timestamp(),
        //     "IN_TEST": true,
        //     "INFO": info_json,
        // });

        let mut payload_json = match serde_json::to_value(info_payload) {
            Ok(json) => json,
            Err(e) => serde_json::to_value(e.to_string()).expect("Cannot error here"), //TODO fix code smell
        };

        if let Some(map) = payload_json.as_object_mut() {
            map.insert("AGENT_VERSION".into(), json!(AGENT_VERSION));
            map.insert("TIME_STAMP".into(), json!(self.info.get_timestamp()));
            map.insert("IN_TEST".into(), json!(true));
        }

        //: Save payload debug
        if cfg!(debug_assertions) {
            self.store_payload_md(&payload_json, "just_send");

            if let Ok(last_info) = serde_json::to_value(&self.last_info_payload) {
                self.store_payload_md(&last_info, "last_info");
            } else {
                log::debug!("Cannot save last info!");
            }
            log::debug!("Save payload debug");
        }

        payload_json
    }
}

/// Helper to compare new scan value with the old cached value to see if it is valid to add to the payload with in thresshold limit.
fn check_diff_update<T: CheckDiffUpdate>(new_value: T, old_value: &mut Option<AgentValue>, limit: &Option<AgentValue>) -> Option<AgentValue> {
    new_value.check_diff_update(old_value, limit)
}

//: Generic trait
pub trait CheckDiffUpdate {
    fn check_diff_update(self, old_value: &mut Option<AgentValue>, limit: &Option<AgentValue>) -> Option<AgentValue>;
}

impl<T: CheckDiffUpdate> CheckDiffUpdate for Option<T> {
    fn check_diff_update(self, old_value: &mut Option<AgentValue>, limit: &Option<AgentValue>) -> Option<AgentValue> {
        match self {
            Some(val) => val.check_diff_update(old_value, limit),
            None => None,
        }
    }
}

impl CheckDiffUpdate for bool {
    fn check_diff_update(self, old_value: &mut Option<AgentValue>, _limit: &Option<AgentValue>) -> Option<AgentValue> {
        match old_value.as_ref() {
            Some(AgentValue::Bool(old)) => {
                if self != *old {
                    *old_value = Some(AgentValue::Bool(self));
                    Some(AgentValue::Bool(self))
                } else {
                    None
                }
            },
            _ => {
                *old_value = Some(AgentValue::Bool(self));
                Some(AgentValue::Bool(self))
            },
        }
    }
}

impl CheckDiffUpdate for u64 {
    fn check_diff_update(self, old_value: &mut Option<AgentValue>, limit: &Option<AgentValue>) -> Option<AgentValue> {
        match (old_value.as_ref(), limit) {
            (Some(AgentValue::Int(old)), Some(AgentValue::Int(lim))) => {
                if self.abs_diff(*old) >= *lim {
                    *old_value = Some(AgentValue::Int(self));
                    Some(AgentValue::Int(self))
                } else {
                    None
                }
            },
            (Some(AgentValue::Int(old)), None) => {
                if self != *old {
                    *old_value = Some(AgentValue::Int(self));
                    Some(AgentValue::Int(self))
                } else {
                    None
                }
            },
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
            (Some(AgentValue::Float(old)), Some(AgentValue::Float(lim))) => {
                if (self - old).abs() >= *lim {
                    *old_value = Some(AgentValue::Float(self));
                    Some(AgentValue::Float(self))
                } else {
                    None
                }
            },
            (Some(AgentValue::Float(old)), None) => {
                if self != *old {
                    *old_value = Some(AgentValue::Float(self));
                    Some(AgentValue::Float(self))
                } else {
                    None
                }
            },
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