pub enum ScheduleMode {
  /// On reset, add interval to current time.  Good for many non-exact applications.
  DELAY,
  /// On reset, add interval to prior next_time.  Maintains average rate, at the cost of pileups.
  PERIOD,
  /// On reset, add interval to prior next_time, until it's in the future.  Maintains average rate, UNTIL we have to skip one or more events entirely.  A pretty safe option.
  PERIOD_SKIP,
}

pub type Micros = u64; //RAINY Optionize somehow?

//RAINY Also permit non-repeating timers?
/**
 * A repeating timer you check periodically to see if it's going off.
```
let mut ik_timer = Schedule::new(ScheduleMode::PERIOD_SKIP, clock.get_as_micros(), 1000000*10);
ik_timer.enable(clock.get_as_micros());
loop {
  // ...
  if ik_timer.check(clock.get_as_micros()) {
    // Do stuff
  }
  // ...
}
```
 */
pub struct Schedule {
  pub mode: ScheduleMode,
  pub next_time: Option<Micros>,
  pub interval: Micros,
}

impl Schedule {
  pub fn default() -> Self {
    return Schedule {
      mode: ScheduleMode::PERIOD_SKIP,
      next_time: Some(0),
      interval: 1_000_000, // 1 second interval
    };
  }

  pub fn new(mode: ScheduleMode, now: Micros, interval: Micros) -> Self {
    return Schedule {
      mode,
      next_time: Some(now),
      interval,
    };
  }

  /// Check, but do not update, timer.
  pub fn peek(&self, time: Micros) -> bool {
    match self.next_time {
        Some(nt) => time >= nt, //DUMMY May not survive a wraparound
        None => false,
    }
  }

  /// Check and update timer
  pub fn check(&mut self, time: Micros) -> bool {
    return match self.next_time {
      Some(nt) => {
        if time >= nt { //DUMMY May not survive a wraparound
          match self.mode {
            ScheduleMode::DELAY => self.next_time = Some(time.wrapping_add(self.interval)),
            ScheduleMode::PERIOD => self.next_time = Some(nt.wrapping_add(self.interval)),
            ScheduleMode::PERIOD_SKIP => {
              //CHECK This is a bit opaque; it should be (N - ((N-X) % I) + I) , and check it works right
              self.next_time = Some(time.wrapping_sub(time.wrapping_sub(nt) % self.interval).wrapping_add(self.interval));
            },
          };
          true
        } else {
          false
        }
      },
      None => false,
    };
  }

  /// Clears timer.
  pub fn disable(&mut self) {
    self.next_time = None;
  }

  /// Sets timer for `interval` from `now`.
  pub fn enable(&mut self, now: Micros) {
    self.next_time = Some(now.wrapping_add(self.interval));
  }
}