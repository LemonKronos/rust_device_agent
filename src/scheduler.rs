///
/// Timer wheel 
/// 

use std::cmp::Ordering;
use tokio::time::Instant;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BinaryHeap;
use std::fmt;
use std::time::{Duration, SystemTime};

use crate::utils::FormatTime;

/// ScheduledTask in Timer Wheel
 #[derive(Debug)]
pub struct ScheduledTask {
    pub id: String, // "module.component(.sub component)"
    pub cycle_time: u64,
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

    pub fn pop(&mut self) -> Option<ScheduledTask> {
        self.heap.pop()
    }

    pub fn peek(&self) -> Option<&ScheduledTask> {
        self.heap.peek()
    }

    pub fn into_vec(self) -> Vec<ScheduledTask> {
        self.heap.into_vec()
    }
}

// RAM to DISK
impl Serialize for TimerWheel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // We are serializing into a JSON map: { "id": [cycle, unix_timestamp] }
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

            // Serialize as "module.component": [cycle_time, timestamp]
            map.serialize_entry(&task.id, &(task.cycle_time, unix_timestamp))?;
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
                while let Some((id, (cycle_time, saved_timestamp))) =
                    access.next_entry::<String, (u64, u64)>()?
                {
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

