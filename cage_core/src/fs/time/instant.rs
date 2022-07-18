use super::*;

/// One monotic instant time, in UTC, after Epoch (1970).
#[derive(Debug, Copy, Clone)]
pub struct Instant {
    // Number of seconds since 1970-01-01 00:00:00.
    second: u64,
    // Number of nano second since in the second.
    nano: u32,
}

impl Instant {
    /// The Unix Epoch: `January 1st 1970, 00:00:00.000 UTC`. All other value is greater than it.
    pub const EPOCH: Instant = Instant { second: 0, nano: 0 };
    /// Create a new date from second and nanosecons.
    pub fn unix(second: u64, nano: u32) -> Self {
        Self {
            second: second + (nano as u64 / 1_000_000_000),
            nano: nano % 1_000_000_000,
        }
    }
    pub fn humain_date(self) -> HumainDate {
        const SECONDS_IN_DAY: u64 = (24 * 60 * 60) as u64;
        let (year, month, day, week_day) = epoch_to_date(self.second / SECONDS_IN_DAY);
        HumainDate {
            year,
            month,
            day,
            week_day,
            hour: (self.second / (60 * 60) % 24) as u8,
            minute: (self.second / 60 % 60) as u8,
            second: (self.second % 60) as u8,
            nano: self.nano,
        }
    }
}
#[test]
fn instant_new() {
    assert_eq!(
        Instant {
            second: 1633111375,
            nano: 123_456_789
        },
        Instant::unix(1633111372, 3_123_456_789)
    );
}
#[test]
fn test_instant2humain_date() {
    // Random date. For a more important test, see tests/time/main.rs
    assert_eq!(
        HumainDate {
            year: 2800,
            month: Month::March,
            day: 19,
            week_day: WeekDay::Sunday,
            hour: 0,
            minute: 39,
            second: 51,
            nano: 123_456_789,
        },
        Instant::unix(26198987991, 123_456_789).humain_date(),
    );
}

impl std::fmt::Display for Instant {
    /// Return the time and date in the format "2006-01-02T15:04:05.999999999Z".
    ///
    /// See <https://rfc-editor.org/rfc/rfc3339.html>.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let HumainDate {
        	year,
        	month,
        	day,
        	hour,
        	minute,
        	second,
        	nano,
        	.. // for week_day
        } = self.humain_date();

        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}Z",
            year, month as u8, day, hour, minute, second, nano
        )
    }
}

impl core::cmp::PartialEq for Instant {
    fn eq(&self, other: &Self) -> bool {
        self.second == other.second && self.nano == other.nano
    }
    fn ne(&self, other: &Self) -> bool {
        self.second != other.second || self.nano != other.nano
    }
}
impl core::cmp::Eq for Instant {}
impl core::cmp::Ord for Instant {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.second
            .cmp(&other.second)
            .then_with(|| self.nano.cmp(&other.nano))
    }
}
impl core::cmp::PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[test]
fn instant_cmp() {
    let sec = 1633111372;
    assert_eq!(true, Instant::unix(sec, 135) == Instant::unix(sec, 135));
    assert_eq!(
        false,
        Instant::unix(sec + 1, 135) == Instant::unix(sec, 135)
    );
    assert_eq!(
        false,
        Instant::unix(sec, 135 + 1) == Instant::unix(sec, 135)
    );

    assert_eq!(false, Instant::unix(sec, 135) != Instant::unix(sec, 135));
    assert_eq!(true, Instant::unix(sec + 1, 135) != Instant::unix(sec, 135));
    assert_eq!(true, Instant::unix(sec, 135 + 1) != Instant::unix(sec, 135));

    assert_eq!(true, Instant::unix(sec, 135 + 1) > Instant::unix(sec, 135));
    assert_eq!(true, Instant::unix(sec + 1, 135) > Instant::unix(sec, 135));
}
