pub struct Instant {
    /// The number of microseconds since
    /// J2000 (2000-01-01 12:00:00 TT)
    pub raw: i64,
}

const TAI2TT: i64 = 32_184_000;
const TT2TAI: i64 = -32_184_000;
const LEAP_SECONDS_AT_EPOCH: i64 = 32;
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
/// The first element is the number of microseconds since J2000 epoch
/// The second element is the number of leap seconds to add
const LEAP_SECOND_TABLE: [(i64, i64); 28] = [
    (536500868184000, 37),  // 2017-01-01
    (488980867184000, 36),  // 2015-07-01
    (394372866184000, 35),  // 2012-07-01
    (284040065184000, 34),  // 2009-01-01
    (189345664184000, 33),  // 2006-01-01
    (-31579136816000, 32),  // 1999-01-01
    (-79012737816000, 31),  // 1997-07-01
    (-126273538816000, 30), // 1996-01-01
    (-173707139816000, 29), // 1994-07-01
    (-205243140816000, 28), // 1993-07-01
    (-236779141816000, 27), // 1992-07-01
    (-284039942816000, 26), // 1991-01-01
    (-315575943816000, 25), // 1990-01-01
    (-378734344816000, 24), // 1988-01-01
    (-457703945816000, 23), // 1985-07-01
    (-520862346816000, 22), // 1983-07-01
    (-552398347816000, 21), // 1982-07-01
    (-583934348816000, 20), // 1981-07-01
    (-631195149816000, 19), // 1980-01-01
    (-662731150816001, 18), // 1979-01-01
    (-694267151816000, 17), // 1978-01-01
    (-725803152816000, 16), // 1977-01-01
    (-757425553816000, 15), // 1976-01-01
    (-788961554816000, 14), // 1975-01-01
    (-820497555816000, 13), // 1974-01-01
    (-852033556816000, 12), // 1973-01-01
    (-867931157816000, 11), // 1972-07-01
    (-883655958816000, 10), // 1972-01-01
];

/// Return the number of leap seconds at "raw" time,
/// which is microseconds since J2000 epoch
fn leapseconds(raw: i64) -> i64 {
    for (t, ls) in LEAP_SECOND_TABLE.iter() {
        if raw >= *t {
            return *ls;
        }
    }
    0
}

impl Instant {
    /// Construct a new Instant from raw microseconds
    ///
    /// # Arguments
    /// * `raw` - The number of microseconds since J2000 epoch
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
    /// * `unixtime` - The Unix time in seconds (since 1970-01-01 00:00:00 UTC)
    ///
    /// # Returns
    /// A new Instant object representing the input Unix time
    pub fn from_unixtime(unixtime: f64) -> Self {
        let raw = (unixtime * 1.0e6) as i64 + Instant::UNIX_EPOCH.raw;
        Self { raw }
    }

    /// Convert Instant to Unix time
    ///
    /// # Returns
    /// The Unix time in seconds (since 1970-01-01 00:00:00 UTC)
    ///
    /// # Note
    /// Unixtime is the number of non-leap seconds since
    /// 1970-01-01 00:00:00 UTC.  It is not the same as TAI.
    pub fn as_unixtime(&self) -> f64 {
        (self.raw - Instant::UNIX_EPOCH.raw) as f64 * 1.0e-6
    }

    /// J2000 epoch is 2000-01-01 12:00:00 TT
    pub const J2000: Self = Instant { raw: 0 };

    /// Unix epoch is 1970-01-01 00:00:00 UTC
    pub const UNIX_EPOCH: Self = Instant {
        raw: -946771135816000,
    };

    /// GPS epoch is 1980-01-06 00:00:00 UTC
    pub const GPS_EPOCH: Self = Instant {
        raw: -630763148816000,
    };

    pub const MJD_EPOCH: Self = Instant {
        raw: -4453487935816000,
    };

    /// As Modified Julian Date
    /// (MJD = JD - 2400000.5)
    /// Days since 1858-11-17 00:00:00 UTC
    pub fn as_mjd(&self) -> f64 {
        // Make sure to account for le
        ((self.raw - Instant::MJD_EPOCH.raw) as f64 * 1.0e-6
            - (leapseconds(self.raw) - LEAP_SECONDS_AT_EPOCH) as f64)
            / 86_400.0
    }

    /// As Julian Date
    pub fn as_jd(&self) -> f64 {
        self.as_mjd() + 2400000.5
    }

    /// Return the Gregorian date and time
    /// (year, month, day, hour, minute, second), UTC
    pub fn gregorian(&self) -> (i32, i32, i32, i32, i32, f64) {
        // Fractional part of UTC day, accounting for leapseconds and TT - TAI
        let tt_usec_of_day = self.raw % 86_400_000_000;
        let mut utc_usec_of_day = tt_usec_of_day - leapseconds(self.raw) * 1_000_000 + TT2TAI;
        let mut jdadd: i64 = 0;
        if utc_usec_of_day > 86_400_000_000 {
            jdadd = 1;
            utc_usec_of_day -= 86_400_000_000;
        }
        let hour = (utc_usec_of_day / 3_600_000_000) as i32;
        if hour < 12 {
            jdadd += 1
        }
        let minute = ((utc_usec_of_day % 3_600_000_000) / 60_000_000) as i32;
        let second = ((utc_usec_of_day % 60_000_000) as f64) * 1.0e-6;

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

        (year as i32, month as i32, day as i32, hour, minute, second)
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
        // Account for additional leap seconds from j2000 epoch
        raw = raw + (leapseconds(raw) - LEAP_SECONDS_AT_EPOCH) * 1_000_000;

        Self { raw }
    }

    pub fn now() -> Self {
        let now = std::time::SystemTime::now();
        let since_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let mut raw = since_epoch.as_micros() as i64 + Instant::UNIX_EPOCH.raw;
        raw += (leapseconds(raw) - LEAP_SECONDS_AT_EPOCH) * 1_000_000;
        Self { raw }
    }
}

impl std::fmt::Display for Instant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (year, month, day, hour, minute, second) = self.gregorian();
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:06.3}Z",
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

    #[test]
    fn test_instant() {
        use chrono::{DateTime, Utc};
        //let now = Instant::from_gregorian(2022, 11, 24, 13, 0, 13.0);
        let now = Instant::now();
        let tnow: DateTime<Utc> = std::time::SystemTime::now().into();
        let tnow = tnow.to_rfc3339();
        println!("now = {}", now);
        println!("systemtime = {}", tnow);
    }
}
