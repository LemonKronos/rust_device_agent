//!
//! # Scheduler where each scan task is being stored as an element in min-heap
//! 
//! With the lightweight and robust goal in mind, the scheduler is designed to be init directly from the config.
//! "The scheduler is the config, the config is the scheduler".
//! Thus, we flatten the json structure, get the ID, get the scan interval, limit, next execute time to store as config.
//! The scheduler then get **deserialized directly** from the config.
//! When the config change by server, the scheduler get updated and immediately save as config.
//! When the Agent get normally shutdown, the current scheduler is serialize to config.
//! 

use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;
use std::time::{Duration, SystemTime};
use tokio::time::{Instant, sleep_until};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as Json;

use shared_libs::utils::FormatTime;
use  crate::config_handler;

//TODO Maybe this is no longer needed?
/// AgentValue as dynamic type for limit, which could be either bool, u64, f64 or String.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AgentValue {
    Text(String),
    Int(u64),
    Float(f64),
    Bool(bool),
}

/// Flatten from JSON payload to `"<module>.<component>"`.
/// This is all the topic-info type that Agent can get from scanning the machine.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
    #[serde(rename = "machine.serial")]
    MachineSerial,

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
    #[serde(rename = "disk.physical.model")]
    DiskPhysicalModel,
    #[serde(rename = "disk.physical.interface")]
    DiskPhysicalInterface,
    #[serde(rename = "disk.physical.partition_num")]
    DiskPhysicalNumPartition,
    #[serde(rename = "disk.physical.partition")]
    DiskPhysicalPartition,

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

    //: Software, currently only allow scan all
    #[serde(rename = "software")]
    Software,

    //TODO feature
    // #[serde(rename = "software.name")]
    // SoftwareName,
    // #[serde(rename = "software.version")]
    // SoftwareVersion,
    // #[serde(rename = "sofware.source")]
    // SoftwareSource,
    // #[serde(rename = "software.size")]
    // SoftwareSize,
    // #[serde(rename = "software.install_date")]
    // SoftwareInstallDate,

    // Catch-all
    #[serde(other)]
    Unknown,
}

/// ScheduledTask.
/// 
/// All the `PartialEq`, `Eq`, `Ord` and `PartialOrd` are for **min** heap, since [`Scheduler`] using `std::collections::BinaryHeap` - which is a max heap.
 #[derive(Debug, Clone)]
pub struct ScheduledTask {
    /// Unique ID for each task
    pub id: TaskID,

    /// The configured time (in second) between 2 scans
    pub cycle_time: u64,

    /// Thresshold for a valid info change. Only meaningfull for numeric type.
    pub limit: Option<AgentValue>,

    /// When will the task next due
    pub execute_at: Instant,
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

#[derive(Debug, Default)]
pub struct Scheduler {
    /// The active task queue
    pub heap: BinaryHeap<ScheduledTask>,

    /// The list of one-off init task, safely stored aways
    dead_tasks: Vec<ScheduledTask>,

