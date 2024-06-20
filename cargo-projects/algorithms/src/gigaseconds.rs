use time::Duration;
use time::PrimitiveDateTime as DateTime;

pub fn gigaseconds_v0(start: DateTime) -> time::PrimitiveDateTime {
    let giga_seconds = Duration::seconds(1000000000);
    start + giga_seconds
}

#[cfg(test)]
mod gigaseconds_testst {
    use super::*;
    use time::PrimitiveDateTime as DateTime;

    fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
        use time::{Date, Time};
        DateTime::new(
            Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
            Time::from_hms(hour, minute, second).unwrap(),
        )
    }

    #[test]
    fn gigaseconds_works() {
        let start_date = dt(2011, 4, 25, 0, 0, 0);

        assert_eq!(gigaseconds_v0(start_date), dt(2043, 1, 1, 1, 46, 40));
    }
}
