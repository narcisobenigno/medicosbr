use std::time::SystemTime;

use crate::common::es::{AggregateId, Version, VersionedEvent};

#[derive(Debug, PartialEq, Clone)]
pub struct WrittenEvent {
    pub aggregate_id: AggregateId,
    pub version: Version,
    pub name: String,
    pub payload: String,
    recorded_at: SystemTime,
    position: u64,
}

impl WrittenEvent {
    pub fn new(event: &VersionedEvent, recorded_at: SystemTime, position: u64) -> WrittenEvent {
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
