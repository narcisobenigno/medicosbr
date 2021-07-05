use std::str::FromStr;

use crate::common::es::written_event::WrittenEvent;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct AggregateId {
    value: Uuid,
}

impl AggregateId {
    pub fn is_nil(&self) -> bool {
        self.value == Uuid::nil()
    }
}

impl From<&Uuid> for AggregateId {
    fn from(value: &Uuid) -> Self {
        AggregateId {
            value: value.clone(),
        }
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

pub trait Aggregate: Default {
    fn handle(&mut self, event: &WrittenEvent);
}
