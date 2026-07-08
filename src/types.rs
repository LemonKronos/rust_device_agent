
//_ RAM
#[derive(Debug, Default)]
pub struct Ram {
    name: String,
    serial: String,
    type_: String,
    speed: String,
    size: String,
}

impl Ram {
    pub fn new(name: String, serial: String, type_: String, speed: String, size: String) -> Self {
        Self { name, serial, type_, speed, size }
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

    pub fn get_speed(&self) -> &str {
        &self.speed
    }

    pub fn get_size(&self) -> &str {
        &self.size
    }
}

//_ GPU
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

    pub fn get_driver(&self) -> String {
        self.gpu.detail.get("Driver Version")
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn get_util(&self) -> f64 {
        self.gpu.utilization
    }

    pub fn get_temp(&self) -> u32 {
        self.gpu.temperature
    }

    pub fn get_vram_total(&self) -> u64 {
        self.gpu.total_memory
    }

    pub fn get_vram_usage(&self) -> u64 {
        self.gpu.used_memory
    }

    pub fn get_freq(&self) -> u32 {
        self.gpu.frequency
    }

    pub fn get_max_clock(&self) -> Option<u32> {
        self.gpu.detail.get("clock_graphics_max")
            .and_then(|s| s.trim().parse().ok())
    }

    pub fn get_serial(&self) -> String {
        self.gpu.detail.get("Serial Number")
        .or_else(|| self.gpu.detail.get("Serial"))
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string())
    }
}

//_ Disk
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

    pub fn get_removable(&self) -> bool {
        self.disk.is_removable()
    }

    pub fn get_mount_point(&self) -> String {
        self.disk.mount_point().to_string_lossy().to_string()
    }

    pub fn get_total(&self) -> u64 {
        self.disk.total_space()
    }

    pub fn get_used(&self) -> u64 {
        self.disk.total_space() - self.disk.available_space()
    }

    pub fn get_model(&self) -> &str {
        todo!()
    }

    pub fn get_serial(&self) -> &str {
        todo!()
    }

    pub fn get_firmware(&self) -> &str {
        todo!()
    }

    pub fn get_size(&self) -> u64 {
        todo!()
    }
}

#[derive(Debug)]
pub struct HardwareDisk {
    pub drive: String,
    pub model: String,
    pub serial: String,
    pub firmware: String,
    pub size: u32,
}

impl HardwareDisk {
    pub fn new(drive: String, model: String, serial: String, firmware: String, size: u32) -> Self {
        Self { drive, model, serial, firmware, size }
    }

    pub fn get_drive(&self) -> &str {
        &self.drive
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

    pub fn get_size(&self) -> u32 {
        self.size
    }
}

//_ Network
#[derive(Debug)]
pub struct Network<'a> {
    name : &'a String,
    data: &'a sysinfo::NetworkData,
    card: String,
    speed: u32,
    ssid: String,
}

impl<'a> Network<'a> {
    pub fn new(name: &'a String, data: &'a sysinfo::NetworkData) -> Self {
        Self { name, data, 
            card: "Unknown".to_string(),
            speed: 0,
            ssid: "Unknown".to_string(),
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

    pub fn get_mtu(&self) -> u64 {
        self.data.mtu()
    }

    pub fn get_upload(&self) -> u64 {
        self.data.total_transmitted()
    }

    pub fn get_download(&self) -> u64 {
        self.data.total_received()
    }

    pub fn set_card(&mut self, card: String) {
        self.card = card
    }

    pub fn set_config_speed(&mut self, speed: u32) {
        self.speed = speed
    }

    pub fn set_ssid(&mut self, ssid: String) {
        self.ssid = ssid
    }
    
    pub fn get_card(&self) -> String {
        self.card.clone()
    }

    pub fn get_config_speed(&self) -> u32 {
        self.speed
    }

    pub fn get_ssid(&self) -> String {
        self.ssid.clone()
    }
}

//_ Process
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

    pub fn get_cpu_usage(&self) -> f32 {
        self.process.cpu_usage()
    }

    pub fn get_memory(&self) -> u64 {
        self.process.memory()
    }

    pub fn get_runtime(&self) -> u64 {
        self.process.run_time()
    }
}

//_ Software
#[derive(Debug, Default)]
pub struct Software {
    pub name: String,
    pub version: String,
    pub source: String,

    //TODO
    pub license: String,
    pub expiration: String,
}

impl Software {
    pub fn new(name: String, version: String, source: String, license: String, expiration: String) -> Self {
        Self { name, version, source, license, expiration }
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

    pub fn get_license(&self) -> &str {
        &self.license
    }

    pub fn get_expiration(&self) -> &str {
        &self.expiration
    }
}

