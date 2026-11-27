use crate::pac;

pub struct SPI1 {
    _spi: pac::Spi1,
}

impl SPI1 {
    fn init(&mut self, iomux: &pac::Iomux) {
        // IOMUX
        iomux.iomux_pincm(48).write(|w| {
            unsafe { w.pf().bits(3) };
            w.pc().connected()
        }); // CLOCK PA17

        iomux.iomux_pincm(28).write(|w| {
            unsafe { w.pf().bits(3) };
            w.pc().connected()
        }); // PICO PB8
        iomux.iomux_pincm(27).write(|w| {
            unsafe { w.pf().bits(3) };
            w.pc().connected()
        }); // POCI PB7
        iomux.iomux_pincm(26).write(|w| {
            unsafe { w.pf().bits(3) };
            w.pc().connected()
        }); // CS0 PB6

        self._spi.spi1_gprcm(0).spi1_rstctl().write(|w| {
            w.resetassert().assert();
            w.resetstkyclr().clr();
            w.key_unlock().unlock()
        });

        self._spi.spi1_gprcm(0).spi1_pwren().write(|w| {
            w.enable().enable();
            w.key_unlock().unlock()
        });

        // Configuration
        self._spi.spi1_ctl1().write(|w| w.enable().disable());
        self._spi.spi1_clksel().write(|w| w.sysclk_sel().enable());
        self._spi.spi1_clkdiv().write(|w| w.ratio().div_by_1());

        self._spi.spi1_ctl0().write(|w| {
            w.frf().motorola_4wire();
            w.dss().dss_8();
            w.cssel().cssel_1()
        });

        self._spi.spi1_ctl1().write(|w| {
            w.pes().disable();
            w.pren().disable();
            w.pten().disable();
            w.msb().enable();
            w.cp().enable()
        });
        self._spi.spi1_ctl1().modify(|_, w| w.enable().enable());

        self._spi
            .spi1_clkctl()
            .modify(|_, w| unsafe { w.scr().bits(31) });

        self._spi.spi1_ctl1().modify(|_, w| w.cdmode().command());

        self._spi.spi1_ifls().write(|w| {
            w.rxiflsel().lvl_1_2();
            w.txiflsel().lvl_1_2()
        });
    }

    pub fn new(_spi: pac::Spi1, iomux: &pac::Iomux) -> Self {
        let mut result = Self { _spi };

        Self::init(&mut result, iomux);

        result
    }

    pub fn write_byte(&self, data: u8) {
        self._spi.spi1_txdata().write(|w| unsafe {w.data().bits(data as u16)});
    }
}
