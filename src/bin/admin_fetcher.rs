///
/// Separate binary, handle task that need root privelege
/// 

use gsoft_device_agent::utils::*;
use gsoft_device_agent::config_handler::*;

fn main() {
    init_logger();

    let _ = load_config();
}