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
mod checkpoint {
    #![allow(unsafe_code, non_upper_case_globals)]
    pub mod my_flash {
        #![allow(unsafe_code, unused, non_upper_case_globals)]
        #![no_main]
        #![no_std]
        use core::mem;
        use core::ptr;
        use cortex_m::asm::{nop, self};
        use cortex_m_rt::entry;
        use ::core::arch::asm;
        use cortex_m_semihosting::{debug, hprintln};
        use stm32f3xx_hal_v2::{
            self as hal, pac, prelude::*, flash::ACR, pac::Peripherals, pac::FLASH,
        };
        use volatile::Volatile;
        use stm32f3xx_hal_v2::hal::blocking::rng::Read;
        const UNLOCK_KEY1: u32 = 0x4567_0123;
        const UNLOCK_KEY2: u32 = 0xCDEF_89AB;
        pub fn unlock(flash: &mut FLASH) -> bool {
            if flash.cr.read().lock().bit_is_clear() {
                return true;
            }
            flash.keyr.write(|w| unsafe { w.bits(UNLOCK_KEY1) });
            flash.keyr.write(|w| unsafe { w.bits(UNLOCK_KEY2) });
            if flash.cr.read().lock().bit_is_clear() {
                return true;
            } else {
                return false;
            }
        }
        pub fn wait_ready(flash: &FLASH) {
            while flash.sr.read().bsy().bit() {}
        }
        pub fn clear_error_flags(flash: &FLASH) {
            if flash.sr.read().wrprterr().bit_is_set() {
                flash.sr.modify(|_, w| w.wrprterr().set_bit());
            }
            if flash.sr.read().pgerr().bit_is_set() {
                flash.sr.modify(|_, w| w.pgerr().set_bit());
            }
        }
        pub fn erase_page(flash: &mut FLASH, page: u32) {
            if flash.sr.read().bsy().bit_is_set() {
                ::cortex_m_semihosting::export::hstdout_str("Flash is busy.\n");
            }
            clear_error_flags(&flash);
            flash.cr.modify(|_, w| w.per().set_bit());
            flash.ar.write(|w| unsafe { w.bits(page as u32) });
            flash.cr.modify(|_, w| w.strt().set_bit());
            while flash.sr.read().bsy().bit_is_set() {}
            while flash.sr.read().bsy().bit_is_set() {}
            flash.cr.modify(|_, w| w.lock().set_bit());
        }
        pub fn write_to_flash(flash: &mut FLASH, addr: u32, data: u32) {
            unlock(flash);
            if flash.sr.read().bsy().bit_is_set() {
                ::cortex_m_semihosting::export::hstdout_str("Flash is busy.\n");
            }
            clear_error_flags(&flash);
            flash.cr.modify(|_, w| w.pg().set_bit());
            unsafe {
                ptr::write_volatile(addr as *mut u16, data as u16);
                ptr::write_volatile(
                    (addr as usize + 2) as *mut u16,
                    (data.wrapping_shr(16)) as u16,
                );
            }
            while flash.sr.read().bsy().bit_is_set() {}
            flash.cr.modify(|_, w| w.lock().set_bit());
            if flash.sr.read().eop().bit_is_set() {
                flash.sr.modify(|_, w| w.eop().set_bit());
            }
            flash.cr.modify(|_, w| w.pg().clear_bit());
        }
    }
    use my_flash::{unlock, wait_ready, clear_error_flags, erase_page, write_to_flash};
    use core::mem;
    use core::ptr;
    use cortex_m::asm::{nop, self};
    use cortex_m_semihosting::hprintln;
    use ::core::arch::asm;
    use stm32f3xx_hal_v2::{pac::Peripherals, pac::FLASH};
    use volatile::Volatile;
    pub static mut transcation_log: u32 = 0x60004000;
    pub static mut execution_mode: bool = true;
    pub static mut counter: *mut u8 = 0x60003002 as *mut u8;
    pub fn save_variables<T>(mem_loc: *const T, size: usize) {
        unsafe {
            let mem_loc_u8 = mem_loc as *const u8;
            for i in 0..4 {
                let byte = (mem_loc_u8 as u32 >> (i * 8)) as u8;
                ::cortex_m_semihosting::export::hstdout_fmt(
                    format_args!("bytes {0:0x}\n", byte),
                );
                ptr::write((transcation_log + 2 * i as u32) as *mut u8, byte);
            }
            transcation_log += 2 * 4;
            ptr::write(transcation_log as *mut u8, size as u8);
            transcation_log += 2 * 1;
            for i in 0..size {
                let byte = *mem_loc_u8.add(i);
                ::cortex_m_semihosting::export::hstdout_fmt(
                    format_args!("the logged byte {0}\n", byte),
                );
                ptr::write((transcation_log + 2 * i as u32) as *mut u8, byte);
            }
            transcation_log = transcation_log + 2 * size as u32;
            *counter += 1;
        }
        ::cortex_m_semihosting::export::hstdout_fmt(
            format_args!("Address: {0:p}, Size: {1} bytes\n", mem_loc, size),
        );
    }
    pub fn start_atomic() {
        unsafe {
            execution_mode = false;
        }
    }
    pub fn end_atomic() {
        unsafe {
            transcation_log = 0x60004000;
        }
        unsafe {
            execution_mode = true;
        }
    }
    #[no_mangle]
    pub fn checkpoint(c_type: bool) {
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
            asm!("MOV {0}, r6", out(reg) r6_value);
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
            asm!("MOV {0}, r14", out(reg) r14_lr);
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
        unsafe {
            let dp = Peripherals::steal();
            let mut flash = dp.FLASH;
            unlock(&mut flash);
            wait_ready(&flash);
            let mut start_address: u32;
            let end_address = r13_sp;
            asm!("movw r0, 0xFFFC\n             movt r0, 0x2000");
            asm!("MOV {0}, r0", out(reg) start_address);
            let stack_size = (start_address - end_address) + 4;
            let mut flash_start_address = Volatile::new(0x0803_0000);
            let mut flash_end_address = Volatile::new(0x0807_FFFF);
            let mut checkpoint_size = Volatile::new(0u32);
            asm::dmb();
            checkpoint_size.write(stack_size + 4 + 16 * 4 + 4 + 4);
            asm::dmb();
            loop {
                let mut offset = ptr::read_volatile(
                    flash_start_address.read() as *const u32,
                );
                if offset == 0xffff_ffff {
                    break;
                }
                flash_start_address.write(flash_start_address.read() + offset);
                if flash_start_address.read() + checkpoint_size.read()
                    >= flash_end_address.read()
                {
                    erase_all(&mut flash);
                    flash_start_address = Volatile::new(0x0803_0000);
                    break;
                }
            }
            asm::dmb();
            write_to_flash(
                &mut flash,
                (flash_start_address.read()) as u32,
                checkpoint_size.read() as u32,
            );
            flash_start_address.write(flash_start_address.read() + 4);
            asm::dmb();
            asm::dmb();
            if c_type {
                write_to_flash(
                    &mut flash,
                    flash_start_address.read() as u32,
                    0xDEADBEEF as u32,
                );
            } else {
                write_to_flash(
                    &mut flash,
                    flash_start_address.read() as u32,
                    0x0000_0001 as u32,
                );
            }
            while start_address >= end_address {
                let mut data = Volatile::new(0u32);
                data.write(core::ptr::read_volatile(start_address as *const u32));
                write_to_flash(
                    &mut flash,
                    flash_start_address.read() as u32,
                    data.read() as u32,
                );
                flash_start_address.write(flash_start_address.read() + 1 * 4);
                start_address = start_address - 4;
            }
            asm::dmb();
            asm::dmb();
            write_to_flash(
                &mut flash,
                (flash_start_address.read()) as u32,
                0xf1f1_f1f1 as u32,
            );
            flash_start_address.write(flash_start_address.read() + 4);
            asm::dmb();
        }
    }
    pub fn erase_all(flash: &mut FLASH) {
        let start_address = 0x0803_0000;
        for i in 0..100 {
            let page = start_address + i * 2 * 1024;
            erase_page(flash, page);
        }
    }
    pub fn restore_globals() {
        unsafe {
            let mut restore_ctr: u8 = 0;
            loop {
                if *counter == restore_ctr {
                    break;
                }
                let mut combined: u32 = 0;
                for i in 0..4 {
                    combined
                        |= (ptr::read((transcation_log + i) as *const u32) << (i * 8));
                }
                let mut size: u8 = ptr::read(transcation_log as *const u8);
                for i in 0..size {
                    ptr::write(
                        (combined + i as u32) as *mut u8,
                        *((transcation_log + i as u32) as *const u8),
                    );
                }
                combined = combined + size as u32;
                let end = ptr::read(combined as *const u8);
                restore_ctr += 1;
            }
        }
    }
    pub fn restore() -> bool {
        unsafe {
            let mut flash_start_address = 0x0803_0000;
            let packet_size = ptr::read_volatile(0x0803_0000 as *const u32);
            if packet_size == 0xffff_ffff {
                return false;
            }
            if ptr::read_volatile((flash_start_address + packet_size) as *const u32)
                == 0xffff_ffff
            {
                return false;
            }
            let mut offset: u32 = 0;
            loop {
                offset = ptr::read_volatile(flash_start_address as *const u32);
                if ptr::read_volatile((flash_start_address + offset) as *const u32)
                    == 0xffff_ffff
                {
                    break;
                }
                flash_start_address += offset;
            }
            flash_start_address += 4;
            if ptr::read_volatile(flash_start_address as *const u32) == 0xDEAD_BEEF {
                restore_globals();
                *counter = 0;
            }
            flash_start_address += 4;
            asm!("mov r0, {0}", in (reg) flash_start_address);
            asm!("movw r1, 0xfff8\n        movt r1, 0x02000");
            asm!("msr msp, r1");
            asm!("movw r3, 0xf1f1\n        movt r3, 0xf1f1");
            asm!(
                "1:\n            ldr r1, [r0, #4]\n            cmp r1, r3\n            beq 2f\n            push {{r1}}\n            adds r0, r0, #4\n            b 1b\n            2:"
            );
            asm!("adds r0, r0, #4");
            asm!("adds r0, r0, #4");
            asm!("LDR r1, [r0]");
            asm!("Push {{r1}}");
            asm!("adds r0, r0, #4");
            asm!("LDR r1, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r2, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r3, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r4, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r5, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r6, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r7, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r8, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r9, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r10, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r11, [r0]");
            asm!("adds r0, r0, #4");
            asm!("LDR r12, [r0]");
            asm!("adds r0, r0, #4");
            asm!("adds r0, r0, #4");
            asm!("LDR r14, [r0]");
            asm!("POP {{r0}}");
            asm!("cpsie i");
            asm!("mov r15, r14");
            asm!("adds sp, sp, #56");
            asm!("adds sp, sp, #8");
            asm!("POP {{r0, r1, r2, r3}}");
            asm!("adds sp, sp, #4");
            asm!("POP {{r4}}");
            asm!("adds sp, sp, #16");
            asm!("adds sp, sp, #64");
            asm!("mov pc, r4");
        }
        return true;
    }
    pub fn delete_pg(page: u32) {
        unsafe {
            let mut dp = Peripherals::steal();
            let mut flash = &mut dp.FLASH;
            unlock(&mut flash);
            wait_ready(&flash);
            erase_page(&mut flash, page);
        }
    }
    pub fn delete_all_pg() {
        let start_address = 0x0803_0000;
        unsafe {
            let mut dp = Peripherals::steal();
            let mut flash = &mut dp.FLASH;
            for i in 0..25 {
                let page = start_address + i * 2 * 1024;
                unlock(&mut flash);
                wait_ready(&flash);
                erase_page(&mut flash, page);
            }
        }
    }
    #[no_mangle]
    pub fn c_checkpoint(c_type: bool) {
        unsafe {
            asm!("push {{r1}}");
        }
        unsafe {
            asm!("add sp, #80");
        }
        unsafe { asm!("pop\t{{r8, r9, sl, fp}}") };
        unsafe { asm!("pop\t{{r4, r5, r6, r7, lr}}") };
        unsafe { asm!("push\t{{r4, r5, r6, r7, lr}}") };
        unsafe { asm!("push\t{{r8, r9, sl, fp}}") };
        unsafe {
            asm!("sub sp, #80");
        }
        unsafe {
            asm!("pop {{r1}}");
        }
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
            asm!("add r0, #108");
        }
        unsafe {
            asm!("MOV {0}, r0", out(reg) r13_sp);
        }
        unsafe {
            let dp = Peripherals::steal();
            let mut flash = dp.FLASH;
            unlock(&mut flash);
            wait_ready(&flash);
            let mut start_address: u32;
            let end_address = r13_sp;
            asm!("movw r0, 0xFFFC\n         movt r0, 0x2000");
            asm!("MOV {0}, r0", out(reg) start_address);
            let stack_size = (start_address - end_address) + 4;
            let mut flash_start_address = Volatile::new(0x0803_0000);
            let mut flash_end_address = Volatile::new(0x0807_FFFF);
            let mut checkpoint_size = Volatile::new(0u32);
            checkpoint_size.write(stack_size + 4 + 16 * 4 + 4 + 4);
            asm::dmb();
            loop {
                let mut offset = ptr::read_volatile(
                    flash_start_address.read() as *const u32,
                );
                if offset == 0xffff_ffff {
                    break;
                }
                flash_start_address.write(flash_start_address.read() + offset);
                if flash_start_address.read() + checkpoint_size.read()
                    >= flash_end_address.read()
                {
                    erase_all(&mut flash);
                    flash_start_address = Volatile::new(0x0803_0000);
                    break;
                }
            }
            asm::dmb();
            write_to_flash(
                &mut flash,
                (flash_start_address.read()) as u32,
                checkpoint_size.read() as u32,
            );
            flash_start_address.write(flash_start_address.read() + 4);
            asm::dmb();
            asm::dmb();
            if c_type {
                write_to_flash(
                    &mut flash,
                    flash_start_address.read() as u32,
                    0xDEADBEEF as u32,
                );
            } else {
                write_to_flash(
                    &mut flash,
                    flash_start_address.read() as u32,
                    0x0000_0001 as u32,
                );
            }
            while start_address >= end_address {
                let mut data = Volatile::new(0u32);
                data.write(core::ptr::read_volatile(start_address as *const u32));
                write_to_flash(
                    &mut flash,
                    flash_start_address.read() as u32,
                    data.read() as u32,
                );
                flash_start_address.write(flash_start_address.read() + 1 * 4);
                start_address = start_address - 4;
            }
            asm::dmb();
            asm::dmb();
            write_to_flash(
                &mut flash,
                (flash_start_address.read()) as u32,
                0xf1f1_f1f1 as u32,
            );
            flash_start_address.write(flash_start_address.read() + 4);
            asm::dmb();
            write_to_flash(
                &mut flash,
                flash_start_address.read() as u32,
                r0_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 4 as u32,
                r1_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 8 as u32,
                r2_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 12 as u32,
                r3_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 16 as u32,
                r4_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 20 as u32,
                r5_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 24 as u32,
                r6_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 28 as u32,
                r7_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 32 as u32,
                r8_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 36 as u32,
                r9_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 40 as u32,
                r10_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 44 as u32,
                r11_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 48 as u32,
                r12_value as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 52 as u32,
                r13_sp as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 56 as u32,
                r14_lr as u32,
            );
            write_to_flash(
                &mut flash,
                flash_start_address.read() + 60 as u32,
                r15_pc as u32,
            );
        }
    }
}
use checkpoint::{
    checkpoint, restore, delete_pg, delete_all_pg, transcation_log, execution_mode,
    counter, start_atomic, end_atomic,
};
use volatile::Volatile;
use checkpoint::my_flash::{
    unlock, wait_ready, clear_error_flags, erase_page, write_to_flash,
};
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
    use crate::checkpoint::end_atomic;
    use crate::checkpoint::start_atomic;
    use crate::checkpoint::save_variables;
    use crate::checkpoint::{
        self, delete_all_pg, restore, checkpoint, c_checkpoint, erase_all,
    };
    use crate::checkpoint::my_flash::{
        unlock, wait_ready, clear_error_flags, erase_page, write_to_flash,
    };
    use stm32f3xx_hal_v2::{
        pac::{self, NVIC},
        pac::Peripherals, pac::FLASH, pac::Interrupt, gpio::{gpioa::PA0, Input, PullUp},
    };
    use volatile::Volatile;
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
        delete_all_pg();
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
        ::cortex_m_semihosting::export::hstdout_str("idle\n");
        unsafe {
            asm!("mov r1, #6");
            asm!("mov r2, #2");
            asm!("mov r3, #4");
            asm!("mov r4, #6");
        }
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
                rtic::export::pend(stm32f3xx_hal_v2::pac::interrupt::TIM2);
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
                rtic::export::pend(stm32f3xx_hal_v2::pac::interrupt::TIM3);
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
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_async_task3_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_async_task3_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_async_task3_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    /// Spawns the task directly
    #[allow(non_snake_case)]
    #[doc(hidden)]
    pub fn __rtic_internal_async_task3_spawn() -> Result<(), ()> {
        unsafe {
            let exec = rtic::export::executor::AsyncTaskExecutor::from_ptr_1_args(
                async_task3,
                &__rtic_internal_async_task3_EXEC,
            );
            if exec.try_allocate() {
                exec.spawn(async_task3(unsafe { async_task3::Context::new() }));
                rtic::export::pend(stm32f3xx_hal_v2::pac::interrupt::TIM4);
                Ok(())
            } else {
                Err(())
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod async_task3 {
        #[doc(inline)]
        pub use super::__rtic_internal_async_task3_Context as Context;
        #[doc(inline)]
        pub use super::__rtic_internal_async_task3_spawn as spawn;
    }
    #[allow(non_snake_case)]
    async fn async_task1<'a>(mut cx: async_task1::Context<'a>) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("I am in task1 before cp\n");
        c_checkpoint(false);
        ::cortex_m_semihosting::export::hstdout_str("I am in task1 after cp\n");
    }
    #[allow(non_snake_case)]
    async fn async_task2<'a>(mut cx: async_task2::Context<'a>) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("I am in task2\n");
        ::cortex_m_semihosting::export::hstdout_str("I am doing more operation\n");
        ::cortex_m_semihosting::export::hstdout_str("I am doing more operation 3\n");
        async_task3::spawn().ok();
    }
    #[allow(non_snake_case)]
    async fn async_task3<'a>(mut cx: async_task3::Context<'a>) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        unsafe {
            asm!("NOP");
        }
    }
    #[allow(non_upper_case_globals)]
    static __rtic_internal_async_task1_EXEC: rtic::export::executor::AsyncTaskExecutorPtr = rtic::export::executor::AsyncTaskExecutorPtr::new();
    #[allow(non_upper_case_globals)]
    static __rtic_internal_async_task2_EXEC: rtic::export::executor::AsyncTaskExecutorPtr = rtic::export::executor::AsyncTaskExecutorPtr::new();
    #[allow(non_upper_case_globals)]
    static __rtic_internal_async_task3_EXEC: rtic::export::executor::AsyncTaskExecutorPtr = rtic::export::executor::AsyncTaskExecutorPtr::new();
    #[allow(non_snake_case)]
    ///Interrupt handler to dispatch async tasks at priority 3
    #[no_mangle]
    unsafe fn TIM2() {
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
                    rtic::export::pend(stm32f3xx_hal_v2::pac::interrupt::TIM2);
                });
            },
        );
    }
    #[allow(non_snake_case)]
    ///Interrupt handler to dispatch async tasks at priority 4
    #[no_mangle]
    unsafe fn TIM3() {
        /// The priority of this interrupt handler
        const PRIORITY: u8 = 4u8;
        rtic::export::run(
            PRIORITY,
            || {
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
                    rtic::export::pend(stm32f3xx_hal_v2::pac::interrupt::TIM3);
                });
            },
        );
    }
    #[allow(non_snake_case)]
    ///Interrupt handler to dispatch async tasks at priority 5
    #[no_mangle]
    unsafe fn TIM4() {
        /// The priority of this interrupt handler
        const PRIORITY: u8 = 5u8;
        rtic::export::run(
            PRIORITY,
            || {
                let exec = rtic::export::executor::AsyncTaskExecutor::from_ptr_1_args(
                    async_task3,
                    &__rtic_internal_async_task3_EXEC,
                );
                exec.poll(|| {
                    let exec = rtic::export::executor::AsyncTaskExecutor::from_ptr_1_args(
                        async_task3,
                        &__rtic_internal_async_task3_EXEC,
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
                        "Maximum priority used by interrupt vector \'TIM2\' is more than supported by hardware",
                    ),
                );
            };
        };
        core.NVIC
            .set_priority(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::TIM2,
                rtic::export::cortex_logical2hw(
                    3u8,
                    stm32f3xx_hal_v2::pac::NVIC_PRIO_BITS,
                ),
            );
        rtic::export::NVIC::unmask(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::TIM2,
        );
        const _: () = if (1 << stm32f3xx_hal_v2::pac::NVIC_PRIO_BITS) < 4u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'TIM3\' is more than supported by hardware",
                    ),
                );
            };
        };
        core.NVIC
            .set_priority(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::TIM3,
                rtic::export::cortex_logical2hw(
                    4u8,
                    stm32f3xx_hal_v2::pac::NVIC_PRIO_BITS,
                ),
            );
        rtic::export::NVIC::unmask(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::TIM3,
        );
        const _: () = if (1 << stm32f3xx_hal_v2::pac::NVIC_PRIO_BITS) < 5u8 as usize {
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
                    5u8,
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
        let executor = ::core::mem::ManuallyDrop::new(
            rtic::export::executor::AsyncTaskExecutor::new_1_args(async_task3),
        );
        executors_size += ::core::mem::size_of_val(&executor);
        __rtic_internal_async_task3_EXEC.set_in_main(&executor);
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
