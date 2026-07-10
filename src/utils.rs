
use std::time::{SystemTime, UNIX_EPOCH};

// _ TIME Conversion
pub trait FormatTime { 
    fn to_datetime_string(&self) -> String;
    fn to_time_sec(&self) -> u64;
}

impl FormatTime for SystemTime {
    fn to_datetime_string(&self) -> String {
        chrono::DateTime::<chrono::Local>::from(*self)
            .format("%H:%M:%S %d/%m/%Y").to_string()
    }

    fn to_time_sec(&self) -> u64 {
        match self.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_secs(),
            _ => 0,
        }
    }
}

pub trait FormatUptime {
    fn to_uptime_string(&self) -> String;
}

impl FormatUptime for u64 {
    fn to_uptime_string(&self) -> String {
        format!("{:03}:{:02}:{:02}:{:02}", self / 86400, (self % 86400) / 3600, (self % 3600) / 60, self % 60)
    }
}

// _ MEMORY Conversion
pub trait FormatMem {
    fn b_to_mb(&self) -> f64;
    fn b_to_gb(&self) -> f64;
    fn kb_to_mb(&self) -> f64;
    fn kb_to_gb(&self) -> f64;
    fn mb_to_gb(&self) -> f64;
    fn gb_to_mb(&self) -> f64;
}

impl FormatMem for u64 {
    fn b_to_mb(&self) -> f64 { *self as f64 / 1_048_576.0 }
    fn b_to_gb(&self) -> f64 { *self as f64 / 1_073_741_824.0 }
    fn kb_to_mb(&self) -> f64 { *self as f64 / 1024.0 }
    fn kb_to_gb(&self) -> f64 { *self as f64 / 1_048_576.0 }
    fn mb_to_gb(&self) -> f64 { *self as f64 / 1024.0 }
    fn gb_to_mb(&self) -> f64 { *self as f64 * 1024.0 }
}