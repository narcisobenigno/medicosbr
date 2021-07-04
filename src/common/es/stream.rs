use std::collections::HashMap;
use std::fmt;

use crate::common::clock::{Clock, InMemoryClock};
use crate::common::es::VersionedEvents;

use super::{AggregateId, Event, Version, WrittenEvent};

#[derive(Debug, PartialEq)]
pub enum Error {
    OptimistLockViolation,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Stream {
    fn read_by_aggregate_id(&self, aggregate_id: &AggregateId) -> Vec<&WrittenEvent>;
    fn write(&mut self, events: &mut VersionedEvents) -> Result<(), Error>;
}

pub struct InMemoryStream {
    events: HashMap<AggregateId, HashMap<Version, WrittenEvent>>,
    current_position: u64,
    clock: InMemoryClock,
}

impl InMemoryStream {
    pub fn new(clock: InMemoryClock) -> Self {
        Self {
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

    fn write(&mut self, events: &mut VersionedEvents) -> Result<(), Error> {
        for event in events.into_iter() {
            let aggregate_events = self
                .events
                .entry(event.aggregate_id.clone())
                .or_insert(HashMap::new());
            if aggregate_events.contains_key(&event.version) {
                return Err(Error::OptimistLockViolation);
            }

            aggregate_events.insert(
                event.version.clone(),
                WrittenEvent::new(&event, self.clock.now(), self.current_position),
            );
            self.clock.tick();
            self.current_position += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use chrono::DateTime;
    use uuid::Uuid;

    use crate::common::clock;
    use crate::common::es::{Payload, VersionedEvent};

    use super::super::{TestEvent, Version, WrittenEvent};
    use super::*;

    #[test]
    fn it_loads_events_by_aggregate_id() {
        let time = SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap());
        let stream = &mut InMemoryStream::new(clock::InMemoryClock::new(time));

        let namespace = Uuid::new_v4();
        let aggregate_id_1 = AggregateId::from(&Uuid::new_v5(&namespace, "aggregate-1".as_bytes()));
        let aggregate_id_2 = AggregateId::from(&Uuid::new_v5(&namespace, "aggregate-2".as_bytes()));

        assert_eq!(
            Ok(()),
            stream.write(&mut VersionedEvents::new(
                Version::from(1),
                vec![
                    Event::new(
                        &aggregate_id_1,
                        &TestEvent {
                            name: "event-name".to_string(),
                            content: "event-1".to_string(),
                        },
                    ),
                    Event::new(
                        &aggregate_id_2,
                        &TestEvent {
                            name: "event-name".to_string(),
                            content: "event-2".to_string(),
                        },
                    ),
                    Event::new(
                        &aggregate_id_1,
                        &TestEvent {
                            name: "event-name".to_string(),
                            content: "event-3".to_string(),
                        },
                    ),
                    Event::new(
                        &aggregate_id_2,
                        &TestEvent {
                            name: "event-name".to_string(),
                            content: "event-4".to_string(),
                        },
                    ),
                ]
            ))
        );

        assert_eq!(
            vec![
                &WrittenEvent::new(
                    &VersionedEvent::new(
                        Version::from(2),
                        aggregate_id_1.clone(),
                        TestEvent {
                            name: "event-name".to_string(),
                            content: "event-1".to_string(),
                        }
                        .marshal_json(),
                        "event-name".to_string(),
                    ),
                    SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap()),
                    1,
                ),
                &WrittenEvent::new(
                    &VersionedEvent::new(
                        Version::from(4),
                        aggregate_id_1.clone(),
                        TestEvent {
                            name: "event-name".to_string(),
                            content: "event-3".to_string(),
                        }
                        .marshal_json(),
                        "event-name".to_string(),
                    ),
                    SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:04Z").unwrap()),
                    3,
                )
            ],
            stream.read_by_aggregate_id(&aggregate_id_1)
        )
    }

    #[test]
    fn it_violates_optimistic_lock_when_version_exists_for_aggregate() {
        let time = SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap());
        let stream = &mut InMemoryStream::new(InMemoryClock::new(time));

        let aggregate_id_1 =
            AggregateId::from(&Uuid::new_v5(&Uuid::new_v4(), "aggregate-1".as_bytes()));

        assert_eq!(
            Ok(()),
            stream.write(&mut VersionedEvents::new(
                Version::from(1),
                vec![Event::new(
                    &aggregate_id_1,
                    &TestEvent {
                        name: "event-name".to_string(),
                        content: "event-1".to_string(),
                    },
                )]
            ))
        );

        assert_eq!(
            Err(Error::OptimistLockViolation),
            stream.write(&mut VersionedEvents::new(
                Version::from(1),
                vec![Event::new(
                    &aggregate_id_1,
                    &TestEvent {
                        name: "event-name".to_string(),
                        content: "event-1".to_string(),
                    },
                )]
            ))
        );
    }
}
