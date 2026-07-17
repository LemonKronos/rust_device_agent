```json
{
  "CPU": {
    "core": 12,
    "frequency": 3899,
    "name": "AMD Ryzen 5 5600H with Radeon Graphics",
    "temperature": 70.25,
    "usage": 16.107383728027344
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
      "vram_usage": 5242880
    },
    {
      "driver": "3.64.0",
      "frequency": 400,
      "max_clock": null,
      "name": "AMD Radeon Graphics",
      "serial": "Unknown",
      "temperature": 54,
      "utilization": 100.0,
      "vram_total": 536870912,
      "vram_usage": 474329088
    }
  ],
  "RAM": {
    "hardware": null,
    "total": 16081776640,
    "usage": 12747227136
  },
  "SWAP": {
    "total": 4294963200,
    "usage": 4291866624
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
        "used": 354252386304
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
    "run_time": 8800
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
    "tempe": 46.0
  },
  "networks": [
    {
      "card": "MEDIATEK Corp. MT7921 802.11ax PCI Express Wireless Network Adapter",
      "config_speed": null,
      "download": 313103054,
      "ipv4": "172.20.1.20/22",
      "ipv6": "fe80::515a:3399:7f4f:c499/64",
      "mac": "e0:0a:f6:be:44:9f",
      "mtu": 1500,
      "name": "wlp5s0",
      "ssid": "GGROUP-LAU1",
      "upload": 39065278
    },
    {
      "card": null,
      "config_speed": null,
      "download": 752994,
      "ipv4": "127.0.0.1/8",
      "ipv6": "::1/128",
      "mac": "00:00:00:00:00:00",
      "mtu": 65536,
      "name": "lo",
      "ssid": null,
      "upload": 752994
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
  "process_count": 1863,
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
  "time_stamp": 1784260301,
  "top_processes": [
    {
      "cpu": 0.0,
      "memory": 692871168,
      "name": "chrome",
      "runtime": 8779
    },
    {
      "cpu": 0.0,
      "memory": 46866432,
      "name": "waybar",
      "runtime": 8783
    },
    {
      "cpu": 0.0,
      "memory": 75718656,
      "name": "ThreadPoolForeg",
      "runtime": 8681
    },
    {
      "cpu": 0.0,
      "memory": 32894976,
      "name": "Timer",
      "runtime": 8779
    },
    {
      "cpu": 0.0,
      "memory": 1761280,
      "name": "wl-paste",
      "runtime": 8783
    },
    {
      "cpu": 0.0,
      "memory": 405934080,
      "name": "ThreadPoolForeg",
      "runtime": 8580
    },
    {
      "cpu": 0.0,
      "memory": 120614912,
      "name": "hyprloc:disk$0",
      "runtime": 2210
    },
    {
      "cpu": 0.0,
      "memory": 49938432,
      "name": "Xwayland:gl0",
      "runtime": 8783
    },
    {
      "cpu": 0.0,
      "memory": 220024832,
      "name": "WorkerThread",
      "runtime": 8689
    },
    {
      "cpu": 0.0,
      "memory": 26062848,
      "name": "dconf worker",
      "runtime": 8783
    }
  ]
}
```