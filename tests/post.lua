wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = [[
{
  "CPU": {
    "core": 12,
    "frequency": 2841,
    "name": "AMD Ryzen 5 5600H with Radeon Graphics",
    "temperature": 67.25,
    "usage": 5.3413920402526855
  },
  "GPUs": [
    {
      "driver": "580.159.03",
      "name": "NVIDIA GeForce GTX 1650",
      "temperature": 48,
      "utilization": 0.0,
      "vram_total": 4294967296,
      "vram_usage": 117768192
    },
    {
      "driver": "3.64.0",
      "name": "AMD Radeon Graphics",
      "temperature": 55,
      "utilization": 100.0,
      "vram_total": 536870912,
      "vram_usage": 492310528
    }
  ],
  "RAM": {
    "total": 16081772544,
    "usage": 9776496640
  },
  "SWAP": {
    "total": 4294963200,
    "usage": 3889324032
  },
  "battery": {
    "is_plugged_in": true,
    "percentage": 80.0
  },
  "disks": [
    {
      "name": "/dev/nvme0n1p2",
      "removable": false,
      "total": 489999179776,
      "type": "SSD",
      "used": 0
    },
    {
      "name": "/dev/nvme0n1p1",
      "removable": false,
      "total": 1124999168,
      "type": "SSD",
      "used": 0
    }
  ],
  "machine": {
    "architecture": "x86_64",
    "machine_type": "Laptop / Notebook",
    "model": "Nitro AN515-45",
    "motherboard": "Scala_CAS",
    "os_name": "linux",
    "producer": "Acer"
  },
  "networks": [
    {
      "download": 11486312479,
      "ipv4": "127.0.0.1/8",
      "ipv6": "::1/128",
      "mac": "00:00:00:00:00:00",
      "mtu": 65536,
      "name": "lo",
      "upload": 11486312479
    },
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
      "download": 548044553,
      "ipv4": "172.20.1.20/22",
      "ipv6": "fe80::7d79:4d55:fe01:574a/64",
      "mac": "e0:0a:f6:be:44:9f",
      "mtu": 1500,
      "name": "wlp5s0",
      "upload": 86715042
    }
  ],
  "processes": [
    {
      "cpu": 1.3557661771774292,
      "memory": 444731392,
      "name": "code",
      "runtime": 11320
    },
    {
      "cpu": 0.7513884902000427,
      "memory": 25432064,
      "name": "eww",
      "runtime": 11445
    },
    {
      "cpu": 0.6207122206687927,
      "memory": 67612672,
      "name": "Hyprland",
      "runtime": 11446
    },
    {
      "cpu": 0.5227050185203552,
      "memory": 694927360,
      "name": "brave",
      "runtime": 9452
    },
    {
      "cpu": 0.24501799046993256,
      "memory": 43499520,
      "name": "waybar",
      "runtime": 11445
    },
    {
      "cpu": 0.24501799046993256,
      "memory": 444731392,
      "name": "Compositor",
      "runtime": 11320
    },
    {
      "cpu": 0.24501799046993256,
      "memory": 19853312,
      "name": "swww-daemon",
      "runtime": 11445
    },
    {
      "cpu": 0.2286834567785263,
      "memory": 19853312,
      "name": "animation",
      "runtime": 11407
    },
    {
      "cpu": 0.1960143893957138,
      "memory": 66289664,
      "name": "code",
      "runtime": 11321
    },
    {
      "cpu": 0.17967985570430756,
      "memory": 53751808,
      "name": "gsoft_device_ag",
      "runtime": 299
    }
  ],
  "system": {
    "boot_time": 1782196300,
    "kernel": "6.17.0-35-generic",
    "os": "Ubuntu",
    "os_version": "24.04",
    "run_time": 11458
  },
  "temperature": 49.0,
  "time_stamp": 1782207758,
  "uuid": "dde467f4afbae184d2303cbf25e9a43cf6cdf5a1de4ab012571be53bf4303c4e"
]]