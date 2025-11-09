#[macro_export]
macro_rules! heap_init {
    [ ] => {
        // Import linker symbols for heap region
        unsafe extern "C" {
            static mut _sdata: u8;
            static mut _edata: u8;
        }

        unsafe {
            let heap_start = core::ptr::addr_of_mut!(_sdata) as usize;
            let heap_end = core::ptr::addr_of_mut!(_edata) as usize;
            let heap_size = heap_end - heap_start;
            ALLOCATOR.init(heap_start, heap_size)
        }
    };
}
