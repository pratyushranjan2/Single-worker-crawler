use std::sync::atomic::{AtomicUsize, Ordering};

static IN_FLIGHT: AtomicUsize = AtomicUsize::new(0);

static PEAK_IN_FLIGHT: AtomicUsize = AtomicUsize::new(0);

/// Counts a page as in flight for as long as it is alive, tracking the peak.
pub struct InFlightGuard;

impl InFlightGuard {
    pub fn new() -> Self {
        let now = IN_FLIGHT.fetch_add(1, Ordering::Relaxed) + 1;
        PEAK_IN_FLIGHT.fetch_max(now, Ordering::Relaxed);
        InFlightGuard
    }
}

impl Drop for InFlightGuard {
    fn drop(&mut self) {
        IN_FLIGHT.fetch_sub(1, Ordering::Relaxed);
    }
}

pub fn peak_in_flight() -> usize {
    PEAK_IN_FLIGHT.load(Ordering::Relaxed)
}
