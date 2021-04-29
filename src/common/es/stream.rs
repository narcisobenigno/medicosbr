use std::collections::HashMap;

use super::{AggregateId, Event, Version, WrittenEvent};
use crate::common::clock::{Clock, InMemoryClock};

#[derive(Debug, PartialEq)]
pub enum Error {
    OptimistLockViolation,
}

pub trait Stream {
    fn read_by_aggregate_id(&self, aggregate_id: &AggregateId) -> Vec<&WrittenEvent>;
    fn write(&mut self, events: &mut Vec<Event>) -> Result<(), Error>;
}

pub struct InMemoryStream {
    events: HashMap<AggregateId, HashMap<Version, WrittenEvent>>,
    current_position: u64,
    clock: InMemoryClock,
}

impl InMemoryStream {
    pub fn new(clock: InMemoryClock) -> Self {
        InMemoryStream {
            events: HashMap::new(),
            current_position: 1,
            clock,
        }
    }
}

impl Stream for InMemoryStream {
    fn read_by_aggregate_id(&self, aggregate_id: &AggregateId) -> Vec<&WrittenEvent> {
        let unsorted = match self.events.get(aggregate_id) {
            Some(versions) => versions.values().collect(),
            _ => vec![],
        };

        let mut events = unsorted.clone();
        events.sort_by(|e1, e2| e1.version.cmp(&e2.version));
        events
    }

    fn write(&mut self, events: &mut Vec<Event>) -> Result<(), Error> {
        for event in events.iter() {
            let aggregate_events = self
                .events
                .entry(event.aggregate_id.clone())
                .or_insert(HashMap::new());
            if aggregate_events.contains_key(&event.version) {
                return Err(Error::OptimistLockViolation);
            }

            aggregate_events.insert(
                event.version.clone(),
                WrittenEvent::new(event, self.clock.now(), self.current_position),
            );
            self.clock.tick();
            self.current_position += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use std::time::SystemTime;

    use chrono::DateTime;
    use uuid::Uuid;

    use crate::common::clock::InMemoryClock;

    use super::super::{Payload, Version, WrittenEvent};
    use super::*;

    #[test]
    fn it_loads_events_by_aggregate_id() {
        let time = SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap());
        let stream = &mut InMemoryStream::new(InMemoryClock::new(time));

        let namespace = Uuid::new_v4();
        let aggregate_id_1 = Uuid::new_v5(&namespace, "aggregate-1".as_bytes()).to_string();
        let aggregate_id_2 = Uuid::new_v5(&namespace, "aggregate-2".as_bytes()).to_string();

        assert_eq!(
            Ok(()),
            stream.write(&mut vec![
                Event::new(
                    AggregateId::from(aggregate_id_1.as_str()),
                    Version::from(1),
                    &TestEvent {
                        name: "event-1".to_string(),
                    },
                ),
                Event::new(
                    AggregateId::from(aggregate_id_2.as_str()),
                    Version::from(1),
                    &TestEvent {
                        name: "event-2".to_string(),
                    },
                ),
                Event::new(
                    AggregateId::from(aggregate_id_1.as_str()),
                    Version::from(2),
                    &TestEvent {
                        name: "event-3".to_string(),
                    },
                ),
                Event::new(
                    AggregateId::from(aggregate_id_2.as_str()),
                    Version::from(2),
                    &TestEvent {
                        name: "event-4".to_string(),
                    },
                ),
            ])
        );

        assert_eq!(
            vec![
                &WrittenEvent::new(
                    &Event::new(
                        AggregateId::from(aggregate_id_1.as_str()),
                        Version::from(1),
                        &TestEvent {
                            name: "event-1".to_string(),
                        },
                    ),
                    SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap()),
                    1,
                ),
                &WrittenEvent::new(
                    &Event::new(
                        AggregateId::from(aggregate_id_1.as_str()),
                        Version::from(2),
                        &TestEvent {
                            name: "event-3".to_string(),
                        },
                    ),
                    SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:02Z").unwrap()),
                    3,
                )
            ],
            stream.read_by_aggregate_id(&AggregateId::from(aggregate_id_1.as_str()))
        )
    }

    #[test]
    fn it_violates_optimistic_lock_when_version_exists_for_aggregate() {
        let time = SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap());
        let stream = &mut InMemoryStream::new(InMemoryClock::new(time));

        let namespace = Uuid::new_v4();
        let aggregate_id_1 = Uuid::new_v5(&namespace, "aggregate-1".as_bytes()).to_string();

        assert_eq!(
            Ok(()),
            stream.write(&mut vec![Event::new(
                AggregateId::from(aggregate_id_1.as_str()),
                Version::from(1),
                &TestEvent {
                    name: "event-1".to_string(),
                },
            )])
        );

        assert_eq!(
            Err(Error::OptimistLockViolation),
            stream.write(&mut vec![Event::new(
                AggregateId::from(aggregate_id_1.as_str()),
                Version::from(1),
                &TestEvent {
                    name: "event-1".to_string(),
                },
            )])
        );
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEvent {
        name: String,
    }

    impl Payload for TestEvent {
        fn name(&self) -> String {
            "TestEvent".to_string()
        }

        fn marshal_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }
    }
}
