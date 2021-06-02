use crate::common;
use crate::common::es;
use crate::domain;

#[derive(Default)]
pub struct RegionalWeeklyReport {}

impl RegionalWeeklyReport {
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
