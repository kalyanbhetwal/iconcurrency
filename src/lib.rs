#![no_main]
#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};
use defmt_brtt as _; // global logger

use panic_probe as _;

// TODO(6) Import your HAL
use stm32f3xx_hal_v2 as _; // memory layout
use cortex_m::peripheral::SCB;
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::exception;
use cortex_m_semihosting::hprintln; // Import the `hprintln!` macro


// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

static COUNT: AtomicUsize = AtomicUsize::new(0);
defmt::timestamp!("{=usize}", {
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n
});

/// Terminates the application and makes `probe-rs` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    let scb = unsafe { &*(SCB::PTR as *const SCB) };
    
    // Read fault status registers
    let hfsr = scb.hfsr.read();
    let cfsr = scb.cfsr.read();
    let mmfar = scb.mmfar.read();
    let bfar = scb.bfar.read();

    // Extract fault information
    let mem_manage_fault = (cfsr & 0xFF) as u8;
    let bus_fault = ((cfsr >> 8) & 0xFF) as u8;
    let usage_fault = ((cfsr >> 16) & 0xFF) as u8;

    // Print detailed fault information
    hprintln!("HardFault occurred!").ok();
    hprintln!("HFSR: 0x{:08X}", hfsr).ok();
    hprintln!("CFSR: 0x{:08X}", cfsr).ok();
    hprintln!("MMFAR: 0x{:08X}", mmfar).ok();
    hprintln!("BFAR: 0x{:08X}", bfar).ok();
    hprintln!("ExceptionFrame: {:?}", ef).ok();

    // Print detailed explanations of the faults
    print_cfsr_explanations(mem_manage_fault, bus_fault, usage_fault);
    
    // Halt execution
    loop {
        cortex_m::asm::bkpt(); // Trigger a breakpoint for debugging
    }
}

fn print_cfsr_explanations(mem_manage_fault: u8, bus_fault: u8, usage_fault: u8) {
    hprintln!("Configurable Fault Status Register (CFSR):").ok();

    // Memory Management Fault
    hprintln!("Memory Management Fault Status:").ok();
    if (mem_manage_fault & 0x01) != 0 {
        hprintln!("  - Memory Management Fault occurred due to a failed MPU check.").ok();
    }
    if (mem_manage_fault & 0x02) != 0 {
        hprintln!("  - Access violation caused by a failed MPU region.").ok();
    }
    if (mem_manage_fault & 0x04) != 0 {
        hprintln!("  - DWT Unit: Debug Watchpoint Unit event caused a fault.").ok();
    }
    
    // Bus Fault
    hprintln!("Bus Fault Status:").ok();
    if (bus_fault & 0x01) != 0 {
        hprintln!("  - Instruction Bus Error").ok();
    }
    if (bus_fault & 0x02) != 0 {
        hprintln!("  - Data Bus Error").ok();
    }
    if (bus_fault & 0x08) != 0 {
        hprintln!("  - Unstacking Error").ok();
    }
    if (bus_fault & 0x10) != 0 {
        hprintln!("  - Stack Error").ok();
    }
    
    // Usage Fault
    hprintln!("Usage Fault Status:").ok();
    if (usage_fault & 0x01) != 0 {
        hprintln!("  - Undefined Instruction").ok();
    }
    if (usage_fault & 0x02) != 0 {
        hprintln!("  - Invalid State").ok();
    }
    if (usage_fault & 0x08) != 0 {
        hprintln!("  - Invalid PC Load").ok();
    }
    if (usage_fault & 0x10) != 0 {
        hprintln!("  - Division by Zero").ok();
    }
}
