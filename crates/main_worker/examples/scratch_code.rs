
use main_worker::info_gatherer;
use shared_libs::utils::*;

fn main() {
    init_logger();
    let mut info = info_gatherer::Info::new();
    info.prepare();
    println!("{:?}", info.get_is_secure_boot());
}