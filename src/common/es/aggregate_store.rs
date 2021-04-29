use super::{Aggregate, AggregateId, Event, Stream, Version};

struct AggregateStore {
    stream: Box<dyn Stream>,
}

impl AggregateStore {
    pub fn load<T: Aggregate>(&self, id: &AggregateId) -> T {
        let mut t = T::default();
        for x in self.stream.read_by_aggregate_id(id) {
            t.handle(&x.name, &x.aggregate_id, &x.payload)
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
    use crate::common::es::{AggregateId, InMemoryStream, Payload};
    use chrono::DateTime;
    use serde::{Deserialize, Serialize};
    use std::time::SystemTime;
    use uuid::Uuid;

    #[test]
    fn it_loads_aggregate() {
        let time = SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap());
        let mut stream = InMemoryStream::new(InMemoryClock::new(time));
        let namespace = Uuid::new_v4();
        let aggregate_id_1 = Uuid::new_v5(&namespace, "aggregate-1".as_bytes());
        let aggregate_id_2 = Uuid::new_v5(&namespace, "aggregate-2".as_bytes());
        assert_eq!(
            Ok(()),
            stream.write(&mut vec![
                Event::new(
                    AggregateId::from(&aggregate_id_1),
                    Version::from(1),
                    &TestEvent {
                        name: "event-1".to_string(),
                    },
                ),
                Event::new(
                    AggregateId::from(&aggregate_id_2),
                    Version::from(1),
                    &TestEvent {
                        name: "event-2".to_string(),
                    },
                ),
                Event::new(
                    AggregateId::from(&aggregate_id_1),
                    Version::from(2),
                    &TestEvent {
                        name: "event-3".to_string(),
                    },
                ),
                Event::new(
                    AggregateId::from(&aggregate_id_2),
                    Version::from(2),
                    &TestEvent {
                        name: "event-4".to_string(),
                    },
                ),
            ]),
        );
        let store = AggregateStore::new(Box::new(stream));

        let aggregate: MyAggregate = store.load(&AggregateId::from(&aggregate_id_1));

        assert_eq!(
            &MyAggregate {
                values: vec!["event-1".to_string(), "event-3".to_string()]
            },
            &aggregate,
        )
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestEvent {
        name: String,
    }

    impl Payload for TestEvent {
        fn name(&self) -> String {
            "event-name".to_string()
        }

        fn marshal_json(&self) -> String {
            serde_json::to_string(self).unwrap().to_string()
        }
    }

    #[derive(Debug, PartialEq)]
    struct MyAggregate {
        values: Vec<String>,
    }

    impl Default for MyAggregate {
        fn default() -> Self {
            MyAggregate { values: vec![] }
        }
    }

    impl Aggregate for MyAggregate {
        fn handle(&mut self, event_name: &String, _: &AggregateId, payload: &String) {
            match event_name.as_str() {
                "event-name" => {
                    let result: TestEvent = serde_json::from_str(payload).unwrap();
                    self.values.push(result.name);
                }
                _ => {}
            }
        }
    }
}
