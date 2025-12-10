use core::marker::PhantomData;
use paste::paste;

use embedded_hal::digital::{ErrorType, OutputPin};

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
            pub mod [<gpio $GPIO_PORT _mod>] {

                use crate::pac::$GPIO_PAC as $GPIO_PAC;
                use crate::pac::Iomux as Iomux;
                use super::{OutputPin, ErrorType, Input, Output, PinMode};
                use super::PhantomData;


                pub struct Pin<const N: u8, MODE: PinMode = Input> {
                    _mode: PhantomData<MODE>,
                }

                impl<const N: u8, MODE: PinMode> Pin<N, MODE> {
                    pub fn new() -> Self {
                        Self { _mode: PhantomData }
                    }

                    fn enable_output(&self) {
                        let gpio = unsafe { &*$GPIO_PAC::ptr() };

                        let iomux = unsafe { &*Iomux::ptr() };
                        let pincm: usize = match stringify!($GPIO_PORT) {
                            "a" =>  (match N {
                                            0| 1 => N+1,
                                            2| 3| 4| 5| 6 => N+5,
                                            7 => N+7,
                                            8| 9 => N+11,
                                            10| 11 => N+15,
                                            12| 13| 14| 15| 16 => N+26,
                                            17| 18| 19| 20 => N + 32,
                                            21| 22 => N + 35,
                                            23| 24| 25 => N + 44,
                                            26| 27 => N + 47,
                                            28| 29| 30 => N - 25,
                                            _ => panic!("What is this pin??")
                                        }) as usize - 1,
                            "b" => (match N {
                                            0| 1 => N+12,
                                            2| 3| 4| 5 => N+13,
                                            6| 7| 8| 9| 10| 11| 12| 13| 14| 15| 16 => N+21,
                                            17| 18| 19 => N + 36,
                                            20| 21| 22| 23| 24 => N+42,
                                            25| 26| 27 => N+45,
                                            28| 29| 30| 31 => N-7,

                                            _ => panic!("What is this pin??")
                                        }) as usize - 1,
                            "c" => 0,
                            _ => panic!("Unknown GPIO Port")
                            };
                        iomux.iomux_pincm(pincm).write(|w| unsafe { w.bits(0x80 | 0x1) });
                        gpio.[<gpio $GPIO_PORT _doeset31_0>]().write(|w| unsafe { w.bits(1 << N) });
                    }

                    pub fn into_output(&self) -> Pin::<N, Output> {
                        let pin = Pin::<N, Output>::new();
                        pin.enable_output();
                        pin
                    }
                }

                impl<const N: u8> Pin<N, Output> {
                    pub fn set_low(&mut self) {
                        let gpio = unsafe { &*$GPIO_PAC::ptr() };
                        gpio.[<gpio $GPIO_PORT _ doutclr31_0>]().write(|w|
                            unsafe { w.bits(1 << N) });
                    }
                    pub fn set_high(&mut self) {
                        let gpio = unsafe { &*$GPIO_PAC::ptr() };
                        gpio.[<gpio $GPIO_PORT _ doutset31_0>]().write(|w|
                            unsafe { w.bits(1 << N) });
                    }
                }

                impl<const N: u8, MODE: PinMode> ErrorType for Pin<N, MODE> {
                    type Error = core::convert::Infallible;
                }

                impl<const N: u8> OutputPin for Pin<N, Output> {
                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        let gpio = unsafe { &*$GPIO_PAC::ptr() };
                        gpio.[<gpio $GPIO_PORT _ doutclr31_0>]().write(|w|
                            unsafe { w.bits(1 << N) });
                        Ok(())
                    }
                    fn set_high(&mut self)-> Result<(), Self::Error>  {
                        let gpio = unsafe { &*$GPIO_PAC::ptr() };
                        gpio.[<gpio $GPIO_PORT _ doutset31_0>]().write(|w|
                            unsafe { w.bits(1 << N) });
                        Ok(())
                    }
                }

                pub struct Pins {
                    $(pub [<pin $PIN_NUMS>]: Pin<$PIN_NUMS>),+
                }

                pub struct GpioPeripheral {
                    _gpio: $GPIO_PAC
                }

                impl GpioPeripheral {
                    pub fn new(gpio: $GPIO_PAC) -> Self {
                        gpio.[<gpio $GPIO_PORT _ gprcm>](0).[<gpio $GPIO_PORT _ rstctl>]().write(|w| {
                            w.resetassert().assert();
                            w.resetstkyclr().clr();
                            w.key_unlock().unlock()
                        });
                        gpio.[<gpio $GPIO_PORT _ gprcm>](0).[<gpio $GPIO_PORT _ pwren>]().write(|w| {
                            w.enable().enable();
                            w.key_unlock().unlock()
                        });

                        Self {
                            _gpio: gpio
                        }
                    }

                    pub fn pins(&self) -> Pins {

                        Pins {
                            $([<pin $PIN_NUMS>]: Pin::<$PIN_NUMS>::new()),+
                        }

                    }
                }
            }
            pub use [<gpio $GPIO_PORT _ mod>]::GpioPeripheral as [<gpio $GPIO_PORT>];
        }
    }
}

gpio!(
    a,
    0,
    Gpioa,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30
    ]
);
gpio!(
    b,
    1,
    Gpiob,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
    ]
);
gpio!(c, 2, Gpioc, [0, 1, 2, 3]);
