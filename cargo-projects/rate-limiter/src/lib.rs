#[cfg(feature = "mock-instant")]
use mock_instant::thread_local::SystemTime;
use std::time::Duration;
#[cfg(not(feature = "mock-instant"))]
use std::time::SystemTime;

#[derive(Debug)]
pub struct RateLimiter {
    quota: u16,
    current_timestamp: SystemTime,
    current_quota: u16,
}

impl RateLimiter {
    pub fn new(actions_per_second: u16) -> Self {
        RateLimiter {
            quota: actions_per_second,
            current_quota: 0,
            current_timestamp: SystemTime::now(),
        }
    }

    pub fn is_quota_exceeded(&mut self) -> bool {
        let elapsed_time = self.current_timestamp.elapsed().unwrap();
        let one_second = Duration::from_millis(1000);

        if elapsed_time >= one_second {
            // let diff = elapsed_time - one_second;
            // println!("{:?}", diff.as_millis());
            self.current_timestamp = SystemTime::now();
            self.current_quota = 1;
            false
        } else if self.current_quota == self.quota {
            true
        } else {
            self.current_quota += 1;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialisation_works() {
        let mut rate_limiter = RateLimiter::new(10);
        assert!(!rate_limiter.is_quota_exceeded());
    }

    #[test]
    fn quota_exceed_works() {
        let mut rate_limiter = RateLimiter::new(10);

        for _ in 0..10 {
            assert!(!rate_limiter.is_quota_exceeded());
        }

        assert!(rate_limiter.is_quota_exceeded());
    }

    #[test]
    #[cfg(feature = "mock-instant")]
    fn quota_reset_works() {
        let actions_per_second = 10;
        let mut rate_limiter = RateLimiter::new(actions_per_second);

        for _ in 0..actions_per_second - 1 {
            assert!(!rate_limiter.is_quota_exceeded());
        }

        assert!(rate_limiter.current_quota == 9);
        assert!(!rate_limiter.is_quota_exceeded());

        mock_instant::thread_local::MockClock::advance_system_time(Duration::from_millis(1000));

        assert!(!rate_limiter.is_quota_exceeded());
    }
}
