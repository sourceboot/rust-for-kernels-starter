//! LAB 05 — printing with no heap
//!
//! This file is yours to write. Lab 05's project names every public item it
//! must hold; add the module's line to lib.rs when you start. The tests below
//! are yours too: each name is one lab 05 asks for, and each body waits for
//! the task that writes it.

#[allow(unused_imports)]
use core::fmt;
#[allow(unused_imports)]
use core::sync::atomic::AtomicU64;

#[allow(unused_imports)]
use crate::sync::SpinLock;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sink_formats_like_std() {
        todo!("lab 05 task 3")
    }

    #[test]
    fn sink_is_all_or_nothing() {
        todo!("lab 05 task 3")
    }

    #[test]
    fn macros_reach_the_console() {
        todo!("lab 05 task 6")
    }
}
