/// Structure representing a duration in time
/// This can be used to add or subtract times from
/// an `Instant` object.
///
/// Duration is represented natively in microseconds.
pub struct Duration {
    pub(crate) usec: i64,
}

impl Duration {
    /// Create a new duration object
    ///
    /// # Arguments
    /// * `usec` - The duration in microseconds
    ///
    /// # Returns
    /// A new `Duration` object
    pub fn new(usec: i64) -> Self {
        Self { usec }
    }

    /// Create a new duration object from seconds
    ///
    /// # Arguments
    /// * `seconds` - The duration in seconds
    ///
    /// # Returns
    /// A new `Duration` object representing the time interval in seconds
    pub fn from_seconds(seconds: f64) -> Self {
        Self {
            usec: (seconds * 1_000_000.0) as i64,
        }
    }

    /// Create a new duration object from microseconds
    ///
    /// # Arguments
    /// * `usec` - The duration in microseconds
    ///
    /// # Returns
    /// A new `Duration` object representing the time interval in microseconds
    pub fn from_microseconds(usec: i64) -> Self {
        Self { usec }
    }

    /// Create a new duration object from milliseconds
    ///
    /// # Arguments
    /// * `milliseconds` - The duration in milliseconds
    ///
    /// # Returns
    /// A new `Duration` object representing the time interval in milliseconds
    pub fn from_milliseconds(milliseconds: f64) -> Self {
        Self {
            usec: (milliseconds * 1_000.0) as i64,
        }
    }

    /// Create a new duration object from days
    ///
    /// # Arguments
    /// * `days` - The duration in days
    ///
    /// # Returns
    /// A new `Duration` object representing the time interval in days
    pub fn from_days(days: f64) -> Self {
        Self {
            usec: (days * 86_400_000_000.0) as i64,
        }
    }

    /// Represent duration as days
    ///
    /// # Returns
    /// The duration in days
    pub fn as_days(&self) -> f64 {
        self.usec as f64 / 86_400_000_000.0
    }

    /// Represent duration as seconds
    ///
    /// # Returns
    /// The duration in seconds
    pub fn as_seconds(&self) -> f64 {
        self.usec as f64 / 1_000_000.0
    }

    /// Represent duration as microseconds
    ///
    /// # Returns
    /// The duration in microseconds
    pub fn as_microseconds(&self) -> i64 {
        self.usec
    }
}

/// Add two durations together
impl std::ops::Add<Duration> for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            usec: self.usec + other.usec,
        }
    }
}

/// Subtract two durations
impl std::ops::Sub<Duration> for Duration {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            usec: self.usec - other.usec,
        }
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.usec < 1_000_000 {
            return write!(f, "{} usec", self.usec);
        } else {
            if self.usec > 86_400_000_000 {
                write!(f, "{} days ", self.usec / 86_400_000_000)?;
            }
            if self.usec > 3_600_000_000 {
                write!(f, "{} hours ", self.usec / 3_600_000_000)?;
            }
            if self.usec > 60_000_000 {
                write!(f, "{} minutes ", self.usec / 60_000_000)?;
            }
            write!(f, "{:.6} seconds", self.usec as f64 / 1_000_000.0)?;
        }
        Ok(())
    }
}
