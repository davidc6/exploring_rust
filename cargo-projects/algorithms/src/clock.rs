use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // hours
        let h = &self.hours;
        let mut hh = format!("{}", h);

        if h < &10 {
            hh = format!("0{}", h);
        }

        // minutes
        let m = &self.minutes;
        let mut mm = format!("{}", m);

        if m < &10 {
            mm = format!("0{}", m);
        }

        write!(f, "{}:{}", hh, mm)
    }
}

struct Hours(i32);

impl Hours {
    fn new(hours: i32, minutes_hours: i32) -> Self {
        let mut hours = if hours >= 0 {
            hours + minutes_hours
        } else {
            hours
        };

        if hours > 24 {
            let whole_hours = hours / 24;
            hours -= whole_hours * 24;
        }

        if hours == 24 {
            hours = 0;
        }

        if hours < 0 {
            let whole_hours = hours / 24;
            let diff = if whole_hours == -1 {
                24 + (24 + hours)
            } else {
                24 + (hours + whole_hours * 24)
            };
            dbg!(diff);
            hours = diff;
        }

        Hours(hours)
    }

    fn value(self) -> i32 {
        self.0
    }
}

struct Minutes {
    minutes: i32,
    hours: i32,
}

impl Minutes {
    fn new(minutes: i32, hours: i32) -> Self {
        let mut minutes = minutes;

        if minutes >= 60 {
            let whole_hours = minutes / 60;
            minutes -= whole_hours * 60;

            return Minutes {
                minutes,
                hours: whole_hours,
            };
        }

        if minutes < 0 {
            let mut minutes_day = 24 * 60;
            minutes_day += (hours * 60) + minutes;

            // hours
            let mut total_hours = minutes_day / 60;
            let diff_minutes = minutes_day - total_hours * 60;
            // let diff = minutes_day - 24 * 60;

            if total_hours > 24 {
                total_hours -= 24;
            }

            return Minutes {
                minutes: diff_minutes,
                hours: total_hours - 1,
            };
        }

        Minutes { minutes, hours: 0 }
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_s = Minutes::new(minutes, hours);

        let hours = Hours::new(hours, minutes_s.hours).value();

        Clock {
            hours,
            minutes: minutes_s.minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        println!("{:?} {:?}", self.minutes, self.hours);
        let total_minutes = self.hours * 60 + self.minutes + minutes; // -1
        let mut total_hours = total_minutes / 60; // rounded

        let minutes = total_minutes - (total_hours * 60);

        if total_hours >= 24 {
            let temp_hours = total_hours / 24;
            total_hours -= temp_hours * 24;
        } else if minutes < 0 {
            let total_minutes = 60 * 24 + minutes;
            let total_hours = total_minutes / 60; // 23
            let diff_minutes = 60 - (60 * 24 - total_minutes);

            return Clock {
                hours: total_hours,
                minutes: diff_minutes,
            };
        }

        Clock {
            hours: total_hours,
            minutes,
        }
    }
}

#[cfg(test)]
mod clock_tests {
    use super::*;

    #[test]

    fn on_the_hour() {
        assert_eq!(Clock::new(8, 0).to_string(), "08:00");
    }

    #[test]

    fn past_the_hour() {
        assert_eq!(Clock::new(11, 9).to_string(), "11:09");
    }

    #[test]

    fn midnight_is_zero_hours() {
        assert_eq!(Clock::new(24, 0).to_string(), "00:00");
    }

    #[test]

    fn hour_rolls_over() {
        assert_eq!(Clock::new(25, 0).to_string(), "01:00");
    }

    #[test]

    fn hour_rolls_over_continuously() {
        assert_eq!(Clock::new(100, 0).to_string(), "04:00");
    }

    #[test]

    fn sixty_minutes_is_next_hour() {
        assert_eq!(Clock::new(1, 60).to_string(), "02:00");
    }

    #[test]

    fn minutes_roll_over() {
        assert_eq!(Clock::new(0, 160).to_string(), "02:40");
    }

