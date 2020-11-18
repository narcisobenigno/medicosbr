use std::fmt;

pub trait Event: PartialEq + fmt::Debug {
    fn name() -> String;
}

#[derive(PartialEq, Debug)]
pub struct EventEnvelop<E>
where
    E: Event,
{
    aggregate_id: String,
    payload: E,
}

impl<E> EventEnvelop<E>
where
    E: Event,
{
    fn new(aggregate_id: String, payload: E) -> Self {
        Self {
            aggregate_id: aggregate_id,
            payload: payload,
        }
    }
}

pub trait EventStore<T: Event>: fmt::Debug {
    fn load(&self, id: String) -> Vec<&EventEnvelop<T>>;
}

#[derive(Debug)]
pub struct MemoryEventStore<T: Event> {
    items: Vec<EventEnvelop<T>>,
}

impl<T> EventStore<T> for MemoryEventStore<T>
where
    T: Event,
{
    fn load(&self, id: String) -> Vec<&EventEnvelop<T>> {
        self.items
            .iter()
            .filter(|item| item.aggregate_id == id)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::common::cqrs;
    use crate::common::cqrs::EventStore;

    #[test]
    fn it_adds_to_mem_event_store() {
        let mut items: Vec<cqrs::EventEnvelop<TestEvent>> = vec![
            cqrs::EventEnvelop::new(
                "aggregate-id-1".to_string(),
                TestEvent::Event1(Event1 { value: 1 }),
            ),
            cqrs::EventEnvelop::new(
                "aggregate-id-2".to_string(),
                TestEvent::Event1(Event1 { value: 2 }),
            ),
        ];
        let store = cqrs::MemoryEventStore { items: items };
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
