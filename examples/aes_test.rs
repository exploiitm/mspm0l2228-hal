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
use mspm0l2228_hal::uart::UartWrite;

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
    let dma = mspm0l2228_hal::dma::Dma::new(p.dma);
    let dma_chans = dma.chans();

    let dma_chan0 = dma_chans.chan0;
    let dma_chan1 = dma_chans.chan1;

    let aesadv = mspm0l2228_hal::aes::AesAdv::new(p.aesadv, [0, 0, 0, 0]);
    let mut uart = mspm0l2228_hal::uart::Uart0::new(p.uart0);

    uart.write_byte(0xde as u8);
    delay(0x20);
    uart.write_byte(0xad as u8);
    delay(0x20);
    uart.write_byte(0xc0 as u8);
    delay(0x20);
    uart.write_byte(0xde as u8);
    delay(0x20);

    let arr = [0; 4];
    let mut res = [0; 4];
    aesadv.encrypt(&arr, &mut res);
    for x in res {
        let word1 = x & 0xff;
        let word2 = (x >> 8) & 0xff;
        let word3 = (x >> 16) & 0xff;
        let word4 = (x >> 24) & 0xff;

        uart.write_byte(word1 as u8);
        uart.write_byte(word2 as u8);
        uart.write_byte(word3 as u8);
        uart.write_byte(word4 as u8);

        delay(0x20);
    }

    let arr = [0; 8];
    let iv = [0; 4];
    let mut res = [0; 8];
    let aesadv = aesadv.use_dma(dma_chan0, dma_chan1).to_cbc(iv);
    aesadv.encrypt(&dma, &arr, &mut res);
    for x in res {
        let word1 = x & 0xff;
        let word2 = (x >> 8) & 0xff;
        let word3 = (x >> 16) & 0xff;
        let word4 = (x >> 24) & 0xff;

        uart.write_byte(word1 as u8);
        uart.write_byte(word2 as u8);
        uart.write_byte(word3 as u8);
        uart.write_byte(word4 as u8);

        delay(0x20);
    }
    uart.write_byte(0xde as u8);
    delay(0x20);
    uart.write_byte(0xad as u8);
    delay(0x20);
    uart.write_byte(0xc0 as u8);
    delay(0x20);
    uart.write_byte(0xde as u8);
    delay(0x20);

    loop {
        // uart.write_byte(0x61);
        delay(0x2000);
    }
}
