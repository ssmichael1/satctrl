/// A module for handling time and date
/// conversions.  Time is stored natively as
/// the number of microseconds since the
/// Unix epoch (1970-01-01 00:00:00 UTC)
/// with leap seconds accounted for.
///
/// The Instant struct provides methods for
/// converting to and from Unix time, GPS time,
/// Julian Date, Modified Julian Date, and
/// Gregorian calendar date.
///
#[derive(Clone, Copy)]
pub struct Instant {
    /// The number of microseconds since
    /// J2000 (2000-01-01 12:00:00 TT)
    pub(crate) raw: i64,
}

/// For conversian between Julian day and
/// Gregorian calendar date
/// See: https://en.wikipedia.org/wiki/Julian_day
/// or Expl. Suppl. Astron. Almanac, P. 619
mod gregorian_coefficients {
    #[allow(non_upper_case_globals)]
    pub const y: i64 = 4716;
    #[allow(non_upper_case_globals)]
    pub const j: i64 = 1401;
    #[allow(non_upper_case_globals)]
    pub const m: i64 = 2;
    #[allow(non_upper_case_globals)]
    pub const n: i64 = 12;
    #[allow(non_upper_case_globals)]
    pub const r: i64 = 4;
    #[allow(non_upper_case_globals)]
    pub const p: i64 = 1461;
    #[allow(non_upper_case_globals)]
    pub const v: i64 = 3;
    #[allow(non_upper_case_globals)]
    pub const u: i64 = 5;
    #[allow(non_upper_case_globals)]
    pub const s: i64 = 153;
    #[allow(non_upper_case_globals)]
    pub const t: i64 = 2;
    #[allow(non_upper_case_globals)]
    pub const w: i64 = 2;
    pub const A: i64 = 184;
    pub const B: i64 = 274_277;
    pub const C: i64 = -38;
}

/// Leap second table
/// The first element is the number of microseconds since unixtime epoch
/// The second element is the number of leap seconds to add as microseconds
const LEAP_SECOND_TABLE: [(i64, i64); 28] = [
    (1483228836000000, 37000000), // 2017-01-01
    (1435708835000000, 36000000), // 2015-07-01
    (1341100834000000, 35000000), // 2012-07-01
    (1230768033000000, 34000000), // 2009-01-01
    (1136073632000000, 33000000), // 2006-01-01
    (915148831000000, 32000000),  // 1999-01-01
    (867715230000000, 31000000),  // 1997-07-01
    (820454429000000, 30000000),  // 1996-01-01
    (773020828000000, 29000000),  // 1994-07-01
    (741484827000000, 28000000),  // 1993-07-01
    (709948826000000, 27000000),  // 1992-07-01
    (662688025000000, 26000000),  // 1991-01-01
    (631152024000000, 25000000),  // 1990-01-01
    (567993623000000, 24000000),  // 1988-01-01
    (489024022000000, 23000000),  // 1985-07-01
    (425865621000000, 22000000),  // 1983-07-01
    (394329620000000, 21000000),  // 1982-07-01
    (362793619000000, 20000000),  // 1981-07-01
    (315532818000000, 19000000),  // 1980-01-01
    (283996817000000, 18000000),  // 1979-01-01
    (252460816000000, 17000000),  // 1978-01-01
    (220924815000000, 16000000),  // 1977-01-01
    (189302414000000, 15000000),  // 1976-01-01
    (157766413000000, 14000000),  // 1975-01-01
    (126230412000000, 13000000),  // 1974-01-01
    (94694411000000, 12000000),   // 1973-01-01
    (78796810000000, 11000000),   // 1972-07-01
    (63072009000000, 10000000),   // 1972-01-01
];

/// Return the number of leap "micro" seconds at "raw" time,
/// which is microseconds since unixtime epoch
fn microleapseconds(raw: i64) -> i64 {
    for (t, ls) in LEAP_SECOND_TABLE.iter() {
        if raw > *t {
            return *ls;
        }
    }
    0
}

impl Instant {
    /// Construct a new Instant from raw microseconds
    ///
    /// # Arguments
    /// * `raw` - The number of microseconds since unixtime epoch
    ///
    /// # Returns
    /// A new Instant object
    ///
    /// # Example
    ///
    /// ```
    /// use satctrl::Instant;
    /// let now = Instant::new(1234567890);
    /// ```
    pub fn new(raw: i64) -> Self {
        Self { raw }
    }

    /// Construct a new Instant from GPS week and second of week
    ///
    /// # Arguments
    /// * `week` - The GPS week number
    /// * `sow` - The second of week
    ///
    /// # Returns
    /// A new Instant object
    ///
    pub fn from_gps_week_and_sow(week: i32, sow: f64) -> Self {
        let week = week as i64;
        let raw = week * 14_515_200_000_000 + (sow * 1.0e6) as i64 + Instant::GPS_EPOCH.raw;
        Self { raw }
    }

