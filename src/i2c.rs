use crate::pac;
use crate::utils::update_reg;
use pac::Iomux;
#[repr(u32)]
pub enum I2cClock {
    /// Selects BUSCLK as the clock source
    BusClk = 0x0000_0008,
    /// Selects MFCLK as the clock source
    MfClk = 0x0000_0004,
    /// Selects LFCLK as the clock source
    LfClk = 0x0000_0002,
}

/// UART clock divide ratio
#[repr(u32)]
pub enum I2cClockDivide {
    /// UART source clock divide ratio set to 1
    Div1 = 0,
    /// UART source clock divide ratio set to 2
    Div2 = 1,
    /// UART source clock divide ratio set to 3
    Div3 = 2,
    /// UART source clock divide ratio set to 4
    Div4 = 3,
    /// UART source clock divide ratio set to 5
    Div5 = 4,
    /// UART source clock divide ratio set to 6
    Div6 = 5,
    /// UART source clock divide ratio set to 7
    Div7 = 6,
    /// UART source clock divide ratio set to 8
    Div8 = 7,
}

pub struct I2cClockConfig {
    pub source: I2cClock,
    pub divider: I2cClockDivide,
}

#[repr(u32)]
#[allow(dead_code)]
enum I2cTxFifoLevel {
    LevelEmpty,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
}

#[repr(u32)]
#[allow(dead_code)]
enum I2cRxFifoLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
}

#[repr(u32)]
#[allow(dead_code)]
enum GpioInversion {
    Enable = 0x04000000,
    Disable = 0x00000000,
}
#[repr(u32)]
#[allow(dead_code)]
enum GpioResistor {
    None = 0x0,
    PullUp = 131072,
    PullDown = 65536,
}

#[repr(u32)]
#[allow(dead_code)]
enum GpioHysteresis {
    Enable = 0,
    Disable = 524288,
}

#[repr(u32)]
#[allow(dead_code)]
enum GpioWakeup {
    Enable = 0x08000000,
    Disable = 0x00000000,
    WakeupOn2 = 134217728 | 268435456,
}

pub struct I2C0 {
    _i2c: pac::I2c0,
}

impl I2C0 {
    fn init_peripheral_input_function_features(
        pincm_index: usize,
        function: u32,
        inversion: GpioInversion,
        internal_resistor: GpioResistor,
        hysteresis: GpioHysteresis,
        wakeup: GpioWakeup,
    ) {
        const IOMUX_PINCM_PC_CONNECTED: u32 = 0x00000080;
        const IOMUX_PINCM_INENA_ENABLE: u32 = 0x00040000;
        const IOMUX_PINCM_WCOMP_MASK: u32 = 0x10000000;
        const IOMUX_PINCM_WUEN_MASK: u32 = 0x08000000;
        let iomux = unsafe { &*Iomux::ptr() };
        iomux.iomux_pincm(pincm_index).modify(|r, w| unsafe {
            w.bits(
                function
                    | IOMUX_PINCM_PC_CONNECTED
                    | IOMUX_PINCM_INENA_ENABLE
                    | inversion as u32
                    | internal_resistor as u32
                    | hysteresis as u32
                    | (wakeup as u32
                        & (IOMUX_PINCM_WCOMP_MASK | IOMUX_PINCM_WUEN_MASK)),
            )
        });
    }
    fn set_timer_period(&mut self, period: u8) {
        self._i2c
            .i2c0_controller(0)
            .i2c0_ctpr()
            .write(|w| unsafe { w.bits(period as u32) });
    }

    fn reset_peripheral(&mut self) {
        self._i2c.i2c0_gprcm(0).i2c0_rstctl().write(|w| unsafe {
            const I2C_RSTCTL_KEY_UNLOCK_W: u32 = 0xB1000000;
            const I2C_RSTCTL_RESETSTKYCLR_CLR: u32 = 0x00000002;
            const I2C_RSTCTL_RESETASSERT_ASSERT: u32 = 0x00000001;

            w.bits(
                I2C_RSTCTL_KEY_UNLOCK_W
                    | I2C_RSTCTL_RESETSTKYCLR_CLR
                    | I2C_RSTCTL_RESETASSERT_ASSERT,
            )
        });
    }

