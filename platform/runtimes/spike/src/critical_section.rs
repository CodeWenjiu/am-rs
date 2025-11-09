//! Critical section implementation for QEMU platform
//!
//! This module provides critical section support by disabling/enabling
//! machine-mode interrupts via the mstatus CSR register.
//!
//! Uses a nesting counter to support nested critical sections.
//! For single-core bare-metal systems, we use UnsafeCell with manual Sync impl.

use core::cell::UnsafeCell;

/// Nesting counter for critical sections
///
/// This is safe in single-core bare-metal environments because:
/// 1. We're single-threaded (no preemption during critical section)
/// 2. Interrupts are disabled during critical section
/// 3. Only accessed within acquire/release which are already synchronized
struct CriticalSectionCount(UnsafeCell<usize>);

// SAFETY: In a single-core bare-metal environment with no preemption,
// this is safe because interrupts are disabled when accessing the counter
unsafe impl Sync for CriticalSectionCount {}

impl CriticalSectionCount {
    const fn new() -> Self {
        Self(UnsafeCell::new(0))
    }

    unsafe fn get(&self) -> usize {
        unsafe { *self.0.get() }
    }

    unsafe fn set(&self, value: usize) {
        unsafe { *self.0.get() = value };
    }
}

static CRITICAL_SECTION_COUNT: CriticalSectionCount = CriticalSectionCount::new();

struct CriticalSection;
critical_section::set_impl!(CriticalSection);

unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        // Get current count and increment
        let count = unsafe { CRITICAL_SECTION_COUNT.get() };
        unsafe { CRITICAL_SECTION_COUNT.set(count + 1) };

        // Only disable interrupts on the first acquire (count was 0)
        if count == 0 {
            unsafe {
                // Clear MIE bit (bit 3) in mstatus to disable machine interrupts
                core::arch::asm!("csrci mstatus, 0x8", options(nomem, nostack));
            }
        }
    }

    unsafe fn release(_token: critical_section::RawRestoreState) {
        // Get current count and decrement
        let count = unsafe { CRITICAL_SECTION_COUNT.get() };
        unsafe { CRITICAL_SECTION_COUNT.set(count - 1) };

        // Only re-enable interrupts when exiting the outermost critical section
        if count == 1 {
            unsafe {
                // Set MIE bit (bit 3) in mstatus to enable machine interrupts
                core::arch::asm!("csrsi mstatus, 0x8", options(nomem, nostack));
            }
        }
    }
}
