#![feature(prelude_import)]
//! examples/spawn_loop.rs
#![no_main]
#![no_std]
#![allow(warnings)]
#![deny(missing_docs)]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use test_app as _;
use stm32f3xx_hal_v2::pac::Interrupt;
use cortex_m::peripheral::NVIC;
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use stm32f3xx_hal_v2::pac as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    /// Holds the maximum priority level for use by async HAL drivers.
    #[no_mangle]
    static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8 = 1 << stm32f3xx_hal_v2::pac::NVIC_PRIO_BITS;
    use core::arch::asm;
    use core::mem;
    use core::ptr;
    use cortex_m_semihosting::{debug, hprintln};
    use cortex_m::asm::{nop, self};
    use stm32f3xx_hal_v2::{
        pac::{self, NVIC},
        pac::Peripherals, pac::FLASH, pac::Interrupt, gpio::{gpioa::PA0, Input, PullUp},
    };
    use volatile::Volatile;
    fn restore() {}
    /// User code end
    ///Shared resources
    struct Shared {}
    ///Local resources
    struct Local {}
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_init_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// The space used to allocate async executors in bytes.
        pub executors_size: usize,
        /// Core peripherals
        pub core: rtic::export::Peripherals,
        /// Device peripherals (PAC)
        pub device: stm32f3xx_hal_v2::pac::Peripherals,
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<'a>,
    }
    impl<'a> __rtic_internal_init_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(
            core: rtic::export::Peripherals,
            executors_size: usize,
        ) -> Self {
            __rtic_internal_init_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                core: core,
                device: stm32f3xx_hal_v2::pac::Peripherals::steal(),
                cs: rtic::export::CriticalSection::new(),
                executors_size,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Initialization function
    pub mod init {
        #[doc(inline)]
        pub use super::__rtic_internal_init_Context as Context;
    }
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(ctx: init::Context) -> (Shared, Local) {
        restore();
        ::cortex_m_semihosting::export::hstdout_str("init\n");
        async_task1::spawn().ok();
        async_task2::spawn().ok();
        ::cortex_m_semihosting::export::hstdout_str("after spawn init\n");
        (Shared {}, Local {})
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_idle_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_idle_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_idle_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Idle loop
    pub mod idle {
        #[doc(inline)]
        pub use super::__rtic_internal_idle_Context as Context;
    }
    #[allow(non_snake_case)]
    fn idle(_: idle::Context) -> ! {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        loop {
            debug::exit(debug::EXIT_SUCCESS);
        }
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_async_task1_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_async_task1_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_async_task1_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    /// Spawns the task directly
    #[allow(non_snake_case)]
    #[doc(hidden)]
    pub fn __rtic_internal_async_task1_spawn() -> Result<(), ()> {
        unsafe {
            let exec = rtic::export::executor::AsyncTaskExecutor::from_ptr_1_args(
                async_task1,
                &__rtic_internal_async_task1_EXEC,
            );
            if exec.try_allocate() {
                exec.spawn(async_task1(unsafe { async_task1::Context::new() }));
                rtic::export::pend(stm32f3xx_hal_v2::pac::interrupt::TIM4);
                Ok(())
            } else {
                Err(())
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod async_task1 {
        #[doc(inline)]
        pub use super::__rtic_internal_async_task1_Context as Context;
        #[doc(inline)]
        pub use super::__rtic_internal_async_task1_spawn as spawn;
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_async_task2_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_async_task2_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_async_task2_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    /// Spawns the task directly
    #[allow(non_snake_case)]
    #[doc(hidden)]
    pub fn __rtic_internal_async_task2_spawn() -> Result<(), ()> {
        unsafe {
            let exec = rtic::export::executor::AsyncTaskExecutor::from_ptr_1_args(
                async_task2,
                &__rtic_internal_async_task2_EXEC,
            );
            if exec.try_allocate() {
                exec.spawn(async_task2(unsafe { async_task2::Context::new() }));
                rtic::export::pend(stm32f3xx_hal_v2::pac::interrupt::TIM4);
                Ok(())
            } else {
                Err(())
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod async_task2 {
        #[doc(inline)]
        pub use super::__rtic_internal_async_task2_Context as Context;
        #[doc(inline)]
        pub use super::__rtic_internal_async_task2_spawn as spawn;
    }
    #[allow(non_snake_case)]
    async fn async_task1<'a>(mut cx: async_task1::Context<'a>) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("I am in task1 before cp\n");
        ::cortex_m_semihosting::export::hstdout_str("I am in task1 after cp\n");
    }
    #[allow(non_snake_case)]
    async fn async_task2<'a>(mut cx: async_task2::Context<'a>) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("I am in task2\n");
    }
    #[allow(non_upper_case_globals)]
    static __rtic_internal_async_task1_EXEC: rtic::export::executor::AsyncTaskExecutorPtr = rtic::export::executor::AsyncTaskExecutorPtr::new();
    #[allow(non_upper_case_globals)]
    static __rtic_internal_async_task2_EXEC: rtic::export::executor::AsyncTaskExecutorPtr = rtic::export::executor::AsyncTaskExecutorPtr::new();
    #[allow(non_snake_case)]
    ///Interrupt handler to dispatch async tasks at priority 3
    #[no_mangle]
    unsafe fn TIM4() {
        /// The priority of this interrupt handler
        const PRIORITY: u8 = 3u8;
        rtic::export::run(
            PRIORITY,
            || {
                let exec = rtic::export::executor::AsyncTaskExecutor::from_ptr_1_args(
                    async_task1,
                    &__rtic_internal_async_task1_EXEC,
                );
                exec.poll(|| {
                    let exec = rtic::export::executor::AsyncTaskExecutor::from_ptr_1_args(
                        async_task1,
                        &__rtic_internal_async_task1_EXEC,
                    );
                    exec.set_pending();
                    rtic::export::pend(stm32f3xx_hal_v2::pac::interrupt::TIM4);
                });
                let exec = rtic::export::executor::AsyncTaskExecutor::from_ptr_1_args(
                    async_task2,
                    &__rtic_internal_async_task2_EXEC,
                );
                exec.poll(|| {
                    let exec = rtic::export::executor::AsyncTaskExecutor::from_ptr_1_args(
                        async_task2,
                        &__rtic_internal_async_task2_EXEC,
                    );
                    exec.set_pending();
                    rtic::export::pend(stm32f3xx_hal_v2::pac::interrupt::TIM4);
                });
            },
        );
    }
    #[doc(hidden)]
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        rtic::export::interrupt::disable();
        let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
            .into();
        let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::TIM2;
        let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::TIM3;
        let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::TIM4;
        const _: () = if (1 << stm32f3xx_hal_v2::pac::NVIC_PRIO_BITS) < 3u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'TIM4\' is more than supported by hardware",
                    ),
                );
            };
        };
        core.NVIC
            .set_priority(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::TIM4,
                rtic::export::cortex_logical2hw(
                    3u8,
                    stm32f3xx_hal_v2::pac::NVIC_PRIO_BITS,
                ),
            );
        rtic::export::NVIC::unmask(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::TIM4,
        );
        #[inline(never)]
        fn __rtic_init_resources<F>(f: F)
        where
            F: FnOnce(),
        {
            f();
        }
        let mut executors_size = 0;
        let executor = ::core::mem::ManuallyDrop::new(
            rtic::export::executor::AsyncTaskExecutor::new_1_args(async_task1),
        );
        executors_size += ::core::mem::size_of_val(&executor);
        __rtic_internal_async_task1_EXEC.set_in_main(&executor);
        let executor = ::core::mem::ManuallyDrop::new(
            rtic::export::executor::AsyncTaskExecutor::new_1_args(async_task2),
        );
        executors_size += ::core::mem::size_of_val(&executor);
        __rtic_internal_async_task2_EXEC.set_in_main(&executor);
        extern "C" {
            pub static _stack_start: u32;
            pub static __ebss: u32;
        }
        let stack_start = &_stack_start as *const _ as u32;
        let ebss = &__ebss as *const _ as u32;
        if stack_start > ebss {
            if rtic::export::msp::read() <= ebss {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Stack overflow after allocating executors"),
                    );
                };
            }
        }
        __rtic_init_resources(|| {
            let (shared_resources, local_resources) = init(
                init::Context::new(core.into(), executors_size),
            );
            rtic::export::interrupt::enable();
        });
        idle(idle::Context::new())
    }
}
