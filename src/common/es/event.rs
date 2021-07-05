use serde::Serialize;
use std::fmt::Debug;

use super::AggregateId;

pub trait Payload: Debug + Sized {
    type UnmarshalErr;

    fn name(&self) -> String;
    fn marshal_json(&self) -> String;
    fn unmarshal_json(payload: &str) -> Result<Self, Self::UnmarshalErr>;
}

#[derive(Debug, PartialEq)]
pub struct Event {
    pub(super) aggregate_id: AggregateId,
    pub(super) name: String,
    pub(super) payload: String,
}

impl Event {
    pub fn new<T: Payload + Serialize>(aggregate_id: &AggregateId, payload: &T) -> Self {
        Event {
            aggregate_id: aggregate_id.clone(),
            name: payload.name().to_string(),
            payload: payload.marshal_json(),
        }
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::common::es::test_support::TestEvent;

    use super::{AggregateId, Event};

    #[test]
    fn it_is_eq_comparable() {
        let namespace = Uuid::new_v4();

        let id = AggregateId::from(&Uuid::new_v5(&namespace, "aggregate-1".as_bytes()));
        assert_eq!(
            Event::new(
                &id,
                &TestEvent {
                    name: "event-name".to_string(),
                    content: "event-1".to_string()
                }
            ),
            Event::new(
                &id,
                &TestEvent {
                    name: "event-name".to_string(),
                    content: "event-1".to_string()
                }
            ),
        )
    }
}
