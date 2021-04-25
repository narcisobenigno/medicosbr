#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
pub struct Version {
    value: u64,
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        if value == 0 {
            panic!("version should be greater than zero");
        }
        Self { value }
    }
}

#[cfg(test)]
mod test {
    use super::Version;

    #[test]
    fn it_creates_new_version() {
        assert_eq!(Version::from(1), Version::from(1))
    }

    #[test]
    #[should_panic]
    fn it_does_not_allow_smaller_than_one_version() {
        Version::from(0);
    }
}
