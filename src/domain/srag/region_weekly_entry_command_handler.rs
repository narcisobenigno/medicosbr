use crate::common;
use crate::common::es;
use crate::domain;
use crate::domain::srag::vo;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RegionWeeklyEventDetected(
    pub vo::Region,
    pub vo::Case,
    pub vo::TotalReported,
    pub vo::YearWeek,
);

impl es::Payload for RegionWeeklyEventDetected {
    type UnmarshalErr = ();

    fn name(&self) -> String {
        "RegionWeeklyEventDetected".to_string()
    }

    fn marshal_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn unmarshal_json(payload: &str) -> Result<Self, Self::UnmarshalErr> {
        match serde_json::from_str::<Self>(payload) {
            Ok(payload) => Ok(payload),
            Err(_) => Err(()),
        }
    }
}

pub struct RegionWeeklyCommandHandler<'a> {
    aggregate_store: &'a mut es::AggregateStore<'a>,
}

impl<'a> RegionWeeklyCommandHandler<'a> {
    fn new(aggregate_store: &'a mut es::AggregateStore<'a>) -> RegionWeeklyCommandHandler<'a> {
        RegionWeeklyCommandHandler { aggregate_store }
    }

    pub(crate) fn handle(&mut self, command: RegionWeeklyUpload) -> Result<(), common::Error> {
        let aggregate_id = es::AggregateId::from(&Uuid::new_v5(
            &Uuid::from_str("a385bf4a-e6c0-48ee-a5e0-701e92f1e592").unwrap(),
            format!("{}{}", command.region.name(), vo::YearWeek(2019, 10)).as_bytes(),
        ));

        let aggregate = self
            .aggregate_store
            .load::<domain::RegionalWeeklyReport>(&aggregate_id);
        match aggregate.upload(command) {
            Ok(mut events) => self
                .aggregate_store
                .write(&mut events)
                .or_else(|err| Err(common::Error::new(err.to_string().as_str()))),
            Err(err) => Err(common::Error::new(err.to_string().as_str())),
        }
    }
}

pub struct RegionWeeklyUpload {
    pub aggregate_id: es::AggregateId,
    pub region: vo::Region,
    pub case: vo::Case,
    pub total_reported: vo::TotalReported,
    pub year_week: vo::YearWeek,
}

impl RegionWeeklyUpload {
    pub fn new_id(region: vo::Region, year_week: vo::YearWeek) -> es::AggregateId {
        es::AggregateId::from(&Uuid::new_v5(
            &Uuid::from_str("a385bf4a-e6c0-48ee-a5e0-701e92f1e592").unwrap(),
            format!("{}{}", region.name(), year_week).as_bytes(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::clock::InMemoryClock;
    use crate::common::es;
    use crate::common::es::Stream;
    use crate::domain::srag::vo;
    use chrono::DateTime;
    use std::time::SystemTime;

    #[test]
    fn it_uploads_a_new() {
        let time = SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap());
        let clock = InMemoryClock::new(time);
        let mut stream = es::InMemoryStream::new(clock);
        let mut aggregate_store = es::AggregateStore::new(&mut stream);
        let mut handler = RegionWeeklyCommandHandler::new(&mut aggregate_store);

        let aggregate_id = RegionWeeklyUpload::new_id(vo::Region::Alagoas, vo::YearWeek(2019, 10));
        let result = handler.handle(RegionWeeklyUpload {
            aggregate_id: aggregate_id.clone(),
            region: vo::Region::Alagoas,
            case: vo::Case::SARS,
            total_reported: vo::TotalReported(10),
            year_week: vo::YearWeek(2019, 10),
        });
        assert_eq!(Ok(()), result);
        assert_eq!(
            vec![&es::WrittenEvent::new(
                &es::Event::new(
                    &aggregate_id,
                    &es::Version::from(1),
                    &RegionWeeklyEventDetected(
                        vo::Region::Alagoas,
                        vo::Case::SARS,
                        vo::TotalReported(10),
                        vo::YearWeek(2019, 10),
                    )
                ),
                time,
                1,
            )],
            stream.read_by_aggregate_id(&aggregate_id),
        )
    }
}
