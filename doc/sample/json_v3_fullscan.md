```json
{
  "CPU": {
    "core": 12,
    "frequency": 2529,
    "name": "AMD Ryzen 5 5600H with Radeon Graphics",
    "temperature": 61.75,
    "usage": 8.834259033203125
  },
  "GPUs": [
    {
      "driver": "580.159.03",
      "frequency": 300,
      "max_clock": 1785,
      "name": "NVIDIA GeForce GTX 1650",
      "serial": "Unknown",
      "temperature": 45,
      "utilization": 0.0,
      "vram_total": 4294967296,
      "vram_usage": 130416640
    },
    {
      "driver": "3.64.0",
      "frequency": 400,
      "max_clock": null,
      "name": "AMD Radeon Graphics",
      "serial": "Unknown",
      "temperature": 49,
      "utilization": 100.0,
      "vram_total": 536870912,
      "vram_usage": 458682368
    }
  ],
  "RAM": {
    "hardware": [
      {
        "bank": "P0 CHANNEL A",
        "form_factor": "SODIMM",
        "name": "Kingston KVR32S22S6/8GB 3200",
        "serial": "40D9D109",
        "size": 8192,
        "speed": "3200",
        "type": "DDR4"
      },
      {
        "bank": "P0 CHANNEL B",
        "form_factor": "SODIMM",
        "name": "Kingston ACR32D4S2S1ME-8",
        "serial": "25AB2457",
        "size": 8192,
        "speed": "3200",
        "type": "DDR4"
      }
    ],
    "total": 16081772544,
    "usage": 10712608768
  },
  "SWAP": {
    "total": 4294963200,
    "usage": 3608420352
  },
  "agent_version": "3.0.0",
  "battery": {
    "is_plugged_in": true,
    "percentage": 80
  },
  "bios": {
    "secure_boot": false,
    "vendor": "Insyde Corp.",
    "version": "24.04"
  },
  "disks": {
    "hardware": [
      {
        "drive": "nvme0n1",
        "firmware": "EDFK0S03",
        "index": 0,
        "interface": "NVMe",
        "media": "SSD",
        "model": "KINGSTON OM8PDP3512B-AA1",
        "partition": [
          {
            "name": "0",
            "size": 96.64
          },
          {
            "name": "1",
            "size": 6.96
          },
          {
            "name": "2",
            "size": 0.27
          },
          {
            "name": "3",
            "size": 1.07
          },
          {
            "name": "4",
            "size": 407.14
          },
          {
            "name": "5",
            "size": 0.02
          }
        ],
        "partitions": 1,
        "serial": "50026B76857E7C0B",
        "size": 512,
        "status": "live"
      },
      {
        "drive": "nvme1n1",
        "firmware": "P9CR313",
        "index": 1,
        "interface": "NVMe",
        "media": "SSD",
        "model": "CT500P3SSD8",
        "partition": [
          {
            "name": "0",
            "size": 498.98
          },
          {
            "name": "1",
            "size": 1.13
          }
        ],
        "partitions": 1,
        "serial": "2404468C8B64",
        "size": 500,
        "status": "live"
      }
    ],
    "logical": [
      {
        "file_system": "ext4",
        "mount_point": "/",
        "name": "/dev/nvme1n1p2",
        "removable": false,
        "total": 489999179776,
        "used": 352570200064
      },
      {
        "file_system": "vfat",
        "mount_point": "/boot/efi",
        "name": "/dev/nvme1n1p1",
        "removable": false,
        "total": 1124999168,
        "used": 6541312
      }
    ]
  },
  "general": {
    "boot_time": 1784163319,
    "host": "AcerLinux",
    "run_time": 31130
  },
  "machine": {
    "architecture": "x86_64",
    "machine_type": "Laptop: Notebook",
    "model": "Nitro AN515-45",
    "producer": "Acer",
    "serial": "NHQBMSV0062131BB0D3400"
  },
  "motherboard": {
    "cpu_socket": 1,
    "gpu_socket": 2,
    "name": "Scala_CAS",
    "ram_socket": 2,
    "serial": "NBQBM11001213B72373400",
    "tempe": 45.0
  },
  "networks": [
    {
      "card": "",
      "config_speed": 0,
      "download": 0,
      "ipv4": "172.17.0.1/16",
      "ipv6": "None",
      "mac": "d6:e9:86:f7:02:f9",
      "mtu": 1500,
      "name": "docker0",
      "ssid": "Unknown",
      "upload": 0
    },
    {
      "card": "MEDIATEK Corp. MT7921 802.11ax PCI Express Wireless Network Adapter",
      "config_speed": 0,
      "download": 2255336553,
      "ipv4": "172.20.1.20/22",
      "ipv6": "fe80::515a:3399:7f4f:c499/64",
      "mac": "e0:0a:f6:be:44:9f",
      "mtu": 1500,
      "name": "wlp5s0",
      "ssid": "GGROUP-LAU1",
      "upload": 625585381
    },
    {
      "card": "",
      "config_speed": 0,
      "download": 84,
      "ipv4": "172.18.0.1/16",
      "ipv6": "fe80::f8ff:a5ff:fee8:b0ac/64",
      "mac": "fa:ff:a5:e8:b0:ac",
      "mtu": 1500,
      "name": "br-b68dd5d7c3e7",
      "ssid": "Unknown",
      "upload": 18657677
    },
    {
      "card": "Realtek Semiconductor Co., Ltd. Killer E2600 GbE Controller (rev 21)",
      "config_speed": 0,
      "download": 342856275,
      "ipv4": "None",
      "ipv6": "None",
      "mac": "08:8f:c3:56:4e:01",
      "mtu": 1500,
      "name": "enp4s0",
      "ssid": "Unknown",
      "upload": 9312677
    },
    {
      "card": "",
      "config_speed": 0,
      "download": 0,
      "ipv4": "172.19.0.1/16",
      "ipv6": "None",
      "mac": "7e:84:a7:69:b5:96",
      "mtu": 1500,
      "name": "br-9e4b968fd9d1",
      "ssid": "Unknown",
      "upload": 0
    },
    {
      "card": "",
      "config_speed": 0,
      "download": 238152260,
      "ipv4": "127.0.0.1/8",
      "ipv6": "::1/128",
      "mac": "00:00:00:00:00:00",
      "mtu": 65536,
      "name": "lo",
      "ssid": "Unknown",
      "upload": 238152260
    }
  ],
  "os": {
    "kernel": "6.17.0-35-generic",
    "os_distro": "Ubuntu",
    "os_name": "linux",
    "os_version": "24.04"
  },
  "process_count": 1966,
  "softwares": [
    {
      "install_date": 1763425716,
      "name": "2to3",
      "size": 32.0,
      "source": "Ubuntu Developers",
      "version": "3.12.3-0ubuntu2.1"
    },
    {
      "install_date": 1742312014,
      "name": "7zip",
      "size": 6158.0,
      "source": "Ubuntu Developers",
      "version": "23.01+dfsg-11"
    },
    {
      "install_date": 1782108774,
      "name": "aardvark-dns",
      "size": 2510.0,
      "source": "Ubuntu Developers",
      "version": "1.4.0-5"
    },
    {
      "install_date": 1739607037,
      "name": "accountsservice",
      "size": 524.0,
      "source": "Ubuntu Developers",
      "version": "23.13.9-2ubuntu6"
    },
    {
      "install_date": 1746454674,
      "name": "acl",
      "size": 192.0,
      "source": "Ubuntu Developers",
      "version": "2.3.2-1build1.1"
    }
  ],
  "time_stamp": 1784194449,
  "top_processes": [
    {
      "cpu": 23.21966552734375,
      "memory": 458584064,
      "name": "code",
      "runtime": 17167
    },
    {
      "cpu": 23.029340744018555,
      "memory": 593170432,
      "name": "chrome",
      "runtime": 29479
    },
    {
      "cpu": 21.126089096069336,
      "memory": 458584064,
      "name": "DedicatedWorker",
      "runtime": 2451
    },
    {
      "cpu": 17.509912490844727,
      "memory": 523939840,
      "name": "chrome",
      "runtime": 27925
    },
    {
      "cpu": 6.661379337310791,
      "memory": 401989632,
      "name": "brave",
      "runtime": 30089
    },
    {
      "cpu": 4.948453426361084,
      "memory": 76247040,
      "name": "chrome",
      "runtime": 30929
    },
    {
      "cpu": 4.187152862548828,
      "memory": 523939840,
      "name": "Compositor",
      "runtime": 27925
    },
    {
      "cpu": 3.9968278408050537,
      "memory": 76247040,
      "name": "VizCompositorTh",
      "runtime": 30929
    },
    {
      "cpu": 3.8065028190612793,
      "memory": 43732992,
      "name": "main_worker",
      "runtime": 38
    },
    {
      "cpu": 3.616177558898926,
      "memory": 3211100160,
      "name": "rust-analyzer",
      "runtime": 17163
    }
  ]
}
```