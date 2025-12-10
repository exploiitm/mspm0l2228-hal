use crate::i2c::I2C0;
use crate::i2c::clock_config;
use crate::i2c::gpio_utils;
use crate::i2c::txrx;
use crate::pac;
use crate::utils::update_reg;
use pac::Iomux;

pub trait Target {
    fn new(i2c: pac::I2c0, iomux: &Iomux, own_address: u8) -> Self;

    fn is_rx_fifo_empty(&self) -> bool;
    fn recieve_data(&self) -> u8;
}

impl Target for I2C0 {
    fn is_rx_fifo_empty(&self) -> bool {
        const I2C_SFIFOSR_RXFIFOCNT_MASK: u32 = 0x0000000F;
        const I2C_SFIFOSR_RXFIFOCNT_MINIMUM: u32 = 0;
        self._i2c.i2c0_target(0).i2c0_tfifosr().read().bits()
            & I2C_SFIFOSR_RXFIFOCNT_MASK
            == I2C_SFIFOSR_RXFIFOCNT_MINIMUM
    }

    fn recieve_data(&self) -> u8 {
        (self._i2c.i2c0_target(0).i2c0_trxdata().read().bits() & 0xFF) as u8
    }

    fn new(i2c: pac::I2c0, iomux: &Iomux, own_address: u8) -> Self {
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

        // Disable HFXT
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
                    txrx::I2cTxFifoLevel::Level1 as u32,
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
                    txrx::I2cRxFifoLevel::Level1 as u32,
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
