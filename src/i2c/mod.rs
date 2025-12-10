use crate::pac;
use pac::Iomux;
mod clock_config;
mod controller;
mod gpio_utils;
mod target;
mod txrx;

pub struct I2C0 {
    _i2c: pac::I2c0,
}

pub use controller::Controller;
pub use target::Target;

impl I2C0 {
    fn init_peripheral_input_function_features(
        pincm_index: usize,
        function: u32,
        inversion: gpio_utils::GpioInversion,
        internal_resistor: gpio_utils::GpioResistor,
        hysteresis: gpio_utils::GpioHysteresis,
        wakeup: gpio_utils::GpioWakeup,
        iomux: &Iomux,
    ) {
        const IOMUX_PINCM_PC_CONNECTED: u32 = 0x00000080;
        const IOMUX_PINCM_INENA_ENABLE: u32 = 0x00040000;
        const IOMUX_PINCM_WCOMP_MASK: u32 = 0x10000000;
        const IOMUX_PINCM_WUEN_MASK: u32 = 0x08000000;
        iomux.iomux_pincm(pincm_index).write(|w| unsafe {
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
