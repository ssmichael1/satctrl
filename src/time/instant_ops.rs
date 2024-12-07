use super::Duration;
use super::Instant;

impl std::ops::Add<Duration> for Instant {
    type Output = Self;

    fn add(self, other: Duration) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Add<&Duration> for Instant {
    type Output = Self;

    fn add(self, other: &Duration) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Add<Duration> for &Instant {
    type Output = Instant;

    fn add(self, other: Duration) -> Instant {
        Instant(self.0 + other.0)
    }
}

impl std::ops::Add<&Duration> for &Instant {
    type Output = Instant;

    fn add(self, other: &Duration) -> Instant {
        Instant(self.0 + other.0)
    }
}

impl std::ops::Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, other: Duration) -> Self {
        Self(self.0 - other.0)
    }
}

impl std::ops::Sub<&Duration> for Instant {
    type Output = Self;

    fn sub(self, other: &Duration) -> Self {
        Self(self.0 - other.0)
    }
}

impl std::ops::Sub<Duration> for &Instant {
    type Output = Instant;

    fn sub(self, other: Duration) -> Instant {
        Instant(self.0 - other.0)
    }
}

impl std::ops::Sub<&Duration> for &Instant {
    type Output = Instant;

    fn sub(self, other: &Duration) -> Instant {
        Instant(self.0 - other.0)
    }
}

impl std::ops::Sub<Instant> for &Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        Duration(self.0 - other.0)
    }
}

impl std::ops::Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        Duration(self.0 - other.0)
    }
}

impl std::ops::Sub<&Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: &Instant) -> Duration {
        Duration(self.0 - other.0)
    }
}

impl std::ops::Sub<&Instant> for &Instant {
    type Output = Duration;

    fn sub(self, other: &Instant) -> Duration {
        Duration(self.0 - other.0)
    }
}

impl std::ops::AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        self.0 += other.0;
    }
}

impl std::ops::SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        self.0 -= other.0;
    }
}

impl std::ops::AddAssign<&Duration> for Instant {
    fn add_assign(&mut self, other: &Duration) {
        self.0 += other.0;
    }
}

impl std::ops::SubAssign<&Duration> for Instant {
    fn sub_assign(&mut self, other: &Duration) {
        self.0 -= other.0;
    }
}

impl std::ops::AddAssign<Duration> for &mut Instant {
    fn add_assign(&mut self, other: Duration) {
        self.0 += other.0;
    }
}

impl std::ops::SubAssign<Duration> for &mut Instant {
    fn sub_assign(&mut self, other: Duration) {
        self.0 -= other.0;
    }
}

impl std::ops::AddAssign<&Duration> for &mut Instant {
    fn add_assign(&mut self, other: &Duration) {
        self.0 += other.0;
    }
}

impl std::ops::SubAssign<&Duration> for &mut Instant {
    fn sub_assign(&mut self, other: &Duration) {
        self.0 -= other.0;
    }
}

impl std::cmp::PartialEq for Instant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Add two durations together
impl std::ops::Add<Duration> for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::AddAssign<Duration> for Duration {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl std::ops::AddAssign<&Duration> for Duration {
    fn add_assign(&mut self, other: &Self) {
        self.0 += other.0;
    }
}

impl std::ops::SubAssign<Duration> for Duration {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl std::ops::SubAssign<&Duration> for Duration {
    fn sub_assign(&mut self, other: &Self) {
        self.0 -= other.0;
    }
}

impl std::cmp::PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::cmp::PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

/// Subtract two durations
impl std::ops::Sub<Duration> for Duration {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl std::cmp::Eq for Instant {}

impl std::cmp::PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
