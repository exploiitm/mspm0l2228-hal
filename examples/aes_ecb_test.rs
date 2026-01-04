#![no_std]
#![no_main]

use core::arch::asm;
use cortex_m_rt::entry;
use mspm0l2228_hal::uart::UartWrite;
use mspm0l2228_hal::{pac, trng};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let dma = mspm0l2228_hal::dma::Dma::new(p.dma);
    let dma_chans = dma.chans();
    let dma_chan0 = dma_chans.chan0;
    let dma_chan1 = dma_chans.chan1;
    let uart = mspm0l2228_hal::uart::Uart0::new(p.uart0, &p.iomux);
    let trng =
        mspm0l2228_hal::trng::Trng::new(p.trng, trng::ClockDiv::Div2, 0x3)
            .unwrap();

    send_start_frame(&uart);
    let mut key = [0x00; 4];
    for i in 0..4 {
        key[i] = trng.trng_gen_u32();
    }
    send_blocks(&uart, &key);
    let aesadv = mspm0l2228_hal::aes::AesAdv::new(p.aesadv, key).to_ecb();

    for _ in 0..5 {
        let mut arr = [0x00; 4];
        for i in 0..4 {
            arr[i] = trng.trng_gen_u32();
        }

        let mut res = [0x00; 4];
        aesadv.encrypt(&arr, &mut res);
        send_blocks(&uart, &arr);
        send_blocks(&uart, &res);
    }

    for _ in 0..5 {
        let mut arr = [0x00; 4];
        for i in 0..4 {
            arr[i] = trng.trng_gen_u32();
        }

        let mut res = [0x00; 4];
        aesadv.decrypt(&arr, &mut res);
        send_blocks(&uart, &arr);
        send_blocks(&uart, &res);
    }

    let aesadv = aesadv.use_dma(dma_chan0, dma_chan1);
    for _ in 0..5 {
        let mut arr = [0x00; 16];
        for i in 0..16 {
            arr[i] = trng.trng_gen_u32();
        }
        let mut res = [0; 16];
        aesadv.encrypt(&dma, &arr, &mut res).unwrap();
        send_blocks(&uart, &arr);
        send_blocks(&uart, &res);
    }

    for _ in 0..5 {
        let mut arr = [0x00; 16];
        for i in 0..16 {
            arr[i] = trng.trng_gen_u32();
        }
        let mut res = [0; 16];
        aesadv.decrypt(&dma, &arr, &mut res).unwrap();
        send_blocks(&uart, &arr);
        send_blocks(&uart, &res);
    }

    loop {}
}

fn send_blocks(uart: &mspm0l2228_hal::uart::Uart0, res: &[u32]) {
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
}

fn send_start_frame(uart: &mspm0l2228_hal::uart::Uart0) {
    uart.write_byte(0xde as u8);
    delay(0x20);
    uart.write_byte(0xad as u8);
    delay(0x20);
    uart.write_byte(0xc0 as u8);
    delay(0x20);
    uart.write_byte(0xde as u8);
    delay(0x20);
}

#[inline(always)]
fn delay(millis: u32) {
    // 75449 -> .98126864
    for _ in 0..(millis * 76889 / 1000) {
        unsafe { asm!("nop") };
    }
}
