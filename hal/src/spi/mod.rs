use crate::pac;

pub struct SPI0 {
    _spi: pac::Spi0,
}

impl SPI0 {
    fn init(&mut self, iomux: &pac::Iomux) {
        // IOMUX
        iomux.iomux_pincm(15).write(|w| {
            unsafe { w.pf().bits(11) };
            w.pc().connected()
        }); // CLOCK

        iomux.iomux_pincm(14).write(|w| {
            unsafe { w.pf().bits(11) };
            w.pc().connected()
        }); // PICO
        iomux.iomux_pincm(27).write(|w| {
            unsafe { w.pf().bits(3) };
            w.pc().connected()
        }); // POCI
        iomux.iomux_pincm(26).write(|w| {
            unsafe { w.pf().bits(9) };
            w.pc().connected()
        }); // CS1

        self._spi.spi0_gprcm(0).spi0_rstctl().write(|w| {
            w.resetassert().assert();
            w.resetstkyclr().clr();
            w.key_unlock().unlock()
        });

        self._spi.spi0_gprcm(0).spi0_pwren().write(|w| {
            w.enable().enable();
            w.key_unlock().unlock()
        });

        // Configuration
        self._spi.spi0_ctl1().write(|w| w.enable().disable());
        self._spi.spi0_clksel().write(|w| w.sysclk_sel().enable());
        self._spi.spi0_clkdiv().write(|w| w.ratio().div_by_1());

        self._spi.spi0_ctl0().write(|w| {
            w.frf().motorola_4wire();
            w.dss().dss_8();
            w.cssel().cssel_1()
        });

        self._spi.spi0_ctl1().write(|w| {
            w.pes().disable();
            w.pren().disable();
            w.pten().disable();
            w.msb().enable();
            w.cp().enable()
        });
        self._spi.spi0_ctl1().modify(|_, w| w.enable().enable());

        self._spi
            .spi0_clkctl()
            .modify(|_, w| unsafe { w.scr().bits(31) });

        self._spi.spi0_ctl1().modify(|_, w| w.cdmode().command());

        self._spi.spi0_ifls().write(|w| {
            w.rxiflsel().lvl_1_2();
            w.txiflsel().lvl_1_2()
        });
    }

    pub fn new(_spi: pac::Spi0, iomux: &pac::Iomux) -> Self {
        let mut result = Self { _spi };

        Self::init(&mut result, iomux);

        result
    }

    pub fn write_byte(&self, data: u8) {
        self._spi.spi0_txdata().write(|w| unsafe {w.data().bits(data as u16)});
    }
}
