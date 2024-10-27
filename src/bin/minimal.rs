//! examples/spawn_loop.rs

#![no_main]
#![no_std]
#![allow(warnings)]
// #![deny(unsafe_code)]
#![deny(missing_docs)]

use test_app as _;
use stm32f3xx_hal_v2::pac::Interrupt;
use cortex_m::peripheral::NVIC;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};

mod checkpoint;
use checkpoint::{checkpoint, restore, delete_pg, delete_all_pg, transcation_log, execution_mode, counter,start_atomic, end_atomic};
use volatile::Volatile;
use checkpoint::my_flash::{unlock, wait_ready, clear_error_flags, erase_page, write_to_flash};

#[rtic::app(device = stm32f3xx_hal_v2::pac, dispatchers = [TIM2, TIM3, TIM4])]
mod app {
    use core::arch::asm;
    use core::mem;
    use core::ptr;
    use cortex_m::asm::{nop, self};
    use crate::checkpoint::end_atomic;
    use crate::checkpoint::start_atomic;
    use crate::checkpoint::save_variables;
    use crate::checkpoint::{self, delete_all_pg, restore, checkpoint,c_checkpoint, erase_all};
    use crate::checkpoint::my_flash::{unlock, wait_ready, clear_error_flags, erase_page, write_to_flash};
    use cortex_m::peripheral::syst::SystClkSource;
    use cortex_m_rt:: exception;

    use stm32f3xx_hal_v2::{pac::{self, NVIC},pac::Peripherals, pac::FLASH, pac::Interrupt, gpio::{gpioa::PA0,Input, PullUp}};
    use volatile::Volatile;
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {
        a: u32,
        b: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        hprintln!("init");

        async_task1::spawn(1).ok();

        (Shared { a: 0, b: 0 }, Local {})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            hprintln!("idle");
            debug::exit(debug::EXIT_SUCCESS);
        }
    }
    
    #[task(priority = 1, shared = [a, b])]
    async fn async_task1(mut cx: async_task1::Context, inc: u32) {
        //let mut xyz= 1;
        start_atomic();
        async_task2::spawn().ok();
        cx.shared.a.lock(|a| {  *a += inc;});
        hprintln!(
            "hello from async 1 a {}", cx.shared.a.lock(|a|{*a}));
        
        end_atomic();
    }

    #[task(priority = 2, shared = [a, b])]
    async fn async_task2(mut cx: async_task2::Context) {
        start_atomic();
        cx.shared.a.lock(|a| {*a += 1;});
        hprintln!(
            "hello from async 2 a {}", cx.shared.a.lock(|a|{*a}));
        end_atomic();
    }
}
