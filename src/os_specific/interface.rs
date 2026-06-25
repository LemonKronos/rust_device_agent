
pub trait TemperatureInterface {
    fn ready_temp(&mut self);
    fn get_temp_mobo(&self) -> f32;
    fn get_temp_cpu(&self) -> f32;
}
pub trait BatteryInterface: Sized {
    fn get_percentage(&self) -> f32;
    fn get_is_plugged_in(&self) -> bool;
}

pub trait MachineInterface {
    fn get_serial(&self) -> &str;
    fn get_architecture(&self) -> &str;
    fn get_os_name(&self) -> &str;
    fn get_producer(&self) -> &str;
    fn get_system_model(&self) -> &str;
    fn get_motherboard(&self) -> &str;
    fn get_machine_type(&self) -> &str;
}
