#[cfg(test)]
use mock_instant::thread_local::SystemTime;
#[cfg(not(test))]
use std::time::SystemTime;
use std::{
    cell::{Cell, RefCell},
    time::Duration,
};

#[derive(Debug)]
pub struct RateLimiter {
    pub quota: u16,
    pub current_timestamp: RefCell<SystemTime>,
    current_quota: Cell<u16>,
}

impl RateLimiter {
    pub fn new(actions_per_second: u16) -> Self {
        RateLimiter {
            quota: actions_per_second,
            current_quota: Cell::new(0),
            current_timestamp: RefCell::new(SystemTime::now()),
        }
    }

    pub fn is_quota_exceeded(&self) -> bool {
        let elapsed_time = self.current_timestamp.borrow().elapsed().unwrap();
        let one_second = Duration::from_millis(1000);

        self.current_quota
            .swap(&Cell::new(self.current_quota.take() + 1));

        if elapsed_time >= one_second {
            self.current_timestamp.replace(SystemTime::now());
            self.current_quota.swap(&Cell::new(1));
            return false;
        }

        self.current_quota.get() == self.quota
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialisation_works() {
        let rate_limiter = RateLimiter::new(10);
        assert!(!rate_limiter.is_quota_exceeded());
    }

    #[test]
    fn quota_exceed_works() {
        let rate_limiter = RateLimiter::new(10);

        for _ in 0..9 {
            assert!(!rate_limiter.is_quota_exceeded());
        }

        assert!(rate_limiter.is_quota_exceeded());
    }

    #[test]
    fn quota_reset_works() {
        let actions_per_second = 10;
        let rate_limiter = RateLimiter::new(actions_per_second);

        // Call the method 9 times
        for _ in 0..actions_per_second - 1 {
            assert!(!rate_limiter.is_quota_exceeded());
        }

        assert!(rate_limiter.current_quota.get() == 9);
        // Quota should not have been exceeded yet since we only called it 9 times (quota is 10)
        assert!(rate_limiter.is_quota_exceeded());

        // Progress by 1s
        mock_instant::thread_local::MockClock::advance_system_time(Duration::from_millis(1000));

        // Quota has been reset
        assert!(!rate_limiter.is_quota_exceeded());
        assert!(rate_limiter.current_quota.get() == 1);
    }
}
