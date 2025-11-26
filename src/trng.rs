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
        trng.trng_gprcm(0).trng_rstctl().write(|w| {
            w.resetassert().assert();
            w.resetstkyclr().clr();
            w.key_unlock().unlock()
        });

        trng.trng_gprcm(0).trng_pwren().write(|w| {
            w.enable().enable();
            w.key_unlock().unlock()
        });

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

        trng.trng_ctl().write(|w| w.cmd().pwrup_dig());
        while !trng.trng_ris().read().irq_cmd_done().is_set() {}
        if trng.trng_test_results().read().dig_test().bits() != 0xFF {
            return Err(TrngInitError::DigitalBlockHealthCheck(
                trng.trng_test_results().read().dig_test().bits(),
            ));
        }

        trng.trng_ctl().write(|w| w.cmd().pwrup_ana());
        while !trng.trng_ris().read().irq_cmd_done().is_set() {}
        if trng.trng_test_results().read().ana_test() == false {
            return Err(TrngInitError::AnalogBlockHealthCheck);
        }

        trng.trng_iclr().write(|w| w.irq_captured_rdy().clr());
        trng.trng_ctl()
            .write(|w| unsafe { w.decim_rate().bits(0x7 & decim) });

        trng.trng_imask().write(|w| {
            w.irq_health_fail().disabled();
            w.irq_captured_rdy().disabled()
        });

        while !trng.trng_mis().read().irq_captured_rdy().is_set() {}
        let _discard: u32 = trng.trng_data_capture().read().bits();

        Ok(Self { _trng: trng })
    }

    pub fn gen_u32(&self) -> u32 {
        while !self._trng.trng_mis().read().irq_captured_rdy().is_set() {}
        self._trng.trng_data_capture().read().bits()
    }
}
