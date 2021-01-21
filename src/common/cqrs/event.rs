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
    pub fn new(aggregate_id: &str, payload: E) -> Self {
        Self {
            aggregate_id: aggregate_id.to_string(),
            payload: payload,
        }
    }

    pub fn has_id(&self, id: &str) -> bool {
        self.aggregate_id == id.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_has_id() {
        let event = EventEnvelop::new("aggregate-id-1", Event1 { value: 3 });
        assert!(event.has_id("aggregate-id-1"));
    }

    #[test]
    fn it_has_no_id() {
        let event = EventEnvelop::new("aggregate-id-1", Event1 { value: 3 });
        assert!(!event.has_id("no-aggregate-id"));
    }

    #[derive(Debug, PartialEq)]
    struct Event1 {
        value: i64,
    }

    impl Event for Event1 {
        fn name() -> String {
            "event-1".to_string()
        }
    }
}
