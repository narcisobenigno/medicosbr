use crate::common::es::{Aggregate, Event, Version, WrittenEvent};
use crate::domain::{RegionWeeklyEventDetected, RegionWeeklyUpload};

#[derive(Default)]
pub struct RegionalWeeklyReport {}

impl RegionalWeeklyReport {
    pub(crate) fn upload(&self, report: RegionWeeklyUpload) -> Vec<Event> {
        vec![Event::new(
            &report.aggregate_id,
            &Version::from(1),
            &RegionWeeklyEventDetected(
                report.region,
                report.case,
                report.total_reported,
                report.year_week,
            ),
        )]
    }
}

impl Aggregate for RegionalWeeklyReport {
    fn handle(&mut self, event: &WrittenEvent) {}
}
