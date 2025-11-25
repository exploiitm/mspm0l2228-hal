use paste::paste;

struct Pin<const G: u8, const N: u8>;

macro_rules! gpio {
    ($GPIO_PORT:ident, $GPIO_NUM:literal, $GPIO_PAC: ident, [$($PIN_NUMS:literal),*]) => {

        paste! {
            pub mod [<g p i o $GPIO_PORT _ m o d>] {
                
                use crate::pac::$GPIO_PAC as $GPIO_PAC;
                use super::Pin;

                pub struct Pins {
                    $([<p i n $PIN_NUMS>]: Pin<$GPIO_NUM, $PIN_NUMS>),+
                    
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
                }
            }
            pub use [<g p i o $GPIO_PORT _ m o d>]::GpioPeripheral as [<g p i o $GPIO_PORT>];
        }
    }
}



gpio!(a, 0, Gpioa, [0, 1, 2, 3]);
// gpio!(b, 1, Gpiob, [0, 1, 2, 3]);
// gpio!(c, 2, Gpioc, [0, 1, 2, 3]);

