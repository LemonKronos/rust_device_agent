
//_ RAM
#[derive(Debug, Default)]
pub struct Ram {
    
}

impl Ram {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn get_manufacturer(&self) -> &str {
        todo!()
    }

    pub fn get_part_number(&self) -> &str {
        todo!()
    }

    pub fn get_serial(&self) -> &str {
        todo!()
    }

    pub fn get_type(&self) -> &str {
        todo!()
    }

    pub fn get_size(&self) -> &str {
        todo!()
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
}

//_ Disk
#[derive(Debug)]
pub struct Disk<'a> {
    disk: &'a sysinfo::Disk,
}

impl<'a> Disk<'a> {
    pub fn new(disk: &'a sysinfo::Disk) -> Self {
        Self { disk }
    }
    
    pub fn get_name(&self) -> String {
        self.disk.name().to_string_lossy().into_owned()
    }
    
    pub fn get_type(&self) -> String {
        self.disk.kind().to_string()
    }

    pub fn get_removable(&self) -> bool {
        self.disk.is_removable()
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

//_ Network
#[derive(Debug)]
pub struct Network<'a> {
    name : &'a String,
    data: &'a sysinfo::NetworkData
}

impl<'a> Network<'a> {
    pub fn new(name: &'a String, data: &'a sysinfo::NetworkData) -> Self {
        Self { name, data}
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
    // pub license: String,
    // pub expiration_date: String,
}

impl Software {
    pub fn new(name: String, version: String, source: String) -> Self {
        Self { name, version, source }
    }
}

