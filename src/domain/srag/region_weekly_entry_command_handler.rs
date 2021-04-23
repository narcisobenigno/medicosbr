use super::vo::{Case, Region, TotalReported, Year};

pub struct RegionWeeklyCommandHandler {}

impl RegionWeeklyCommandHandler {
    pub(crate) fn handle(&self, _command: RegionWeeklyUpload) {}
}

impl RegionWeeklyCommandHandler {
    fn new() -> RegionWeeklyCommandHandler {
        RegionWeeklyCommandHandler {}
    }
}

pub struct RegionWeeklyUpload {
    pub region: Region,
    pub case: Case,
    pub total_reported: TotalReported,
    pub year: Year,
}

#[cfg(test)]
mod tests {
    use super::super::vo;
    use super::RegionWeeklyCommandHandler;
    use super::RegionWeeklyUpload;

    #[test]
    fn it_uploads_a_new() {
        let handler = RegionWeeklyCommandHandler::new();
        handler.handle(RegionWeeklyUpload {
            region: vo::Region::Alagoas,
            case: vo::Case::SARS,
            total_reported: vo::TotalReported(10),
            year: vo::Year(2019),
        });
    }
}
