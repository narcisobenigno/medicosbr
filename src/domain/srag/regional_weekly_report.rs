use std::str::FromStr;

use crate::common;
use crate::common::es;
use crate::common::es::{Payload, WrittenEvent};
use crate::domain;
use crate::domain::srag::vo;

#[derive(Default)]
pub struct RegionalWeeklyReport {
    id: es::AggregateId,
    total_reported: vo::TotalReported,
    version: es::Version,
}

impl RegionalWeeklyReport {
    pub fn new_id(region: vo::Region, year_week: vo::YearWeek) -> es::AggregateId {
        es::AggregateId::from(&uuid::Uuid::new_v5(
            &uuid::Uuid::from_str("a385bf4a-e6c0-48ee-a5e0-701e92f1e592").unwrap(),
            format!("{}{}", region.name(), year_week).as_bytes(),
        ))
    }

    pub(crate) fn upload(
        &self,
        report: domain::RegionWeeklyUpload,
    ) -> Result<es::VersionedEvents, common::Error> {
        if self.id.is_nil() {
            return Ok(es::VersionedEvents::new(
                es::Version::default(),
                vec![es::Event::new(
                    &report.aggregate_id,
                    &domain::RegionWeeklyEventDetected(
                        report.region,
                        report.case,
                        report.total_reported,
                        report.year_week,
                    ),
                )],
            ));
        }

        return Ok(es::VersionedEvents::new(
            self.version.clone(),
            vec![es::Event::new(
                &self.id,
                &domain::RegionWeeklyEventTotalReportedChanged(report.total_reported),
            )],
        ));
    }
}

impl es::Aggregate for RegionalWeeklyReport {
    fn handle(&mut self, event: &WrittenEvent) {
        match event.name.as_str() {
            domain::REGION_WEEKLY_EVENT_DETECTED_TYPE => {
                let payload =
                    domain::RegionWeeklyEventDetected::unmarshal_json(event.payload.as_str())
                        .unwrap();
                self.id = event.aggregate_id.clone();
                self.total_reported = payload.2;
                self.version = event.version.clone()
            }
            _ => {}
        }
    }
}
