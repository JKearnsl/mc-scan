use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
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
            match (start_str.trim().parse::<IpAddr>(), end_str.trim().parse::<IpAddr>()) {
                (Ok(IpAddr::V4(a)), Ok(IpAddr::V4(b))) => {
                    result.extend(range_to_cidrs(u32::from(a) as u128, u32::from(b) as u128, 32, |v| {
                        IpAddr::V4(Ipv4Addr::from(v as u32))
                    }));
                }
                (Ok(IpAddr::V6(a)), Ok(IpAddr::V6(b))) => {
                    result.extend(range_to_cidrs(u128::from(a), u128::from(b), 128, |v| {
                        IpAddr::V6(Ipv6Addr::from(v))
                    }));
                }
                _ => {}
            }
        }
    }
    result
}

fn range_to_cidrs(start: u128, end: u128, bits: u32, to_addr: impl Fn(u128) -> IpAddr) -> Vec<IpNet> {
    let mut result = Vec::new();
    if start > end {
        return result;
    }
    let mut s = start;
    loop {
        let mut host_bits = if s == 0 { bits } else { s.trailing_zeros().min(bits) };
        while host_bits > 0 && !matches!(block_end(s, host_bits), Some(e) if e <= end) {
            host_bits -= 1;
        }
        result.push(IpNet::new(to_addr(s), (bits - host_bits) as u8).unwrap());
        match next_start(s, host_bits) {
            Some(next) if next <= end => s = next,
            _ => break,
        }
    }
    result
}

fn block_end(s: u128, host_bits: u32) -> Option<u128> {
    if host_bits >= 128 {
        Some(u128::MAX)
    } else {
        s.checked_add((1u128 << host_bits) - 1)
    }
}

fn next_start(s: u128, host_bits: u32) -> Option<u128> {
    if host_bits >= 128 {
        None
    } else {
        s.checked_add(1u128 << host_bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nets(input: &str) -> Vec<IpNet> {
        parse_ip_ranges(input)
    }

    #[test]
    fn ipv4_range_splits_into_cidrs() {
        assert_eq!(nets("10.0.0.0 - 10.0.0.3"), vec!["10.0.0.0/30".parse().unwrap()]);
        assert_eq!(nets("10.0.0.1-10.0.0.2").len(), 2);
    }

    #[test]
    fn ipv6_range_splits_into_cidrs() {
        assert_eq!(nets("2001:db8::0-2001:db8::3"), vec!["2001:db8::/126".parse().unwrap()]);
    }

    #[test]
    fn mixed_family_or_reversed_range_yields_nothing() {
        assert!(nets("10.0.0.0-2001:db8::1").is_empty());
        assert!(nets("10.0.0.5-10.0.0.0").is_empty());
    }
}
