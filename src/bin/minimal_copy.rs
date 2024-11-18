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
    use crate::checkpoint::initialization;


    #[shared]
    struct Shared {
        shared1: u32,
        shared2: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        foo::spawn().unwrap();

        (Shared { shared1: 0, shared2:0 }, Local {})
    }

    // when omitted priority is assumed to be `1`
    #[task(shared = [shared1])]
    async fn foo(mut c: foo::Context) {
        hprintln!("A");

        // the lower priority task requires a critical section to access the data
        c.shared.shared1.lock(|shared1| {
            // data can only be modified within this critical section (closure)
              // baz does not contend for `shared` so it's allowed to run now
            baz::spawn().unwrap();

            *shared1 += 1;

            // bar will *not* run right now due to the critical section
            bar::spawn().unwrap();

            hprintln!("B - shared1 = {}", *shared1);

          
        });

        // critical section is over: bar can now start

        hprintln!("E");

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }

    #[task(priority = 2, shared = [shared1])]
    async fn bar(mut c: bar::Context) {
        // the higher priority task does still need a critical section
        let shared = c.shared.shared1.lock(|shared1| {
            *shared1 += 1;

            *shared1
        });

        hprintln!("D - shared1 = {}", shared);
    }

    #[task(priority = 3,  shared = [shared2])]
    async fn baz(mut c: baz::Context) {

        let shared = c.shared.shared2.lock(|shared2| {

            *shared2 += 1;

            *shared2

        });
        hprintln!("C - shared2 = {}", shared);
       // hprintln!("C");
    }

}
