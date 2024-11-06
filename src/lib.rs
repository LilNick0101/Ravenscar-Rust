#![no_main]
#![no_std]

pub mod production_workload;
pub mod activation_log;
pub mod activation_manager;
pub mod auxiliary;
pub mod constants;
pub mod system_overhead;

use core::sync::atomic::{AtomicUsize, Ordering};
use defmt_brtt as _; // global logger

use panic_probe as _;

use rtic_monotonics::stm32::prelude::*;
use stm32f3xx_hal as _; // memory layout

stm32_tim2_monotonic!(Mono,1_000_000);

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
/*
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
*/

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
