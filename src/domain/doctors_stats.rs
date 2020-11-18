use crate::domain::doctors_stats_events;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DoctorsStats {}

pub trait EventApplication<E> {
    fn apply(&mut self, event: E);
}

impl EventApplication<doctors_stats_events::DoctorsPer1000Received> for DoctorsStats {
    fn apply(&mut self, event: doctors_stats_events::DoctorsPer1000Received) {}
}

impl Default for DoctorsStats {
    fn default() -> Self {
        Self {}
    }
}

impl Aggregate for DoctorsStats {
    fn aggregate_type() -> &'static str {
        "doctor_stats"
    }
}