    /// Construct a new Instant from Unix time
    ///
    /// # Arguments
    /// * `unixtime` - The Unix time in seconds
    ///
    /// # Returns
    /// A new Instant object representing the input Unix time
    ///
    /// # Note:
    /// Unixtime is the number of non-leap seconds since Jan 1 1970 00:00:00 UTC
    /// (Leap seconds are ignored!!)
    pub fn from_unixtime(unixtime: f64) -> Self {
        let mut raw = (unixtime * 1.0e6) as i64 + Instant::UNIX_EPOCH.raw;

        // Add leapseconds since unixtime ignores them
        let ls = microleapseconds(raw);
        raw += ls;
        // Make sure adding the leapseconds didn't cross another
        // leapsecond boundary
        raw += microleapseconds(raw) - ls;
        Self { raw }
    }

    /// Convert Instant to Unix time
    ///
    /// # Returns
    /// The Unix time in seconds (since 1970-01-01 00:00:00 UTC)
    ///
    /// # Note
    /// Unixtime is the number of non-leap seconds since
    /// 1970-01-01 00:00:00 UTC.
    pub fn as_unixtime(&self) -> f64 {
        // Subtract leap seconds since unixtime ignores them
        (self.raw - Instant::UNIX_EPOCH.raw - microleapseconds(self.raw)) as f64 * 1.0e-6
    }

    /// J2000 epoch is 2000-01-01 12:00:00 TT
    /// TT (Terristrial Time) is 32.184 seconds ahead of TAI
    pub const J2000: Self = Instant {
        raw: 946728064184000,
    };

    /// Unix epoch is 1970-01-01 00:00:00 UTC
    pub const UNIX_EPOCH: Self = Instant { raw: 0 };

    /// GPS epoch is 1980-01-06 00:00:00 UTC
    pub const GPS_EPOCH: Self = Instant {
        raw: 315964819000000,
    };

    /// Modified Julian day epoch is
    /// 1858-11-17 00:00:00 UTC
    pub const MJD_EPOCH: Self = Instant {
        raw: -3506716800000000,
    };

    /// As Modified Julian Date (UTC)
    /// Days since 1858-11-17 00:00:00 UTC
    /// where each day is 86,400 seconds
    /// (no leap seconds)
    pub fn as_mjd(&self) -> f64 {
        // Make sure to account for leap seconds
        (self.raw - Instant::MJD_EPOCH.raw - microleapseconds(self.raw)) as f64 / 86_400_000_000.0
    }

    /// As Julian Date (UTC)
    /// Days since 4713 BC January 1, 12:00 UTC
    /// where each day is 86,400 seconds
    /// (no leap seconds)
    pub fn as_jd(&self) -> f64 {
        self.as_mjd() + 2400000.5
    }

    /// Return the Gregorian date and time
    /// (year, month, day, hour, minute, second), UTC
    pub fn gregorian(&self) -> (i32, i32, i32, i32, i32, f64) {
        // Fractional part of UTC day, accounting for leapseconds and TT - TAI
        let utc_usec_of_day = (self.raw - microleapseconds(self.raw)) % 86_400_000_000;
        let mut jdadd: i64 = 0;

        let mut hour = utc_usec_of_day / 3_600_000_000;
        if hour < 12 {
            jdadd += 1
        }
        let mut minute = (utc_usec_of_day - (hour * 3_600_000_000)) / 60_000_000;
        let mut second =
            (utc_usec_of_day - (hour * 3_600_000_000) - (minute * 60_000_000)) as f64 * 1.0e-6;

        // Rare case where we are in a leap-second
        for (t, _) in LEAP_SECOND_TABLE.iter() {
            if self.raw >= *t && self.raw - *t < 1_000_000 {
                hour = 23;
                minute = 59;
                if second == 0.0 {
                    second = 60.0;
                } else {
                    second += 1.0;
                }
                jdadd -= 1;
            }
        }

        /// See: https://en.wikipedia.org/wiki/Julian_day
        /// or Expl. Suppl. Astron. Almanac, P. 619
        use gregorian_coefficients as gc;
        let mut jd = self.as_jd().floor() as i64;
        jd += jdadd;
        let f = jd + gc::j + (((4 * jd + gc::B) / 146097) * 3) / 4 + gc::C;
        let e = gc::r * f + gc::v;
        let g = (e % gc::p) / gc::r;
        let h = gc::u * g + gc::w;
        let day = ((h % gc::s) / gc::u) + 1;
        let month = ((h / gc::s + gc::m) % gc::n) + 1;
        let year = (e / gc::p) - gc::y + (gc::n + gc::m - month) / gc::n;

        (
            year as i32,
            month as i32,
            day as i32,
            hour as i32,
            minute as i32,
            second,
        )
    }

