#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub struct Version {
    value: u64,
}

impl Version {
    pub fn next(&self) -> Self {
        Version {
            value: self.value + 1,
        }
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[cfg(test)]
mod test {
    use super::Version;

    #[test]
    fn it_creates_new_version() {
        assert_eq!(Version::from(1), Version::from(1));
        assert_ne!(Version::from(1), Version::from(2));
    }
}
