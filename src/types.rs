///
/// Define custom types here
/// 

use std::time::SystemTime;

/// RAM
#[derive(Debug, Default)]
pub struct Ram {
    pub name: String,
    pub serial: String,
    pub type_: String,
    pub speed: String,
    pub size: u64,
    pub bank: String,
    pub form_factor: String,
}

impl Ram {
    pub fn new(
        name: String,
        serial: String,
        type_: String,
        speed: String,
        size: u64, 
        bank: String,
        form_factor: String,
    ) -> Self {
        Self { name, serial, type_, speed, size, bank, form_factor }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_serial(&self) -> &str {
        &self.serial
    }

    pub fn get_type(&self) -> &str {
        &self.type_
    }

    /// return RAM configured speed in MT/s
    pub fn get_speed(&self) -> &str {
        &self.speed
    }

    /// Return RAM sale size in Gigabyte
    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_bank(&self) -> &str {
        &self.bank
    }

    pub fn get_form_factor(&self) -> &str {
        &self.form_factor
    }
}

/// GPU
#[derive(Debug)]
pub struct Gpu {
    gpu: all_smi::device::types::GpuInfo
}

impl Gpu {
    pub fn new(gpu: all_smi::device::types::GpuInfo) -> Self {
        Self {
            gpu: gpu
        }
    }

    pub fn get_name(&self) -> String {
        self.gpu.name.to_string()
    }

    pub fn get_driver_version(&self) -> String {
        self.gpu.detail.get("Driver Version")
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string())
    }

    /// Return GPU usage in %
    pub fn get_util(&self) -> f64 {
        self.gpu.utilization
    }

    /// Return GPU temperature in ℃
    pub fn get_tempe(&self) -> u32 {
        self.gpu.temperature
    }

    /// Return GPU total VRAM in Byte
    pub fn get_vram_total(&self) -> u64 {
        self.gpu.total_memory
    }

    pub fn get_vram_usage(&self) -> u64 {
        self.gpu.used_memory
    }

    /// Return GPU current frequency in MHz
    pub fn get_freq(&self) -> u32 {
        self.gpu.frequency
    }

    /// Return GPU maximum frequency in MHz
    pub fn get_max_clock(&self) -> Option<u32> {
        self.gpu.detail.get("clock_graphics_max")
            .and_then(|s| s.trim().parse().ok())
    }

    /// Try to return GPU serial number, however that info usually is hidden behind proprietary firmware
    pub fn get_serial(&self) -> String {
        self.gpu.detail.get("Serial Number")
        .or_else(|| self.gpu.detail.get("Serial"))
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string())
    }
}

/// Disk
#[derive(Debug)]
pub struct LogicalDisk<'a> {
    disk: &'a sysinfo::Disk,
}

impl<'a> LogicalDisk<'a> {
    pub fn new(disk: &'a sysinfo::Disk) -> Self {
        Self { disk }
    }
    
    pub fn get_name(&self) -> String {
        self.disk.name().to_string_lossy().into_owned()
    }
    
    pub fn get_file_system(&self) -> String {
        self.disk.file_system().to_string_lossy().into_owned()
    }

    /// Return in bool, true if the disk is removable
    pub fn get_removable(&self) -> bool {
        self.disk.is_removable()
    }

    pub fn get_mount_point(&self) -> String {
        self.disk.mount_point().to_string_lossy().to_string()
    }

    /// Return total disk space in Byte
    pub fn get_total(&self) -> u64 {
        self.disk.total_space()
    }

    /// Return in used disk space in Byte
    pub fn get_used(&self) -> u64 {
        self.disk.total_space() - self.disk.available_space()
    }
}

#[derive(Debug)]
pub struct PhysicalDisk {
    pub drive: String,
    pub index: u32,
    pub model: String,
    pub serial: String,
    pub firmware: String,
    pub size: u64,
    pub media: String,
    pub interface: String,
    pub status: String,
    pub partition: Option<Vec<Partition>>,

}

impl PhysicalDisk {
    pub fn new(
        drive: String,
        index: u32,
        model: String,
        serial: String,
        firmware: String,
        size: u64,
        media: String,
        interface: String,
        status: String
    ) -> Self {
        Self { drive, index, model, serial, firmware, size, media, interface, status, partition: None }
    }

