<div align="center">

# mc-scan

Minecraft server scanner — Java & Bedrock

<br/>

![Rust](https://img.shields.io/badge/rust-1.85+-f74c00?style=flat-square&logo=rust&logoColor=white)
![Platform](https://img.shields.io/badge/linux-x11%20%2F%20wayland-5c8dbf?style=flat-square&logo=linux&logoColor=white)
![Iced](https://img.shields.io/badge/iced-0.14-7c5cbf?style=flat-square)
![License](https://img.shields.io/badge/license-Apache--2.0-4c9c6b?style=flat-square)

<br/>

<img src="docs/hero-themes.png" width="700" alt="mc-scan main window — light and dark themes"/>

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

<img src="docs/hero-preview.png" width="700" alt="Server detail"/>

</div>

<br/>

## License

Licensed under the [Apache License 2.0](LICENSE). You are free to use, modify
and redistribute it, including commercially, as long as you keep the copyright
notice and state your changes.

The **mc-scan** name and logo are not covered by the license (see [NOTICE](NOTICE)):
fork freely, but rebrand your fork with your own name and icon.