    /// Flag to skip next sleep to work on server command(s)
    skip_sleep: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            dead_tasks: Vec::new(),
            skip_sleep: false,
        }
    }

    pub fn push(&mut self, task: ScheduledTask) {
        self.heap.push(task);
    }

    /// Sleep until next task is due
    pub async fn go_sleep(&mut self) {
        if self.skip_sleep {
            self.skip_sleep = false;
            return;
        }

        let now = Instant::now();
        match self.heap.peek().map(|task| task.execute_at) {
            Some(wake_time) => {
                if wake_time > now {

                    log::info!("Agent go to sleep for {}", {
                        let secs = (wake_time - now).as_secs();
                        if secs < 60 { format!("{secs} seconds") } else { format!("{} minutes", secs / 60) }
                    });
                    sleep_until(wake_time).await;
                } else {
                    log::info!("Task already due, skipping sleep");
                }
            },
            None => {
                if cfg!(debug_assertions) {   
                    log::warn!("Scheduler empty! Temporary sleep for 30 sec.");
                    sleep_until(now + Duration::from_secs(30)).await;
                } else {
                    log::warn!("Scheduler empty! Temporary sleep for 5 min.");
                    sleep_until(now + Duration::from_mins(5)).await;
                }
            },
        }
    }

    /// Skip the next sleep
    pub fn skip_sleep(&mut self) {
        log::info!("Agent skip sleep");
        self.skip_sleep = true;
    }

    /// Make a batch of due or pass due tasks, ready to be processed
    pub fn pop_due_batch(&mut self) -> Vec<ScheduledTask> {
        let mut batch = Vec::new();
        let now = Instant::now();

        while let Some(head_task) = self.heap.peek() {
            if head_task.execute_at <= now {
                if let Some(due_task) = self.heap.pop() {
                    batch.push(due_task);
                }
                else {
                    break;
                }
            } else {
                break;
            }
        }
        batch
    }

    /// Re-added the batch to the queue base on it `cycle_time` and `Instant::now()`.
    /// If the task is an one-off, put it to the `dead_tasks` vector.
    /// If a full scan have just happpen, reschedule all the task (include dead ones) from `Instant::now()`.
    pub fn reschedule_batch(&mut self, batch: Vec<ScheduledTask>, full_scan: bool) {
        let now  = Instant::now();
        if full_scan { // Put all to a new heap
            let mut updated = Vec::new();

            for mut task in batch {
                if task.cycle_time > 0 {
                    task.execute_at = now + Duration::from_secs(task.cycle_time);
                    updated.push(task);
                } else {
                    self.dead_tasks.push(task);
                }
            }

            for mut task in self.heap.drain() {
                if task.cycle_time > 0 {
                    task.execute_at = now + Duration::from_secs(task.cycle_time);
                    updated.push(task);
                } else {
                    self.dead_tasks.push(task);
                }
            }

            for task in &mut self.dead_tasks {
                task.execute_at = now;
            }

            self.heap = BinaryHeap::from(updated);
        }
        else { // Normal reschedule
            for mut task in batch {
                if task.cycle_time > 0 { // cycle task being re-schedule
                    task.execute_at = now + Duration::from_secs(task.cycle_time);
                    // log::info!("task {:?} have been rescheduled", &task.id);
                    self.heap.push(task);
                } else { // one-pass task, wait to be re-added and saved
                    self.dead_tasks.push(task);
                }
            }
        }
    }

    /// Apply the config change directly to the scheduler, and immediately save
    pub fn update(&mut self, config: Json) {
        let mut patch_map: FxHashMap<TaskID, (u64, Option<AgentValue>)> = FxHashMap::default();
        let now = Instant::now();

        // 1. Parse the Server Delta into a fast lookup map
        if let Some(map) = config.as_object() {
            for (key, val_array) in map {
                let id: TaskID = match serde_json::from_value(Json::String(key.clone())) {
                    Ok(parsed) => parsed,
                    Err(_) => continue,
                };

                if id == TaskID::Unknown {
                    continue;
                }

                if let Some(arr) = val_array.as_array() {
                    if arr.len() >= 2 {
                        let cycle_time = arr[0].as_u64().unwrap_or(0);
                        let limit: Option<AgentValue> = serde_json::from_value(arr[1].clone()).unwrap_or(None);
                        patch_map.insert(id, (cycle_time, limit));
                    }
                }
            }
        }

        let mut new_heap = Vec::new();
        let mut new_graveyard = Vec::new();

        // 2. Scan the Active Heap
        for mut task in self.heap.drain() {
            if let Some((new_cycle, new_limit)) = patch_map.remove(&task.id) {
                // If it was ticking, and is still ticking, do the Time Math
                if task.cycle_time > 0 {
                    // Find exactly when it fired last
                    let last_fired = task.execute_at
                        .checked_sub(Duration::from_secs(task.cycle_time)) // prevents crash if it underflows monotonic clock
                        .unwrap_or(now);
                    
                    task.execute_at = last_fired + Duration::from_secs(new_cycle);
                } 
                // If cycle_time was 0, it means it hasn't fired its boot-scan yet, so we don't touch execute_at (it will just fire normally)

                task.cycle_time = new_cycle;
                task.limit = new_limit;
            }
            
            // Regardless of update, anything in the heap currently HAS NOT fired yet, so we keep it in the active heap to fire
            new_heap.push(task);
        }

        // 3. Scan the Graveyard
        for mut task in self.dead_tasks.drain(..) {
            if let Some((new_cycle, new_limit)) = patch_map.remove(&task.id) {
                task.cycle_time = new_cycle;
                task.limit = new_limit;

                if new_cycle > 0 {
                    // Resurrected! It gets a fresh start right now
                    task.execute_at = now;
                    new_heap.push(task);
                } else {
                    // Still a graveyard task, it just had its limit updated
                    // Keep it dead
                    new_graveyard.push(task);
                }
            } else {
                // Unchanged, stays dead
                new_graveyard.push(task);
            }
        }

        // 4. Handle Newcomers
        // Anything left in the patch_map is a brand new TaskID we didn't have before
        for (id, (cycle_time, limit)) in patch_map {
            new_heap.push(ScheduledTask {
                id,
                cycle_time,
                execute_at: now, // Newcomers always fire instantly
                limit,
            });
        }

        // 5. Rebuild Everything
        self.heap = BinaryHeap::from(new_heap);
        self.dead_tasks = new_graveyard;
        log::info!("Scheduler successfully patched with server config");

        self.save();
        log::info!("New updated config saved");
    }

    /// Save current Scheduler as config file
    pub fn save(&mut self) {
        for task in self.dead_tasks.drain(..) {
            self.heap.push(task);
        }

        if !self.heap.is_empty() {
            log::info!("Save Scheduler as config");
            config_handler::save_config(&self);
        } else {
            log::warn!("Scheduler emptry, no config save");
        }
    }
}

