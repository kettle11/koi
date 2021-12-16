#![feature(alloc_error_hook)]

use std::{
    alloc::{GlobalAlloc, Layout},
    sync::atomic::AtomicUsize,
};

pub struct TracingAllocator<A>(pub A)
where
    A: GlobalAlloc;

static MEMORY_USED: AtomicUsize = AtomicUsize::new(0);
static PEAK_MEMORY_USED: AtomicUsize = AtomicUsize::new(0);

unsafe impl<A> GlobalAlloc for TracingAllocator<A>
where
    A: GlobalAlloc,
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let pointer = self.0.alloc(layout);
        let previous = MEMORY_USED.fetch_add(size, std::sync::atomic::Ordering::Relaxed);
        PEAK_MEMORY_USED.fetch_max(previous + size, std::sync::atomic::Ordering::Relaxed);
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let size = layout.size();
        self.0.dealloc(pointer, layout);
        MEMORY_USED.fetch_sub(size, std::sync::atomic::Ordering::Relaxed);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let pointer = self.0.alloc_zeroed(layout);
        let previous = MEMORY_USED.fetch_add(size, std::sync::atomic::Ordering::Relaxed);
        PEAK_MEMORY_USED.fetch_max(previous + size, std::sync::atomic::Ordering::Relaxed);
        pointer
    }

    unsafe fn realloc(&self, old_pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old_size = layout.size();
        let new_pointer = self.0.realloc(old_pointer, layout, new_size);
        let change = new_size as i64 - old_size as i64;
        if change > 0 {
            let previous =
                MEMORY_USED.fetch_add(change as usize, std::sync::atomic::Ordering::Relaxed);
            PEAK_MEMORY_USED.fetch_max(
                previous + change as usize,
                std::sync::atomic::Ordering::Relaxed,
            );
        } else {
            MEMORY_USED.fetch_sub(-change as usize, std::sync::atomic::Ordering::Relaxed);
        }

        new_pointer
    }
}

/// Gets the current heap memory used in bytes.
pub fn get_memory_used() -> usize {
    MEMORY_USED.load(std::sync::atomic::Ordering::Relaxed)
}

/// Gets the current heap memory used in bytes.
pub fn get_peak_memory_usage() -> usize {
    PEAK_MEMORY_USED.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_alloc_error_hook() {
    fn alloc_error_hook(layout: Layout) {
        klog::log!("COUILD NOT ALLOCATE!");
        klog::log!("CURRENT MEMORY USAGE: {:?}", get_memory_used());
        klog::log!("SIZE ALLOCATING: {:?}", layout.size());
    }
    std::alloc::set_alloc_error_hook(alloc_error_hook)
}
