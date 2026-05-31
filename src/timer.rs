use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
    Idle,
    Pomodoro,
    ShortBreak,
    LongBreak,
}

pub struct Timer {
    total_duration: Duration,
    remaining: Duration,
    running: bool,
    paused: bool,
    finished: Rc<Cell<bool>>,
    last_tick: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            total_duration: Duration::ZERO,
            remaining: Duration::ZERO,
            running: false,
            paused: false,
            finished: Rc::new(Cell::new(false)),
            last_tick: None,
        }
    }

    pub fn start_pomodoro(&mut self, minutes: u64) {
        self.total_duration = Duration::from_secs(minutes * 60);
        self.remaining = self.total_duration;
        self.running = true;
        self.paused = false;
        self.finished.set(false);
        self.last_tick = Some(Instant::now());
    }

    pub fn start_break(&mut self, minutes: u64) {
        self.total_duration = Duration::from_secs(minutes * 60);
        self.remaining = self.total_duration;
        self.running = true;
        self.paused = false;
        self.finished.set(false);
        self.last_tick = Some(Instant::now());
    }

    pub fn pause(&mut self) {
        if self.running && !self.paused {
            // Snapshot remaining time before pausing
            self.remaining = self.remaining();
            self.paused = true;
            self.last_tick = None;
        }
    }

    pub fn resume(&mut self) {
        if self.running && self.paused {
            self.paused = false;
            self.last_tick = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.paused = false;
        self.finished.set(false);
        self.remaining = Duration::ZERO;
        self.last_tick = None;
    }

    pub fn is_idle(&self) -> bool {
        !self.running && !self.finished.get()
    }

    pub fn is_running(&self) -> bool {
        self.running && !self.paused
    }

    pub fn is_paused(&self) -> bool {
        self.running && self.paused
    }

    pub fn remaining(&self) -> Duration {
        if self.running && !self.paused {
            if let Some(last) = self.last_tick {
                let elapsed = last.elapsed();
                if elapsed >= self.remaining {
                    return Duration::ZERO;
                }
                return self.remaining - elapsed;
            }
        }
        self.remaining
    }

    pub fn check_finished(&self) -> bool {
        if self.running && !self.paused && !self.finished.get() {
            if let Some(last) = self.last_tick {
                let elapsed = last.elapsed();
                if elapsed >= self.remaining {
                    self.finished.set(true);
                    return true;
                }
            }
        }
        false
    }

    pub fn progress(&self) -> f64 {
        if self.total_duration.as_secs() == 0 {
            return 0.0;
        }
        let elapsed = self.total_duration.as_secs_f64() - self.remaining().as_secs_f64();
        (elapsed / self.total_duration.as_secs_f64()).clamp(0.0, 1.0)
    }

}
