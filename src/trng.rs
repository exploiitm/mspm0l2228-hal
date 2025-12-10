use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use core::error::Error;

pub enum ClockDiv {
    Div1,
    Div2,
    Div4,
    Div6,
    Div8,
}

pub struct Trng {
    _trng: crate::pac::Trng,
}

#[derive(Debug)]
pub enum TrngInitError {
    DigitalBlockHealthCheck(u8),
    AnalogBlockHealthCheck,
}

impl Trng {
    pub fn new(
        trng: crate::pac::Trng,
        div: ClockDiv,
        decim: u8,
    ) -> Result<Self, TrngInitError> {
        Self::reset(&trng);
        Self::enable_power(&trng);

        trng.trng_clkdivide().write(|w| match div {
            ClockDiv::Div1 => w.ratio().div_by_1(),
            ClockDiv::Div2 => w.ratio().div_by_2(),
            ClockDiv::Div4 => w.ratio().div_by_4(),
            ClockDiv::Div6 => w.ratio().div_by_6(),
            ClockDiv::Div8 => w.ratio().div_by_8(),
        });

        trng.trng_imask().write(|w| {
            w.irq_health_fail().disabled();
            w.irq_cmd_fail().disabled();
            w.irq_cmd_done().disabled();
            w.irq_captured_rdy().disabled()
        });

        trng.trng_ctl().write(|w| w.cmd().norm_func());
        while !trng.trng_ris().read().irq_cmd_done().is_set() {}
        trng.trng_iclr().write(|w| w.irq_cmd_done().clr());

        // Check Digital Block Health
        trng.trng_ctl().write(|w| w.cmd().pwrup_dig());
        while !trng.trng_ris().read().irq_cmd_done().is_set() {}
        trng.trng_iclr().write(|w| w.irq_cmd_done().clr());

        if trng.trng_test_results().read().dig_test().bits() != 0xFF {
            return Err(TrngInitError::DigitalBlockHealthCheck(
                trng.trng_test_results().read().dig_test().bits(),
            ));
        }

        // Check Analog Block Health
        trng.trng_ctl().write(|w| w.cmd().pwrup_ana());
        while !trng.trng_ris().read().irq_cmd_done().is_set() {}
        trng.trng_iclr().write(|w| w.irq_cmd_done().clr());

        if trng.trng_test_results().read().ana_test() == false {
            return Err(TrngInitError::AnalogBlockHealthCheck);
        }

        trng.trng_iclr().write(|w| w.irq_captured_rdy().clr());
        trng.trng_ctl()
            .write(|w| unsafe { w.decim_rate().bits(0x7 & decim) });

        trng.trng_imask().write(|w| {
            w.irq_health_fail().enabled();
            w.irq_captured_rdy().enabled()
        });

        trng.trng_ctl().write(|w| w.cmd().norm_func());
        while !trng.trng_ris().read().irq_cmd_done().is_set() {}

        while !trng.trng_mis().read().irq_captured_rdy().is_set() {}
        trng.trng_iclr().write(|w| w.irq_captured_rdy().clr());

        // Discard first byte - deterministic
        let _discard: u32 = trng.trng_data_capture().read().bits();

        Ok(Self { _trng: trng })
    }

    pub fn trng_gen_u32(&self) -> u32 {
        while self._trng.trng_mis().read().irq_captured_rdy().is_clr() {}
        self._trng.trng_iclr().write(|w| w.irq_captured_rdy().clr());
        self._trng.trng_data_capture().read().bits()
    }

    pub fn create_rng(&self) -> Rng {
        let mut init: [u8; 44] = [0x0; 44];

        for n in 0..11 {
            let x = self.trng_gen_u32();
            init[0 + 4 * n] = ((x >> 0x00) & 0xFF) as u8;
            init[1 + 4 * n] = ((x >> 0x08) & 0xFF) as u8;
            init[2 + 4 * n] = ((x >> 0x10) & 0xFF) as u8;
            init[3 + 4 * n] = ((x >> 0x18) & 0xFF) as u8;
        }
        let k32: [u8; 32] = init[0..32].try_into().unwrap();
        let n12: [u8; 12] = init[32..44].try_into().unwrap();
        Rng {
            cipher: ChaCha20::new(&k32.into(), &n12.into()),
        }
    }

    fn reset(trng: &crate::pac::Trng) {
        trng.trng_gprcm(0).trng_rstctl().write(|w| {
            w.resetassert().assert();
            w.resetstkyclr().clr();
            w.key_unlock().unlock()
        });
    }

    fn enable_power(trng: &crate::pac::Trng) {
        trng.trng_gprcm(0).trng_pwren().write(|w| {
            w.enable().enable();
            w.key_unlock().unlock()
        });
    }
}

pub struct Rng {
    cipher: ChaCha20,
}

impl Rng {
    pub fn gen_u32(&mut self) -> u32 {
        let mut buffer: [u8; 4] = [0x0; 4];
        self.cipher.apply_keystream(&mut buffer);
        buffer[0] as u32 >> 0x00
            | buffer[1] as u32 >> 0x08
            | buffer[2] as u32 >> 0x10
            | buffer[3] as u32 >> 0x18
    }
}
