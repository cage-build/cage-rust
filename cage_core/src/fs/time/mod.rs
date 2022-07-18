mod epoch_to_date;
use epoch_to_date::epoch_to_date;

mod instant;
pub use instant::Instant;

/// A humain date, used to dispal the time.
#[derive(Debug, PartialEq)]
pub struct HumainDate {
    pub year: u64,
    pub month: Month,
    pub day: u8,
    pub week_day: WeekDay,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nano: u32,
}

/// A month, used by [`HumainDate`].
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
