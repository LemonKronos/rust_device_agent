```json
{
  "CPU": {
    "core": 12,
    "frequency": 3502,
    "name": "AMD Ryzen 5 5600H with Radeon Graphics",
    "temperature": 67.875,
    "usage": 15.95441722869873
  },
  "GPUs": [
    {
      "driver": "580.159.03",
      "frequency": 300,
      "max_clock": 1785,
      "name": "NVIDIA GeForce GTX 1650",
      "serial": "Unknown",
      "temperature": 47,
      "utilization": 0.0,
      "vram_total": 4294967296,
      "vram_usage": 71434240
    },
    {
      "driver": "3.64.0",
      "frequency": 1800,
      "max_clock": null,
      "name": "AMD Radeon Graphics",
      "serial": "Unknown",
      "temperature": 54,
      "utilization": 100.0,
      "vram_total": 536870912,
      "vram_usage": 460214272
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
    "total": 16081776640,
    "usage": 11288678400
  },
  "SWAP": {
    "total": 4294963200,
    "usage": 2395889664
  },
  "agent_version": "3.0.0",
  "battery": {
    "is_plugged_in": true,
    "percentage": 80.0
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
        "used": 358436626432
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
    "boot_time": 1783904292,
    "host": "AcerLinux",
    "run_time": 12628
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
    "tempe": 46.0
  },
  "networks": [
    {
      "card": "MEDIATEK Corp. MT7921 802.11ax PCI Express Wireless Network Adapter",
      "config_speed": 0,
      "download": 470655099,
      "ipv4": "172.20.1.20/22",
      "ipv6": "fe80::515a:3399:7f4f:c499/64",
      "mac": "e0:0a:f6:be:44:9f",
      "mtu": 1500,
      "name": "wlp5s0",
      "ssid": "GGROUP-LAU1",
      "upload": 60116705
    },
    {
      "card": "Realtek Semiconductor Co., Ltd. Killer E2600 GbE Controller (rev 21)",
      "config_speed": 0,
      "download": 0,
      "ipv4": "None",
      "ipv6": "None",
      "mac": "08:8f:c3:56:4e:01",
      "mtu": 1500,
      "name": "enp4s0",
      "ssid": "Unknown",
      "upload": 0
    },
    {
      "card": "Unknown",
      "config_speed": 0,
      "download": 1184563,
      "ipv4": "127.0.0.1/8",
      "ipv6": "::1/128",
      "mac": "00:00:00:00:00:00",
      "mtu": 65536,
      "name": "lo",
      "ssid": "Unknown",
      "upload": 1184563
    }
  ],
  "os": {
    "kernel": "6.17.0-35-generic",
    "os_distro": "Ubuntu",
    "os_name": "linux",
    "os_version": "24.04"
  },
  "process_count": 1942,
  "softwares": [
    {
      "install_date": 1763425716,
      "name": "2to3",
      "size": 32,
      "source": "Ubuntu Developers",
      "version": "3.12.3-0ubuntu2.1"
    },
    {
      "install_date": 1742312014,
      "name": "7zip",
      "size": 6158,
      "source": "Ubuntu Developers",
      "version": "23.01+dfsg-11"
    },
    {
      "install_date": 1782108774,
      "name": "aardvark-dns",
      "size": 2510,
      "source": "Ubuntu Developers",
      "version": "1.4.0-5"
    },
    {
      "install_date": 1739607037,
      "name": "accountsservice",
      "size": 524,
      "source": "Ubuntu Developers",
      "version": "23.13.9-2ubuntu6"
    },
    {
      "install_date": 1746454674,
      "name": "acl",
      "size": 192,
      "source": "Ubuntu Developers",
      "version": "2.3.2-1build1.1"
    }
  ],
  "time_stamp": 1783916921,
  "top_processes": [
    {
      "cpu": 47.86324691772461,
      "memory": 38920192,
      "name": "main_worker",
      "runtime": 1
    },
    {
      "cpu": 30.76923179626465,
      "memory": 562536448,
      "name": "brave",
      "runtime": 12304
    },
    {
      "cpu": 27.35042953491211,
      "memory": 692523008,
      "name": "code",
      "runtime": 11466
    },
    {
      "cpu": 23.931623458862305,
      "memory": 137986048,
      "name": "code",
      "runtime": 11466
    },
    {
      "cpu": 23.931623458862305,
      "memory": 404492288,
      "name": "chrome",
      "runtime": 12262
    },
    {
      "cpu": 13.675214767456055,
      "memory": 107868160,
      "name": "Hyprland",
      "runtime": 12606
    },
    {
      "cpu": 10.256410598754883,
      "memory": 692523008,
      "name": "Compositor",
      "runtime": 11466
    },
    {
      "cpu": 6.837607383728027,
      "memory": 562536448,
      "name": "dav1d-worker",
      "runtime": 803
    },
    {
      "cpu": 6.837607383728027,
      "memory": 6070272,
      "name": "cava",
      "runtime": 12604
    },
    {
      "cpu": 6.837607383728027,
      "memory": 146886656,
      "name": "VizCompositorTh",
      "runtime": 12477
    }
  ]
}
```