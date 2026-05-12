use crate::pac;

pub struct SPI0 {
    _spi: pac::Spi0,
}

impl SPI0 {
    fn init(&mut self, iomux: &pac::Iomux) {
        // IOMUX
        iomux.iomux_pincm(48).write(|w| unsafe { w.pf().bits(3) }); // CLOCK
        iomux.iomux_pincm(28).write(|w| unsafe { w.pf().bits(3) }); // PICO
        iomux.iomux_pincm(27).write(|w| unsafe { w.pf().bits(3) }); // POCI
        iomux.iomux_pincm(73).write(|w| unsafe { w.pf().bits(3) }); // CS1

        // Configuration
        self._spi.spi0_ctl1().write(|w| w.enable().clear_bit());
        self._spi.spi0_clksel().write(|w| w.sysclk_sel().enable());
        self._spi.spi0_clkdiv().write(|w| w.ratio().div_by_1());
        self._spi
            .spi0_gprcm(0)
            .spi0_pwren()
            .write(|w| w.enable().set_bit());
        self._spi.spi0_clkctl().modify(|r, w| unsafe {
            w.scr().bits((r.bits() as u16 & !0x3ff) | (31 & 0x3ff))
        });
        self._spi
            .spi0_ifls()
            .write(|w| w.rxiflsel().lvl_1_2().txiflsel().lvl_1_2());
        self._spi.spi0_ctl0().write(|w| {
            w.frf().motorola_4wire().dss().dss_8().cssel().cssel_1()
        });
        self._spi.spi0_ctl1().write(|w| {
            w.pes()
                .disable()
                .pren()
                .disable()
                .pten()
                .disable()
                .msb()
                .enable()
                .cp()
                .enable()
                .enable()
                .enable()
        });
    }

    pub fn new(_spi: pac::Spi0, iomux: &pac::Iomux) -> Self {
        let mut result = Self { _spi };

        Self::init(&mut result, iomux);

        result
    }
}
