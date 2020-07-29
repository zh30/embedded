#![no_std]
#![no_main]

extern crate panic_halt;

// use core::ptr;
use cortex_m::asm;
use cortex_m_rt::entry;
// use stm32f103xx;

#[entry]
fn main() -> ! {
    asm::nop();

    loop { }
}