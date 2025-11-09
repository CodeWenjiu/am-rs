# 栈初始化重构总结

## 问题
原先的实现将栈初始化硬编码在 `runtime-common` 中：
```rust
li sp, 0x88000000  # 硬编码地址
```

这样的问题是：
1. 不同平台的内存布局不同
2. 栈地址硬编码，不灵活
3. 平台特定的汇编代码混在通用代码中

## 解决方案

### 1. 分离关注点
- **runtime-common**: 只提供 `__start__` 函数（调用 main）
- **平台 runtime**: 各自实现 `_start` 函数（初始化栈）

### 2. 使用链接脚本符号
在链接脚本中定义：
```ld
_stack_top = ORIGIN(RAM) + LENGTH(RAM);
```

在 `_start` 中使用：
```asm
la sp, _stack_top
j __start__
```

### 3. 文件结构

**runtime-common/src/lib.rs**
```rust
#[macro_export]
macro_rules! common_startup {
    () => {
        #[unsafe(export_name = "__start__")]
        pub unsafe extern "C" fn __start__() -> ! {
            unsafe extern "Rust" {
                unsafe fn main() -> !;
            }
            unsafe { main() }
        }
    };
}
```

**platform/qemu/src/startup.rs**
```rust
#[unsafe(naked)]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "
        la sp, _stack_top
        j __start__
        "
    )
}
```

**platform/qemu/linker_scripts/riscv32i_link.x**
```ld
_stack_top = ORIGIN(RAM) + LENGTH(RAM);
_sheap = .;
_eheap = ORIGIN(RAM) + LENGTH(RAM) - 0x100000;
```

## 优势

1. ✅ **平台独立性**: 通用代码不包含平台特定的内容
2. ✅ **灵活性**: 每个平台可以自定义内存布局
3. ✅ **可维护性**: 代码职责清晰
4. ✅ **可扩展性**: 添加新平台只需实现 startup.rs
5. ✅ **类型安全**: 使用链接脚本符号，编译时检查

## 启动流程

```
CPU Reset
    ↓
_start (platform/qemu/src/startup.rs)
    ├─ la sp, _stack_top      # 从链接脚本加载栈顶地址
    └─ j __start__            # 跳转到通用启动代码
        ↓
__start__ (runtime-common)
    └─ Call main()
        ↓
main (entry! 宏生成)
    ├─ heap_init!()           # 使用 _sheap 和 _eheap
    └─ User's main()
```

## 添加新平台

只需创建 3 个文件：

1. **startup.rs** - 栈初始化
2. **stdio.rs** - putc() 实现
3. **linker_scripts/*.x** - 定义 _stack_top 等符号

不需要修改任何通用代码！

## 测试结果

✅ hello_world 成功输出
✅ QEMU 平台正常工作
✅ NEMU 平台正常工作
✅ 所有架构 (RV32I, RV32IM, RV32IMV) 正常编译
