///
/// Timer wheel 
/// 

use std::cmp::Ordering;
use tokio::time::{Instant, sleep_until};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BinaryHeap;
use std::fmt;
use std::time::{Duration, SystemTime};

use crate::utils::FormatTime;

/// AgentValue as dynamic type for last_value and limit, which could be either u64, f64 or String
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AgentValue {
    Text(String),
    Int(u64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskID {
    //: General
    #[serde(rename = "general.host")]
    GeneralHost,
    #[serde(rename = "general.boot_time")]
    GeneralBootTime,
    #[serde(rename = "general.run_time")]
    GeneralRunTime,

    //: Machine
    #[serde(rename = "machine.architecture")]
    MachineArchitecture,
    #[serde(rename = "machine.producer")]
    MachineProducer,
    #[serde(rename = "machine.model")]
    MachineModel,
    #[serde(rename = "machine.type")]
    MachineType,

    //: Motherboard
    #[serde(rename = "motherboard.name")]
    MotherboardName,
    #[serde(rename = "motherboard.serial")]
    MotherboardSerial,
    #[serde(rename = "motherboard.tempe")]
    MotherboardTempe,
    #[serde(rename = "motherboard.cpu_slot")]
    MotherboardCpuSlot,
    #[serde(rename = "motherboard.ram_slot")]
    MotherboardRamSlot,
    #[serde(rename = "motherboard.gpu_slot")]
    MotherboardGpuSlot,

    //: BIOS
    #[serde(rename = "bios.version")]
    BiosVersion,
    #[serde(rename = "bios.vendor")]
    BiosVendor,
    #[serde(rename = "bios.sercure_boot")]
    BiosSecureBoot,

    //: OS
    #[serde(rename = "os.distro")]
    OsDistro,
    #[serde(rename = "os.name")]
    OsName,
    #[serde(rename = "os.version")]
    OsVersion,
    #[serde(rename = "os.kernel")]
    OsKernel,

    //: CPU
    #[serde(rename = "cpu.name")]
    CpuName,
    #[serde(rename = "cpu.core")]
    CpuCore,
    #[serde(rename = "cpu.usage")]
    CpuUsage,
    #[serde(rename = "cpu.frequency")]
    CpuFreq,
    #[serde(rename = "cpu.temperature")]
    CpuTempe,

    //: RAM
    #[serde(rename = "ram.total")]
    RamTotal,
    #[serde(rename = "ram.usage")]
    RamUsage,
    #[serde(rename = "ram.physical.name")]
    RamPhysicalName,
    #[serde(rename = "ram.physical.serial")]
    RamPhysicalSerial,
    #[serde(rename = "ram.physical.type")]
    RamPhysicalType,
    #[serde(rename = "ram.physical.config_speed")]
    RamPhysicalConfigSpeed,
    #[serde(rename = "ram.physical.size")]
    RamPhysicalSize,
    #[serde(rename = "ram.physical.bank")]
    RamPhysicalBank,
    #[serde(rename = "ram.physical.form_factor")]
    RamPhysicalFormFactor,

    //: Swap
    #[serde(rename = "swap.total")]
    SwapTotal,
    #[serde(rename = "swap.usage")]
    SwapUsage,

    //: GPU
    #[serde(rename = "gpu.name")]
    GpuName,
    #[serde(rename = "gpu.driver")]
    GpuDriver,
    #[serde(rename = "gpu.utilization")]
    GpuUtilization,
    #[serde(rename = "gpu.temperature")]
    GpuTempe,
    #[serde(rename = "gpu.frequency")]
    GpuFreq,
    #[serde(rename = "gpu.vram_total")]
    GpuVramTotal,
    #[serde(rename = "gpu.vram_usage")]
    GpuVramUsage,
    #[serde(rename = "gpu.max_clock")]
    GpuMaxClock,
    #[serde(rename = "gpu.serial")]
    GpuSerial,

    //: Disk Logical
    #[serde(rename = "disk.logical.name")]
    DiskLogicalName,
    #[serde(rename = "disk.logical.file_name")]
    DiskLogicalFileName,
    #[serde(rename = "disk.logical.mount_point")]
    DiskLogicalMountPoint,
    #[serde(rename = "disk.logical.removable")]
    DiskLogicalRemovable,
    #[serde(rename = "disk.logical.total")]
    DiskLogicalTotal,
    #[serde(rename = "disk.logical.used")]
    DiskLogicalUsed,

    //: Disk Physical
    #[serde(rename = "disk.physical.drive")]
    DiskPhysicalDrive,
    #[serde(rename = "disk.physical.index")]
    DiskPhysicalIndex,
    #[serde(rename = "disk.physical.serial")]
    DiskPhysicalSerial,
    #[serde(rename = "disk.physical.firmware")]
    DiskPhysicalFirmware,
    #[serde(rename = "disk.physical.size")]
    DiskPhysicalSize,
    #[serde(rename = "disk.physical.status")]
    DiskPhysicalStatus,
    #[serde(rename = "disk.physical.media")]
    DiskPhysicalMedia,
    #[serde(rename = "disk.physical.interface")]
    DiskPhysicalInterface,
    #[serde(rename = "disk.physical.parted")]
    DiskPhysicalParted,
    #[serde(rename = "disk.physical.partition.name")]
    DiskPhysicalPartitionName,
    #[serde(rename = "disk.physical.partition.size")]
    DiskPhysicalPartitionSize,

    //: Network
    #[serde(rename = "network.name")]
    NetworkName,
    #[serde(rename = "network.ipv4")]
    NetworkIpv4,
    #[serde(rename = "network.ipv6")]
    NetworkIpv6,
    #[serde(rename = "network.mac")]
    NetworkMac,
    #[serde(rename = "network.mtu")]
    NetworkMtu,
    #[serde(rename = "network.upload")]
    NetworkUpload,
    #[serde(rename = "network.download")]
    NetworkDownload,
    #[serde(rename = "network.card")]
    NetworkCard,
    #[serde(rename = "network.config_speed")]
    NetworkConfigSpeed,
    #[serde(rename = "network.ssid")]
    NetworkSsid,

    //: Top Processes
    #[serde(rename = "process_count")]
    ProcessCount,
    #[serde(rename = "top_process.name")]
    TopProcessName,
    #[serde(rename = "top_process.cpu")]
    TopProcessCpu,
    #[serde(rename = "top_process.memory")]
    TopProcessMemory,
    #[serde(rename = "top_process.runtime")]
    TopProcessRuntime,

    //: All Processes
    #[serde(rename = "all_process.name")]
    AllProcessName,
    #[serde(rename = "all_process.cpu")]
    AllProcessCpu,
    #[serde(rename = "all_process.memory")]
    AllProcessMemory,
    #[serde(rename = "all_process.runtime")]
    AllProcessRuntime,

    //: Battery
    #[serde(rename = "battery.percentage")]
    BatteryPercentage,
    #[serde(rename = "battery.is_plugged_in")]
    BatteryIsPluggedIn,

    //: Software
    #[serde(rename = "software.name")]
    SoftwareName,
    #[serde(rename = "software.version")]
    SoftwareVersion,
    #[serde(rename = "sofware.source")]
    SoftwareSource,
    #[serde(rename = "software.size")]
    SoftwareSize,
    #[serde(rename = "software.install_date")]
    SoftwareInstallDate,

    // Catch-all
    #[serde(other)]
    Unknown,
}

/// ScheduledTask in Timer Wheel
 #[derive(Debug)]
pub struct ScheduledTask {
    pub id: TaskID,
    pub cycle_time: u64,
    pub execute_at: Instant,
    pub limit: Option<AgentValue>,
    // pub last_value: Option<AgentValue>,
}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        self.execute_at == other.execute_at
    }
}

