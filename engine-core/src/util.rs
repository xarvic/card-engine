use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

#[repr(transparent)]
#[derive(Copy, Clone, Hash, Debug, PartialOrd, PartialEq, Ord)]
pub struct AnyID(NonZeroU64);

impl AnyID {
    pub const fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);

        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        AnyID(unsafe {NonZeroU64::new_unchecked(id)})
    }

    pub fn raw(&self) -> u64 {
        self.0.get()
    }
}