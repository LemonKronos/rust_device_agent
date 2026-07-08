
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::System;
use all_smi::AllSmi;

use crate::types::*;
use crate::os_specific::OsSpecific;
use crate::os_specific::interface::OsSpecificInterface;

//TODO impl trait Debug for this
pub struct Info {
    sys: System,
    smi: AllSmi,
    disks: sysinfo::Disks,
    networks: sysinfo::Networks,
    os_specific: OsSpecific,
}

impl Info {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
            smi: AllSmi::new().expect("Error init AllSmi: "),
            disks: sysinfo::Disks::new_with_refreshed_list(),
            networks: sysinfo::Networks::new_with_refreshed_list(),
            os_specific: OsSpecific::new(),
        }
    }

    //: call this before each check
    pub fn prepare(&mut self) {
        self.sys.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);
        self.os_specific.refresh();
    }

    //: Timestamp
    pub fn get_timestamp(&self) -> u64 {
        SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time broken!")
        .as_secs()
    }

    //: General info
    pub fn get_boot_time(&self) -> u64 {
        System::boot_time()
    }

    pub fn get_run_time(&self) -> u64 {
        System::uptime()
    }

    //: Machine info
    pub fn get_product_serial(&self) -> String {
        self.os_specific.get_product_serial()
    }

    pub fn get_architecture(&self) -> String {
        self.os_specific.get_architecture()
    }

    pub fn get_producer(&self) -> String {
        self.os_specific.get_producer()
    }

    pub fn get_system_model(&self) -> String {
        self.os_specific.get_system_model()
    }

    pub fn get_machine_type(&self) -> String {
        self.os_specific.get_machine_type()
    }

    //: Motherboard info
    pub fn get_motherboard(&self) -> String {
        self.os_specific.get_motherboard()
    }

    pub fn get_motherboard_serial(&self) -> String {
        self.os_specific.get_motherboard_serial()
    }

    pub fn get_temp_mobo(&self) -> f32 {
        self.os_specific.get_tempe_mobo()
    }

    pub fn get_cpu_socket(&self) -> Option<u32> {
        self.os_specific.get_cpu_socket()
    }

    pub fn get_ram_socket(&self) -> Option<u32> {
        self.os_specific.get_ram_socket()
    }

    pub fn get_gpu_socket(&self) -> Option<u32> {
        self.os_specific.get_gpu_socket()
    }

    //: Bios info
    pub fn get_bios_version(&self) -> String {
        self.os_specific.get_bios_version()
    }
    
    pub fn get_bios_vendor(&self) -> String {
        self.os_specific.get_bios_vendor()
    }

    pub fn get_is_secure_boot(&self) -> Option<bool> {
        self.os_specific.get_is_secure_boot()
    }

    //: OS info
    pub fn get_os_name(&self) -> String {
        self.os_specific.get_os_name()
    }

    pub fn get_os(&self) -> String {
        System::name().unwrap_or_default()
    }

    pub fn get_os_version(&self) -> String {
        System::os_version().unwrap_or_default()
    }

    pub fn get_kernel(&self) -> String {
        System::kernel_version().unwrap_or_default()
    }

    //: CPU info
    pub fn get_cpu_name(&self) -> String {
        self.sys.cpus().first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknow CPU".to_string())
    }

    pub fn get_cpu_core(&self) -> u32 {
        self.sys.cpus().len() as u32
    }

    pub fn get_cpu_usage(&self) -> f32 {
        self.sys.global_cpu_usage()
    }

    pub fn get_cpu_freq(&self) -> u64 {
        let total: u64 = self.sys.cpus().iter().map(|core| core.frequency()).sum();
        total / (self.sys.cpus().len() as u64).max(1)
    }

    pub fn get_cpu_temp(&self) -> f32 {
        self.os_specific.get_tempe_cpu()
    }

    //: RAM info
    pub fn get_ram_total(&self) -> u64 {
        self.sys.total_memory()
    }

    pub fn get_ram_usage(&self) -> u64 {
        self.sys.used_memory()
    }

    pub fn get_ram_list(&self) -> Option<Vec<Ram>> {
        self.os_specific.get_ram_list()
    }

    //: SWAP info
    pub fn get_swap_total(&self) -> u64 {
        self.sys.total_swap()
    }

    pub fn get_swap_usage(&self) -> u64 {
        self.sys.used_swap()
    }

    //: GPU list
    pub fn get_gpu_list(&self) -> Vec<Gpu> {
        self.smi.get_gpu_info().into_iter()
            .map(|gpu| Gpu::new(gpu))
            .collect()
    }

    //: Disk list
    pub fn get_logical_disk_list(&self) -> Vec<LogicalDisk<'_>> {
        self.disks.iter()
            .map(|d| LogicalDisk::new(d))
            .collect()
    }

    pub fn get_hardware_disk_list(&self) -> Option<Vec<HardwareDisk>> {
        self.os_specific.get_hardware_disk_list()
    }

    //: Network info
    pub fn get_network_list(&self) -> Vec<Network<'_>> {
        // Handle by sysinfo
        let mut list = self.networks.iter().map(|(name, data)| Network::new(name, data)).collect();
        // Populate hardware info for each OS
        self.os_specific.fill_network_hardware(&mut list);
        return list;
    }

    //: Processes info
    pub fn get_top_process_list(&self) -> Vec<Process<'_>> {
        let mut procs: Vec<_> = self.sys.processes().values().collect();
        procs.sort_by(|a, b| b.cpu_usage().total_cmp(&a.cpu_usage()));
        procs.into_iter().take(10).map(|p| Process::new(p)).collect()
    }

    pub fn get_full_process_list(&self) -> Vec<Process<'_>> {
        let mut procs: Vec<_> = self.sys.processes().values().collect();
        procs.sort_by(|a, b| b.cpu_usage().total_cmp(&a.cpu_usage()));
        procs.into_iter().map(|p| Process::new(p)).collect()
    }

    //: Battery info
    pub fn get_battery_percentage(&self) -> f32 {
        self.os_specific.get_percentage()
    }

    pub fn get_battery_is_plugged_in(&self) -> bool {
        self.os_specific.get_is_plugged_in()
    }

    //: Software info
    pub fn get_software_list(&self) -> Option<Vec<Software>> {
        self.os_specific.get_software_list()
    }
}

