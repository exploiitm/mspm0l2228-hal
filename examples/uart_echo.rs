/*
*   Tests UART
* */
#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

use core::arch::asm;
use mspm0l2228_hal::pac;
use mspm0l2228_hal::uart::{UartRead, UartWrite};

#[inline(always)]
fn delay(millis: u32) {
    // 75449 -> .98126864
    for _ in 0..(millis * 76889 / 1000) {
        unsafe { asm!("nop") };
    }
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let uart = mspm0l2228_hal::uart::Uart0::new(p.uart0, &p.iomux);

    // Start-frame
    uart.write_byte(0xde as u8);
    delay(0x20);
    uart.write_byte(0xad as u8);
    delay(0x20);
    uart.write_byte(0xc0 as u8);
    delay(0x20);
    uart.write_byte(0xde as u8);
    delay(0x20);

    loop {
        uart.write_byte('>' as u8);
        delay(0x20);
        let x = uart.read_byte_blocking();
        uart.write_byte(x);
        delay(0x200);
    }
}
