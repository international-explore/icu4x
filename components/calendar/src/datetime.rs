// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::types::Time;
use crate::{AsCalendar, Date, Iso};

/// A date+time for a given calendar.
///
/// This can work with wrappers around [`Calendar`](crate::Calendar) types,
/// e.g. `Rc<C>`, via the [`AsCalendar`] trait, much like
/// [`Date`].
///
/// ```rust
/// use icu::calendar::{DateTime, types::IsoHour, types::IsoMinute, types::IsoSecond};
///
/// // Example: Construction of ISO datetime from integers.
/// let datetime_iso = DateTime::new_iso_datetime_from_integers(1970, 1, 2, 13, 1, 0)
///     .expect("Failed to initialize ISO DateTime instance.");
///
/// assert_eq!(datetime_iso.date.year().number, 1970);
/// assert_eq!(datetime_iso.date.month().number, 1);
/// assert_eq!(datetime_iso.date.day_of_month().0, 2);
/// assert_eq!(datetime_iso.time.hour.number(), 13);
/// assert_eq!(datetime_iso.time.minute.number(), 1);
/// assert_eq!(datetime_iso.time.second.number(), 0);
/// ```
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct DateTime<A: AsCalendar> {
    pub date: Date<A>,
    pub time: Time,
}

impl<A: AsCalendar> DateTime<A> {
    pub fn new(date: Date<A>, time: Time) -> Self {
        DateTime { date, time }
    }

    /// Construct a DateTime from an ISO datetime and some calendar representation
    #[inline]
    pub fn new_from_iso(iso: DateTime<Iso>, calendar: A) -> Self {
        let date = Date::new_from_iso(iso.date, calendar);
        DateTime {
            date,
            time: iso.time,
        }
    }

    /// Convert the DateTime to an ISO DateTime
    #[inline]
    pub fn to_iso(&self) -> DateTime<Iso> {
        DateTime {
            date: self.date.to_iso(),
            time: self.time,
        }
    }

    /// Convert the DateTime to a DateTime in a different calendar
    #[inline]
    pub fn to_calendar<A2: AsCalendar>(&self, calendar: A2) -> DateTime<A2> {
        DateTime {
            date: self.date.to_calendar(calendar),
            time: self.time,
        }
    }
}
