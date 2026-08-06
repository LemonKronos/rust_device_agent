```json
{
  "AGENT_VERSION": "0.3.0",
  "BATTERY": {
    "PERCENT": 100,
    "POWER_PLUGGED": true
  },
  "BIOS": {
    "SECURE_BOOT": false,
    "VENDOR": "Insyde Corp.",
    "VERSION": "24.04"
  },
  "CPU": {
    "CPU_NAME": "AMD Ryzen 5 5600H with Radeon Graphics",
    "CORES": 12,
    "SPEED": 3426,
    "CPU_USAGE_RATE": 11.80124282836914,
    "TEMPERATURE": 66.875
  },
  "DISK": {
    "PHYSICAL_DISK_Lst": [
      {
        "DISK_NAME": "nvme0n1",
        "FIRMWARE": "EDFK0S03",
        "PHYSICAL_DISK_INDEX": 0,
        "INTERFACE_TYPE": "NVMe",
        "MEDIA_TYPE": "SSD",
        "MODEL": "KINGSTON OM8PDP3512B-AA1",
        "NUMBER_OF_PARTITION": 6,
        "PARTITION_Lst": [
          {
            "PARTITION_INDEX": "0",
            "SIZE": 96.64
          },
          {
            "PARTITION_INDEX": "1",
            "SIZE": 6.96
          },
          {
            "PARTITION_INDEX": "2",
            "SIZE": 0.27
          },
          {
            "PARTITION_INDEX": "3",
            "SIZE": 1.07
          },
          {
            "PARTITION_INDEX": "4",
            "SIZE": 407.14
          },
          {
            "PARTITION_INDEX": "5",
            "SIZE": 0.02
          }
        ],
        "SERIAL_NUMBER": "50026B76857E7C0B",
        "SIZE": 512,
        "PHYSICAL_DISK_STATUS": "live"
      },
      {
        "DISK_NAME": "nvme1n1",
        "MODEL": "CT500P3SSD8",
        "INTERFACE_TYPE": "NVMe",
        "MEDIA_TYPE": "SSD",
        "FIRMWARE": "P9CR313",
        "PHYSICAL_DISK_INDEX": 1,
        "NUMBER_PARTITION": 2,
        "PARTITION_Lst": [
          {
            "PARTITION_INDEX": "0",
            "SIZE": 498.98
          },
          {
            "PARTITION_INDEX": "1",
            "SIZE": 1.13
          }
        ],
        "SERIAL_NUMBER": "2404468C8B64",
        "SIZE": 500,
        "PHYSICAL_DISK_STATUS": "live"
      }
    ],
    "LOGICAL_DISK_Lst": [
      {
        "FILE_SYSTEM_TYPE": "ext4",
        "MOUNT_POINT": "/",
        "LOGICAL_DISK_NAME": "/dev/nvme1n1p2",
        "REMOVABLE": false,
        "TOTAL": 489999179776,
        "USAGE": 372342898688
      },
      {
        "FILE_SYSTEM_TYPE": "vfat",
        "MOUNT_POINT": "/boot/efi",
        "LOGICAL_DISK_NAME": "/dev/nvme1n1p1",
        "REMOVABLE": false,
        "TOTAL": 1124999168,
        "USAGE": 6541312
      },
      {
        "FILE_SYSTEM_TYPE": "overlay",
        "MOUNT_POINT": "/var/lib/docker/overlay2/06ec881ff472e8082c5f0fa3d54ec8b06cd5fce0ff6104898e63439f0e84499b/merged",
        "LOGICAL_DISK_NAME": "overlay",
        "REMOVABLE": false,
        "TOTAL": 489999179776,
        "USAGE": 372342898688
      },
      {
        "FILE_SYSTEM_TYPE": "overlay",
        "MOUNT_POINT": "/var/lib/docker/overlay2/6981e29be77e7afdaf5631f656f54f7fc5588cd499bdb790761e72de2cc0d37a/merged",
        "LOGICAL_DISK_NAME": "overlay",
        "REMOVABLE": false,
        "TOTAL": 489999179776,
        "USAGE": 372342898688
      }
    ]
  },
  "GENERAL": {
    "BOOT_TIME_EPOCH": 1785199967,
    "FULL_COMPUTER_NAME": "AcerLinux",
    "RUN_TIME": 10817
  },
  "GPU_Lst": [
    {
      "DRIVER_VERSION": "580.173.02",
      "FREQUENCY": 300,
      "MAX_CLOCK_SPEED": 1785,
      "GPU_NAME": "NVIDIA GeForce GTX 1650",
      "SERIAL": "Unknown",
      "TEMPERATURE": 45,
      "UTILIZATION": 0.0,
      "VRAM_TOTAL": 4294967296,
      "VRAM_USAGE": 107610112
    },
    {
      "DRIVER_VERSION": "3.64.0",
      "FREQUENCY": 400,
      "MAX_CLOCK_SPEED": null,
      "GPU_NAME": "AMD Radeon Graphics",
      "SERIAL": "Unknown",
      "TEMPERATURE": 51,
      "UTILIZATION": 100.0,
      "VRAM_TOTAL": 536870912,
      "VRAM_USAGE": 493096960
    }
  ],
  "MACHINE": {
    "SYSTEM_TYPE": "x86_64",
    "SYSTEM_MODEL": "Nitro AN515-45",
    "PRODUCER": "Acer",
    "SERIAL_NUMBER": "NHQBMSV0062131BB0D3400",
    "COMPUTER_TYPE": "Laptop: Notebook"
  },
  "MOTHER_BOARD": {
    "NAME": "Scala_CAS",
    "SERIAL_NUMBER": "NBQBM11001213B72373400",
    "CPU_SLOT": 1,
    "GPU_SLOT": 2,
    "RAM_SLOT": 2,
    "TEMPERATURE": 44.0
  },
  "NETWORK_Lst": [
    {
      "INET_CARD": "Realtek Semiconductor Co., Ltd. Killer E2600 GbE Controller (rev 21)",
      "INET_CARD_SPEED": null,
      "DOWNLOAD": 0,
      "IPV4": "None",
      "IPV6": "None",
      "MAC_ADDRESS": "08:8f:c3:56:4e:01",
      "MTU": 1500,
      "NAME": "enp4s0",
      "SSID": null,
      "UPLOAD": 0
    },
    {
      "INET_CARD": null,
      "INET_CARD_SPEED": null,
      "DOWNLOAD": 126,
      "IPV4": "None",
      "IPV6": "fe80::67:8ff:fe39:4281/64",
      "MAC_ADDRESS": "02:67:08:39:42:81",
      "MTU": 1500,
      "NAME": "veth84d791d",
      "SSID": null,
      "UPLOAD": 1866
    },
   
  ],
  "OS": {
    "DISTRO": "Ubuntu",
    "KERNEL": "7.0.0-28-generic",
    "NAME": "linux",
    "VERSION": "24.04"
  },
  "PROCESS_COUNT": 1937,
  "RAM": {
    "DETAILS": [
      {
        "BANK_LABEL": "P0 CHANNEL A",
        "CAPACITY": 8192,
        "SPEED": "3200",
        "RAM_TYPE": "DDR4",
        "FORM_FACTOR": "SODIMM",
        "MANUFACTURER": "Kingston KVR32S22S6/8GB 3200",
        "SERIAL_NUMBER": "40D9D109"
      },
      {
        "BANK_LABEL": "P0 CHANNEL B",
        "CAPACITY": 8192,
        "SPEED": "3200",
        "RAM_TYPE": "DDR4",
        "FORM_FACTOR": "SODIMM",
        "MANUFACTURER": "Kingston ACR32D4S2S1ME-8",
        "SERIAL_NUMBER": "25AB2457"
      }
    ],
    "TOTAL": 16078426112,
    "USAGE": 8773693440
  },
  "SOFTWARE": [
    {
      "SOFTWARE_NAME": "2to3", 
      "VERSION": "3.12.3-0ubuntu2.1", 
      "PUBLISHER": "Ubuntu Developers", 
      "INSTALL_DATE": 1763425716, 
      "SIZE": 32.0
    },
    {
      "SOFTWARE_NAME": "7zip", 
      "VERSION": "23.01+dfsg-11", 
      "PUBLISHER": "Ubuntu Developers", 
      "INSTALL_DATE": 1742312014, 
      "SIZE": 6158.0
    },
    {
      "SOFTWARE_NAME": "aardvark-dns", 
      "VERSION": "1.4.0-5", 
      "PUBLISHER": "Ubuntu Developers", 
      "INSTALL_DATE": 1782108774, 
      "SIZE": 2510.0
    },
    {
      "SOFTWARE_NAME": "accountsservice", 
      "VERSION": "23.13.9-2ubuntu6.1", 
      "PUBLISHER": "Ubuntu Developers", 
      "INSTALL_DATE": 1784770780, 
      "SIZE": 524.0
    },
    {
      "SOFTWARE_NAME": "acl", 
      "VERSION": "2.3.2-1build1.1",
      "PUBLISHER": "Ubuntu Developers", 
      "INSTALL_DATE": 1746454674, "SIZE": 192.0
    }
  ],
  "swap": {
    "total": 4294963200,
    "usage": 4030664704
  },
  "time_stamp": 1785210785,
  "top_process": [
    {
      "cpu": 48.44720458984375,
      "memory": 42385408,
      "name": "main_worker",
      "runtime": 1
    },
    {
      "cpu": 37.2670783996582,
      "memory": 638259200,
      "name": "chrome",
      "runtime": 10728
    },
    {
      "cpu": 33.54037094116211,
      "memory": 533819392,
      "name": "chrome",
      "runtime": 10696
    },
    {
      "cpu": 29.81366539001465,
      "memory": 877764608,
      "name": "chrome",
      "runtime": 10709
    },
    {
      "cpu": 22.360246658325195,
      "memory": 560287744,
      "name": "code",
      "runtime": 10402
    },
    {
      "cpu": 14.906832695007324,
      "memory": 430891008,
      "name": "chrome",
      "runtime": 10690
    },
    {
      "cpu": 11.180123329162598,
      "memory": 78905344,
      "name": "VizCompositorTh",
      "runtime": 10728
    },
    {
      "cpu": 11.180123329162598,
      "memory": 78905344,
      "name": "chrome",
      "runtime": 10728
    },
    {
      "cpu": 7.453416347503662,
      "memory": 560275456,
      "name": "Compositor",
      "runtime": 10402
    },
    {
      "cpu": 7.453416347503662,
      "memory": 638259200,
      "name": "Compositor",
      "runtime": 10728
    }
  ]
}
```