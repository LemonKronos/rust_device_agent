```json
{
  "CPU": {
    "core": 12,
    "frequency": 3575,
    "name": "AMD Ryzen 5 5600H with Radeon Graphics",
    "temperature": 76.25,
    "usage": 57.54060745239258
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
      "vram_usage": 3932160
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
      "vram_usage": 497922048
    }
  ],
  "RAM": {
    "hardware": null,
    "total": 16078426112,
    "usage": 9503617024
  },
  "SWAP": {
    "total": 4294963200,
    "usage": 3590787072
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
    ],
    "logical": [
      {
        "file_system": "ext4",
        "mount_point": "/",
        "name": "/dev/nvme0n1p2",
        "removable": false,
        "total": 489999179776,
        "used": 359769608192
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
        "total": 0,
        "used": 0
      },
      {
        "file_system": "overlay",
        "mount_point": "/var/lib/docker/overlay2/06ec881ff472e8082c5f0fa3d54ec8b06cd5fce0ff6104898e63439f0e84499b/merged",
        "name": "overlay",
        "removable": false,
        "total": 0,
        "used": 0
      },
      {
        "file_system": "ntfs3",
        "mount_point": "/media/01DD05DDDC2FE2E0",
        "name": "/dev/nvme1n1p3",
        "removable": false,
        "total": 6964637696,
        "used": 45490176
      }
    ]
  },
  "general": {
    "boot_time": 1784508907,
    "host": "AcerLinux",
    "run_time": 22567
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
      "download": 1268391,
      "ipv4": "127.0.0.1/8",
      "ipv6": "::1/128",
      "mac": "00:00:00:00:00:00",
      "mtu": 65536,
      "name": "lo",
      "ssid": null,
      "upload": 1268391
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
      "upload": 138408
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
      "upload": 136982
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
      "card": "MEDIATEK Corp. MT7921 802.11ax PCI Express Wireless Network Adapter",
      "config_speed": null,
      "download": 669699123,
      "ipv4": "172.20.1.20/22",
      "ipv6": "fe80::515a:3399:7f4f:c499/64",
      "mac": "e0:0a:f6:be:44:9f",
      "mtu": 1500,
      "name": "wlp5s0",
      "ssid": "GGROUP-LAU1",
      "upload": 62410150
    }
  ],
  "os": {
    "kernel": "7.0.0-28-generic",
    "os_distro": "Ubuntu",
    "os_name": "linux",
    "os_version": "24.04"
  },
  "process_count": 2025,
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
  "time_stamp": 1784531475,
  "top_processes": [
    {
      "cpu": 0.0,
      "memory": 7491584,
      "name": "xdg-document-po",
      "runtime": 22551
    },
    {
      "cpu": 0.0,
      "memory": 105070592,
      "name": "ThreadPoolServi",
      "runtime": 12
    },
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "kworker/R-comp_1.0.0",
      "runtime": 22561
    },
    {
      "cpu": 0.0,
      "memory": 620318720,
      "name": "ThreadPoolForeg",
      "runtime": 21272
    },
    {
      "cpu": 0.0,
      "memory": 189374464,
      "name": "ImageIO",
      "runtime": 22548
    },
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "kworker/u48:8-sdma0",
      "runtime": 7139
    },
    {
      "cpu": 0.0,
      "memory": 183549952,
      "name": "chrome:disk$2",
      "runtime": 21283
    },
    {
      "cpu": 0.0,
      "memory": 0,
      "name": "kworker/R-kvfree_rcu_reclaim",
      "runtime": 22566
    },
    {
      "cpu": 0.0,
      "memory": 123928576,
      "name": "ThreadPoolForeg",
      "runtime": 21241
    },
    {
      "cpu": 0.0,
      "memory": 21196800,
      "name": "fcitx5",
      "runtime": 22551
    }
  ]
}
```