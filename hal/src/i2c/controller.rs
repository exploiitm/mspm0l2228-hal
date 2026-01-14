use crate::i2c::I2C0;
use crate::i2c::clock_config;
use crate::i2c::gpio_utils;
use crate::i2c::txrx;
use crate::pac;
use crate::utils::update_reg;
use pac::Iomux;

pub trait Controller {
    fn new(i2c: pac::I2c0, iomux: &Iomux) -> Self;
    fn is_controller_idle(&self) -> bool;
    fn is_controller_busy(&self) -> bool;
    fn is_controller_error(&self) -> bool;
    fn get_controller_status(&self) -> u32;

    fn is_txfifo_full(&self) -> bool;
    fn fill_txfifo(&mut self, buffer: &str);
    fn transmit_byte(&mut self, byte: u8);

    fn is_rxfifo_empty(&self) -> bool;
    fn recieve_byte(&self) -> u8;

    fn start_tranfer(
        &mut self,
        target_addr: u32,
        direction: gpio_utils::I2cControllerDirction,
        length: usize,
    );
}

impl Controller for I2C0 {
    fn new(i2c: pac::I2c0, iomux: &Iomux) -> Self {
        let mut result = Self { _i2c: i2c };

        result.reset_peripheral();
        result.enable_power();

        // IOMUX init
        const SDA_PINCM: usize = 0;
        const SCL_PINCM: usize = 1;

        Self::init_peripheral_input_function_features(
            SDA_PINCM,
            0x3,
            gpio_utils::GpioInversion::Disable,
            gpio_utils::GpioResistor::None,
            gpio_utils::GpioHysteresis::Disable,
            gpio_utils::GpioWakeup::Disable,
            iomux,
        );
        Self::init_peripheral_input_function_features(
            SCL_PINCM,
            0x3,
            gpio_utils::GpioInversion::Disable,
            gpio_utils::GpioResistor::None,
            gpio_utils::GpioHysteresis::Disable,
            gpio_utils::GpioWakeup::Disable,
            iomux,
        );

        const HIZ_ENABLE: u32 = 0x02000000;
        iomux
            .iomux_pincm(SDA_PINCM)
            .modify(|r, w| unsafe { w.bits(r.bits() | HIZ_ENABLE) }); // SDA 
        iomux
            .iomux_pincm(SCL_PINCM)
            .modify(|r, w| unsafe { w.bits(r.bits() | HIZ_ENABLE) }); // SCL 

        let systcl = unsafe { &*pac::Sysctl::ptr() };

        systcl.sysctl_borthreshold().write(|w| unsafe { w.bits(0) });

        const SYSCTL_SYSOSCCFG_FREQ_MASK: u32 = 3;
        const DL_SYSCTL_SYSOSC_FREQ_BASE: u32 = 0;

        systcl.sysctl_sysosccfg().modify(|r, w| unsafe {
            update_reg!(
                r,
                w,
                DL_SYSCTL_SYSOSC_FREQ_BASE,
                SYSCTL_SYSOSCCFG_FREQ_MASK
            )
        });

        systcl
            .sysctl_hsclken()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(0x1)) });

        // Clock Config
        let clock_config = clock_config::I2cClockConfig {
            source: clock_config::I2cClock::BusClk,
            divider: clock_config::I2cClockDivide::Div1,
        };

        const I2C_CLKSEL_BUSCLK_SEL_MASK: u32 = 8;
        const I2C_CLKSEL_MFCLK_SEL_MASK: u32 = 4;
        result._i2c.i2c0_clksel().modify(|r, w| unsafe {
            update_reg!(
                r,
                w,
                clock_config.source as u32,
                (I2C_CLKSEL_BUSCLK_SEL_MASK | I2C_CLKSEL_MFCLK_SEL_MASK)
            )
        });
        const I2C_CLKDIV_RATIO_MASK: u32 = 7;
        result._i2c.i2c0_clkdiv().modify(|r, w| unsafe {
            update_reg!(
                r,
                w,
                clock_config.divider as u32,
                I2C_CLKDIV_RATIO_MASK
            )
        });

        // analog glitch filter
        // TODO

        // Configure Controller Mode
        //
        // reset controller transfer
        result
            ._i2c
            .i2c0_controller(0)
            .i2c0_cctr()
            .write(|w| unsafe { w.bits(0x0) });

        // Set frequency 400,000 Hz
        result.set_timer_period(7);
        // set tx threshold
        result
            ._i2c
            .i2c0_controller(0)
            .i2c0_cfifoctl()
            .modify(|r, w| unsafe {
                const I2C_MFIFOCTL_TXTRIG_MASK: u32 = 0x00000007;
                update_reg!(
                    r,
                    w,
                    txrx::I2cTxFifoLevel::LevelEmpty as u32,
                    I2C_MFIFOCTL_TXTRIG_MASK
                )
            });
        // set rx threshold
        result
            ._i2c
            .i2c0_controller(0)
            .i2c0_cfifoctl()
            .modify(|r, w| unsafe {
                const I2C_MFIFOCTL_RXTRIG_MASK: u32 = 0x00000700;
                update_reg!(
                    r,
                    w,
                    txrx::I2cRxFifoLevel::Level1 as u32,
                    I2C_MFIFOCTL_RXTRIG_MASK
                )
            });

        // enable controller clock streching
        const I2C_MCR_CLKSTRETCH_ENABLE: u32 = 4;
        result
            ._i2c
            .i2c0_controller(0)
            .i2c0_ccr()
            .modify(|r, w| unsafe {
                w.bits(r.bits() | I2C_MCR_CLKSTRETCH_ENABLE)
            });

        // enable controller
        const I2C_MCR_ACTIVE_ENABLE: u32 = 4;
        result
            ._i2c
            .i2c0_controller(0)
            .i2c0_ccr()
            .modify(|r, w| unsafe { w.bits(r.bits() | I2C_MCR_ACTIVE_ENABLE) });

        let scb = unsafe { &*pac::SCB::PTR };
        let scr = scb.scr.read();
        unsafe { scb.scr.write(scr | 0x4) };

        const SYSCTL_PMODECFG_DSLEEP_STOP: u32 = 0x00000000;
        systcl
            .sysctl_pmodecfg()
            .write(|w| unsafe { w.bits(SYSCTL_PMODECFG_DSLEEP_STOP) });
        const SYSCTL_SYSOSCCFG_USE4MHZSTOP_MASK: u32 = 0x00000100;
        const SYSCTL_SYSOSCCFG_DISABLESTOP_MASK: u32 = 0x00000200;
        systcl.sysctl_sysosccfg().modify(|r, w| unsafe {
            w.bits(
                r.bits()
                    & !(SYSCTL_SYSOSCCFG_USE4MHZSTOP_MASK
                        | SYSCTL_SYSOSCCFG_DISABLESTOP_MASK),
            )
        });

        // Enable I2C
        result
            ._i2c
            .i2c0_controller(0)
            .i2c0_ccr()
            .modify(|r, w| unsafe {
                const I2C_MCR_ACTIVE_ENABLE: u32 = 0x1;
                w.bits(r.bits() | I2C_MCR_ACTIVE_ENABLE)
            });

        result
    }

    #[inline(always)]
    fn get_controller_status(&self) -> u32 {
        self._i2c.i2c0_controller(0).i2c0_csr().read().bits()
    }

    #[inline(always)]
    fn is_controller_idle(&self) -> bool {
        const IDLE_MASK: u32 = 0x00000020;
        (self.get_controller_status() & IDLE_MASK) == 0
    }

    #[inline(always)]
    fn is_controller_busy(&self) -> bool {
        const BUSY_MASK: u32 = 0x00000001;
        (self.get_controller_status() & BUSY_MASK) != 0
    }

    #[inline(always)]
    fn is_controller_error(&self) -> bool {
        const ERROR_MASK: u32 = 0x00000002;
        (self.get_controller_status() & ERROR_MASK) != 0
    }

    #[inline(always)]
    fn is_txfifo_full(&self) -> bool {
        const I2C_MFIFOSR_TXFIFOCNT_MASK: u32 = 0x00000F00;
        (self._i2c.i2c0_controller(0).i2c0_cfifosr().read().bits()
            & I2C_MFIFOSR_TXFIFOCNT_MASK)
            == 0
    }

    fn transmit_byte(&mut self, byte: u8) {
        self._i2c
            .i2c0_controller(0)
            .i2c0_ctxdata()
            .write(|w| unsafe { w.bits(byte as u32) });
    }

    fn fill_txfifo(&mut self, buffer: &str) {
        for c in buffer.bytes() {
            while self.is_txfifo_full() {}
            self.transmit_byte(c);
        }
    }

    fn start_tranfer(
        &mut self,
        target_addr: u32,
        direction: gpio_utils::I2cControllerDirction,
        length: usize,
    ) {
        const I2C_MSA_SADDR_OFS: u32 = 1;
        const I2C_MSA_SADDR_MASK: u32 = 0x000007FE;
        const I2C_MSA_DIR_MASK: u32 = 0x00000001;
        self._i2c
            .i2c0_controller(0)
            .i2c0_csa()
            .modify(|r, w| unsafe {
                update_reg!(
                    r,
                    w,
                    (target_addr << I2C_MSA_SADDR_OFS) | direction as u32,
                    (I2C_MSA_SADDR_MASK | I2C_MSA_DIR_MASK)
                )
            });

        const I2C_MCTR_MBLEN_OFS: u32 = 16;
        const I2C_MCTR_BURSTRUN_ENABLE: u32 = 1;
        const I2C_MCTR_START_ENABLE: u32 = 2;
        const I2C_MCTR_STOP_ENABLE: u32 = 4;
        const I2C_MCTR_MBLEN_MASK: u32 = 0x0FFF0000;
        const I2C_MCTR_BURSTRUN_MASK: u32 = 0x00000001;
        const I2C_MCTR_START_MASK: u32 = 0x00000002;
        const I2C_MCTR_STOP_MASK: u32 = 0x00000004;

        self._i2c
            .i2c0_controller(0)
            .i2c0_cctr()
            .modify(|r, w| unsafe {
                update_reg!(
                    r,
                    w,
                    ((length as u32) << I2C_MCTR_MBLEN_OFS)
                        | I2C_MCTR_BURSTRUN_ENABLE
                        | I2C_MCTR_START_ENABLE
                        | I2C_MCTR_STOP_ENABLE,
                    (I2C_MCTR_MBLEN_MASK
                        | I2C_MCTR_BURSTRUN_MASK
                        | I2C_MCTR_START_MASK
                        | I2C_MCTR_STOP_MASK)
                )
            });
    }

    #[inline(always)]
    fn is_rxfifo_empty(&self) -> bool {
        const I2C_MFIFOSR_RXFIFOCNT_MASK: u32 = 0xF;
        const I2C_MFIFOSR_RXFIFOCNT_MINIMUM: u32 = 0x0;
        self._i2c.i2c0_controller(0).i2c0_cfifosr().read().bits()
            & I2C_MFIFOSR_RXFIFOCNT_MASK
            == I2C_MFIFOSR_RXFIFOCNT_MINIMUM
    }

    fn recieve_byte(&self) -> u8 {
        (self._i2c.i2c0_controller(0).i2c0_crxdata().read().bits() & 0xFF) as u8
    }
}
