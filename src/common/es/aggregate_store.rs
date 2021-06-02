use crate::common::es;

pub struct AggregateStore<'a> {
    stream: &'a mut (dyn es::Stream + 'a),
}

impl<'a> AggregateStore<'a> {
    pub fn new(stream: &'a mut (dyn es::Stream + 'a)) -> AggregateStore<'a> {
        AggregateStore { stream }
    }

    pub fn load<T: es::Aggregate>(&self, id: &es::AggregateId) -> T {
        let mut t = T::default();
        for x in self.stream.read_by_aggregate_id(id) {
            t.handle(&x)
        }
        t
    }

    pub fn write(&mut self, events: &mut Vec<es::Event>) -> Result<(), es::Error> {
        self.stream.write(events)
    }
}

#[cfg(test)]
mod test {
    use crate::common::clock;
    use crate::common::es;
    use crate::common::es::stream::Stream;
    use crate::common::es::test_support::TestEvent;
    use chrono::DateTime;
    use std::time::SystemTime;
    use uuid::Uuid;

    #[test]
    fn it_loads_aggregate() {
        let time = SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap());
        let in_memory_clock = clock::InMemoryClock::new(time);
        let mut stream = es::InMemoryStream::new(in_memory_clock);

        let namespace = Uuid::new_v4();
        let aggregate_id_1 =
            es::AggregateId::from(&Uuid::new_v5(&namespace, "aggregate-1".as_bytes()));
        let aggregate_id_2 =
            es::AggregateId::from(&Uuid::new_v5(&namespace, "aggregate-2".as_bytes()));
        let version1 = es::Version::from(1);
        let version2 = es::Version::from(2);

        assert_eq!(
            Ok(()),
            stream.write(&mut vec![
                es::Event::new(
                    &aggregate_id_1,
                    &version1,
                    &TestEvent {
                        name: "event-name".to_string(),
                        content: "event-1".to_string(),
                    },
                ),
                es::Event::new(
                    &aggregate_id_2,
                    &version1,
                    &TestEvent {
                        name: "event-name".to_string(),
                        content: "event-2".to_string(),
                    },
                ),
                es::Event::new(
                    &aggregate_id_1,
                    &version2,
                    &TestEvent {
                        name: "event-name".to_string(),
                        content: "event-3".to_string(),
                    },
                ),
                es::Event::new(
                    &aggregate_id_2,
                    &version2,
                    &TestEvent {
                        name: "event-name".to_string(),
                        content: "event-4".to_string(),
                    },
                ),
            ]),
        );

        let store = es::AggregateStore::new(&mut stream);
        let aggregate: &mut MyAggregate = &mut store.load(&aggregate_id_1);

        let x = &mut MyAggregate {
            ids: vec![aggregate_id_1.clone(), aggregate_id_1.clone()],
            values: vec!["event-1".to_string(), "event-3".to_string()],
        };
        assert_eq!(&x, &aggregate)
    }

    #[derive(Debug, PartialEq)]
    struct MyAggregate {
        ids: Vec<es::AggregateId>,
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

    impl es::Aggregate for MyAggregate {
        fn handle(&mut self, event: &es::WrittenEvent) {
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
