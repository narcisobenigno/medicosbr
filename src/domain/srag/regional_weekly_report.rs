use crate::common;
use crate::common::es;
use crate::domain;
use crate::domain::srag::vo;
use std::str::FromStr;

#[derive(Default)]
pub struct RegionalWeeklyReport {}

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
    ) -> Result<Vec<es::Event>, common::Error> {
        Ok(vec![es::Event::new(
            &report.aggregate_id,
            &es::Version::from(1),
            &domain::RegionWeeklyEventDetected(
                report.region,
                report.case,
                report.total_reported,
                report.year_week,
            ),
        )])
    }
}

impl es::Aggregate for RegionalWeeklyReport {
    fn handle(&mut self, event: &es::WrittenEvent) {}
}
