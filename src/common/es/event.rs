use std::fmt::Debug;
use std::time::SystemTime;

use serde::Serialize;

use super::{AggregateId, Version};

pub trait Payload: Debug + Sized {
    type UnmarshalErr;

    fn name(&self) -> String;
    fn marshal_json(&self) -> String;
    fn unmarshal_json(payload: &str) -> Result<Self, Self::UnmarshalErr>;
}

#[derive(Debug, PartialEq)]
pub struct Event {
    pub(super) aggregate_id: AggregateId,
    pub(super) version: Version,
    pub(super) name: String,
    pub(super) payload: String,
}

impl Event {
    pub fn new<T: Payload + Serialize>(
        aggregate_id: &AggregateId,
        version: &Version,
        payload: &T,
    ) -> Self {
        Event {
            aggregate_id: aggregate_id.clone(),
            version: version.clone(),
            name: payload.name().to_string(),
            payload: payload.marshal_json(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct WrittenEvent {
    pub(super) aggregate_id: AggregateId,
    pub(super) version: Version,
    pub(super) name: String,
    pub(super) payload: String,
    recorded_at: SystemTime,
    position: u64,
}

impl WrittenEvent {
    pub fn new(event: &Event, recorded_at: SystemTime, position: u64) -> WrittenEvent {
        WrittenEvent {
            aggregate_id: event.aggregate_id.clone(),
            version: event.version.clone(),
            name: event.name.clone(),
            payload: event.payload.clone(),
            recorded_at,
            position,
        }
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use super::super::{AggregateId, Version};
    use super::Event;
    use crate::common::es::test_support::TestEvent;

    #[test]
    fn it_is_eq_comparable() {
        let namespace = Uuid::new_v4();

        let id = AggregateId::from(&Uuid::new_v5(&namespace, "aggregate-1".as_bytes()));
        let version1 = Version::from(1);
        assert_eq!(
            Event::new(
                &id,
                &version1,
                &TestEvent {
                    name: "event-name".to_string(),
                    content: "event-1".to_string()
                }
            ),
            Event::new(
                &id,
                &version1,
                &TestEvent {
                    name: "event-name".to_string(),
                    content: "event-1".to_string()
                }
            ),
        )
    }
}
