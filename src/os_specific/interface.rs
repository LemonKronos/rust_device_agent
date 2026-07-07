
pub trait OsSpecificInterface {
    fn refresh(&mut self);

    fn get_product_serial(&self) -> String;
    fn get_architecture(&self) -> String;
    fn get_producer(&self) -> String;
    fn get_system_model(&self) -> String;
    fn get_machine_type(&self) -> String;

    fn get_os_name(&self) -> String;

    fn get_motherboard(&self) -> String;
    fn get_motherboard_serial(&self) -> String;
    fn get_cpu_slot(&self) -> u32;
    fn get_ram_slot(&self) -> u32;
    fn get_gpu_slot(&self) -> u32;

    fn get_tempe_mobo(&self) -> f32;
    fn get_tempe_cpu(&self) -> f32;

    fn get_percentage(&self) -> f32;
    fn get_is_plugged_in(&self) -> bool;
}
