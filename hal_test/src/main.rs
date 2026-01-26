/*
*   Tests GPIO and Trng
*   Randomly tries lighting all 3 LEDS
* */

#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

use core::{arch::asm, ptr::slice_from_raw_parts};
use mspm0l2228_hal::{
    flash::{CommandSize, Flash, FlashError},
    pac,
    trng::{self, Trng},
    uart::{UartRead, UartWrite},
};

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let gpioa = mspm0l2228_hal::gpio::gpioa::new(p.gpioa);
    let gpiob = mspm0l2228_hal::gpio::gpiob::new(p.gpiob);
    let gpioa_pins = gpioa.pins();
    let gpiob_pins = gpiob.pins();
    let mut blue_led = gpioa_pins.pin16.into_output();
    let mut red_led = gpiob_pins.pin10.into_output();
    let mut green_led = gpiob_pins.pin9.into_output();

    use core::fmt::Write;
    let mut uart0 = mspm0l2228_hal::uart::Uart0::new(p.uart0, &p.iomux);
    let mut uart1 = mspm0l2228_hal::uart::Uart1::new(p.uart1, &p.iomux);
    _ = writeln!(uart0, "New boot, New life");

    //    use p256::{
    //        ecdsa::{SigningKey, signature::Signer},
    //        ecdsa::{VerifyingKey, signature::Verifier},
    //    };

    let mut trng =
        match mspm0l2228_hal::trng::Trng::new(p.trng, mspm0l2228_hal::trng::ClockDiv::Div2, 0x3) {
            Ok(trng) => trng,
            Err(_) => loop {
                red_led.set_high();
            },
        };

    let mut flash = Flash::new(p.flashctl, &mut p.cpuss, &p.factoryregion, &p.sysctl);

    match flash.execute_erase_memory_from_ram(0x0, CommandSize::Bank) {
        Ok(()) => {
            blue_led.set_high();
        }
        Err(e) => loop {
            _ = writeln!(uart0, "error {e:?}");
            red_led.set_high();
        },
    };

    match flash.simple_write(0x10000, &[0x69696969, 0x69696969, 0x69696969, 0x69696969]) {
        Ok(()) => {
            green_led.set_high();
        }
        Err(e) => loop {
            _ = writeln!(uart0, "error {e:?}");
            red_led.set_high();
        },
    };

    let a = 0x10000 as *const u8;
    let mut buffer = [0u8; 16];
    let myslice = unsafe { slice_from_raw_parts(a, 16).as_ref() };
    let myslice = match myslice {
        Some(a) => a,
        None => loop {
            red_led.set_high();
        },
    };
    // _ = writeln!(uart0, "Copying from flash");
    // buffer.copy_from_slice(myslice);
    // _ = writeln!(uart0, "Finshed copying from flash");
    loop {
        green_led.set_high();
        blue_led.set_low();
        red_led.set_high();

        for (index, val) in myslice.iter().enumerate() {
            _ = writeln!(uart0, "{index} : addr = {:?} : {:x}", val as *const u8, val);
        }
    }

    // let pin21 = gpioa_pins.pin21.into_output();
    // let pin22 = gpioa_pins.pin22.into_output();
    // let pin17 = gpioa_pins.pin17.into_output();
    // let device_id = p.factoryregion.deviceid().read().bits();
    // loop {
    //     green_led.set_high();
    //     _ = writeln!(uart0, "Hello {:x} World", device_id);
    // }

    //    loop {
    //        use k256::{EncodedPoint, PublicKey, ecdh::EphemeralSecret};
    //
    //        pin21.set_high();
    //        let alice_secret = EphemeralSecret::random(&mut trng);
    //        let alice_pk = EncodedPoint::from(alice_secret.public_key());
    //        pin21.set_low();
    //
    //        pin21.set_high();
    //        let bob_secret = EphemeralSecret::random(&mut trng);
    //        let bob_pk = EncodedPoint::from(bob_secret.public_key());
    //        pin21.set_low();
    //
    //        let bob_public = match PublicKey::from_sec1_bytes(bob_pk.as_ref()) {
    //            Ok(bp) => bp,
    //            Err(_) => loop {
    //                red_led.set_high();
    //            },
    //        };
    //
    //        pin22.set_high();
    //        let alice_shared = alice_secret.diffie_hellman(&bob_public);
    //        pin22.set_low();
    //
    //        let alice_public = match PublicKey::from_sec1_bytes(alice_pk.as_ref()) {
    //            Ok(ap) => ap,
    //            Err(_) => loop {
    //                red_led.set_high();
    //            },
    //        };
    //
    //        pin22.set_high();
    //        let bob_shared = bob_secret.diffie_hellman(&alice_public);
    //        pin22.set_low();
    //
    //        if !alice_shared
    //            .raw_secret_bytes()
    //            .eq(&bob_shared.raw_secret_bytes())
    //        {
    //            loop {
    //                blue_led.set_high();
    //            }
    //        }
    //    }
    //
    // let mut buffer = [0; 4];
    // loop {
    //     use hmac::{Hmac, Mac};
    //     use sha3::Sha3_512;
    //     type HmacSha3 = Hmac<Sha3_512>;
    //     for i in 0..4 {
    //         let x = uart.read_byte_blocking();
    //         green_led.set_high();
    //         buffer[i] = (x as usize) - 0x30;
    //     }
    //     let length: usize =
    //         (buffer[0] * 1000 + buffer[1] * 100 + buffer[2] * 10 + buffer[3]) as usize;

    //     _ = writeln!(uart, "{length}");

    //     for x in &mut buffer {
    //         *x = 0;
    //     }

    //     pin21.set_high();
    //     pin22.set_high();
    //     pin17.set_high();
    //     let mut mac = match HmacSha3::new_from_slice(b"this is the secret key") {
    //         Ok(mac) => mac,
    //         Err(_) => loop {
    //             red_led.set_high();
    //         },
    //     };
    //     pin17.set_low();
    //     mac.update(&target[0..length]);
    //     pin22.set_low();
    //     let result = mac.finalize();
    //     pin21.set_low();
    //     let code_byes = result.into_bytes();
    //     // for c in code_byes {
    //     // _ = write!(uart, "{c:x}");
    //     // }
    //     // _ = write!(uart, "\n");
    // }

    // loop {
    //     use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

    //     let signing_key = b"12345678123456781234567812345678";
    //     let signing_key = SigningKey::from_bytes(signing_key);

    //     let message = b"We are signing a message of length 37";

    //     pin21.set_high();
    //     let signature: Signature = signing_key.sign(message);
    //     pin21.set_low();

    //     let verifying_key = VerifyingKey::from(&signing_key);
    //     pin17.set_high();
    //     let message = b"We are signing a message of length 37";
    //     match verifying_key.verify(message, &signature) {
    //         Ok(_) => {}
    //         Err(_) => {
    //             pin22.set_high();
    //         }
    //     }
    //     pin17.set_low();

    //     pin22.set_low();
    // }

    // loop {
    // while i < 4 {
    //     let x = uart.read_byte_blocking();
    //     buffer[i] = (x as usize) - 0x30;
    //     i += 1;
    // }
    // i = 0;
    // let length: usize =
    //     (buffer[0] * 1000 + buffer[1] * 100 + buffer[2] * 10 + buffer[3]) as usize;

    // for x in &mut buffer {
    //     *x = 0;
    // }

    // let signing_key = b"123456781234567812345678";
    // let signing_key = match SigningKey::from_slice(signing_key) {
    //     Ok(sk) => sk,
    //     Err(e) => loop {
    //         _ = writeln!(uart, "{e:?}");
    //         red_led.set_high()
    //     },
    // };

    // let message = b"We are signing a message of length 37";

    // pin21.set_high();
    // let signature: Signature = signing_key.sign(message);
    // pin21.set_low();

    // let verifying_key = VerifyingKey::from(&signing_key);
    // pin17.set_high();
    // match verifying_key.verify(message, &signature) {
    //     Ok(_) => {}
    //     Err(_) => loop {
    //         green_led.set_high()
    //     },
    // }
    // pin17.set_low();

    // let mut hasher = Sha3_512::new();
    // hasher.update(&target[0..length]);
    // let hash = hasher.finalize();

    // for c in hash {
    //     _ = write!(uart, "{c:x}");
    // }
    // _ = write!(uart, "\n");
    // }
}
