use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Concurrency(usize);

impl Concurrency {
    pub const MIN: usize = 1;
    // Kept under the fd limit raised in main, with headroom for the window/GPU.
    pub const MAX: usize = 8192;
    pub const DEFAULT: usize = 1024;

    pub fn from_input(raw: &str) -> Self {
        Self(
            raw.trim()
                .parse()
                .unwrap_or(Self::DEFAULT)
                .clamp(Self::MIN, Self::MAX),
        )
    }

    pub fn get(self) -> usize {
        self.0
    }
}

impl Default for Concurrency {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeoutMs(u64);

impl TimeoutMs {
    pub const MIN: u64 = 100;
    pub const DEFAULT: u64 = 1500;

    pub fn from_input(raw: &str) -> Self {
        Self(raw.trim().parse().unwrap_or(Self::DEFAULT).max(Self::MIN))
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

impl Default for TimeoutMs {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Ports(Vec<u16>);

impl Ports {
    pub fn from_input(raw: &str) -> Self {
        Self(super::parse::parse_ports(raw))
    }
}

impl Deref for Ports {
    type Target = [u16];
    fn deref(&self) -> &[u16] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrency_clamps_and_defaults() {
        assert_eq!(Concurrency::from_input("512").get(), 512);
        assert_eq!(Concurrency::from_input("0").get(), Concurrency::MIN);
        assert_eq!(Concurrency::from_input("99999999").get(), Concurrency::MAX);
        assert_eq!(Concurrency::from_input("nope").get(), Concurrency::DEFAULT);
        assert_eq!(Concurrency::default().get(), Concurrency::DEFAULT);
    }

    #[test]
    fn timeout_floors_and_defaults() {
        assert_eq!(TimeoutMs::from_input("2000").get(), 2000);
        assert_eq!(TimeoutMs::from_input("10").get(), TimeoutMs::MIN);
        assert_eq!(TimeoutMs::from_input("").get(), TimeoutMs::DEFAULT);
    }

    #[test]
    fn ports_parse_drops_invalid_entries() {
        assert_eq!(
            &*Ports::from_input("25565, 19132 , x, 70000"),
            &[25565, 19132]
        );
        assert!(Ports::from_input("").is_empty());
    }
}