/// RAM to DISK: when save to config.json, format "module.component": [cycle_time, limit, next_execute]. Example:
/// ```json
/// {
///     "general.host": [10, null, 1786422356],
///     "general.boot_time": [10, null, 1786422356],
///     "general.run_time": [10, null, 1786422356]
/// }
/// ```
impl Serialize for Scheduler {
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

            map.serialize_entry(&task.id, &(task.cycle_time, &task.limit, unix_timestamp))?;
        }
        
        map.end()
    }
}

/// DISK to RAM: when load config.json to Scheduler
impl<'de> Deserialize<'de> for Scheduler {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueueVisitor;

        impl<'de> Visitor<'de> for QueueVisitor {
            type Value = Scheduler;

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
                while let Some((id, (cycle_time, limit, saved_timestamp))) =
                    access.next_entry::<TaskID, (u64, Option<AgentValue>, u64)>()?
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
                        limit,
                        execute_at,
                    });
                }

                Ok(Scheduler {
                    heap: BinaryHeap::from(tasks),
                    dead_tasks: Vec::new(),
                    skip_sleep: false,
                })
            }
        }

        deserializer.deserialize_map(QueueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TaskID::*;

    #[test]
    fn test_min_heap_ordering() {
        let mut sched = Scheduler::new();
        let now = Instant::now();

        sched.push(ScheduledTask { id: GeneralHost, cycle_time: 0, execute_at: now + Duration::from_secs(3), limit: None });
        sched.push(ScheduledTask { id: GeneralBootTime, cycle_time: 0, execute_at: now + Duration::from_secs(1), limit: None });
        sched.push(ScheduledTask { id: GeneralRunTime, cycle_time: 0, execute_at: now + Duration::from_secs(2), limit: None });

        assert!(!sched.heap.is_empty(), "Heap screams: Completely empty after pushes!");

        let t1 = sched.heap.pop().expect("Heap screams: Expected 1st task, found None!");
        assert_eq!(t1.id, GeneralBootTime, "Min-heap policy failed on 1st pop");

        let t2 = sched.heap.pop().expect("Heap screams: Expected 2nd task, found None!");
        assert_eq!(t2.id, GeneralRunTime, "Min-heap policy failed on 2nd pop");

        let t3 = sched.heap.pop().expect("Heap screams: Expected 3rd task, found None!");
        assert_eq!(t3.id, GeneralHost, "Min-heap policy failed on 3rd pop");

        assert!(sched.heap.is_empty(), "Heap screams: Items remaining after all pops!");
    }

    #[test]
    fn test_pop_batch() {
        let mut sched = Scheduler::new();
        let now = Instant::now();

        sched.push(ScheduledTask { id: GeneralRunTime, cycle_time: 0, execute_at: now + Duration::from_secs(2), limit: None });
        sched.push(ScheduledTask { id: GeneralBootTime, cycle_time: 0, execute_at: now, limit: None });

        let dues = sched.pop_due_batch();

        assert!(!dues.is_empty(), "Due task batch is empty!");
        assert_eq!(dues.len(), 1, "Over-pop batch!");
        assert_eq!(dues.first().unwrap().id, GeneralBootTime, "Batching incorrect task!");
    }
}