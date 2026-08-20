//!
//! Utilities function
//! 

use std::time::{SystemTime, UNIX_EPOCH};

// : TIME Conversion
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

// : MEMORY Conversion
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

// : Logger
use flexi_logger::{
    writers::{FileLogWriter, LogWriter},
    Cleanup, Criterion, DeferredNow, FileSpec, Logger, Naming, Record,
};

fn log_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let raw_msg = record.args().to_string();
    let indented_msg = raw_msg.replace('\n', "\n\t");
    
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");

    write!(
        w,
        "[{timestamp} {level} {module}]:\n\t{message}\n",
        timestamp = timestamp,
        level = record.level(),
        module = record.target(),
        message = indented_msg
    )
}

// Custom writer that duplicates logs to two files
struct DualWriter {
    history: FileLogWriter,
    current: FileLogWriter,
}

impl LogWriter for DualWriter {
    fn write(&self, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        // Send the log to both writers
        self.history.write(now, record)?;
        self.current.write(now, record)?;
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        self.history.flush()?;
        self.current.flush()?;
        Ok(())
    }
}

/// Logger for each binary, each will have a continuous log that will get appended for run and a current log for last/current run.
/// Log file size and number of log file to keep can be changed here.
/// Currently in dev cycle hence the heavy logging.
pub fn init_logger(binary_name: &str) {
    let Ok(logger) = Logger::try_with_env_or_str("info") else {
        return eprintln!("Invalid RUST_LOG environment variable. Running without logs.");
    };

    #[cfg(feature = "local_workspace")]
    let log_dir = format!("./doc/logs/{}", binary_name);
    #[cfg(not(feature = "local_workspace"))]
    let log_dir = format!("/var/log/gsoft_device_agent/{}", binary_name);

    // 1. The Continuous Logger (Appends & Rotates)
    let history_writer = match FileLogWriter::builder(
        FileSpec::default()
            .directory(&log_dir)
            .basename(format!("{}_history", binary_name))
            .suppress_timestamp(),
    )
    .append()
    .format(log_format)
    .rotate(
        Criterion::Size(5242880), // 5 MB
        Naming::Numbers,
        Cleanup::KeepLogFiles(1),
    )
    .try_build()
    {
        Ok(w) => w,
        Err(e) => return eprintln!("Failed to build history writer. Error: {}", e),
    };

    // 2. The Current Run Logger (Truncates on startup, no rotation needed)
    let current_writer = match FileLogWriter::builder(
        FileSpec::default()
            .directory(&log_dir)
            .basename(format!("{}_current", binary_name))
            .suppress_timestamp(),
    )
    .format(log_format)
    .try_build() // Default behavior is to truncate the file
    {
        Ok(w) => w,
        Err(e) => return eprintln!("Failed to build current writer. Error: {}", e),
    };

    // Bundle them together
    let dual_writer = Box::new(DualWriter {
        history: history_writer,
        current: current_writer,
    });

    // Start the logger with our custom dual writer
    if let Err(e) = logger.log_to_writer(dual_writer).start() {
        eprintln!("Failed to start logger at '{}' for '{}'. Running silently. Error: {}", log_dir, binary_name, e);
    }
}

// : Formating
/// Flatten Scheduler json to save for easy human read
pub fn flatten_config(pretty_json: &str) -> String {
    let mut output = String::with_capacity(pretty_json.len());
    let mut inside_array = false;

    for line in pretty_json.lines() {
        let trimmed = line.trim();
        
        if inside_array {
            if trimmed == "]," || trimmed == "]" {
                output.push_str(trimmed);
                inside_array = false;
            } else {
                output.push_str(trimmed);
                if trimmed.ends_with(',') { 
                    output.push(' '); 
                }
            }
        } else {
            // Normal JSON lines get a newline (except the very first line)
            if !output.is_empty() { 
                output.push('\n'); 
            }
            output.push_str(line);
            
            // If this line opened an array, flip the switch for the next lines
            if line.ends_with('[') {
                inside_array = true;
            }
        }
    }
    
    output
}
