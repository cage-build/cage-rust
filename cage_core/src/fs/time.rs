use core::cmp::{Eq, Ord, Ordering, PartialOrd};

/// One monotic instant time, in UTC. All value are after Epoch (1970).
#[derive(Debug, Copy, Clone)]
pub struct Time(u64, u32);
impl Time {
    /// The Unic Epoch: `January 1st 1970, 00:00:00.000 UTC`. All other value is greater than it.
    pub const EPOCH: Time = Time(0, 0);
    /// Create a new date from second and nanosecons
    pub fn new(sec: u64, nano: u32) -> Time {
        Self(sec + (nano as u64 / 1_000_000_000), nano % 1_000_000_000)
    }
}
#[test]
fn time_new() {
    assert_eq!(
        Time(1633111375, 123_456_789),
        Time::new(1633111372, 3_123_456_789)
    );
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0 || self.1 != other.1
    }
}
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Time {}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else {
            self.1.cmp(&other.1)
        }
    }
}
#[test]
fn time_cmp() {
    let sec = 1633111372;
    assert_eq!(true, Time(sec, 135) == Time(sec, 135));
    assert_eq!(false, Time(sec + 1, 135) == Time(sec, 135));
    assert_eq!(false, Time(sec, 135 + 1) == Time(sec, 135));

    assert_eq!(false, Time(sec, 135) != Time(sec, 135));
    assert_eq!(true, Time(sec + 1, 135) != Time(sec, 135));
    assert_eq!(true, Time(sec, 135 + 1) != Time(sec, 135));

    assert_eq!(true, Time(sec, 135 + 1) > Time(sec, 135));
    assert_eq!(true, Time(sec + 1, 135) > Time(sec, 135));
}

// debug
// display
// to format: yyyy-mm-dd hh:mm:ss.mmm
// to HTTP format
// to JSON/Js format
// used date for test:
// Array [ Date Fri Oct 01 2021 20:02:52 GMT+0200 (heure d’été d’Europe centrale), 1633111372917 ]

// from sys time use std::time::Instant; and use std::time::SystemTime;