    #[test]

    fn minutes_roll_over_continuously() {
        assert_eq!(Clock::new(0, 1723).to_string(), "04:43");
    }

    #[test]

    fn hours_and_minutes_roll_over() {
        assert_eq!(Clock::new(25, 160).to_string(), "03:40");
    }

    #[test]

    fn hours_and_minutes_roll_over_continuously() {
        assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
    }

    #[test]

    fn hours_and_minutes_roll_over_to_exactly_midnight() {
        assert_eq!(Clock::new(72, 8640).to_string(), "00:00");
    }

    #[test]

    fn negative_hour() {
        assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
    }

    #[test]

    fn negative_hour_roll_over() {
        assert_eq!(Clock::new(-25, 00).to_string(), "23:00");
    }

    #[test]

    fn negative_hour_roll_over_continuously() {
        assert_eq!(Clock::new(-91, 00).to_string(), "05:00");
    }

    #[test]

    fn negative_minutes() {
        assert_eq!(Clock::new(1, -40).to_string(), "00:20");
    }

    #[test]

    fn negative_minutes_roll_over() {
        assert_eq!(Clock::new(1, -160).to_string(), "22:20");
    }

    #[test]

    fn negative_minutes_roll_over_continuously() {
        assert_eq!(Clock::new(1, -4820).to_string(), "16:40");
    }

    #[test]

    fn negative_sixty_minutes_is_prev_hour() {
        assert_eq!(Clock::new(2, -60).to_string(), "01:00");
    }

    #[test]

    fn negative_one_twenty_minutes_is_two_prev_hours() {
        assert_eq!(Clock::new(1, -120).to_string(), "23:00");
    }

    #[test]

    fn negative_hour_and_minutes_both_roll_over() {
        assert_eq!(Clock::new(-25, -160).to_string(), "20:20");
    }

    #[test]

    fn negative_hour_and_minutes_both_roll_over_continuously() {
        assert_eq!(Clock::new(-121, -5810).to_string(), "22:10");
    }

    #[test]

    fn zero_hour_and_negative_minutes() {
        assert_eq!(Clock::new(0, -22).to_string(), "23:38");
    }

    //

    // Clock Math

    //

    #[test]

    fn add_minutes() {
        let clock = Clock::new(10, 0).add_minutes(3);

        assert_eq!(clock.to_string(), "10:03");
    }

    #[test]

    fn add_no_minutes() {
        let clock = Clock::new(6, 41).add_minutes(0);

        assert_eq!(clock.to_string(), "06:41");
    }

    #[test]

    fn add_to_next_hour() {
        let clock = Clock::new(0, 45).add_minutes(40);

        assert_eq!(clock.to_string(), "01:25");
    }

    #[test]

    fn add_more_than_one_hour() {
        let clock = Clock::new(10, 0).add_minutes(61);

        assert_eq!(clock.to_string(), "11:01");
    }

    #[test]

    fn add_more_than_two_hours_with_carry() {
        let clock = Clock::new(0, 45).add_minutes(160);

        assert_eq!(clock.to_string(), "03:25");
    }

    #[test]

    fn add_across_midnight() {
        let clock = Clock::new(23, 59).add_minutes(2);

        assert_eq!(clock.to_string(), "00:01");
    }

    #[test]

    fn add_more_than_one_day() {
        let clock = Clock::new(5, 32).add_minutes(1500);

        assert_eq!(clock.to_string(), "06:32");
    }

    #[test]

    fn add_more_than_two_days() {
        let clock = Clock::new(1, 1).add_minutes(3500);

        assert_eq!(clock.to_string(), "11:21");
    }

    #[test]

    fn subtract_minutes() {
        let clock = Clock::new(10, 3).add_minutes(-3);

        assert_eq!(clock.to_string(), "10:00");
    }

    #[test]

    fn subtract_to_previous_hour() {
        let clock = Clock::new(10, 3).add_minutes(-30);

        assert_eq!(clock.to_string(), "09:33");
    }

