
use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use sysinfo::System;
use all_smi::AllSmi;
// use std::net::{TcpStream, SocketAddr};
// use std::time::{Duration, Instant};

pub struct Info {
    uuid: String,
    sys: System,
    smi: AllSmi,
    disks: sysinfo::Disks,
    networks: sysinfo::Networks
}

pub struct Gpu {
    gpu: all_smi::device::types::GpuInfo
}

pub struct Disk<'a> {
    disk: &'a sysinfo::Disk,
}

pub struct Process<'a> {
    process: &'a sysinfo::Process,
    cpu_core: f32
}

pub struct Network<'a> {
    name : &'a String,
    data: &'a sysinfo::NetworkData
}

impl Info {
    pub fn new() -> Self {
        Self {
            //TODO check store in cache for this
            // create unique uuid for the deivce
            uuid: {
                let mut builder = IdBuilder::new(Encryption::SHA256);

                builder
                .add_component(HWIDComponent::SystemID)
                .add_component(HWIDComponent::CPUID)
                .add_component(HWIDComponent::DriveSerial);

                match builder.build("super_hyper_secret_key") {
                    Ok(hwid) => hwid,
                    Err(e) => panic!("Error generating UUID: {:?}", e),
                }
            },

            sys: System::new_all(),
            smi: AllSmi::new().expect("Error init AllSmi: "),
            disks: sysinfo::Disks::new_with_refreshed_list(),
            networks: sysinfo::Networks::new_with_refreshed_list()
        }
    }

    //: call this before each check
    pub fn get_ready(&mut self) {
        self.sys.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    //: System info
    pub fn get_os() -> String {
        System::name().unwrap_or_default()
    }

    pub fn get_os_version() -> String {
        System::os_version().unwrap_or_default()
    }

    pub fn get_kernel() -> String {
        System::kernel_version().unwrap_or_default()
    }

    pub fn get_boot_time() -> u64 {
        System::boot_time()
    }

    pub fn get_run_time() -> u64 {
        System::uptime()
    }

    //: CPU info
    //TODO deal with multiple CPU name
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

    ///! HERERERENAKDNAKJNDKAN
    pub fn get_cpu_temp(&self) -> f32 {
        todo!("let os_handler do this computer temperature!")
    }

    //: RAM info
    pub fn get_ram_total(&self) -> u64 {
        self.sys.total_memory()
    }

    pub fn get_ram_usage(&self) -> u64 {
        self.sys.used_memory()
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
    pub fn get_disk_info(&self) -> Vec<Disk<'_>> {
        self.disks.iter()
            .map(|d| Disk::new(d))
            .collect()
    }

    //: Network info
    pub fn get_networks(&self) -> Vec<Network<'_>> {
        self.networks.iter().map(|(name, data)| Network::new(name, data)).collect()
    }

    //: Processes info
    pub fn get_running(&self) -> Vec<Process<'_>> {
        let mut procs: Vec<_> = self.sys.processes().values().collect();
        procs.sort_by(|a, b| b.cpu_usage().total_cmp(&a.cpu_usage()));
        procs.into_iter().take(10).map(|p| Process::new(p, self.get_cpu_core())).collect()
    }
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
        self.disk.total_space() - self.disk.total_space()
    }
}

impl<'a> Process<'a> {
    pub fn new(process: &'a sysinfo::Process, cpu_core: u32) -> Self {
        Self { process, cpu_core: cpu_core as f32}
    }

    pub fn get_name(&self) -> String {
        self.process.name().to_string_lossy().into_owned()
    }

    pub fn get_cpu_usage(&self) -> f32 {
        self.process.cpu_usage() / self.cpu_core
    }

    pub fn get_memory(&self) -> u64 {
        self.process.memory()
    }

    pub fn get_runtime(&self) -> u64 {
        self.process.run_time()
    }
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

