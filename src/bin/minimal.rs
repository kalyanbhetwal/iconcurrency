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
    use crate::checkpoint::{self, delete_all_pg, restore, checkpoint, erase_all};
    use crate::checkpoint::my_flash::{unlock, wait_ready, clear_error_flags, erase_page, write_to_flash};

    use stm32f3xx_hal_v2::{pac::{self, NVIC},pac::Peripherals, pac::FLASH, pac::Interrupt, gpio::{gpioa::PA0,Input, PullUp}};
    use volatile::Volatile;
    //use cortex_m::peripheral::NVIC;
    #[shared]
    struct Shared {
        a: u32,
        b: u32,
        flash: pac::FLASH,
    }

    #[local]
    struct Local {
       // pa0: PA0<Input<PullUp>>
    }


    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        //let mut device: stm32f3xx_hal_v2::pac::Peripherals = ctx.device;

        let mut dp = ctx.device;

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
       // delete_all_pg(); 
        restore();
        //update_regs();
        // c_checkpoint::spawn(true).ok();
        hprintln!("init");
        // //async_task1::spawn(1).ok();
        // hprintln!("after spawn init");
        unsafe {
            asm!("mov r1, #6");
            asm!("mov r2, #2");
            asm!("mov r3, #4");
            asm!("mov r4, #6");
            asm!("mov r6, #12");
            asm!("mov r8, #9");
            asm!("mov r9, #6");
            asm!("mov r12, #2");
        }
        //c_checkpoint::spawn(true).ok();
        c_checkpoint(false);
        (Shared { flash: dp.FLASH, a: 0, b: 0 }, Local { })
        
    } 

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            hprintln!("idle");
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
    #[no_mangle]
    fn c_checkpoint( c_type: bool){
        //cortex_m::interrupt::disable();
        unsafe{asm!("add sp, #76");}
        unsafe{asm!("pop	{{r8, r9, sl, fp}}")};
        unsafe{asm!("pop	{{r4, r5, r6, r7, lr}}")};
        unsafe{asm!("push	{{r4, r5, r6, r7, lr}}")};
        unsafe{asm!("push	{{r8, r9, sl, fp}}")};
        unsafe{asm!("sub sp, #76");}
        let r0_value: u32;
        let r1_value: u32;
        let r2_value: u32;
        let r3_value: u32;
        let r4_value: u32;
        let r5_value: u32;
        let r6_value: u32;
        let r7_value: u32;
        let r8_value: u32;
        let r9_value: u32;
        let r10_value: u32;
        let r11_value: u32;
        let r12_value: u32;
        let r13_sp: u32;
        let r14_lr: u32;
        let r15_pc: u32;
        unsafe {
            asm!("MOV {0}, r0", out(reg) r0_value);
        }
        unsafe {
            asm!("MOV {0}, r1", out(reg) r1_value);
        }
        unsafe {
            asm!("MOV {0}, r2", out(reg) r2_value);
        }
        unsafe {
            asm!("MOV {0}, r3", out(reg) r3_value);
        }
        unsafe {
            asm!("MOV {0}, r4", out(reg) r4_value);
        }
        unsafe {
            asm!("MOV {0}, r5", out(reg) r5_value);
        }
        unsafe {
            asm!("MOV {0}, r14", out(reg) r14_lr);
        }
        unsafe {
            asm!("MOV {0}, r7", out(reg) r7_value);
        }
        unsafe {
            asm!("MOV {0}, r8", out(reg) r8_value);
        }
        unsafe {
            asm!("MOV {0}, r9", out(reg) r9_value);
        }
        unsafe {
            asm!("MOV {0}, r10", out(reg) r10_value);
        }
        unsafe {
            asm!("MOV {0}, r11", out(reg) r11_value);
        }
        unsafe {
            asm!("MOV {0}, r12", out(reg) r12_value);
        }
        unsafe {
            asm!("MOV {0}, r6", out(reg) r6_value);
        }
        unsafe {
            asm!("MOV {0}, r15", out(reg) r15_pc);
        }
        unsafe {
            asm!("MOV r0, sp");
        }
        unsafe {
            asm!("add r0, #112");
        }
        unsafe {
            asm!("MOV {0}, r0", out(reg) r13_sp);
        }

       //let mut flash = ctx.shared.flash;
       unsafe{
        let  dp = Peripherals::steal();
        let mut flash= dp.FLASH;
        unlock(&mut flash);
        wait_ready(&flash);

   
        //let  start_address: u32 = 0x2000_fffc as u32;
        let mut start_address:u32;
        let  end_address = r13_sp;
        asm!("movw r0, 0xFFF8
             movt r0, 0x2000");

         asm!(
             "MOV {0}, r0",
             out(reg) start_address
         );

         let stack_size = (start_address - end_address) + 4;
        // leaving first xyz K for program i.e start at 0x0801_0000
         let mut flash_start_address = Volatile::new(0x0803_0000);
         let mut flash_end_address = Volatile::new(0x0807_FFFF);    

        let mut checkpoint_size= Volatile::new(0u32);

         // 1. stack size
        // 2. 4 bytes -> 0xf1f1_f1f1 (end of stack in the frame magic number)
        // 3. 16 * 4 -> all the cpu registers
        // 4. 4 bytes -> size of frame
        // 5. 4 bytes -> 0xDEADBEEF (magic number to indicate the static checkpoint)
        checkpoint_size.write(stack_size+4+16*4 +4 +4);
        asm::dmb();

        loop{
            let mut offset = ptr::read_volatile(flash_start_address.read() as *const u32);
            if offset == 0xffff_ffff{
                break;
            }
            flash_start_address.write(flash_start_address.read() + offset); 
            if flash_start_address.read() + checkpoint_size.read() >= flash_end_address.read() {
               erase_all(&mut flash);
               flash_start_address = Volatile::new(0x0803_0000);
               break;
            }
        }
        asm::dmb();
        //write the size of packet at the begining of the packet
        write_to_flash(&mut flash,  (flash_start_address.read()) as u32, checkpoint_size.read() as u32); 
        flash_start_address.write(flash_start_address.read()+4);
        asm::dmb();
        asm::dmb(); 
        if c_type {
            //write at the begining of checkpoint fram so magic number indicate jit or static checkpoint
            write_to_flash(&mut flash,  flash_start_address.read() as u32, 0xDEADBEEF as u32);
        }
        else{
            write_to_flash(&mut flash,  flash_start_address.read() as u32,  0x0000_0001 as u32);
        }
         while start_address >= end_address{
            let mut data = Volatile::new(0u32);
            data.write(core::ptr::read_volatile(start_address as * const u32));
            write_to_flash(&mut flash,  flash_start_address.read() as u32, data.read() as u32);
            flash_start_address.write(flash_start_address.read() +1* 4);
            // Move to the next address based on the size of the type
            start_address = start_address-4;
            
        }
        asm::dmb();
        asm::dmb();
    //mark the end of the stack
    write_to_flash(&mut flash,  (flash_start_address.read()) as u32, 0xf1f1_f1f1 as u32);
    flash_start_address.write(flash_start_address.read() + 4);
    asm::dmb();

    // for i in 0..15{
    //     write_to_flash(&mut flash,  0x0800_9060 as u32, r0_value as u32);
    //       flash_start_address = flash_start_address + 4;
    // }

    write_to_flash(&mut flash,  flash_start_address.read() as u32, r0_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+4 as u32, r1_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+8 as u32, r2_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+12 as u32, r3_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+16 as u32, r4_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+20 as u32, r5_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+24 as u32, r6_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+28 as u32, r7_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+32 as u32, r8_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+36 as u32, r9_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+40 as u32, r10_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+44 as u32, r11_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+48 as u32, r12_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+52 as u32, r13_sp as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+56 as u32, r14_lr as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+60 as u32, r15_pc as u32);
    // drop(flash);  
    //hprintln!("test  test");

    }
}

    // #[task(priority = 3, shared = [a])]
    // async fn async_task1(mut cx: async_task1::Context, inc: u32) {
    //     //checkpoint(false);
    //     async_task2::spawn().ok();
    //     hprintln!(
    //         "hello from async 1 a {}",
    //         cx.shared.a.lock(|a| {
    //             *a += inc;
    //             *a
    //         })
    //     );
    // }

    // #[task(priority = 4, shared = [a, b])]
    // async fn async_task2(mut cx: async_task2::Context) {
    //     hprintln!("I am in task2");
    //     async_task3::spawn().ok();

    //     // hprintln!(
    //     //     "hello from async 2 a {}",
    //     //     cx.shared.a.lock(|a| {
    //     //         *a += 1;
    //     //         *a
    //     //     })
    //     // );
    // }

    // #[task(priority = 4, shared = [a])]
    // async fn async_task3(mut cx: async_task3::Context) {
    //     hprintln!(
    //         "hello from async 3 a {}",
    //         cx.shared.a.lock(|a| {
    //             *a += 1;
    //             *a
    //         })
    //     );
    // }
}
