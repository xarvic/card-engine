use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Counter(AtomicU64);

impl Counter {
    pub const fn new() -> Self {
        Counter(AtomicU64::new(1))
    }

    pub fn next(&self) -> NonZeroU64 {
        let val = self.0.fetch_add(1, Ordering::SeqCst);
        unsafe { NonZeroU64::new_unchecked(val) }
    }
}