    pub fn get_drive(&self) -> &str {
        &self.drive
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn get_serial(&self) -> &str {
        &self.serial
    }

    pub fn get_firmware(&self) -> &str {
        &self.firmware
    }

    /// Return physical disk sale size in Gigabyte
    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn get_media(&self) -> &str {
        &self.media
    }

    pub fn get_interface(&self) -> &str {
        &self.interface
    }

    pub fn get_partition_number(&self) -> u32 {
        self.partition.iter().count() as u32
    }

    pub fn get_partition(&self) -> Option<&Vec<Partition>> {
        self.partition.as_ref()
    }
}

#[derive(Debug)]
pub struct Partition {
    pub name: String,
    pub size: f64,
}

impl Partition {
    //TODO return name instead of just a number
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Return physical disk partition size in Gigabyte
    pub fn get_size(&self) -> f64 {
        self.size
    }
}

/// Network
#[derive(Debug)]
pub struct Network<'a> {
    name : &'a String,
    data: &'a sysinfo::NetworkData,
    pub ssid: Option<String>,
    pub hardware: Option<NetworkHardware>,
}

impl<'a> Network<'a> {
    pub fn new(name: &'a String, data: &'a sysinfo::NetworkData) -> Self {
        Self { name, data,
            ssid: None,
            hardware: None,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_ipv4(&self) -> String {
        let addrs: Vec<String> = self.data.ip_networks()
            .iter()
            .filter(|net| net.addr.is_ipv4())
            .map(|net| format!("{}/{}", net.addr, net.prefix))
            .collect();
            
        if addrs.is_empty() {
            "None".to_string()
        } else {
            addrs.join(", ")
        }
    }

    pub fn get_ipv6(&self) -> String {
        let addrs: Vec<String> = self.data.ip_networks()
            .iter()
            .filter(|net| net.addr.is_ipv6())
            .map(|net| format!("{}/{}", net.addr, net.prefix))
            .collect();
            
        if addrs.is_empty() {
            "None".to_string()
        } else {
            addrs.join(", ")
        }
    }

    pub fn get_mac(&self) -> String {
        self.data.mac_address().to_string()
    }
    
    /// Returns the Maximum Transfer Unit (MTU) of the interface in Byte
    pub fn get_mtu(&self) -> u64 {
        self.data.mtu()
    }

    /// Return total upload since boot in Byte
    pub fn get_upload(&self) -> u64 {
        self.data.total_transmitted()
    }

    /// Return total download since boot in Byte
    pub fn get_download(&self) -> u64 {
        self.data.total_received()
    }
   
    /// Return internet card info
    pub fn get_card(&self) -> Option<String> {
        self.hardware.as_ref().map(|hw| hw.card.clone())
    }

    /// Return Ethernet config speed in Mbps, will be 0 for Wifi
    pub fn get_config_speed(&self) -> Option<u32> {
        self.hardware.as_ref().and_then(|hw| hw.speed)
    }

    /// Return Wifi network name - Service Set Identifier (SSID), will be "Unknown" for Ethernet
    pub fn get_ssid(&self) -> Option<String> {
        self.ssid.clone()
    }
}

#[derive(Debug, Default, Clone)]
pub struct NetworkHardware {
    pub card: String,
    pub speed: Option<u32>,
}

/// Process
#[derive(Debug)]
pub struct Process<'a> {
    process: &'a sysinfo::Process,
}

impl<'a> Process<'a> {
    pub fn new(process: &'a sysinfo::Process) -> Self {
        Self { process }
    }

    pub fn get_name(&self) -> String {
        self.process.name().to_string_lossy().into_owned()
    }

    /// Return process cpu usage
    ///> [!WARNING] could grow pass 100% if process run multi-core
    pub fn get_cpu_usage(&self) -> f32 {
        self.process.cpu_usage()
    }

    /// Return memory usage of the process in Byte
    pub fn get_memory(&self) -> u64 {
        self.process.memory()
    }

    /// Return the runtime of the process in second
    pub fn get_runtime(&self) -> u64 {
        self.process.run_time()
    }
}

/// Software
//TODO deal with the jungle of Linux software
#[derive(Debug)]
pub struct Software {
    pub name: String,
    pub version: String,
    pub source: String,
    pub size: f64,
    pub install_date: Option<SystemTime>,
}

impl Software {
    pub fn new() -> Self {
        Self {
            name: "Unknown".to_string(),
            version: "Unknown".to_string(),
            source: "Unknown".to_string(),
            size: 0.0,
            install_date: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    /// Return package size in Megabyte
    pub fn get_size(&self) -> f64 {
        self.size
    }

    /// Return install date in SystemTime
    pub fn get_install_date(&self) -> Option<SystemTime> {
        self.install_date
    }
}
