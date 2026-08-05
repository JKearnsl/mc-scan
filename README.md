<div align="center">

# mc-scan

Minecraft server scanner — Java & Bedrock

<br/>

![Rust](https://img.shields.io/badge/rust-1.85+-f74c00?style=flat-square&logo=rust&logoColor=white)
![Platform](https://img.shields.io/badge/linux-x11%20%2F%20wayland-5c8dbf?style=flat-square&logo=linux&logoColor=white)
![Iced](https://img.shields.io/badge/iced-0.14-7c5cbf?style=flat-square)

<br/>

<!-- TODO: main window screenshot -->
<img src="assets/screenshot-main.png" width="780" alt="mc-scan main window"/>

</div>

<br/>

## Build

```sh
cargo build --release
```

## Usage

Enter targets in the sidebar, one per line — CIDR blocks, individual IPs, or ranges:

```
10.0.0.0/8
192.168.1.1
172.16.0.1-172.16.255.254
```

Ports, concurrency, and timeout are configurable in Settings.

<br/>

<div align="center">

<!-- TODO: server detail dialog screenshot -->
<img src="assets/screenshot-preview.png" width="520" alt="Server detail"/>

</div>
