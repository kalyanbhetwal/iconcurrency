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

#[rtic::app(device = stm32f3xx_hal_v2::pac, dispatchers = [TIM2, TIM3, TIM4])]
mod app {
    
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
        async_task2::spawn().ok();
        cx.shared.a.lock(|a| {  *a += inc;});
        
    }

    #[task(priority = 2, shared = [a, b])]
    async fn async_task2(mut cx: async_task2::Context) {
        cx.shared.a.lock(|a| {*a += 1;}); 
        // hprintln!(
        //     "hello from async 2 a {}", cx.shared.a.lock(|a|{*a}));
    }
}
