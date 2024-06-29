use std::{cmp::Ordering, fmt};

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format_args!(
                "{}:{}",
                format!("{:0>2}", &self.hours),
                format!("{:0>2}", &self.minutes)
            )
        )
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // full clock
        if hours == 24 && minutes == 0 {
            return Clock {
                hours: 0,
                minutes: 0,
            };
        }

        // 100 1000                 | example hours and minutes
        // 100 * 60 = 6000 minutes  | total hours to minutes
        let hours_to_minutes = hours * 60;
        // 1000 + 6000 = 7000       | total minutes
        let total_minutes = minutes + hours_to_minutes;
        // 7000 / 60 = 116 (hours)  | round hours
        let round_hours = total_minutes / 60;
        // 7000 - 116 * 60 = 40     | minutes leftover
        let mut minutes_leftover = total_minutes - round_hours * 60;
        // 116 / 24 = 4                | work out days
        let days = round_hours / 24;
        // 116 * 60 - 24 * 4 * 60 = 6960 -  5760 = 1200 / 60 = 20 | hours
        let mut time = if round_hours <= 0 {
            minutes_leftover += 60;
            let t = ((round_hours * 60) - (24 * days * 60)) / 60;
            24 + t - 1
        } else {
            ((round_hours * 60) - (24 * days * 60)) / 60
        };

        match minutes_leftover.cmp(&60) {
            Ordering::Equal => {
                time += 1;
                minutes_leftover = 0;
            }
            Ordering::Greater => {
                time += 1;
                minutes_leftover -= 60;
            }
            Ordering::Less => {}
        }

        if time == 24 {
            time = 0;
        }

        Clock {
            hours: time,
            minutes: minutes_leftover,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.hours * 60 + self.minutes + minutes;
        let mut round_hours = total_minutes / 60; // rounded
        let minutes_leftover = total_minutes - (round_hours * 60);

        if round_hours >= 24 {
            let temp_hours = round_hours / 24;
            round_hours -= temp_hours * 24;
        } else if minutes_leftover < 0 {
            let total_minutes = 60 * 24 + minutes_leftover;
            let mut round_hours = total_minutes / 60 + round_hours;
            let diff_minutes = 60 - (60 * 24 - total_minutes);

            round_hours = if round_hours == 24 || round_hours == -24 {
                0
            } else {
                round_hours
            };

            return Clock {
                hours: round_hours,
                minutes: diff_minutes,
            };
        }

        Clock {
            hours: round_hours,
            minutes: minutes_leftover,
        }
    }
}

#[cfg(test)]
mod clock_tests_2 {
    use super::*;

    #[test]
    fn hello_hi_yeah() {
        assert_eq!(Clock::new(100, 1000).to_string(), "20:40");
    }

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
