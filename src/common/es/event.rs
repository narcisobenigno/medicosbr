use super::{AggregateId, Version};

#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    pub(super) aggregate_id: AggregateId,
    pub(super) version: Version,
}

impl Event {
    pub fn new(aggregate_id: AggregateId, version: Version) -> Self {
        Event {
            aggregate_id,
            version,
        }
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use super::super::{AggregateId, Version};

    #[test]
    fn it_is_eq_comparable() {
        let namespace = Uuid::new_v4();

        assert_eq!(
            super::Event::new(
                AggregateId::from(Uuid::new_v5(&namespace, "aggregate-1".as_bytes())),
                Version::from(1)
            ),
            super::Event::new(
                AggregateId::from(Uuid::new_v5(&namespace, "aggregate-1".as_bytes())),
                Version::from(1)
            ),
        )
    }
}
