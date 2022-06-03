use std::{cmp::Ordering, str::FromStr};

/// Date struct for holding a 3 part symbol delimited date
#[derive(Debug)]
pub struct Date {
    month: u32,
    day: u32,
    year: u32,
}

impl Date {
    /// Returns `Date` from a given string slice and character
    ///
    /// # Arguments
    ///
    /// * `date` - A string slice that holds a date
    /// * `delimiter` - A `char` delimiter
    ///
    pub fn from_str(date: &str, delimiter: char) -> Date {
        let parts: Vec<&str> = date.split(delimiter).collect();

        Date {
            month: u32::from_str(parts[1]).unwrap(),
            day: u32::from_str(parts[2]).unwrap(),
            year: u32::from_str(parts[0]).unwrap(),
        }
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.year != other.year {
            Some(self.year.cmp(&other.year))
        } else if self.month != other.month {
            Some(self.month.cmp(&other.month))
        } else {
            Some(self.day.cmp(&other.day))
        }
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}
