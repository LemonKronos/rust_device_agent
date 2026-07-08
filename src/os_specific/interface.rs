
use crate::types::*;
pub trait OsSpecificInterface {
    fn refresh(&mut self);

    fn get_product_serial(&self) -> String;
    fn get_architecture(&self) -> String;
    fn get_producer(&self) -> String;
    fn get_system_model(&self) -> String;
    fn get_machine_type(&self) -> String;

    fn get_bios_version(&self) -> String;
    fn get_bios_vendor(&self) -> String;
    fn get_is_secure_boot(&self) -> Option<bool>;

    fn get_os_name(&self) -> String;

    fn get_motherboard(&self) -> String;
    fn get_motherboard_serial(&self) -> String;
    fn get_cpu_socket(&self) -> Option<u32>;
    fn get_ram_socket(&self) -> Option<u32>;
    fn get_gpu_socket(&self) -> Option<u32>;

    fn get_ram_list(&self) -> Option<Vec<Ram>>;

    fn get_hardware_disk_list(&self) -> Option<Vec<HardwareDisk>>;

    fn get_tempe_mobo(&self) -> f32;
    fn get_tempe_cpu(&self) -> f32;

    fn get_percentage(&self) -> f32;
    fn get_is_plugged_in(&self) -> bool;

    fn fill_network_hardware(&self, network_list: &mut Vec<Network<'_>>);

    fn get_software_list(&self) -> Option<Vec<Software>>;
}
