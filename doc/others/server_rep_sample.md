```json
{
  "result": {
    "result": "1",
    "errorDesc": null,
    "cmds": [
      {
        "cmd": "AskFullScan"
      },
      {
        "cmd": "UpdateConfig",
        "payload": {
            {
            "machine.model": [0, "test"],
            "machine.type": [0, null],
            "motherboard.tempe": [30, 5],
            "cpu.temperature": [30, 5],
            "ram.usage": [30, 1024],
            "battery.is_plugged_in": [1800, null],
            "disk.logical.used": [30, 1024],
            }
        }
      },
      {
        "cmd": "UpdateAgent",
        "payload": {
          "binary": "main_worker",
          "version": "1.0.0",
          "signature": "NjWz797ntYSLFwg7nKYYdn+On3cCMj4zKz059wadVVlBHxyxe4JrSZxgekwb9AYR5xFxuVE9dTnDSo+gCaW/CQ==",
        }
      },
      {
        "cmd": "LogLevel",
        "payload": {
          "level": "debug"
        }
      },
      {
        "cmd": "AskSendLog",
      },
    ]
  },
  "targetUrl": null,
  "success": true,
  "error": null,
  "unAuthorizedRequest": false,
  "__abp": true
}
```