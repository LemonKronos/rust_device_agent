
pub mod os_specific;
pub mod info_gatherer;
pub mod sender;
pub mod utils;




#[cfg(test)]
mod tests {
    use crate::info_gatherer::Info;
    use serde_json::json;

    #[test]
    fn print_json() {
        let info = Info::new();

        let gpus_json: Vec<_> = info.get_gpu_list().iter().map(|gpu| {
            json!({
                "name": gpu.get_name(),
                "driver": gpu.get_driver(),
                "utilization": gpu.get_util(), // %
                "temperature": gpu.get_temp(), // ℃
                "vram_total": gpu.get_vram_total(), // byte
                "vram_usage": gpu.get_vram_usage(), // byte
            })
        }).collect();

        let disks_json: Vec<_> = info.get_disk_info().iter().map(|disk| {
            json!({
                "name": disk.get_name(),
                "type": disk.get_type(),
                "removable": disk.get_removable(), // bool
                "total": disk.get_total(), // byte
                "used": disk.get_used(), // byte
            })
        }).collect();

        let networks_json: Vec<_> = info.get_networks().iter().map(|network| {
            json!({
                "name": network.get_name(),
                "ipv4": network.get_ipv4(),
                "ipv6": network.get_ipv6(),
                "mac": network.get_mac(),
                "mtu": network.get_mtu(), // byte
                "upload": network.get_upload(), // byte
                "download": network.get_download(), // byte
            })
        }).collect();

        let processes_json: Vec<_> = info.get_running().iter().map(|process| {
            json!({
                "name": process.get_name(),
                "cpu": process.get_cpu_usage(), // %
                "memory": process.get_memory(), // byte
                "runtime": process.get_runtime(), // second
            })
        }).collect();

        let payload = json!({
            "uuid": info.get_uuid().to_string(),
            "machine": {   
                "architecture": info.get_architecture(),
                "os_name": info.get_os_name(),
                "producer": info.get_producer(),
                "model": info.get_system_model(),
                "motherboard": info.get_motherboard(),
                "machine_type": info.get_machine_type(),
            },
            "system": {
                "os": info.get_os(),
                "os_version": info.get_os_version(),
                "kernel": info.get_kernel(),
                "boot_time": info.get_boot_time(), // second
                "run_time": info.get_run_time(), // second
            },
            "CPU": {
                "name": info.get_cpu_name(),
                "core": info.get_cpu_core(),
                "usage": info.get_cpu_usage(), // %
                "frequency": info.get_cpu_freq(), // MHz
                "temperature": info.get_cpu_temp(), // ℃
            },
            "RAM": {
                "total": info.get_ram_total(), // byte
                "usage": info.get_ram_usage(), // byte
            },
            "SWAP": {
                "total": info.get_swap_total(), // byte
                "usage": &info.get_swap_usage(), // byte
            },
            "GPUs": gpus_json, // list
            "disks": disks_json, // list
            "networks": networks_json, // list
            "processes": processes_json, // list
            "battery": {
                "percentage": info.get_battery_percentage(),
                "is_plugged_in": info.get_battery_is_plugged_in(), // bool, note that "plugged in" is different from "charging"
            },
            "temperature": info.get_temp_mobo(), // ℃
        });

        let pretty_json = serde_json::to_string_pretty(&payload).unwrap();
        
        println!("\n=== GENERATED HARDWARE PAYLOAD ===");
        println!("{}", pretty_json);
        println!("==================================\n");
    }
}