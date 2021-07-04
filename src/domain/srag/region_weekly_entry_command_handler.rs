use crate::common;
use crate::common::es;
use crate::domain;
use crate::domain::srag::vo;
use crate::domain::RegionalWeeklyReport;
use serde::{Deserialize, Serialize};

pub const REGION_WEEKLY_EVENT_DETECTED_TYPE: &str = "RegionWeeklyEventDetected";
pub const REGION_WEEKLY_EVENT_TOTAL_REPORTED_CHANGED: &str =
    "RegionWeeklyEventTotalReportedChanged";

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RegionWeeklyEventDetected(
    pub vo::Region,
    pub vo::Case,
    pub vo::TotalReported,
    pub vo::YearWeek,
);

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RegionWeeklyEventTotalReportedChanged(pub vo::TotalReported);

impl es::Payload for RegionWeeklyEventDetected {
    type UnmarshalErr = ();

    fn name(&self) -> String {
        REGION_WEEKLY_EVENT_DETECTED_TYPE.to_string()
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

impl es::Payload for RegionWeeklyEventTotalReportedChanged {
    type UnmarshalErr = ();

    fn name(&self) -> String {
        REGION_WEEKLY_EVENT_TOTAL_REPORTED_CHANGED.to_string()
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
        let aggregate_id =
            &RegionalWeeklyReport::new_id(command.region.clone(), command.year_week.clone());
        let aggregate = self
            .aggregate_store
            .load::<domain::RegionalWeeklyReport>(aggregate_id);
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

#[cfg(test)]
mod tests {
    use crate::common::clock;
    use crate::common::es;
    use crate::common::es::{Payload, Stream, VersionedEvents};
    use crate::domain;
    use crate::domain::srag::vo;
    use chrono::DateTime;
    use std::time::SystemTime;

    #[test]
    fn it_uploads_a_new() {
        let time = SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap());
        let clock = clock::InMemoryClock::new(time);
        let mut stream = es::InMemoryStream::new(clock);
        let mut aggregate_store = es::AggregateStore::new(&mut stream);
        let mut handler = super::RegionWeeklyCommandHandler::new(&mut aggregate_store);

        let aggregate_id =
            domain::RegionalWeeklyReport::new_id(vo::Region::Alagoas, vo::YearWeek(2019, 10));
        let result = handler.handle(super::RegionWeeklyUpload {
            aggregate_id: aggregate_id.clone(),
            region: vo::Region::Alagoas,
            case: vo::Case::SARS,
            total_reported: vo::TotalReported(10),
            year_week: vo::YearWeek(2019, 10),
        });
        assert_eq!(Ok(()), result);
        assert_eq!(
            vec![&es::WrittenEvent::new(
                &es::VersionedEvent::new(
                    es::Version::from(1),
                    aggregate_id.clone(),
                    domain::RegionWeeklyEventDetected(
                        vo::Region::Alagoas,
                        vo::Case::SARS,
                        vo::TotalReported(10),
                        vo::YearWeek(2019, 10),
                    )
                    .marshal_json(),
                    domain::REGION_WEEKLY_EVENT_DETECTED_TYPE.to_string(),
                ),
                time,
                1,
            )],
            stream.read_by_aggregate_id(&aggregate_id.clone()),
        )
    }

    #[test]
    fn it_uploads_an_update_to_a_report() {
        let time = SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:00Z").unwrap());
        let clock = clock::InMemoryClock::new(time);
        let mut stream = es::InMemoryStream::new(clock);
        let mut aggregate_store = &mut es::AggregateStore::new(&mut stream);

        let aggregate_id1 =
            domain::RegionalWeeklyReport::new_id(vo::Region::Alagoas, vo::YearWeek(2019, 10));
        let aggregate_id2 =
            domain::RegionalWeeklyReport::new_id(vo::Region::Alagoas, vo::YearWeek(2019, 11));
        assert_eq!(
            Ok(()),
            aggregate_store.write(&mut VersionedEvents::new(
                es::Version::from(0),
                vec![es::Event::new(
                    &aggregate_id1,
                    &domain::RegionWeeklyEventDetected(
                        vo::Region::Alagoas,
                        vo::Case::SARS,
                        vo::TotalReported(10),
                        vo::YearWeek(2019, 10),
                    ),
                ),]
            ))
        );
        assert_eq!(
            Ok(()),
            aggregate_store.write(&mut VersionedEvents::new(
                es::Version::from(0),
                vec![es::Event::new(
                    &aggregate_id2,
                    &domain::RegionWeeklyEventDetected(
                        vo::Region::Alagoas,
                        vo::Case::SARS,
                        vo::TotalReported(20),
                        vo::YearWeek(2019, 11),
                    )
                ),]
            ))
        );

        let result = super::RegionWeeklyCommandHandler::new(&mut aggregate_store).handle(
            super::RegionWeeklyUpload {
                aggregate_id: aggregate_id1.clone(),
                region: vo::Region::Alagoas,
                case: vo::Case::SARS,
                total_reported: vo::TotalReported(30),
                year_week: vo::YearWeek(2019, 10),
            },
        );
        assert_eq!(Ok(()), result);
        let events = stream.read_by_aggregate_id(&aggregate_id1);
        assert_eq!(
            [&es::WrittenEvent::new(
                &es::VersionedEvent::new(
                    es::Version::from(2),
                    aggregate_id1.clone(),
                    domain::RegionWeeklyEventTotalReportedChanged(vo::TotalReported(30))
                        .marshal_json(),
                    domain::REGION_WEEKLY_EVENT_TOTAL_REPORTED_CHANGED.to_string(),
                ),
                SystemTime::from(DateTime::parse_from_rfc3339("2021-01-01T01:01:04Z").unwrap()),
                3,
            )],
            events[1..],
        )
    }
}