impl Eq for ScheduledTask {}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        other.execute_at.cmp(&self.execute_at)
    }
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Timer Wheel
#[derive(Debug, Default)]
pub struct TimerWheel {
    pub heap: BinaryHeap<ScheduledTask>,
}

impl TimerWheel {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, task: ScheduledTask) {
        self.heap.push(task);
    }

    pub async fn go_sleep(&self) {
        match self.heap.peek().map(|task| task.execute_at) {
            Some(wake_time) => {
                log::info!("Agent go to sleep for {}", {
                    let secs = (wake_time - Instant::now()).as_secs();
                    if secs < 60 { format!("{secs} seconds") } else { format!("{} minutes", secs / 60) }
                });
                sleep_until(wake_time).await;
            },
            None => {
                log::warn!("Timer Wheel empty! Temporary sleep for 5 min.");
                sleep_until(Instant::now() + Duration::from_mins(5)).await;
            },
        }
    }

    pub fn pop_due_batch(&mut self) -> Vec<ScheduledTask> {
        let mut batch = Vec::new();
        while let Some(head_task) = self.heap.peek() {
            if head_task.execute_at <= Instant::now() {
                if let Some(due_task) = self.heap.pop() {
                    batch.push(due_task);
                }
                else {
                    break;
                }
            }
        }
        batch
    }

    pub fn reschedule_batch(&mut self, batch: Vec<ScheduledTask>) {
        for mut task in batch {
            if task.cycle_time > 0 {
                task.execute_at = Instant::now() + Duration::from_secs(task.cycle_time);
                self.heap.push(task);
            }
        }
    }
}

// RAM to DISK
impl Serialize for TimerWheel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.heap.len()))?;
        
        let now_sys = SystemTime::now();
        let now_inst = Instant::now();

        for task in &self.heap {
            let target_sys_time = if task.execute_at <= now_inst {
                // Task is due right now (or past due)
                now_sys
            } else {
                // Task is in the future. Calculate the duration from now.
                now_sys + (task.execute_at - now_inst)
            };

            let unix_timestamp = target_sys_time.to_time_sec();

            // Serialize as "module.component": [cycle_time, timestamp, limit]
            map.serialize_entry(&task.id, &(task.cycle_time, unix_timestamp, &task.limit))?;
        }
        
        map.end()
    }
}

// DISK to RAM
impl<'de> Deserialize<'de> for TimerWheel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueueVisitor;

        impl<'de> Visitor<'de> for QueueVisitor {
            type Value = TimerWheel;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map of string keys to [u64, u64] arrays")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut tasks = match access.size_hint() {
                    Some(size) => Vec::with_capacity(size),
                    None => Vec::new(),
                };

                let now_sys = SystemTime::now().to_time_sec();
                let now_inst = Instant::now();

                // Stream the JSON map directly into ScheduledTask structs
                while let Some((id, (cycle_time, saved_timestamp, limit))) =
                    access.next_entry::<TaskID, (u64, u64, Option<AgentValue>)>()?
                {
                    if id == TaskID::Unknown {
                        continue; 
                    }

                    let execute_at = if saved_timestamp <= now_sys {
                        // Catch-up check: Timestamp is in the past, fire immediately
                        now_inst
                    } else {
                        // Task is in the future, project it onto tokio's monotonic clock
                        now_inst + Duration::from_secs(saved_timestamp - now_sys)
                    };

                    tasks.push(ScheduledTask {
                        id,
                        cycle_time,
                        execute_at,
                        limit,
                        last_value: None,
                    });
                }

                Ok(TimerWheel {
                    heap: BinaryHeap::from(tasks),
                })
            }
        }

        deserializer.deserialize_map(QueueVisitor)
    }
}

