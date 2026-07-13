```json
{
  "CPU": {
    "core": 12,
    "frequency": 2852, // MHz
    "name": "AMD Ryzen 5 5600H with Radeon Graphics",
    "temperature": 69.125, // ℃
    "usage": 9.785393714904785 // %, will not pass 100%
  },
  "GPUs": [
    {
      "driver": "580.159.03", // maybe this is for the "VERSION" on server
      "frequency": 300, // MHz
      "max_clock": 1785,  // MHz
      "name": "NVIDIA GeForce GTX 1650", // This is delicated GPU
      "temperature": 48, // ℃
      "utilization": 0.0, // %
      "vram_total": 4294967296, // byte
      "vram_usage": 13107200, // byte
      "serial": "todo", //TODO, string
    },
    {
      "driver": "3.64.0",
      "frequency": 400,
      "max_clock": null, // Correct for integrated GPU
      "name": "AMD Radeon Graphics", // This is integrated GPU
      "temperature": 54,
      "utilization": 100.0,
      "vram_total": 536870912,
      "vram_usage": 466296832,
      "serial": "todo", // Not exist for integrated GPU
    }
  ],
  "RAM": {
    "hardware": [
      {
        "name": "Kingston KVR32S22S6/8GB 3200",
        "serial": "40D9D109",
        "size": "8192 MB",
        "speed": "3200", // MT/s
        "type": "DDR4"
      },
      {
        "name": "Kingston ACR32D4S2S1ME-8",
        "serial": "25AB2457",
        "size": "8192 MB",
        "speed": "3200",
        "type": "DDR4"
      }
    ],
    "total": 16081768448, // byte
    "usage": 11251494912 // byte
  },
  "SWAP": {
    "total": 4294963200, // byte
    "usage": 1029337088 // byte
  },
  "battery": {
    "is_plugged_in": true, // bool, note that "plugged in" is different from "charging"
    "percentage": 80.0 //%
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
        "model": "CT500P3SSD8",
        "serial": "2404468C8B64",
        "size": 500 // GB
      },
      {
        "drive": "nvme1n1",
        "firmware": "EDFK0S03",
        "model": "KINGSTON OM8PDP3512B-AA1",
        "serial": "50026B76857E7C0B",
        "size": 512
      }
    ],
    "logical": [
      {
        "mount_point": "/", // Linux Root, will be C:\ on Windows
        "name": "/dev/nvme0n1p2", // will be C on Windows
        "removable": false, // bool
        "total": 489999179776, // byte
        "type": "ext4",
        "used": 353748967424 // byte
      },
      {
        "mount_point": "/boot/efi",
        "name": "/dev/nvme0n1p1",
        "removable": false,
        "total": 1124999168,
        "type": "vfat",
        "used": 6541312
      }
    ]
  },
  "general": {
    "boot_time": 1783470721, // second
    "run_time": 16666 // second
  },
  "machine": {
    "architecture": "x86_64",
    "machine_type": "Laptop: Notebook", // see *
    "model": "Nitro AN515-45",
    "producer": "Acer",
    "serial": "NHQBMSV0062131BB0D3400"
  },
  "motherboard": {
    "cpu_socket": 1, // available on motherboard, not mean currently in used
    "gpu_socket": 2, // available on motherboard, not mean currently in used
    "name": "Scala_CAS",
    "ram_socket": 2, // available on motherboard, not mean currently in used
    "serial": "NBQBM11001213B72373400",
    "tempe": 48.0  // ℃
  },
  "networks": [
    {
      "card": "MEDIATEK Corp. MT7921 802.11ax PCI Express Wireless Network Adapter",
      "config_speed": 0, // Mbps, will be 0 for Wifi
      "download": 1661197781, // byte
      "ipv4": "172.20.1.20/22",
      "ipv6": "fe80::7d79:4d55:fe01:574a/64",
      "mac": "e0:0a:f6:be:44:9f",
      "mtu": 1500, // byte
      "name": "wlp5s0", // Wifi
      "ssid": "GGROUP-LAU2",
      "upload": 93933647 // byte
    },
    {
      "card": "Unknown", // correct for loopback
      "config_speed": 0, // correct for loopback
      "download": 2235954,
      "ipv4": "127.0.0.1/8",
      "ipv6": "::1/128", // correct for loopback
      "mac": "00:00:00:00:00:00", // correct for loopback
      "mtu": 65536,
      "name": "lo", // loopback
      "ssid": "Unknown", // correct for loopback
      "upload": 2235954
    },
    {
      "card": "Realtek Semiconductor Co., Ltd. Killer E2600 GbE Controller (rev 21)",
      "config_speed": 0, // will be likely 1000 when plugged-in
      "download": 0,
      "ipv4": "None", // only show when plugged-in
      "ipv6": "None", // only show when plugged-in
      "mac": "08:8f:c3:56:4e:01",
      "mtu": 1500,
      "name": "enp4s0", // Ethernet
      "ssid": "Unknown", // correct for Ethernet
      "upload": 0
    }
  ],
  "os": {
    "kernel": "6.17.0-35-generic",
    "os": "Ubuntu",
    "os_name": "linux",
    "os_version": "24.04"
  },
  "softwares": [
    {
      "expiration": "todo",
      "license": "todo",
      "name": "2to3",
      "source": "Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>",
      "version": "3.12.3-0ubuntu2.1"
    },
    {
      "expiration": "todo",
      "license": "todo",
      "name": "7zip",
      "source": "Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>",
      "version": "23.01+dfsg-11"
    },
    {
      "expiration": "todo",
      "license": "todo",
      "name": "aardvark-dns",
      "source": "Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>",
      "version": "1.4.0-5"
    },
    {
      "expiration": "todo",
      "license": "todo",
      "name": "accountsservice",
      "source": "Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>",
      "version": "23.13.9-2ubuntu6"
    },
    {
      "expiration": "todo",
      "license": "todo",
      "name": "acl",
      "source": "Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>",
      "version": "2.3.2-1build1.1"
    }
  ],
  "time_stamp": 1783487387, // second
  "all_processes": [], // just like top_processes, but show all
  "top_processes": [ // top 10 (configurable) running processes
    {
      "cpu": 29.019859313964844, // %, may go pass 100% if program run multi-core
      "memory": 476786688, // byte
      "name": "chrome",
      "runtime": 11684 // second
    },
    {
      "cpu": 21.5246639251709,
      "memory": 846573568,
      "name": "chrome",
      "runtime": 11675
    },
    {
      "cpu": 19.02626609802246,
      "memory": 1074249728,
      "name": "chrome",
      "runtime": 11704
    },
    {
      "cpu": 16.9122371673584,
      "memory": 757420032,
      "name": "code",
      "runtime": 13722
    },
    {
      "cpu": 9.224855422973633,
      "memory": 603045888,
      "name": "brave",
      "runtime": 15591
    },
    {
      "cpu": 6.918642044067383,
      "memory": 757420032,
      "name": "DedicatedWorker",
      "runtime": 9787
    },
    {
      "cpu": 5.188981533050537,
      "memory": 180060160,
      "name": "chrome",
      "runtime": 16649
    },
    {
      "cpu": 4.420243740081787,
      "memory": 476786688,
      "name": "Compositor",
      "runtime": 11684
    },
    {
      "cpu": 4.035874366760254,
      "memory": 1074249728,
      "name": "Compositor",
      "runtime": 11704
    },
    {
      "cpu": 4.035874366760254,
      "memory": 180060160,
      "name": "VizCompositorTh",
      "runtime": 16649
    }
  ],
  "peripheral": [] //TODO
}
```