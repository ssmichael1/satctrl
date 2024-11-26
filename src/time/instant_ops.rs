use crate::Duration;
use crate::Instant;

impl std::ops::Add<Duration> for Instant {
    type Output = Self;

    fn add(self, other: Duration) -> Self {
        Self {
            raw: self.raw + other.usec,
        }
    }
}

impl std::ops::Add<&Duration> for Instant {
    type Output = Self;

    fn add(self, other: &Duration) -> Self {
        Self {
            raw: self.raw + other.usec,
        }
    }
}

impl std::ops::Add<Duration> for &Instant {
    type Output = Instant;

    fn add(self, other: Duration) -> Instant {
        Instant {
            raw: self.raw + other.usec,
        }
    }
}

impl std::ops::Add<&Duration> for &Instant {
    type Output = Instant;

    fn add(self, other: &Duration) -> Instant {
        Instant {
            raw: self.raw + other.usec,
        }
    }
}

impl std::ops::Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, other: Duration) -> Self {
        Self {
            raw: self.raw - other.usec,
        }
    }
}

impl std::ops::Sub<&Duration> for Instant {
    type Output = Self;

    fn sub(self, other: &Duration) -> Self {
        Self {
            raw: self.raw - other.usec,
        }
    }
}

impl std::ops::Sub<Duration> for &Instant {
    type Output = Instant;

    fn sub(self, other: Duration) -> Instant {
        Instant {
            raw: self.raw - other.usec,
        }
    }
}

impl std::ops::Sub<&Duration> for &Instant {
    type Output = Instant;

    fn sub(self, other: &Duration) -> Instant {
        Instant {
            raw: self.raw - other.usec,
        }
    }
}

impl std::ops::Sub<Instant> for &Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        Duration {
            usec: self.raw - other.raw,
        }
    }
}

impl std::ops::Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        Duration {
            usec: self.raw - other.raw,
        }
    }
}

impl std::ops::Sub<&Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: &Instant) -> Duration {
        Duration {
            usec: self.raw - other.raw,
        }
    }
}

impl std::ops::Sub<&Instant> for &Instant {
    type Output = Duration;

    fn sub(self, other: &Instant) -> Duration {
        Duration {
            usec: self.raw - other.raw,
        }
    }
}

impl std::ops::AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        self.raw += other.usec;
    }
}

impl std::ops::SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        self.raw -= other.usec;
    }
}

impl std::ops::AddAssign<&Duration> for Instant {
    fn add_assign(&mut self, other: &Duration) {
        self.raw += other.usec;
    }
}

impl std::ops::SubAssign<&Duration> for Instant {
    fn sub_assign(&mut self, other: &Duration) {
        self.raw -= other.usec;
    }
}

impl std::ops::AddAssign<Duration> for &mut Instant {
    fn add_assign(&mut self, other: Duration) {
        self.raw += other.usec;
    }
}

impl std::ops::SubAssign<Duration> for &mut Instant {
    fn sub_assign(&mut self, other: Duration) {
        self.raw -= other.usec;
    }
}

impl std::ops::AddAssign<&Duration> for &mut Instant {
    fn add_assign(&mut self, other: &Duration) {
        self.raw += other.usec;
    }
}

impl std::ops::SubAssign<&Duration> for &mut Instant {
    fn sub_assign(&mut self, other: &Duration) {
        self.raw -= other.usec;
    }
}

impl std::cmp::PartialEq for Instant {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl std::cmp::Eq for Instant {}

impl std::cmp::PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}
