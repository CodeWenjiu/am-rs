//! Critical section 实现
//! 
//! 为单核 bare-metal 系统提供临界区保护

struct CriticalSection;
critical_section::set_impl!(CriticalSection);

unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        // 在 bare-metal 单核环境中,我们不需要禁用中断
        // 因为当前没有中断系统
        // 如果将来需要支持中断,可以在这里禁用 RISC-V 中断
    }

    unsafe fn release(_token: critical_section::RawRestoreState) {
        // 恢复中断状态(当前为空实现)
    }
}
