
use crate::types::*;
pub trait OsSpecificInterface {
    fn refresh(&mut self);

    fn get_product_serial(&self) -> Option<&str>;
    fn get_architecture(&self) -> &str;
    fn get_producer(&self) -> &str;
    fn get_system_model(&self) -> &str;
    fn get_machine_type(&self) -> &str;

    fn get_bios_vendor(&self) -> &str;
    fn get_bios_version(&self) -> &str;
    fn get_is_secure_boot(&self) -> Option<bool>;

    fn get_os_name(&self) -> &str;

    fn get_motherboard(&self) -> &str;
    fn get_motherboard_serial(&self) -> Option<&str>;
    fn get_cpu_socket(&self) -> Option<u64>;
    fn get_ram_socket(&self) -> Option<u64>;
    fn get_gpu_socket(&self) -> Option<u64>;

    fn get_ram_list(&self) -> Option<&Vec<Ram>>;

    fn get_physical_disk_list(&self) -> Option<&Vec<PhysicalDisk>>;

    fn get_tempe_mobo(&self) -> Option<f64>;
    fn get_tempe_cpu(&self) -> Option<f64>;

    fn get_battery_percentage(&self) -> Option<u64>;
    fn get_is_plugged_in(&self) -> Option<bool>;

    fn fill_network_hardware(&self, network_list: &mut Vec<Network<'_>>);

    fn get_software_list(&self) -> Option<Vec<Software>>;
}
