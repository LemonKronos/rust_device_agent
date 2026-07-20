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
    "frequency": 2760,
    "name": "AMD Ryzen 5 5600H with Radeon Graphics",
    "temperature": 64.125,
    "usage": 10.917030334472656
  },
  "disks": {
    "logical": [
      {
        "file_system": "ext4",
        "mount_point": "/",
        "name": "/dev/nvme0n1p2",
        "removable": false,
        "total": 489999179776,
        "used": 359725690880
      },
      {
        "file_system": "vfat",
        "mount_point": "/boot/efi",
        "name": "/dev/nvme0n1p1",
        "removable": false,
        "total": 1124999168,
        "used": 6541312
      },
      {
        "file_system": "overlay",
        "mount_point": "/var/lib/docker/overlay2/6981e29be77e7afdaf5631f656f54f7fc5588cd499bdb790761e72de2cc0d37a/merged",
        "name": "overlay",
        "removable": false,
        "total": 489999179776,
        "used": 359725690880
      },
      {
        "file_system": "overlay",
        "mount_point": "/var/lib/docker/overlay2/06ec881ff472e8082c5f0fa3d54ec8b06cd5fce0ff6104898e63439f0e84499b/merged",
        "name": "overlay",
        "removable": false,
        "total": 489999179776,
        "used": 359725690880
      },
      {
        "file_system": "ntfs3",
        "mount_point": "/media/01DD05DDDC2FE2E0",
        "name": "/dev/nvme1n1p3",
        "removable": false,
        "total": 6964637696,
        "used": 45490176
      }
    ],
    "physical": [
      {
        "drive": "nvme0n1",
        "firmware": "P9CR313",
        "index": 0,
        "interface": "NVMe",
        "media": "SSD",
        "model": "CT500P3SSD8",
        "partition": [
          {
            "name": "0",
            "size": 1.13
          },
          {
            "name": "1",
            "size": 498.98
          }
        ],
        "partitions": 1,
        "serial": "2404468C8B64",
        "size": 500,
        "status": "live"
      },
      {
        "drive": "nvme1n1",
        "firmware": "EDFK0S03",
        "index": 1,
        "interface": "NVMe",
        "media": "SSD",
        "model": "KINGSTON OM8PDP3512B-AA1",
        "partition": [
          {
            "name": "0",
            "size": 407.14
          },
          {
            "name": "1",
            "size": 0.02
          },
          {
            "name": "2",
            "size": 96.64
          },
          {
            "name": "3",
            "size": 6.96
          },
          {
            "name": "4",
            "size": 0.27
          },
          {
            "name": "5",
            "size": 1.07
          }
        ],
        "partitions": 1,
        "serial": "50026B76857E7C0B",
        "size": 512,
        "status": "live"
      }
    ]
  },
  "general": {
    "boot_time": 1784508906,
    "host": "AcerLinux",
    "run_time": 11019
  },
  "gpu": [
    {
      "driver": "580.159.03",
      "frequency": 300,
      "max_clock": 1785,
      "name": "NVIDIA GeForce GTX 1650",
      "serial": "Unknown",
      "temperature": 46,
      "utilization": 0.0,
      "vram_total": 4294967296,
      "vram_usage": 3932160
    },
    {
      "driver": "3.64.0",
      "frequency": 1750,
      "max_clock": null,
      "name": "AMD Radeon Graphics",
      "serial": "Unknown",
      "temperature": 52,
      "utilization": 100.0,
      "vram_total": 536870912,
      "vram_usage": 470925312
    }
  ],
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
      "card": null,
      "config_speed": null,
      "download": 0,
      "ipv4": "172.19.0.1/16",
      "ipv6": "None",
      "mac": "02:14:fc:09:e9:62",
      "mtu": 1500,
      "name": "br-9e4b968fd9d1",
      "ssid": null,
      "upload": 0
    },
    {
      "card": null,
      "config_speed": null,
      "download": 126,
      "ipv4": "None",
      "ipv6": "fe80::bc4e:35ff:fe04:f7fb/64",
      "mac": "be:4e:35:04:f7:fb",
      "mtu": 1500,
      "name": "veth611b088",
      "ssid": null,
      "upload": 85620
    },
    {
      "card": "MEDIATEK Corp. MT7921 802.11ax PCI Express Wireless Network Adapter",
      "config_speed": null,
      "download": 646002219,
      "ipv4": "10.247.198.129/24",
      "ipv6": "2401:d800:28c1:1de5:9c70:356f:8f14:68f1/64, 2401:d800:fac0:ee37:f44c:306c:bd1e:1dac/64, fe80::9985:939:1da7:d03/64, 2401:d800:fac0:ee37:1f75:9fcc:96c2:2ede/64",
      "mac": "e0:0a:f6:be:44:9f",
      "mtu": 1500,
      "name": "wlp5s0",
      "ssid": "TECNO POVA 6 Neo",
      "upload": 47420766
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
    },
    {
      "card": null,
      "config_speed": null,
      "download": 84,
      "ipv4": "172.18.0.1/16",
      "ipv6": "fe80::8c4c:3dff:fe35:e59e/64",
      "mac": "8e:4c:3d:35:e5:9e",
      "mtu": 1500,
      "name": "br-b68dd5d7c3e7",
      "ssid": null,
      "upload": 84334
    },
    {
      "card": null,
      "config_speed": null,
      "download": 767259,
      "ipv4": "127.0.0.1/8",
      "ipv6": "::1/128",
      "mac": "00:00:00:00:00:00",
      "mtu": 65536,
      "name": "lo",
      "ssid": null,
      "upload": 767259
    },
    {
      "card": null,
      "config_speed": null,
      "download": 0,
      "ipv4": "172.17.0.1/16",
      "ipv6": "None",
      "mac": "92:54:73:cc:9a:21",
      "mtu": 1500,
      "name": "docker0",
      "ssid": null,
      "upload": 0
    }
  ],
  "os": {
    "kernel": "7.0.0-28-generic",
    "os_distro": "Ubuntu",
    "os_name": "linux",
    "os_version": "24.04"
  },
  "process_count": 1989,
  "ram": {
    "logical": {
      "total": 16078426112,
      "usage": 10210508800
    },
    "physical": [
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
    ]
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
    "usage": 1941360640
  },
  "time_stamp": 1784519926,
  "top_processes": [
    {
      "cpu": 68.12227630615234,
      "memory": 44089344,
      "name": "main_worker",
      "runtime": 1
    },
    {
      "cpu": 29.69432258605957,
      "memory": 558645248,
      "name": "chrome",
      "runtime": 9493
    },
    {
      "cpu": 17.467248916625977,
      "memory": 993333248,
      "name": "chrome",
      "runtime": 6499
    },
    {
      "cpu": 8.733624458312988,
      "memory": 351207424,
      "name": "code",
      "runtime": 9695
    },
    {
      "cpu": 5.240174770355225,
      "memory": 190201856,
      "name": "chrome",
      "runtime": 9736
    },
    {
      "cpu": 5.240174770355225,
      "memory": 993333248,
      "name": "Compositor",
      "runtime": 6499
    },
    {
      "cpu": 5.240174770355225,
      "memory": 558645248,
      "name": "Compositor",
      "runtime": 9493
    },
    {
      "cpu": 3.4934499263763428,
      "memory": 122519552,
      "name": "Hyprland",
      "runtime": 11006
    },
    {
      "cpu": 3.4934499263763428,
      "memory": 40214528,
      "name": "pw-data-loop",
      "runtime": 11006
    },
    {
      "cpu": 3.4934499263763428,
      "memory": 190201856,
      "name": "VizCompositorTh",
      "runtime": 9736
    }
  ]
}
```