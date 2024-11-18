pub struct Instant {
    /// The number of microseconds since J2000 epoch
    /// (1st January 2000, 12:00:00)
    /// in International Atomic Time (TAI).
    pub raw: i64,
}

/// Leap second table
/// The first element is the number of microseconds since J2000 epoch
/// The second element is the number of leap seconds to add
const LEAP_SECOND_TABLE: [(i64, i64); 11] = [
    (536544036000000, 37),  // 1st July 2017
    (489024035000000, 36),  // 1st July 2015
    (394416034000000, 35),  // 1st July 2012
    (284083233000000, 34),  // 1st Jan 2009
    (189388832000000, 33),  // 1st Jan 2006
    (-31535969000000, 32),  // 1st Jan 1999
    (-78969570000000, 31),  // 1st July 1997
    (-126230371000000, 30), // 1st Jan 1996
    (-173663972000000, 29), // 1st July 1994
    (-220897973000000, 28), // 1st July 1993
    (-205199973000000, 27), // 1st July 1992
];

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
    pub fn as_unixtime(&self) -> f64 {
        (self.raw - Instant::UNIX_EPOCH.raw) as f64 * 1.0e-6
    }

    /// J2000 epoch is 2000-01-01 12:00:00 TAI
    pub const J2000: Self = Instant { raw: 0 };

    /// Unix epoch is 1970-01-01 00:00:00 UTC
    pub const UNIX_EPOCH: Self = Instant {
        raw: -946684768000000,
    };

    /// GPS epoch is 1980-01-06 00:00:00 UTC
    pub const GPS_EPOCH: Self = Instant {
        raw: -630719981000000,
    };

    pub fn now() -> Self {
        let now = std::time::SystemTime::now();
        let since_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        Self::new(since_epoch.as_nanos() as i64)
    }
}
