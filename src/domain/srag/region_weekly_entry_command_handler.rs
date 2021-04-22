pub struct RegionWeeklyCommandHandler {}

impl RegionWeeklyCommandHandler {
    fn new() -> RegionWeeklyCommandHandler {
        RegionWeeklyCommandHandler {}
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::RegionWeeklyCommandHandler;
    use std::error::Error;

    #[test]
    fn it_uploads_a_new() -> Result<(), Box<dyn Error>> {
        let handler = RegionWeeklyCommandHandler::new();
        // let result = handler.handle(RegionWeeklyUpload::new(Region::Brasil));
        Ok(())
    }
}
