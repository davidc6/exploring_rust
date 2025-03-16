#[cfg(test)]
use mock_instant::thread_local::SystemTime;
use std::time::Duration;
#[cfg(not(test))]
use std::time::SystemTime;

#[derive(Debug)]
pub struct RateLimiter {
    pub quota: u16,
    pub current_timestamp: SystemTime,
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

        self.current_quota += 1;

        if elapsed_time >= one_second {
            // let diff = elapsed_time - one_second;
            // println!("{:?}", diff.as_millis());
            self.current_timestamp = SystemTime::now();
            self.current_quota = 1;
            return false;
        }

        self.current_quota == self.quota
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

        for _ in 0..9 {
            assert!(!rate_limiter.is_quota_exceeded());
        }

        assert!(rate_limiter.is_quota_exceeded());
    }

    #[test]
    fn quota_reset_works() {
        let actions_per_second = 10;
        let mut rate_limiter = RateLimiter::new(actions_per_second);

        // Call the method 9 times
        for _ in 0..actions_per_second - 1 {
            assert!(!rate_limiter.is_quota_exceeded());
        }

        assert!(rate_limiter.current_quota == 9);
        // Quota should not have been exceeded yet since we only called it 9 times (quota is 10)
        assert!(rate_limiter.is_quota_exceeded());

        // Progress by 1s
        mock_instant::thread_local::MockClock::advance_system_time(Duration::from_millis(1000));

        // Quota has been reset
        assert!(!rate_limiter.is_quota_exceeded());

        assert!(rate_limiter.current_quota == 1);
    }
}
