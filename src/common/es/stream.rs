use std::collections::HashMap;

use crate::common::Version;

use super::{AggregateId, Event};

#[derive(Debug, PartialEq)]
pub enum Error {
    OptimistLockViolation,
}

pub trait Stream {
    fn read_by_aggregate_id(&self, aggregate_id: &AggregateId) -> Vec<&Event>;
    fn write(&mut self, events: &mut Vec<Event>) -> Result<(), Error>;
}

pub struct InMemoryStream {
    events: HashMap<AggregateId, HashMap<Version, Event>>,
}

impl Default for InMemoryStream {
    fn default() -> Self {
        InMemoryStream {
            events: HashMap::new(),
        }
    }
}

impl Stream for InMemoryStream {
    fn read_by_aggregate_id(&self, aggregate_id: &AggregateId) -> Vec<&Event> {
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
            aggregate_events.insert(event.version.clone(), event.clone());
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use super::super::Version;
    use super::*;

    #[test]
    fn it_loads_events_by_aggregate_id() {
        let stream = &mut InMemoryStream::default();

        let namespace = Uuid::new_v4();
        let aggregate_id_1 = Uuid::new_v5(&namespace, "aggregate-1".as_bytes()).to_string();
        let aggregate_id_2 = Uuid::new_v5(&namespace, "aggregate-2".as_bytes()).to_string();

        assert_eq!(
            Ok(()),
            stream.write(&mut vec![
                Event::new(AggregateId::from(aggregate_id_1.as_str()), Version::from(1)),
                Event::new(AggregateId::from(aggregate_id_2.as_str()), Version::from(1)),
                Event::new(AggregateId::from(aggregate_id_1.as_str()), Version::from(2)),
                Event::new(AggregateId::from(aggregate_id_2.as_str()), Version::from(2)),
            ])
        );

        assert_eq!(
            vec![
                &Event::new(AggregateId::from(aggregate_id_1.as_str()), Version::from(1)),
                &Event::new(AggregateId::from(aggregate_id_1.as_str()), Version::from(2)),
            ],
            stream.read_by_aggregate_id(&AggregateId::from(aggregate_id_1.as_str()))
        )
    }

    #[test]
    fn it_violates_optimistic_lock_when_version_exists_for_aggregate() {
        let stream = &mut InMemoryStream::default();

        let namespace = Uuid::new_v4();
        let aggregate_id_1 = Uuid::new_v5(&namespace, "aggregate-1".as_bytes()).to_string();

        assert_eq!(
            Ok(()),
            stream.write(&mut vec![Event::new(
                AggregateId::from(aggregate_id_1.as_str()),
                Version::from(1)
            )])
        );

        assert_eq!(
            Err(Error::OptimistLockViolation),
            stream.write(&mut vec![Event::new(
                AggregateId::from(aggregate_id_1.as_str()),
                Version::from(1)
            )])
        );
    }
}
