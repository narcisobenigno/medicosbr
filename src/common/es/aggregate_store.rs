use super::{Aggregate, AggregateId, Event, Stream, Version};

struct AggregateStore {
    stream: Box<dyn Stream>,
}

impl AggregateStore {
    pub fn load<T: Aggregate>(&self, id: &AggregateId) -> T {
        let mut t = T::default();
        for x in self.stream.read_by_aggregate_id(id) {
            t.handle(&x)
        }
        t
    }
}

impl AggregateStore {
    pub fn new(stream: Box<dyn Stream>) -> AggregateStore {
        AggregateStore { stream }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::clock::InMemoryClock;
    use crate::common::es::test_support::TestEvent;
    use crate::common::es::{AggregateId, InMemoryStream, WrittenEvent};
    use chrono::DateTime;
    use std::time::SystemTime;
    use uuid::Uuid;

    #[test]
    fn it_loads_aggregate() {
        let time = SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap());
        let mut stream = InMemoryStream::new(InMemoryClock::new(time));

        let namespace = Uuid::new_v4();
        let aggregate_id_1 = AggregateId::from(&Uuid::new_v5(&namespace, "aggregate-1".as_bytes()));
        let aggregate_id_2 = AggregateId::from(&Uuid::new_v5(&namespace, "aggregate-2".as_bytes()));
        let version1 = Version::from(1);
        let version2 = Version::from(2);

        assert_eq!(
            Ok(()),
            stream.write(&mut vec![
                Event::new(
                    &aggregate_id_1,
                    &version1,
                    &TestEvent {
                        name: "event-name".to_string(),
                        content: "event-1".to_string(),
                    },
                ),
                Event::new(
                    &aggregate_id_2,
                    &version1,
                    &TestEvent {
                        name: "event-name".to_string(),
                        content: "event-2".to_string(),
                    },
                ),
                Event::new(
                    &aggregate_id_1,
                    &version2,
                    &TestEvent {
                        name: "event-name".to_string(),
                        content: "event-3".to_string(),
                    },
                ),
                Event::new(
                    &aggregate_id_2,
                    &version2,
                    &TestEvent {
                        name: "event-name".to_string(),
                        content: "event-4".to_string(),
                    },
                ),
            ]),
        );

        let store = AggregateStore::new(Box::new(stream));
        let aggregate: MyAggregate = store.load(&aggregate_id_1);

        assert_eq!(
            &MyAggregate {
                ids: vec![aggregate_id_1.clone(), aggregate_id_1.clone()],
                values: vec!["event-1".to_string(), "event-3".to_string()]
            },
            &aggregate,
        )
    }

    #[derive(Debug, PartialEq)]
    struct MyAggregate {
        ids: Vec<AggregateId>,
        values: Vec<String>,
    }

    impl Default for MyAggregate {
        fn default() -> Self {
            MyAggregate {
                ids: vec![],
                values: vec![],
            }
        }
    }

    impl Aggregate for MyAggregate {
        fn handle(&mut self, event: &WrittenEvent) {
            match event.name.as_str() {
                "event-name" => {
                    let result: TestEvent = serde_json::from_str(event.payload.as_str()).unwrap();
                    self.values.push(result.content);
                    self.ids.push(event.aggregate_id.clone())
                }
                _ => {}
            }
        }
    }
}
