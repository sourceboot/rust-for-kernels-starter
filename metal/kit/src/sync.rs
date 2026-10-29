//! LAB 03 — shared state
//!
//! This file is yours to write. Lab 03's project names every public item it
//! must hold; add the module's line to lib.rs when you start. The tests below
//! are yours too: each name is one lab 03 asks for, and each body waits for
//! the task that writes it.

#[allow(unused_imports)]
use core::cell::UnsafeCell;
#[allow(unused_imports)]
use core::ops::{Deref, DerefMut};
#[allow(unused_imports)]
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[allow(unused_imports)]
use crate::raw::Bump;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn guard_unlocks_on_drop() {
        todo!("lab 03 task 3")
    }

    #[test]
    fn threads_count_exactly() {
        todo!("lab 03 task 4")
    }

    #[test]
    fn ticks_and_heap() {
        todo!("lab 03 task 6")
    }
}
