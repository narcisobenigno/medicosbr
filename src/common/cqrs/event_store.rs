use crate::common::cqrs;
use std::fmt;

pub trait EventStore<T: cqrs::Event>: fmt::Debug {
    fn load(&self, id: String) -> Vec<&cqrs::EventEnvelop<T>>;
    fn save(&mut self, events: Vec<cqrs::EventEnvelop<T>>) -> Result<(), &'static str>;
}

#[derive(Debug)]
pub struct MemoryEventStore<T: cqrs::Event> {
    items: Vec<cqrs::EventEnvelop<T>>,
}

impl<T> Default for MemoryEventStore<T>
where
    T: cqrs::Event,
{
    fn default() -> Self {
        MemoryEventStore { items: vec![] }
    }
}

impl<T> EventStore<T> for MemoryEventStore<T>
where
    T: cqrs::Event,
{
    fn load(&self, id: String) -> Vec<&cqrs::EventEnvelop<T>> {
        self.items
            .iter()
            .filter(|event| event.has_id(id.to_string()))
            .collect()
    }

    fn save(&mut self, events: Vec<cqrs::EventEnvelop<T>>) -> Result<(), &'static str> {
        self.items.extend(events);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::cqrs;
    use crate::common::cqrs::EventStore;

    #[test]
    fn it_adds_to_mem_event_store() {
        let mut store = MemoryEventStore::default();
        store.save(vec![
            cqrs::EventEnvelop::new(
                "aggregate-id-1".to_string(),
                TestEvent::Event1(Event1 { value: 1 }),
            ),
            cqrs::EventEnvelop::new(
                "aggregate-id-2".to_string(),
                TestEvent::Event1(Event1 { value: 2 }),
            ),
        ]);

        let out = store.load("aggregate-id-1".to_string());

        assert_eq!(
            out,
            vec![&cqrs::EventEnvelop::new(
                "aggregate-id-1".to_string(),
                TestEvent::Event1(Event1 { value: 1 }),
            )]
        )
    }

    #[derive(Debug, PartialEq)]
    enum TestEvent {
        Event1(Event1),
    }

    impl cqrs::Event for TestEvent {
        fn name() -> String {
            "test-events".to_string()
        }
    }

    #[derive(Debug, PartialEq)]
    struct Event1 {
        value: i64,
    }

    impl cqrs::Event for Event1 {
        fn name() -> String {
            "event-1".to_string()
        }
    }
}
