use crate::pac;
use pac::Iomux;
mod clock_config;
mod controller;
mod gpio_utils;
mod target;
mod txrx;
pub use gpio_utils::I2cControllerDirction;

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
        iomux.iomux_pincm(pincm_index).write(|w| {
            match inversion {
                gpio_utils::GpioInversion::Enable => w.inv().enable(),
                gpio_utils::GpioInversion::Disable => w.inv().disable(),
            };
            match internal_resistor {
                gpio_utils::GpioResistor::PullDown => {
                    w.pipd().enable();
                }
                gpio_utils::GpioResistor::PullUp => {
                    w.pipu().enable();
                }

                _ => {}
            };
            match hysteresis {
                gpio_utils::GpioHysteresis::Enable => w.hysten().enable(),
                gpio_utils::GpioHysteresis::Disable => w.hysten().disable(),
            };
            match wakeup {
                gpio_utils::GpioWakeup::Enable => w.wuen().enable(),
                gpio_utils::GpioWakeup::Disable => w.wuen().disable(),
                gpio_utils::GpioWakeup::WakeupOn2 => {
                    w.wcomp().set_bit();
                    w.wuen().enable()
                }
            };
            w.pc().connected();
            w.inena().enable();
            unsafe { w.pf().bits(function as u8) }
        });
    }
    fn set_timer_period(&mut self, period: u8) {
        self._i2c
            .i2c0_controller(0)
            .i2c0_ctpr()
            .write(|w| unsafe { w.bits(period as u32) });
    }

    fn reset_peripheral(&mut self) {
        self._i2c.i2c0_gprcm(0).i2c0_rstctl().write(|w| {
            w.key_unlock().unlock();
            w.resetassert().assert();
            w.resetstkyclr().clr()
        });
    }

    fn enable_power(&mut self) {
        self._i2c.i2c0_gprcm(0).i2c0_pwren().write(|w| {
            w.key_unlock().unlock();
            w.enable().enable()
        });
    }
}
