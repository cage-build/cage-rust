use super::*;

const DAYS_IN_400_YEARS: u64 = 400 * 365 + 100 - 4 + 1;
const DAYS_IN_4_YEARS: u64 = 365 * 4 + 1;

/// Return `(year, month, day)`.
pub fn epoch_to_date(days_since_epoch: u64) -> (u64, Month, u8, WeekDay) {
    let week_day = match days_since_epoch % 7 {
        0 => WeekDay::Thursday,
        1 => WeekDay::Friday,
        2 => WeekDay::Saturday,
        3 => WeekDay::Sunday,
        4 => WeekDay::Monday,
        5 => WeekDay::Tuesday,
        _ => WeekDay::Wednesday,
    };

    let mut days_in_cycle = days_since_epoch % DAYS_IN_400_YEARS;
    if days_in_cycle >= 332 * 365 + 332 / 4 - 2 {
        days_in_cycle += 3;
    } else if days_in_cycle >= 232 * 365 + 232 / 4 - 1 {
        days_in_cycle += 2;
    } else if days_in_cycle >= 132 * 365 + 132 / 4 {
        days_in_cycle += 1;
    }
    let mut year =
        1970 + days_since_epoch / DAYS_IN_400_YEARS * 400 + days_in_cycle / DAYS_IN_4_YEARS * 4;

    let mut day = days_in_cycle % DAYS_IN_4_YEARS;
    loop {
        if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
            if day < 366 {
                break;
            }
            day -= 366;
        } else {
            if day < 365 {
                break;
            }
            day -= 365;
        }
        year += 1;
    }

    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        // Leap year
        if day > 31 + 29 - 1 {
            // After leap day; pretend it wasn't there.
            day -= 1;
        } else if day == 31 + 29 - 1 {
            // The leap day.
            return (year, Month::February, 29, week_day);
        }
    }

    let month;
    if day < 31 {
        month = Month::January;
        day = day;
    } else if day < 31 + 28 {
        month = Month::February;
        day = day - (31);
    } else if day < 31 + 28 + 31 {
        month = Month::March;
        day = day - (31 + 28);
    } else if day < 31 + 28 + 31 + 30 {
        month = Month::April;
        day = day - (31 + 28 + 31);
    } else if day < 31 + 28 + 31 + 30 + 31 {
        month = Month::May;
        day = day - (31 + 28 + 31 + 30);
    } else if day < 31 + 28 + 31 + 30 + 31 + 30 {
        month = Month::June;
        day = day - (31 + 28 + 31 + 30 + 31);
    } else if day < 31 + 28 + 31 + 30 + 31 + 30 + 31 {
        month = Month::July;
        day = day - (31 + 28 + 31 + 30 + 31 + 30);
    } else if day < 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 {
        month = Month::August;
        day = day - (31 + 28 + 31 + 30 + 31 + 30 + 31);
    } else if day < 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 {
        month = Month::September;
        day = day - (31 + 28 + 31 + 30 + 31 + 30 + 31 + 31);
    } else if day < 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 {
        month = Month::October;
        day = day - (31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30);
    } else if day < 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30 {
        month = Month::November;
        day = day - (31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31);
    } else {
        month = Month::December;
        day = day - (31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30);
    };

    (year, month, day as u8 + 1, week_day)
}
