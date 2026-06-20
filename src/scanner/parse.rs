use std::net::{IpAddr, Ipv4Addr};
use ipnet::IpNet;

pub fn parse_ports(input: &str) -> Vec<u16> {
    input.split(',').filter_map(|s| s.trim().parse::<u16>().ok()).collect()
}

pub fn parse_ip_ranges(input: &str) -> Vec<IpNet> {
    let mut result = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(net) = line.parse::<IpNet>() {
            result.push(net);
            continue;
        }
        if let Ok(ip) = line.parse::<IpAddr>() {
            let prefix = if ip.is_ipv4() { 32 } else { 128 };
            if let Ok(net) = IpNet::new(ip, prefix) {
                result.push(net);
            }
            continue;
        }
        if let Some((start_str, end_str)) = line.split_once('-') {
            if let (Ok(start), Ok(end)) = (
                start_str.trim().parse::<Ipv4Addr>(),
                end_str.trim().parse::<Ipv4Addr>(),
            ) {
                result.extend(range_to_cidrs(start, end));
            }
        }
    }
    result
}

fn range_to_cidrs(start: Ipv4Addr, end: Ipv4Addr) -> Vec<IpNet> {
    let mut result = Vec::new();
    let mut s = u32::from(start);
    let e = u32::from(end);
    if s > e {
        return result;
    }
    while s <= e {
        let trailing = if s == 0 { 32u32 } else { s.trailing_zeros() };
        let mut prefix = (32u32 - trailing.min(32)) as u8;
        loop {
            let block_size = 1u64 << (32 - prefix);
            let block_end = s as u64 + block_size - 1;
            if block_end <= e as u64 {
                break;
            }
            prefix += 1;
        }
        result.push(IpNet::new(IpAddr::V4(Ipv4Addr::from(s)), prefix).unwrap());
        let next = s as u64 + (1u64 << (32 - prefix));
        if next > u32::MAX as u64 {
            break;
        }
        s = next as u32;
    }
    result
}
