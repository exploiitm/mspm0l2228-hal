use crate::pac;
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
enum GpioInversion {
    Enable = 0x04000000,
    Disable = 0x00000000,
}
#[repr(u32)]
enum GpioResistor {
    None = 0x0,
    PullUp = 131072,
    PullDown = 65536,
}
#[repr(u32)]
enum GpioHysteresis {
    Enable = 0,
    Disable = 524288,
}
#[repr(u32)]
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
    pub fn new(i2c: pac::I2c0) -> Self {
        let mut result = Self { _i2c: i2c };

        // I2C reset:
        result._i2c.i2c0_gprcm(0).i2c0_rstctl().write(|w| unsafe {
            const I2C_RSTCTL_KEY_UNLOCK_W: u32 = 0xB1000000;
            const I2C_RSTCTL_RESETSTKYCLR_CLR: u32 = 0x00000002;
            const I2C_RSTCTL_RESETASSERT_ASSERT: u32 = 0x00000001;

            w.bits(
                I2C_RSTCTL_KEY_UNLOCK_W
                    | I2C_RSTCTL_RESETSTKYCLR_CLR
                    | I2C_RSTCTL_RESETASSERT_ASSERT,
            )
        });

        // Enable power
        result._i2c.i2c0_gprcm(0).i2c0_pwren().write(|w| unsafe {
            const I2C_PWREN_KEY_UNLOCK_W: u32 = 0x26000000;
            const I2C_PWREN_ENABLE_ENABLE: u32 = 0x00000001;

            w.bits(I2C_PWREN_ENABLE_ENABLE | I2C_PWREN_KEY_UNLOCK_W)
        });

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
        // let scb = pac::SCB ;
        // pac::SCB::set_sleepdeep(&mut scb);

        // ::set_sleepdeep();
        const SYSCTL_PMODECFG_DSLEEP_STOP: u32 = 0x00000000;
        systcl
            .sysctl_pmodecfg()
            .write(|w| unsafe { w.bits(SYSCTL_PMODECFG_DSLEEP_STOP) });

        let clock_config = I2cClockConfig {
            source: I2cClock::BusClk,
            divider: I2cClockDivide::Div1,
        };
        result
            ._i2c
            .i2c0_clksel()
            .write(|w| unsafe { w.bits(clock_config.source as u32) });
        result
            ._i2c
            .i2c0_clkdiv()
            .write(|w| unsafe { w.bits(clock_config.divider as u32) });

        result
    }
}
