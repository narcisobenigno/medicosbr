use crate::domain::doctors_stats;
use crate::domain::doctors_stats::EventApplication;
use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DoctorsStatsEvent {
    DoctorsPer1000Received(DoctorsPer1000Received),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoctorsPer1000Received {
    id: String,
}

impl DomainEvent<doctors_stats::DoctorsStats> for DoctorsStatsEvent {
    fn apply(self, stats: &mut doctors_stats::DoctorsStats) {
        match self {
            DoctorsStatsEvent::DoctorsPer1000Received(e) => stats.apply(e),
        }
    }
}

impl DomainEvent<doctors_stats::DoctorsStats> for DoctorsPer1000Received {
    fn apply(self, stats: &mut doctors_stats::DoctorsStats) {
        stats.apply(self)
    }
}