    fn enable_power(&mut self) {
        self._i2c.i2c0_gprcm(0).i2c0_pwren().write(|w| unsafe {
            const I2C_PWREN_KEY_UNLOCK_W: u32 = 0x26000000;
            const I2C_PWREN_ENABLE_ENABLE: u32 = 0x00000001;

            w.bits(I2C_PWREN_ENABLE_ENABLE | I2C_PWREN_KEY_UNLOCK_W)
        });
    }
}

#[repr(u32)]
pub enum I2cControllerDirction {
    Transmit = 0,
    Recieve = 1,
}
pub trait Controller {
    fn new(i2c: pac::I2c0) -> Self;
    fn is_controller_idle(&self) -> bool;
    fn is_controller_busy(&self) -> bool;
    fn is_controller_error(&self) -> bool;
    fn get_controller_status(&self) -> u32;
    fn is_tx_fifo_full(&self) -> bool;
    fn fill_tx_fifo(&mut self, buffer: &str) -> usize;
    fn transmit_byte(&mut self, byte: u8);

    fn start_tranfer(
        &mut self,
        target_addr: u32,
        direction: I2cControllerDirction,
        length: usize,
    );
}

impl Controller for I2C0 {
    fn new(i2c: pac::I2c0) -> Self {
        let mut result = Self { _i2c: i2c };

        result.reset_peripheral();
        result.enable_power();

        // IOMUX init
        const SDA_PINCM: usize = 0;
        const SCL_PINCM: usize = 1;
        let iomux = unsafe { &*Iomux::ptr() };

        Self::init_peripheral_input_function_features(
            SDA_PINCM,
            0x3,
            GpioInversion::Disable,
            GpioResistor::None,
            GpioHysteresis::Disable,
            GpioWakeup::Disable,
        );
        Self::init_peripheral_input_function_features(
            SCL_PINCM,
            0x3,
            GpioInversion::Disable,
            GpioResistor::None,
            GpioHysteresis::Disable,
            GpioWakeup::Disable,
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
        let clock_config = I2cClockConfig {
            source: I2cClock::BusClk,
            divider: I2cClockDivide::Div1,
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
                    I2cTxFifoLevel::LevelEmpty as u32,
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
                    I2cRxFifoLevel::Level1 as u32,
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
    fn is_tx_fifo_full(&self) -> bool {
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

    fn fill_tx_fifo(&mut self, buffer: &str) -> usize {
        for (i, c) in buffer.chars().enumerate() {
            if !self.is_tx_fifo_full() {
                self.transmit_byte(c as u8);
            } else {
                return i;
            }
        }
        buffer.len()
    }

    fn start_tranfer(
        &mut self,
        target_addr: u32,
        direction: I2cControllerDirction,
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
}

pub trait Target {
    fn new(i2c: pac::I2c0, own_address: u8) -> Self;
}

impl Target for I2C0 {
    fn new(i2c: pac::I2c0, own_address: u8) -> Self {
        let mut result = Self { _i2c: i2c };

        result.reset_peripheral();
        result.enable_power();

        // IOMUX init
        const SDA_PINCM: usize = 0;
        const SCL_PINCM: usize = 1;
        let iomux = unsafe { &*Iomux::ptr() };

        Self::init_peripheral_input_function_features(
            SDA_PINCM,
            0x3,
            GpioInversion::Disable,
            GpioResistor::None,
            GpioHysteresis::Disable,
            GpioWakeup::Disable,
        );
        Self::init_peripheral_input_function_features(
            SCL_PINCM,
            0x3,
            GpioInversion::Disable,
            GpioResistor::None,
            GpioHysteresis::Disable,
            GpioWakeup::Disable,
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

        // Disable HFXT
        systcl
            .sysctl_hsclken()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(0x1)) });

        // Clock Config
        let clock_config = I2cClockConfig {
            source: I2cClock::BusClk,
            divider: I2cClockDivide::Div1,
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

        // Configure Target Mode

        const I2C_SOAR_OAR_MASK: u32 = 0x000003FF;
        result
            ._i2c
            .i2c0_target(0)
            .i2c0_toar()
            .modify(|r, w| unsafe {
                update_reg!(r, w, own_address as u32, I2C_SOAR_OAR_MASK)
            });

        // set tx threshold
        result
            ._i2c
            .i2c0_target(0)
            .i2c0_tfifoctl()
            .modify(|r, w| unsafe {
                const I2C_MFIFOCTL_TXTRIG_MASK: u32 = 0x00000007;
                update_reg!(
                    r,
                    w,
                    I2cTxFifoLevel::Level1 as u32,
                    I2C_MFIFOCTL_TXTRIG_MASK
                )
            });
        // set rx threshold
        result
            ._i2c
            .i2c0_target(0)
            .i2c0_tfifoctl()
            .modify(|r, w| unsafe {
                const I2C_MFIFOCTL_RXTRIG_MASK: u32 = 0x00000700;
                update_reg!(
                    r,
                    w,
                    I2cRxFifoLevel::Level1 as u32,
                    I2C_MFIFOCTL_RXTRIG_MASK
                )
            });
        // enable target clock streching
        const I2C_MCR_CLKSTRETCH_ENABLE: u32 = 4;
        result
            ._i2c
            .i2c0_target(0)
            .i2c0_tctr()
            .modify(|r, w| unsafe {
                w.bits(r.bits() | I2C_MCR_CLKSTRETCH_ENABLE)
            });

        // Disable Target Wakeup
        const I2C_SCTR_SWUEN_MASK: u32 = 0x00000400;
        result
            ._i2c
            .i2c0_target(0)
            .i2c0_tctr()
            .modify(|r, w| unsafe {
                w.bits(r.bits() & !(I2C_SCTR_SWUEN_MASK))
            });

        // Set interrupts

        const DL_I2C_INTERRUPT_TARGET_ARBITRATION_LOST: u32 = 0x40000000;
        const DL_I2C_TARGET_INTERRUPT_OVERFLOW: u32 = 0x80000000;
        const DL_I2C_INTERRUPT_TARGET_RXFIFO_OVERFLOW: u32 = 0x20000000;
        const DL_I2C_INTERRUPT_TARGET_RXFIFO_TRIGGER: u32 = 0x00040000;
        const DL_I2C_INTERRUPT_TARGET_START: u32 = 0x00400000;
        const DL_I2C_INTERRUPT_TARGET_STOP: u32 = 0x00800000;
        const DL_I2C_INTERRUPT_TARGET_TXFIFO_UNDERFLOW: u32 = 0x10000000;

        const INTERRUPT_MASK: u32 = DL_I2C_INTERRUPT_TARGET_ARBITRATION_LOST
            | DL_I2C_TARGET_INTERRUPT_OVERFLOW
            | DL_I2C_INTERRUPT_TARGET_RXFIFO_OVERFLOW
            | DL_I2C_INTERRUPT_TARGET_RXFIFO_TRIGGER
            | DL_I2C_INTERRUPT_TARGET_START
            | DL_I2C_INTERRUPT_TARGET_STOP
            | DL_I2C_INTERRUPT_TARGET_TXFIFO_UNDERFLOW;
        result
            ._i2c
            .i2c0_cpu_int(0)
            .i2c0_cpu_int_imask()
            .modify(|r, w| unsafe { w.bits(r.bits() | INTERRUPT_MASK) });

        // Set frequency 400,000 Hz
        result.set_timer_period(7);

        // enable target
        const I2C_SCTR_ACTIVE_ENABLE: u32 = 0x00000001;
        result
            ._i2c
            .i2c0_target(0)
            .i2c0_tctr()
            .modify(|r, w| unsafe {
                w.bits(r.bits() | I2C_SCTR_ACTIVE_ENABLE)
            });

        result
    }
}
