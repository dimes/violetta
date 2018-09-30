use std::ops::Add;
use std::ops::Sub;
use std::time::Duration;
use std::time::Instant;

pub struct Clock {
    start: Instant,
    pause_duration: Duration,
    time_of_pause: Option<Instant>,
}

impl Clock {
    pub fn new() -> Clock {
        return Clock {
            start: Instant::now(),
            pause_duration: Duration::from_millis(0),
            time_of_pause: None,
        };
    }

    pub fn pause(&mut self) {
        match self.time_of_pause {
            Some(_) => return,
            None => {
                self.time_of_pause = Some(Instant::now());
            }
        }
    }

    pub fn resume(&mut self) {
        if let Some(instant) = self.time_of_pause {
            let pause_duration = Instant::now().duration_since(instant);
            self.pause_duration = self.pause_duration.add(pause_duration);
            self.time_of_pause = None;
        }
    }

    pub fn is_paused(&self) -> bool {
        return self.time_of_pause.is_some();
    }

    pub fn game_time(&self) -> Duration {
        let now = Instant::now();
        let mut duration = now.duration_since(self.start).sub(self.pause_duration);

        if let Some(instant) = self.time_of_pause {
            let pause_duration = now.duration_since(instant);
            duration = duration.sub(pause_duration);
        }

        return duration;
    }
}
