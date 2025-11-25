use core::marker::PhantomData;
use paste::paste;

use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

pub trait PinMode {}

pub struct Input;
impl PinMode for Input {}
pub struct Output;
impl PinMode for Output {}

// impl<const G: u8, const N: u8> InputPin for Pin<G, N, Input> {
//     #[inline(always)]
//     fn is_high(&mut self) -> Result<bool, Self::Error> {
//         Ok(self._is_high())
//     }
//     #[inline(always)]
//     fn is_low(&mut self) -> Result<bool, Self::Error> {
//         Ok(self._is_low())
//     }
// }

macro_rules! gpio {
    ($GPIO_PORT:ident, $GPIO_NUM:literal, $GPIO_PAC: ident, [$($PIN_NUMS:literal),*]) => {

        paste! {
            pub mod [<g p i o $GPIO_PORT _ m o d>] {

                use crate::pac::$GPIO_PAC as $GPIO_PAC;
                use crate::pac::Iomux as Iomux; 
                use super::{InputPin, OutputPin, ErrorType, Input, Output, PinMode};
                use super::PhantomData;


                pub struct Pin<const N: u8, MODE: PinMode = Input> {
                    _mode: PhantomData<MODE>,
                }

                impl<const N: u8, MODE: PinMode> Pin<N, MODE> {
                    pub fn new() -> Self {
                        Self { _mode: PhantomData }
                    }

                    fn enable_output(&self, iomux: Iomux) {
                        let gpio = unsafe { &*$GPIO_PAC::ptr() };
                        iomux.iomux_pincm(41).write(|w| unsafe { w.bits(0x80 | 0x1) });
                        gpio.[<g p i o $GPIO_PORT _ d o e s e t 3 1 _ 0>]().write(|w| unsafe { w.bits(1 << N) });
                    }

                    pub fn into_output(&self, iomux: Iomux) -> Pin::<N, Output> {
                        let mut pin = Pin::<N, Output>::new();
                        pin.enable_output(iomux);
                        pin
                    }
                }

                impl<const N: u8> Pin<N, Output> {
                    pub fn set_low(&mut self) {
                        let gpio = unsafe { &*$GPIO_PAC::ptr() };
                        gpio.[<g p i o $GPIO_PORT _ d o u t c l r 3 1 _ 0>]().write(|w| unsafe { w.bits(1 << N) });
                    }
                    pub fn set_high(&mut self) {
                        let gpio = unsafe { &*$GPIO_PAC::ptr() };
                        gpio.[<g p i o $GPIO_PORT _ d o u t s e t 3 1 _ 0>]().write(|w| unsafe { w.bits(1 << N) });
                    }
                }

                impl<const N: u8, MODE: PinMode> ErrorType for Pin<N, MODE> {
                    type Error = core::convert::Infallible;
                }

                impl<const N: u8> OutputPin for Pin<N, Output> {
                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        let gpio = unsafe { &*$GPIO_PAC::ptr() };
                        gpio.[<g p i o $GPIO_PORT _ d o u t c l r 3 1 _ 0>]().write(|w| unsafe { w.bits(1 << N) });
                        Ok(())
                    }
                    fn set_high(&mut self)-> Result<(), Self::Error>  {
                        let gpio = unsafe { &*$GPIO_PAC::ptr() };
                        gpio.[<g p i o $GPIO_PORT _ d o u t s e t 3 1 _ 0>]().write(|w| unsafe { w.bits(1 << N) });
                        Ok(())
                    }
                }

                pub struct Pins {
                    $(pub [<p i n $PIN_NUMS>]: Pin<$PIN_NUMS>),+
                }

                pub struct GpioPeripheral {
                    _gpio: $GPIO_PAC
                }

                impl GpioPeripheral {
                    pub fn new(gpio: $GPIO_PAC) -> Self {
                        const RSTCTL_KEY_UNLOCK: u32 = 0xB100_0000;
                        const RSTCTL_STICKY_CLEAR: u32 = 0x0000_0002;
                        const RSTCTL_ASSERT: u32 = 0x0000_0001;

                        gpio.[<g p i o $GPIO_PORT _ g p r c m>](0).[<g p i o $GPIO_PORT _ r s t c t l>]().write(|w| unsafe { w.bits(RSTCTL_KEY_UNLOCK | RSTCTL_STICKY_CLEAR | RSTCTL_ASSERT) });

                        const PWREN_KEY_UNLOCK: u32 = 0x2600_0000;
                        const PWREN_ENABLE: u32 = 0x0000_0001;

                        gpio.[<g p i o $GPIO_PORT _ g p r c m>](0).[<g p i o $GPIO_PORT _ p w r e n>]().write(|w| unsafe { w.bits(PWREN_KEY_UNLOCK | PWREN_ENABLE) });

                        Self {
                            _gpio: gpio
                        }
                    }

                    pub fn pins(&self) -> Pins {

                        Pins {
                            $([<p i n $PIN_NUMS>]: Pin::<$PIN_NUMS>::new()),+
                        }

                    }
                }
            }
            pub use [<g p i o $GPIO_PORT _ m o d>]::GpioPeripheral as [<g p i o $GPIO_PORT>];
        }
    }
}

gpio!(
    a,
    0,
    Gpioa,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21
    ]
);
// gpio!(b, 1, Gpiob, [0, 1, 2, 3]);
// gpio!(c, 2, Gpioc, [0, 1, 2, 3]);
