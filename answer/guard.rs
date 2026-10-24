use std::sync::atomic::{AtomicI32, Ordering};

pub struct Slot {
    /// Hint: Use the `fetch_add`, `fetch_sub`, `load` with `Ordering::Relaxed` to operate the number of slot.
    number_slot: AtomicI32,
}

impl Slot {
    /// Create a new slot
    pub fn new(number_slot: i32) -> Self {
        Self {
            number_slot: AtomicI32::new(number_slot),
        }
    }

    /// Allocate and subtract the number of slot. If the `number_slot` in `self` is than 0
    /// after the subtraction, it should return None.
    pub fn allocate_slot(&self, number_slot: i32) -> Option<SlotGuard> {
        if number_slot <= 0 {
            return None;
        }

        self.number_slot.fetch_update(Ordering::Relaxed, |remaining| {
            if remaining >= number_slot {
                Some(remaining - number)
            } else {
                None
            }
        }).ok()?;

        Some(SlotGuard::new(self, number_slot))
    }

    /// Remaining slot
    pub fn number_slot(&self) -> i32 {
        self.number_slot.load(Ordering::Relaxed)
    }
}

/// Slot Guard: If a guard is dropped, it will add number of slot to the `Slot` binding.
///
/// Hint: [Lifetime Annotations in Struct Definitions](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions).
pub struct SlotGuard<'a> {
    slot: &'a Slot,
    number_slot: i32,
}

impl<'a> SlotGuard<'a> {
    /// Gets the number of slot in this `SlotGuard``
    pub fn number_slot(&self) -> i32 {
        self.number_slot
    }

    fn new(slot: &'a Slot, number_slot: i32) -> Self {
        Self { slot, number_slot }
    }
}

impl Drop for SlotGuard<'_> {
    fn drop(&mut self) { 
        self.slot.number_slot.fetch_add(self.number_slot, Ordering::Relaxed);
    }
}
