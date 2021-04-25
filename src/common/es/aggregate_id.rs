use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AggregateId {
    value: Uuid,
}

impl From<Uuid> for AggregateId {
    fn from(value: Uuid) -> Self {
        AggregateId { value }
    }
}

impl From<&str> for AggregateId {
    fn from(value: &str) -> Self {
        match Uuid::from_str(value) {
            Ok(value) => AggregateId { value },
            _ => panic!("could not parse string into uuid"),
        }
    }
}