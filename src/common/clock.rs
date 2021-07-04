use std::ops::Add;
use std::time::{Duration, SystemTime};

pub trait Clock {
    fn now(&mut self) -> SystemTime;
}

pub struct InMemoryClock {
    now: SystemTime,
}

impl InMemoryClock {
    pub fn new(now: SystemTime) -> Self {
        InMemoryClock { now }
    }

    pub fn tick(&mut self) {
        self.now = self.now.add(Duration::from_secs(1));
    }
}

impl Clock for InMemoryClock {
    fn now(&mut self) -> SystemTime {
        let now = self.now;
        self.tick();
        now
    }
}
