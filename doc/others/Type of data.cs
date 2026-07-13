 public class ASS_IT_SCAN_DATA_ENTITY
    {
        [JsonProperty("OPERATING_SYSTEM")]
        public string OPERATING_SYSTEM { get; set; }
        [JsonProperty("BATTERY")]
        public BATTERY_INFO BATTERY { get; set; }
        [JsonProperty("BOOT_TIME")]
        public string BOOT_TIME { get; set; }
        [JsonProperty("BOOT_TIME_EPOCH")]
        public long? BOOT_TIME_EPOCH { get; set;}
        [JsonProperty("COMPUTER_NAME")]
        public string COMPUTER_NAME { get; set; }
        [JsonProperty("CPU")]
        public CPU_INFO CPU { get; set; }
        [JsonProperty("GPU_Lst")]
        public List<GPU_INFO> GPU_Lst { get; set; }
        [JsonProperty("HARD_DISK_Lst")]
        public List<HARD_DISK_INFO> HARD_DISK_Lst { get; set; }
        [JsonProperty("RAM")]
        public RAM_INFO RAM { get; set; }
        [JsonProperty("NETWORK")]
        public NETWORK_INFO NETWORK { get; set; }
        [JsonProperty("NETWORK_Lst")]
        public List<NETWORK_INFO> NETWORK_Lst { get; set; }
        [JsonProperty("PRODUCER")]
        public string PRODUCER { get; set; }
        [JsonProperty("PHYSICAL_DISK_Lst")]
        public List<PHYSICAL_DISK_INFO> PHYSICAL_DISK_Lst { get; set; }
        [JsonProperty("SYSTEM_MODEL")]
        public string SYSTEM_MODEL { get; set; }
        [JsonProperty("BIOS_VERSION")]
        public string BIOS_VERSION { get; set; }
        [JsonProperty("SOFTWARE_Lst")]
        public List<SOFTWARE_INFO> SOFTWARE_Lst { get; set; }
        [JsonProperty("SYSTEM_TYPE")]
        public string SYSTEM_TYPE { get; set; }
        [JsonProperty("MOTHER_BOARD")]
        public string MOTHER_BOARD { get; set; }
        [JsonProperty("COMPUTER_TYPE")]
        public string COMPUTER_TYPE { get; set; }
        [JsonProperty("SERIAL_NUMBER")]
        public string SERIAL_NUMBER { get; set; }
        public string FULL_COMPUTER_NAME { get; set; }
        public string ASSET_CODE { get; set; }
        public string Type { get; set; }
    }

    public class BATTERY_INFO
    {
        [JsonProperty("PERCENT")]
        public string PERCENT { get; set; }
        [JsonProperty("POWER_PLUGGED")]
        public bool POWER_PLUGGED { get; set; }
    }

    public class CPU_INFO
    {
        [JsonProperty("CORES")]
        public int CORES { get; set; }
        [JsonProperty("CPU_USAGE_RATE")]
        public decimal CPU_USAGE_RATE { get; set; }
        [JsonProperty("CPU_NAME")]
        public string CPU_NAME { get; set; }
        [JsonProperty("LOGICAL_PROCESSORS")]
        public int LOGICAL_PROCESSORS { get; set; }
        [JsonProperty("SPEED")]
        public int SPEED { get; set; }
        [JsonProperty("PROCESSES")]
        public int PROCESSES { get; set; }
        [JsonProperty("SOCKETS")]
        public int SOCKETS { get; set; }
        //f"{days}:{hours:02}:{minutes:02}:{seconds:02}"
        [JsonProperty("UPTIME")]
        public string UPTIME { get; set; }
        [JsonProperty("CPU_DESCRIPTION")]
        public string CPU_DESCRIPTION { get; set; }
        [JsonProperty("MACHINE")]
        public string MACHINE { get; set; }
        [JsonProperty("TEMPERATURE")]
        public string TEMPERATURE { get; set; }
    }

    public class GPU_INFO
    {
        [JsonProperty("GPU_NAME")]
        public string GPU_NAME { get; set; }

        [JsonProperty("GLOBAL_MEMORY")]
        public string GLOBAL_MEMORY { get; set; }

        [JsonProperty("VERSION")]
        public string VERSION { get; set; }

        [JsonProperty("DRIVER_VERSION")]
        public string DRIVER_VERSION { get; set; }

        [JsonProperty("COMPUTE_UNITS")]
        public string COMPUTE_UNITS { get; set; }

        [JsonProperty("MAX_CLOCK_SPEED")]
        public string MAX_CLOCK_SPEED { get; set; }

        [JsonProperty("VENDOR")]
        public string VENDOR { get; set; }
        [JsonProperty("TEMPERATURE")]
        public string TEMPERATURE { get; set; }
    }

    public class HARD_DISK_INFO
    {
        [JsonProperty("HARD_DISK_NAME")]
        public string HARD_DISK_NAME { get; set; }
        [JsonProperty("TOTAL")]
        public decimal TOTAL { get; set; }
        [JsonProperty("USAGE")]
        public decimal USAGE { get; set; }
        [JsonProperty("FREE")]
        public decimal FREE { get; set; }
        [JsonProperty("PERCENT")]
        public decimal PERCENT { get; set; }
        [JsonProperty("FILE_SYSTEM_TYPE")]
        public string FILE_SYSTEM_TYPE { get; set; }
        [JsonProperty("MOUNT_POINT")]
        public string MOUNT_POINT { get; set; }
    }

    public class RAM_INFO
    {
        [JsonProperty("TOTAL")]
        public decimal TOTAL { get; set; }
        [JsonProperty("USAGE")]
        public decimal USAGE { get; set; }
        [JsonProperty("PERCENTAGE")]
        public decimal PERCENTAGE { get; set; }
        [JsonProperty("DETAILS")]
        public List<RAM_DETAILS> DETAILS { get; set; }
    }

    public class RAM_DETAILS
    {
        [JsonProperty("BANK_LABEL")]
        public string BANK_LABEL { get; set; }
        [JsonProperty("CAPACITY")]
        public decimal CAPACITY { get; set; }
        [JsonProperty("SPEED")]
        public string SPEED { get; set; }
        [JsonProperty("RAM_TYPE")]
        public string RAM_TYPE { get; set; }
        [JsonProperty("FORM_FACTOR")]
        public string FORM_FACTOR { get; set; }
        [JsonProperty("MANUFACTURER")]
        public string MANUFACTURER { get; set; }
        [JsonProperty("SERIAL_NUMBER")]
        public string SERIAL_NUMBER { get; set; }
    }

    public class NETWORK_INFO
    {
        [JsonProperty("IP")]
        public string IP { get; set; }
        [JsonProperty("MAC_ADDRESS")]
        public string MAC_ADDRESS { get; set; }
        [JsonProperty("CONNECTION_TYPE")]
        public string CONNECTION_TYPE { get; set; }
        [JsonProperty("SSID")]
        public string SSID { get; set; }
        [JsonProperty("INET_CARD")]
        public string INET_CARD { get; set; }
        [JsonProperty("INET_CARD_SPEED")]
        public int INET_CARD_SPEED { get; set; }
    }

    public class PHYSICAL_DISK_INFO
    {
        [JsonProperty("DISK_NAME")]
        public string DISK_NAME { get; set; }
        [JsonProperty("MODEL")]
        public string MODEL { get; set; }
        [JsonProperty("PARTITION_Lst")]
        public List<PARTITIONS_INFO> PARTITION_Lst { get; set; }
        [JsonProperty("SIZE")]
        public decimal SIZE { get; set; }
        [JsonProperty("SERIAL_NUMBER")]
        public string SERIAL_NUMBER { get; set; }
        [JsonProperty("INTERFACE_TYPE")]
        public string INTERFACE_TYPE { get; set; }
        [JsonProperty("MEDIA_TYPE")]
        public string MEDIA_TYPE { get; set; }
        [JsonProperty("NUMBER_PARTITIONS")]
        public int NUMBER_PARTITIONS { get; set; }
        [JsonProperty("PHYSICAL_DISK_STATUS")]
        public string PHYSICAL_DISK_STATUS { get; set; }
        [JsonProperty("FIRMWARE")]
        public string FIRMWARE { get; set; }
        [JsonProperty("PHYSICAL_DISK_INDEX")]
        public string PHYSICAL_DISK_INDEX { get; set; }
    }

    public class PARTITIONS_INFO
    {
        [JsonProperty("PARTITION_INDEX")]
        public string PARTITION_INDEX { get; set; }
        [JsonProperty("SIZE")]
        public decimal SIZE { get; set; }
        public string DISK_NAME { get; set; }
    }

    public class SOFTWARE_INFO
    {
        [JsonProperty("SOFTWARE_NAME")]
        public string SOFTWARE_NAME { get; set; }
        [JsonProperty("VERSION")]
        public string VERSION { get; set; }
        [JsonProperty("PUBLISHER")]
        public string PUBLISHER { get; set; }
        [JsonProperty("INSTALL_DATE")]
        public string INSTALL_DATE { get; set; }
        [JsonProperty("SIZE")]
        public string SIZE { get; set; }
    }

    public class AGENT_INGEST_REQUEST
    {
        public ASS_IT_SCAN_DATA_ENTITY SCAN_DATA { get; set;} 
        public List<string> MAC_LIST { get; set; }
        public string AGENT_VERSION { get; set; }
        
    }