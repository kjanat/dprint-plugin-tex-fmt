use std::sync::atomic::{AtomicU64, Ordering};

pub use std::time::Duration;

static TICK: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(u64);

impl Instant {
    pub fn now() -> Self {
        Self(TICK.fetch_add(1, Ordering::Relaxed))
    }

    pub fn elapsed(&self) -> Duration {
        Duration::from_nanos(TICK.load(Ordering::Relaxed).saturating_sub(self.0))
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration::from_nanos(self.0.saturating_sub(earlier.0))
    }

    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        self.0.checked_sub(earlier.0).map(Duration::from_nanos)
    }

    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        self.duration_since(earlier)
    }

    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        u64::try_from(duration.as_nanos())
            .ok()
            .and_then(|d| self.0.checked_add(d))
            .map(Self)
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        u64::try_from(duration.as_nanos())
            .ok()
            .and_then(|d| self.0.checked_sub(d))
            .map(Self)
    }
}

impl std::ops::Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Instant {
        self.checked_add(rhs)
            .expect("overflow adding duration to Instant")
    }
}

impl std::ops::AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Instant {
        self.checked_sub(rhs)
            .expect("overflow subtracting duration from Instant")
    }
}

impl std::ops::SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl std::ops::Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}

// ── SystemTime ────────────────────────────────────────────────────────────────

pub const UNIX_EPOCH: SystemTime = SystemTime(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime(u64);

impl SystemTime {
    pub const UNIX_EPOCH: SystemTime = SystemTime(0);

    pub fn now() -> Self {
        Self(TICK.load(Ordering::Relaxed))
    }

    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration, SystemTimeError> {
        self.0
            .checked_sub(earlier.0)
            .map(Duration::from_nanos)
            .ok_or_else(|| SystemTimeError(Duration::from_nanos(earlier.0 - self.0)))
    }

    pub fn elapsed(&self) -> Result<Duration, SystemTimeError> {
        Self::now().duration_since(*self)
    }

    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        u64::try_from(duration.as_nanos())
            .ok()
            .and_then(|d| self.0.checked_add(d))
            .map(Self)
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        u64::try_from(duration.as_nanos())
            .ok()
            .and_then(|d| self.0.checked_sub(d))
            .map(Self)
    }
}

impl std::ops::Add<Duration> for SystemTime {
    type Output = SystemTime;
    fn add(self, rhs: Duration) -> SystemTime {
        self.checked_add(rhs)
            .expect("overflow adding duration to SystemTime")
    }
}

impl std::ops::AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Duration> for SystemTime {
    type Output = SystemTime;
    fn sub(self, rhs: Duration) -> SystemTime {
        self.checked_sub(rhs)
            .expect("overflow subtracting duration from SystemTime")
    }
}

impl std::ops::SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

#[derive(Debug, Clone)]
pub struct SystemTimeError(Duration);

impl SystemTimeError {
    pub fn duration(&self) -> Duration {
        self.0
    }
}

impl std::fmt::Display for SystemTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "second time provided was later than self")
    }
}

impl std::error::Error for SystemTimeError {}
