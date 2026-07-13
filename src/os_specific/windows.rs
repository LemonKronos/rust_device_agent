
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Output};
use std::result::Result::Ok;
use std::io::{ErrorKind::PermissionDenied,BufRead, BufReader};
use sysinfo::Components;
use smbioslib::{SMBiosMemoryDevice,SMBiosPhysicalMemoryArray, SMBiosProcessorInformation, table_load_from_device};

use super::interface::OsSpecificInterface;
use super::parse_chassis_type;
use crate::types::*;

#[derive(Debug, Default)]
pub struct OsSpecificBackend {
    components: Components,

    product_serial: Option<String>,
    architecture: Option<String>,
    producer: Option<String>,
    system_model: Option<String>,
    machine_type: Option<String>,
    bios_vendor: Option<String>,
    bios_version: Option<String>,
    is_secure_boot: Option<bool>,

    os_name: Option<String>,

    mobo_name: Option<String>,
    mobo_serial: Option<String>,
    cpu_socket: Option<u32>,
    ram_socket: Option<u32>,
    gpu_socket: Option<u32>,

    ram_list: Option<Ram>,
    physical_disk_list: Option<PhysicalDisk>,

}

impl OsSpecificBackend {
    pub fn new() -> Self {
        Self {
            components: Components::new_with_refreshed_list(),
            ..Default::default()
        }
    }
}

impl OsSpecificInterface for OsSpecificBackend {
    fn refresh(&mut self) {
        self.components.refresh(true);
    }

    fn get_product_serial(&self) -> String {
        todo!()
    }

    fn get_architecture(&self) -> String {
        todo!()
    }

    fn get_producer(&self) -> String {
        todo!()
    }

    fn get_system_model(&self) -> String {
        todo!()
    }

    fn get_machine_type(&self) -> String {
        todo!()
    }

    fn get_bios_version(&self) -> String {
        todo!()
    }

    fn get_bios_vendor(&self) -> String {
        todo!()
    }

    fn get_is_secure_boot(&self) -> Option<bool> {
        todo!()
    }

    fn get_os_name(&self) -> String {
        todo!()
    }

    fn get_motherboard(&self) -> String {
        todo!()
    }

    fn get_motherboard_serial(&self) -> String {
        todo!()
    }

    fn get_cpu_socket(&self) -> Option<u32> {
        todo!()
    }

    fn get_ram_socket(&self) -> Option<u32> {
        todo!()
    }

    fn get_gpu_socket(&self) -> Option<u32> {
        todo!()
    }

    fn get_ram_list(&self) -> Option<Vec<Ram>> {
        todo!()
    }

    fn get_physical_disk_list(&self) -> Option<Vec<PhysicalDisk>> {
        todo!()
    }

    fn get_tempe_mobo(&self) -> f32 {
        todo!()
    }

    fn get_tempe_cpu(&self) -> f32 {
        todo!()
    }

    fn get_battery_percentage(&self) -> f32 {
        todo!()
    }

    fn get_is_plugged_in(&self) -> bool {
        todo!()
    }

    fn fill_network_hardware(&self, network_list: &mut Vec<Network<'_>>) {
        todo!()
    }

    fn get_software_list(&self) -> Option<Vec<Software>> {
        todo!()
    }

}
