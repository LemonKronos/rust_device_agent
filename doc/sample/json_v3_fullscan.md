```json
{
  "CPU": {
    "core": 12,
    "frequency": 3002,
    "name": "AMD Ryzen 5 5600H with Radeon Graphics",
    "temperature": 67.75,
    "usage": 37.89809036254883
  },
  "GPUs": [
    {
      "driver": "580.159.03",
      "frequency": 300,
      "max_clock": 1785,
      "name": "NVIDIA GeForce GTX 1650",
      "serial": "Unknown",
      "temperature": 46,
      "utilization": 0.0,
      "vram_total": 4294967296,
      "vram_usage": 5242880
    },
    {
      "driver": "3.64.0",
      "frequency": 1800,
      "max_clock": null,
      "name": "AMD Radeon Graphics",
      "serial": "Unknown",
      "temperature": 53,
      "utilization": 100.0,
      "vram_total": 536870912,
      "vram_usage": 483848192
    }
  ],
  "RAM": {
    "hardware": null,
    "total": 16081776640,
    "usage": 11834388480
  },
  "SWAP": {
    "total": 4294963200,
    "usage": 4057665536
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
        "used": 353891127296
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
    "boot_time": 1784251501,
    "host": "AcerLinux",
    "run_time": 7587
  },
  "machine": {
    "architecture": "x86_64",
    "machine_type": "Laptop: Notebook",
    "model": "Nitro AN515-45",
    "producer": "Acer",
    "serial": "Unknown"
  },
  "motherboard": {
    "cpu_socket": null,
    "gpu_socket": 2,
    "name": "Scala_CAS",
    "ram_socket": null,
    "serial": "Unknown",
    "tempe": 44.0
  },
  "networks": [
    {
      "card": "MEDIATEK Corp. MT7921 802.11ax PCI Express Wireless Network Adapter",
      "config_speed": null,
      "download": 203005656,
      "ipv4": "172.20.1.20/22",
      "ipv6": "fe80::515a:3399:7f4f:c499/64",
      "mac": "e0:0a:f6:be:44:9f",
      "mtu": 1500,
      "name": "wlp5s0",
      "ssid": "GGROUP-LAU1",
      "upload": 32643576
    },
    {
      "card": null,
      "config_speed": null,
      "download": 630731,
      "ipv4": "127.0.0.1/8",
      "ipv6": "::1/128",
      "mac": "00:00:00:00:00:00",
      "mtu": 65536,
      "name": "lo",
      "ssid": null,
      "upload": 630731
    },
    {
      "card": "Realtek Semiconductor Co., Ltd. Killer E2600 GbE Controller (rev 21)",
      "config_speed": null,
      "download": 0,
      "ipv4": "None",
      "ipv6": "None",
      "mac": "08:8f:c3:56:4e:01",
      "mtu": 1500,
      "name": "enp4s0",
      "ssid": null,
      "upload": 0
    }
  ],
  "os": {
    "kernel": "6.17.0-40-generic",
    "os_distro": "Ubuntu",
    "os_name": "linux",
    "os_version": "24.04"
  },
  "process_count": 1759,
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
  "time_stamp": 1784259088,
  "top_processes": [
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "kworker/u49:6-ttm",
      "runtime": 359
    },
    {
      "cpu": 0.0,
      "memory": 12808192,
      "name": "gmain",
      "runtime": 7585
    },
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "idle_inject/6",
      "runtime": 7587
    },
    {
      "cpu": 0.0,
      "memory": 2138112,
      "name": "chrome_crashpad",
      "runtime": 7523
    },
    {
      "cpu": 0.0,
      "memory": 136228864,
      "name": "brave:traceq0",
      "runtime": 7523
    },
    {
      "cpu": 0.0,
      "memory": 196608,
      "name": "lf",
      "runtime": 7455
    },
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "nv_open_q",
      "runtime": 7583
    },
    {
      "cpu": 0.0,
      "memory": 1916928,
      "name": "exe",
      "runtime": 7571
    },
    {
      "cpu": 0.0,
      "memory": 51929088,
      "name": "Xwayla:sh_opt0",
      "runtime": 7570
    },
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "kworker/R-nvme-delete-wq",
      "runtime": 7587
    }
  ]
}
```