    #[test]

    fn subtract_more_than_an_hour() {
        let clock = Clock::new(10, 3).add_minutes(-70);

        assert_eq!(clock.to_string(), "08:53");
    }

    #[test]

    fn subtract_across_midnight() {
        let clock = Clock::new(0, 3).add_minutes(-4);

        assert_eq!(clock.to_string(), "23:59");
    }

    #[test]

    fn subtract_more_than_two_hours() {
        let clock = Clock::new(0, 0).add_minutes(-160);

        assert_eq!(clock.to_string(), "21:20");
    }

    #[test]

    fn subtract_more_than_two_hours_with_borrow() {
        let clock = Clock::new(6, 15).add_minutes(-160);

        assert_eq!(clock.to_string(), "03:35");
    }

    #[test]

    fn subtract_more_than_one_day() {
        let clock = Clock::new(5, 32).add_minutes(-1500);

        assert_eq!(clock.to_string(), "04:32");
    }

    #[test]

    fn subtract_more_than_two_days() {
        let clock = Clock::new(2, 20).add_minutes(-3000);

        assert_eq!(clock.to_string(), "00:20");
    }

    //

    // Test Equality

    //

    #[test]

    fn compare_clocks_for_equality() {
        assert_eq!(Clock::new(15, 37), Clock::new(15, 37));
    }

    #[test]

    fn compare_clocks_a_minute_apart() {
        assert_ne!(Clock::new(15, 36), Clock::new(15, 37));
    }

    #[test]

    fn compare_clocks_an_hour_apart() {
        assert_ne!(Clock::new(14, 37), Clock::new(15, 37));
    }

    #[test]

    fn compare_clocks_with_hour_overflow() {
        assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
    }

    #[test]

    fn compare_clocks_with_hour_overflow_by_several_days() {
        assert_eq!(Clock::new(99, 11), Clock::new(3, 11));
    }

    #[test]

    fn compare_clocks_with_negative_hour() {
        // dbg!("FINAL");
        // dbg!(Clock::new(-2, 40));
        // dbg!(Clock::new(22, 40));
        assert_eq!(Clock::new(-2, 40), Clock::new(22, 40));
    }

    #[test]

    fn compare_clocks_with_negative_hour_that_wraps() {
        assert_eq!(Clock::new(-31, 3), Clock::new(17, 3));
    }

    #[test]

    fn compare_clocks_with_negative_hour_that_wraps_multiple_times() {
        assert_eq!(Clock::new(-83, 49), Clock::new(13, 49));
    }

    #[test]

    fn compare_clocks_with_minutes_overflow() {
        assert_eq!(Clock::new(0, 1441), Clock::new(0, 1));
    }

    #[test]

    fn compare_clocks_with_minutes_overflow_by_several_days() {
        assert_eq!(Clock::new(2, 4322), Clock::new(2, 2));
    }

    #[test]

    fn compare_clocks_with_negative_minute() {
        assert_eq!(Clock::new(3, -20), Clock::new(2, 40));
    }

    #[test]

    fn compare_clocks_with_negative_minute_that_wraps() {
        assert_eq!(Clock::new(5, -1490), Clock::new(4, 10));
    }

    #[test]

    fn compare_clocks_with_negative_minute_that_wraps_multiple() {
        assert_eq!(Clock::new(6, -4305), Clock::new(6, 15));
    }

    #[test]

    fn compare_clocks_with_negative_hours_and_minutes() {
        assert_eq!(Clock::new(-12, -268), Clock::new(7, 32));
    }

    #[test]

    fn compare_clocks_with_negative_hours_and_minutes_that_wrap() {
        assert_eq!(Clock::new(-54, -11_513), Clock::new(18, 7));
    }

    #[test]

    fn compare_full_clock_and_zeroed_clock() {
        assert_eq!(Clock::new(24, 0), Clock::new(0, 0));
    }
}
