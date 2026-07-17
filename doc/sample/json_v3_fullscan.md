```json
{
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
  "cpu": {
    "core": 12,
    "frequency": 3605,
    "name": "AMD Ryzen 5 5600H with Radeon Graphics",
    "temperature": 75.75,
    "usage": 66.7415771484375
  },
  "disks": {
    "logical": [
      {
        "file_system": "ext4",
        "mount_point": "/",
        "name": "/dev/nvme1n1p2",
        "removable": false,
        "total": 489999179776,
        "used": 355996766208
      },
      {
        "file_system": "vfat",
        "mount_point": "/boot/efi",
        "name": "/dev/nvme1n1p1",
        "removable": false,
        "total": 1124999168,
        "used": 6541312
      }
    ],
    "physical": [
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
    ]
  },
  "general": {
    "boot_time": 1784269939,
    "host": "AcerLinux",
    "run_time": 10137
  },
  "gpu": [
    {
      "driver": "580.159.03",
      "frequency": 300,
      "max_clock": 1785,
      "name": "NVIDIA GeForce GTX 1650",
      "serial": "Unknown",
      "temperature": 49,
      "utilization": 0.0,
      "vram_total": 4294967296,
      "vram_usage": 3932160
    },
    {
      "driver": "3.64.0",
      "frequency": 1625,
      "max_clock": null,
      "name": "AMD Radeon Graphics",
      "serial": "Unknown",
      "temperature": 57,
      "utilization": 100.0,
      "vram_total": 536870912,
      "vram_usage": 497373184
    }
  ],
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
    },
    {
      "card": null,
      "config_speed": null,
      "download": 922231,
      "ipv4": "127.0.0.1/8",
      "ipv6": "::1/128",
      "mac": "00:00:00:00:00:00",
      "mtu": 65536,
      "name": "lo",
      "ssid": null,
      "upload": 922231
    },
    {
      "card": "MEDIATEK Corp. MT7921 802.11ax PCI Express Wireless Network Adapter",
      "config_speed": null,
      "download": 565060681,
      "ipv4": "10.247.198.129/24",
      "ipv6": "2401:d800:940:e36c:35ef:90b5:17cb:6ef5/64, 2401:d800:940:e36c:b911:ecfe:c056:aa2a/64, fe80::9985:939:1da7:d03/64",
      "mac": "e0:0a:f6:be:44:9f",
      "mtu": 1500,
      "name": "wlp5s0",
      "ssid": "TECNO POVA 6 Neo",
      "upload": 36242122
    }
  ],
  "os": {
    "kernel": "6.17.0-40-generic",
    "os_distro": "Ubuntu",
    "os_name": "linux",
    "os_version": "24.04"
  },
  "process_count": 1850,
  "ram": {
    "logical": {
      "total": 16081780736,
      "usage": 9261703168
    },
    "physical": null
  },
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
  "swap": {
    "total": 4294963200,
    "usage": 3089936384
  },
  "time_stamp": 1784280076,
  "top_processes": [
    {
      "cpu": 0.0,
      "memory": 171057152,
      "name": "PerfettoTrace",
      "runtime": 1111
    },
    {
      "cpu": 0.0,
      "memory": 49754112,
      "name": "gdbus",
      "runtime": 10124
    },
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "cpuhp/7",
      "runtime": 10136
    },
    {
      "cpu": 0.0,
      "memory": 16977920,
      "name": "containerd",
      "runtime": 10125
    },
    {
      "cpu": 0.0,
      "memory": 16977920,
      "name": "containerd",
      "runtime": 10125
    },
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "jbd2/nvme1n1p2-8",
      "runtime": 10135
    },
    {
      "cpu": 0.0,
      "memory": 44974080,
      "name": "gsoft_device_ag",
      "runtime": 0
    },
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "kworker/10:3-events",
      "runtime": 237
    },
    {
      "cpu": 0.0,
      "memory": 49487872,
      "name": "pw-data-loop",
      "runtime": 10125
    },
    {
      "cpu": 0.0,
      "memory": 77082624,
      "name": "ThreadPoolForeg",
      "runtime": 9820
    }
  ]
}
```