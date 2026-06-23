# Example JSON:
```json
{
  "CPU": {
    "core": 12,
    "frequency": 3941,
    "name": "AMD Ryzen 5 5600H with Radeon Graphics",
    "temperature": 67.0,
    "usage": 37.66824722290039
  },
  "GPUs": [
    {
      "driver": "580.159.03",
      "name": "NVIDIA GeForce GTX 1650",
      "temperature": 50,
      "utilization": 0.0,
      "vram_total": 4294967296,
      "vram_usage": 21037056
    },
    {
      "driver": "3.64.0",
      "name": "AMD Radeon Graphics",
      "temperature": 55,
      "utilization": 100.0,
      "vram_total": 536870912,
      "vram_usage": 473128960
    }
  ],
  "RAM": {
    "total": 16081776640,
    "usage": 8473710592
  },
  "SWAP": {
    "total": 4294963200,
    "usage": 72454144
  },
  "battery": {
    "is_plugged_in": true,
    "percentage": 80
  },
  "disks": [
    {
      "name": "/dev/nvme1n1p2",
      "removable": false,
      "total": 489999179776,
      "type": "SSD",
      "used": 0
    },
    {
      "name": "/dev/nvme1n1p1",
      "removable": false,
      "total": 1124999168,
      "type": "SSD",
      "used": 0
    }
  ],
  "machine": {
    "architecture": "x86_64",
    "boot_time": 1782181656,
    "kernel": "6.17.0-35-generic",
    "machine_type": "Laptop / Notebook",
    "model": "Nitro AN515-45",
    "motherboard": "Scala_CAS",
    "os": "Ubuntu",
    "os_name": "linux",
    "os_version": "24.04",
    "producer": "Acer"
  },
  "networks": [
    {
      "download": 0,
      "ipv4": "None",
      "ipv6": "None",
      "mac": "08:8f:c3:56:4e:01",
      "mtu": 1500,
      "name": "enp4s0",
      "upload": 0
    },
    {
      "download": 58139260,
      "ipv4": "172.20.1.20/22",
      "ipv6": "fe80::7d79:4d55:fe01:574a/64",
      "mac": "e0:0a:f6:be:44:9f",
      "mtu": 1500,
      "name": "wlp5s0",
      "upload": 11555053
    },
    {
      "download": 619779,
      "ipv4": "127.0.0.1/8",
      "ipv6": "::1/128",
      "mac": "00:00:00:00:00:00",
      "mtu": 65536,
      "name": "lo",
      "upload": 619779
    }
  ],
  "processes": [
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "kworker/u49:5-ttm",
      "runtime": 982
    },
    {
      "cpu": 0.0,
      "memory": 321937408,
      "name": "WRWorkerLP#6",
      "runtime": 3653
    },
    {
      "cpu": 0.0,
      "memory": 205430784,
      "name": "Chrome_ChildIOT",
      "runtime": 2806
    },
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "kworker/8:6-events",
      "runtime": 221
    },
    {
      "cpu": 0.0,
      "memory": 21118976,
      "name": "tests::print_js",
      "runtime": 0
    },
    {
      "cpu": 0.0,
      "memory": 92160000,
      "name": "waybar",
      "runtime": 3656
    },
    {
      "cpu": 0.0,
      "memory": 104046592,
      "name": "DelayedTaskSche",
      "runtime": 2013
    },
    {
      "cpu": 0.0,
      "memory": 33439744,
      "name": "fcitx5",
      "runtime": 3657
    },
    {
      "cpu": 0.0,
      "memory": 41672704,
      "name": "main-async-runt",
      "runtime": 3656
    },
    {
      "cpu": 0.0,
      "memory": 18038784,
      "name": "pw-data-loop",
      "runtime": 3653
    }
  ],
  "temperature": 49,
  "uuid": "dde467f4afbae184d2303cbf25e9a43cf6cdf5a1de4ab012571be53bf4303c4e"
}
```
# Note on units:
```rust
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
```