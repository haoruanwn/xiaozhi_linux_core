测试与服务端链接，发送hello信息，连通了

```bash
Loaded Client ID from file: 83a91c5f-f3e9-40bf-b37b-c3580b105423
Checking activation status via HTTP: https://api.tenclass.net/xiaozhi/ota/
Device is activated. Starting WebSocket...
Xiaozhi Core Started. State: Idle
Connecting to wss://api.tenclass.net/xiaozhi/v1/...
Headers: {"host": "api.tenclass.net", "connection": "Upgrade", "upgrade": "websocket", "sec-websocket-version": "13", "sec-websocket-key": "QxVVpjQ0+oJHkbSy/CcoUA==", "authorization": "Bearer test-token", "device-id": "6c:1f:f7:22:84:a2", "client-id": "83a91c5f-f3e9-40bf-b37b-c3580b105423", "protocol-version": "1"}
Connected!
Sending Hello: {
            "type": "hello",
            "version": 1,
            "transport": "websocket",
            "audio_params": {
                "format": "opus",
                "sample_rate": 16000,
                "channels": 1,
                "frame_duration": 60
            }
        }
WebSocket Connected
Received Text from Server: {"type":"hello","version":1,"transport":"websocket","audio_params":{"format":"opus","sample_rate":24000,"channels":1,"frame_duration":60},"session_id":"f307991a"}

hao in 🌐 fedora in xiaozhi_linux_core on  main [!?] is 📦 v0.1.0 via 🦀 v1.91.1 took 16s 
❯ 
```

## Hardcoded URLs found in C++ source

Based on analysis of `xiaozhi-linux/control_center/control_center.cpp`:

1.  **Device Activation / OTA (HTTP POST)**
    *   URL: `https://api.tenclass.net/xiaozhi/ota/`
    *   Found in: `control_center.cpp` (line 486)

2.  **WebSocket Connection (WSS)**
    *   Hostname: `api.tenclass.net`
    *   Port: `443`
    *   Path: `/xiaozhi/v1/`
    *   Full URL: `wss://api.tenclass.net:443/xiaozhi/v1/`
    *   Found in: `control_center.cpp` (lines 523-525)


交叉编译
```bash
# 例如，编译为 armv7 musleabihf 目标 (静态链接)
cross build \
   --target=armv7-unknown-linux-musleabihf \
   --release \
   --config 'target.armv7-unknown-linux-musleabihf.rustflags=["-C", "target-feature=+crt-static"]'
```