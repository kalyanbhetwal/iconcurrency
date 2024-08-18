//! examples/spawn_loop.rs

#![no_main]
#![no_std]
#![allow(warnings)]
// #![deny(unsafe_code)]
#![deny(missing_docs)]

use test_app as _;
use stm32f3xx_hal_v2::pac::Interrupt;
use cortex_m::peripheral::NVIC;

mod checkpoint;
use checkpoint::{checkpoint, restore, delete_pg, delete_all_pg, transcation_log, execution_mode, counter,start_atomic, end_atomic};
use volatile::Volatile;

use checkpoint::my_flash::{unlock, wait_ready, clear_error_flags, erase_page, write_to_flash};

#[rtic::app(device = stm32f3xx_hal_v2::pac, dispatchers = [TIM2, TIM3, TIM4])]
mod app {
    use core::arch::asm;
    use core::mem;
    use core::ptr;
    use cortex_m_semihosting::{debug, hprintln};
    use cortex_m::asm::{nop, self};
    use crate::checkpoint::end_atomic;
    use crate::checkpoint::start_atomic;
    use crate::checkpoint::save_variables;
    use crate::checkpoint::{self, delete_all_pg, restore, checkpoint,c_checkpoint, erase_all};
    use crate::checkpoint::my_flash::{unlock, wait_ready, clear_error_flags, erase_page, write_to_flash};

    use stm32f3xx_hal_v2::{pac::{self, NVIC},pac::Peripherals, pac::FLASH, pac::Interrupt, gpio::{gpioa::PA0,Input, PullUp}};
    use volatile::Volatile;
    //use cortex_m::peripheral::NVIC;
    #[shared]
    struct Shared {
        // a: u32,
        // b: u32,
        // flash: pac::FLASH,
    }

    #[local]
    struct Local {
       // pa0: PA0<Input<PullUp>>
    }


    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        //let mut device: stm32f3xx_hal_v2::pac::Peripherals = ctx.device;

        //let mut dp = ctx.device;

        //     // Enable the clock for GPIOA and SYSCFG
        // dp.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());
        // dp.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());

        // // Configure PA0 as input
        // dp.GPIOA.moder.modify(|_, w| w.moder0().input());
        // dp.GPIOA.pupdr.modify(|_, w| w.pupdr0().pull_up());

        // dp.SYSCFG.exticr1.modify(|_, w| w.exti0().pa0());

        // // Configure EXTI0 for falling edge trigger and enable it
        // dp.EXTI.imr1.modify(|_, w| w.mr0().set_bit());
        // dp.EXTI.ftsr1.modify(|_, w| w.tr0().set_bit());

        //unsafe{NVIC::unmask(Interrupt::EXTI0)};
        
        // Enable EXTI0 interrupt in the NVIC
        delete_all_pg(); 
        restore();
        //update_regs();
        // c_checkpoint::spawn(true).ok();
        hprintln!("init");
        async_task1::spawn().ok();
        async_task2::spawn().ok();
        hprintln!("after spawn init");
       // static mut a:i32 = 1;
        // start_atomic();
        // unsafe{a = a + 2};
        // unsafe {
        //     asm!("mov r1, #6");
        //     asm!("mov r2, #2");
        //     asm!("mov r12, #2");
        // }  
        // end_atomic();
        //c_checkpoint::spawn(true).ok();
        //c_checkpoint(false);
        (Shared { }, Local { })
        
    } 

    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("idle");
        // start_atomic();
        unsafe {
        asm!("mov r1, #6");
        asm!("mov r2, #2");
        asm!("mov r3, #4");
        asm!("mov r4, #6");
        }
        // end_atomic();
        loop {
            // hprintln!("idle");
            debug::exit(debug::EXIT_SUCCESS);
        }
    }
    // This should be the highest priority task (priority should be set less than 1)
    // #[task(binds = EXTI0, priority = 2)]
    // fn gpio_interrupt(ctx: gpio_interrupt::Context) {
    //     unsafe {
    //         let peripherals = Peripherals::steal();
    //         peripherals.EXTI.pr1.modify(|_, w| w.pr0().set_bit());
    //     }
    //     c_checkpoint::spawn(false);
    // }
    //#[task(priority = 10)]
    //async fn c_checkpoint(ctx: c_checkpoint::Context, c_type: bool){
  
    #[task(priority = 3)]
    async fn async_task1(mut cx: async_task1::Context) {
        //checkpoint(false);
        //async_task2::spawn().ok();
        //start_atomic();
        hprintln!("I am in task1 before cp");
        c_checkpoint(false);// unsafe{asm!("nop")};//
        hprintln!("I am in task1 after cp");
        //end_atomic();
        // hprintln!(
        //     "hello from async 1 a {}",
        //     cx.shared.a.lock(|a| {
        //         *a += inc;
        //         *a
        //     })
        // );
    }

    #[task(priority = 4)]
    async fn async_task2(mut cx: async_task2::Context) {
        hprintln!("I am in task2");
        hprintln!("I am doing more operation");
        hprintln!("I am doing more operation 3");

        //c_checkpoint(false);
        async_task3::spawn().ok();

        // hprintln!(
        //     "hello from async 2 a {}",
        //     cx.shared.a.lock(|a| {
        //         *a += 1;
        //         *a
        //     })
        // );
    }

    #[task(priority = 5)]
    async fn async_task3(mut cx: async_task3::Context) {
        unsafe{asm!("NOP");}
        // hprintln!(
        //     "hello from async 3 a {}",
        //     cx.shared.a.lock(|a| {
        //         *a += 1;
        //         *a
        //     })
        // );
    }
    //fn restore(){}
}
