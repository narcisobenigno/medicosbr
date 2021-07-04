use super::{AggregateId, Event, Version};
use std::vec::IntoIter;

pub struct VersionedEvents {
    current: Version,
    events: Vec<Event>,
}

impl VersionedEvents {
    pub fn new(current: Version, events: Vec<Event>) -> Self {
        Self { current, events }
    }
}

pub struct VersionedEvent {
    pub version: Version,
    pub aggregate_id: AggregateId,
    pub payload: String,
    pub name: String,
}

impl VersionedEvent {
    pub fn new(version: Version, aggregate_id: AggregateId, payload: String, name: String) -> Self {
        Self {
            version,
            aggregate_id,
            payload,
            name,
        }
    }
}

impl VersionedEvents {
    pub fn into_iter(&self) -> IntoIter<VersionedEvent> {
        let mut current = self.current.clone();
        let versioned: Vec<VersionedEvent> = self
            .events
            .iter()
            .map(|event| {
                current = current.next();
                VersionedEvent::new(
                    current.clone(),
                    event.aggregate_id.clone(),
                    event.payload.clone(),
                    event.name.clone(),
                )
            })
            .collect();

        versioned.into_iter()
    }
}
