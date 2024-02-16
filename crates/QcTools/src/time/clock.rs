use std::time::{Duration, Instant};

pub struct Clock {
    time: Instant,
    elapsed: Duration,
    timeSinceStart: Duration,
}

impl Clock {
    pub fn new() -> Self {
        let startTime = Instant::now();
        Self {
            time: Instant::now(),
            elapsed: Duration::from_secs(0),
            timeSinceStart: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self) {
        self.elapsed = self.time.elapsed();
        self.time = Instant::now();

        self.timeSinceStart += self.elapsed;
    }

    pub fn getFrameRate(&self) -> i32 {
        if self.elapsed == Duration::ZERO {
            i32::MAX
        } else {
            (1. / self.elapsed.as_secs_f32()) as i32
        }
    }

    pub fn getDeltaTime(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }

    pub fn getTimeSinceStart(&self) -> f32 {
        self.timeSinceStart.as_secs_f32()
    }
}