    pub fn from_gregorian(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: f64,
    ) -> Self {
        use gregorian_coefficients as gc;
        let h = month as i64 - gc::m;
        let g = year as i64 + gc::y - (gc::n - h) / gc::n;
        let f = (h - 1 + gc::n) % gc::n;
        let e = (gc::p * g) / gc::r + day as i64 - 1 - gc::j;
        let mut jd = e + (gc::s * f + gc::t) / gc::u;
        jd = jd - (3 * ((g + gc::A) / 100)) / 4 - gc::C;

        // Note, JD is the given julian day at noon on given date,
        // so we subtract an additional 0.5 to get midnight
        let jd = jd as f64 - 0.5;
        let mjd = jd - 2400000.5;

        let mut raw = mjd as i64 * 86_400_000_000
            + (hour as i64 * 3_600_000_000)
            + (minute as i64 * 60_000_000)
            + (second * 1_000_000.0) as i64
            + Instant::MJD_EPOCH.raw;
        // Account for additional leap seconds if needed
        let ls = microleapseconds(raw);
        raw += ls;
        // Make sure adding the leapseconds didn't cross another
        // leapsecond boundary
        raw = raw + microleapseconds(raw) - ls;

        Self { raw }
    }

    pub fn now() -> Self {
        let now = std::time::SystemTime::now();
        let since_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let mut raw = since_epoch.as_micros() as i64;
        let ls = microleapseconds(raw);
        raw += ls;
        // Make sure adding the leapseconds didn't cross another
        // leapsecond boundary
        raw += microleapseconds(raw) - ls;
        Self { raw }
    }
}

impl std::fmt::Display for Instant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (year, month, day, hour, minute, second) = self.gregorian();
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:09.6}Z",
            year, month, day, hour, minute, second
        )
    }
}

impl std::fmt::Debug for Instant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (year, month, day, hour, minute, second) = self.gregorian();
        write!(
            f,
            "Instant {{ year: {}, month: {}, day: {}, hour: {}, minute: {}, second: {:06.3} }}",
            year, month, day, hour, minute, second
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Duration;

    #[test]
    fn test_j2000() {
        let g = Instant::J2000.gregorian();
        assert!(g.0 == 2000);
        assert!(g.1 == 1);
        assert!(g.2 == 1);
        assert!(g.3 == 12);
        assert!(g.4 == 0);
        // J2000 is TT time, which is 32.184 seconds
        assert!((g.5 - 32.184).abs() < 1.0e-7);
    }

    #[test]
    fn test_leapsecond() {
        let mut t = Instant::new(LEAP_SECOND_TABLE[0].0);
        let g = t.gregorian();
        assert!(g.0 == 2016);
        assert!(g.1 == 12);
        assert!(g.2 == 31);
        assert!(g.3 == 23);
        assert!(g.4 == 59);
        assert!(g.5 == 60.0);
        t -= Duration::from_seconds(1.0);
        let g = t.gregorian();
        assert!(g.0 == 2016);
        assert!(g.1 == 12);
        assert!(g.2 == 31);
        assert!(g.3 == 23);
        assert!(g.4 == 59);
        assert!(g.5 == 59.0);

        t += Duration::from_seconds(2.0);
        let g = t.gregorian();
        assert!(g.0 == 2017);
        assert!(g.1 == 1);
        assert!(g.2 == 1);
        assert!(g.3 == 0);
        assert!(g.4 == 0);
        assert!(g.5 == 0.0);
    }

    #[test]
    fn test_ops() {
        let t1 = Instant::from_gregorian(2024, 11, 13, 8, 0, 3.0);
        let t2 = Instant::from_gregorian(2024, 11, 13, 8, 0, 4.0);
        let dt = t2 - t1;
        assert!(dt.as_microseconds() == 1_000_000);
        let t2 = Instant::from_gregorian(2024, 11, 13, 8, 0, 2.0);
        let dt = t2 - t1;
        assert!(dt.as_microseconds() == -1_000_000);
        let t2 = Instant::from_gregorian(2024, 11, 13, 8, 1, 3.0);
        let dt = t2 - t1;
        assert!(dt.as_microseconds() == 60_000_000);

        let t3 = t2 + Duration::from_days(1.0);
        let g = t3.gregorian();
        assert!(g.0 == 2024);
        assert!(g.1 == 11);
        assert!(g.2 == 14);
        assert!(g.3 == 8);
        assert!(g.4 == 1);
        assert!(g.5 == 3.0);
    }

    #[test]
    fn test_gps() {
        let g = Instant::GPS_EPOCH.gregorian();
        assert!(g.0 == 1980);
        assert!(g.1 == 1);
        assert!(g.2 == 6);
        assert!(g.3 == 0);
        assert!(g.4 == 0);
        assert!(g.5 == 0.0);
    }

    #[test]
    fn test_jd() {
        let time = Instant::from_gregorian(2024, 11, 24, 12, 0, 0.0);
        assert!(time.as_jd() == 2_460_639.0);
        assert!(time.as_mjd() == 60_638.5);
    }
}
