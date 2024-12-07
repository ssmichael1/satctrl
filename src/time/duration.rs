/// Structure representing a duration in time
/// This can be used to add or subtract times from
/// an `Instant` object.
///
/// Duration is represented natively in microseconds.
#[derive(Clone, Copy)]
pub struct Duration(pub(crate) i64);

impl Duration {
    /// Create a new duration object
    ///
    /// # Arguments
    /// * `usec` - The duration in microseconds
    ///
    /// # Returns
    /// A new `Duration` object
    pub fn new(usec: i64) -> Self {
        Self(usec)
    }

    /// Create a new duration object from seconds
    ///
    /// # Arguments
    /// * `seconds` - The duration in seconds
    ///
    /// # Returns
    /// A new `Duration` object representing the time interval in seconds
    pub fn from_seconds(seconds: f64) -> Self {
        Self((seconds * 1_000_000.0) as i64)
    }

    /// Create a new duration object from minutes
    ///
    /// # Arguments
    /// * `minutes` - The duration in minutes
    ///
    /// # Returns
    /// A new `Duration` object representing the time interval in minutes
    pub fn from_minutes(minutes: f64) -> Self {
        Self((minutes * 60_000_000.0) as i64)
    }

    /// Create a new duration object from microseconds
    ///
    /// # Arguments
    /// * `usec` - The duration in microseconds
    ///
    /// # Returns
    /// A new `Duration` object representing the time interval in microseconds
    pub fn from_microseconds(usec: i64) -> Self {
        Self(usec)
    }

    /// Create a new duration object from milliseconds
    ///
    /// # Arguments
    /// * `milliseconds` - The duration in milliseconds
    ///
    /// # Returns
    /// A new `Duration` object representing the time interval in milliseconds
    pub fn from_milliseconds(milliseconds: f64) -> Self {
        Self((milliseconds * 1_000.0) as i64)
    }

    /// Create a new duration object from days
    ///
    /// # Arguments
    /// * `days` - The duration in days
    ///
    /// # Returns
    /// A new `Duration` object representing the time interval in days
    pub fn from_days(days: f64) -> Self {
        Self((days * 86_400_000_000.0) as i64)
    }

    /// Create a new duration object from hours
    ///
    /// # Arguments
    /// * `hours` - The duration in hours
    ///
    /// # Returns
    /// A new `Duration` object representing the time interval in hours
    pub fn from_hours(hours: f64) -> Self {
        Self((hours * 3_600_000_000.0) as i64)
    }

    /// Represent duration as days
    ///
    /// # Returns
    /// The duration in days
    pub fn as_days(&self) -> f64 {
        self.0 as f64 / 86_400_000_000.0
    }

    /// Represent duration as seconds
    ///
    /// # Returns
    /// The duration in seconds
    pub fn as_seconds(&self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }

    /// Represent duration as hours
    ///
    /// # Returns
    /// The duration in hours
    pub fn as_hours(&self) -> f64 {
        self.0 as f64 / 3_600_000_000.0
    }

    /// Represent duration as minutes
    ///
    /// # Returns
    /// The duration in minutes
    pub fn as_minutes(&self) -> f64 {
        self.0 as f64 / 60_000_000.0
    }

    /// Represent duration as microseconds
    ///
    /// # Returns
    /// The duration in microseconds
    pub fn as_microseconds(&self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0 < 1_000_000 {
            return write!(f, "{} usec", self.0);
        } else {
            if self.0 > 86_400_000_000 {
                write!(f, "{} days ", self.0 / 86_400_000_000)?;
            }
            if self.0 > 3_600_000_000 {
                write!(f, "{} hours ", self.0 / 3_600_000_000)?;
            }
            if self.0 > 60_000_000 {
                write!(f, "{} minutes ", self.0 / 60_000_000)?;
            }
            write!(f, "{:.6} seconds", self.0 as f64 / 1_000_000.0)?;
        }
        Ok(())
    }
}
