use crate::aes::AesAdv;
use crate::aes::ModeCBC;
use crate::aes::UsesDma;
use crate::aes::AesMode;
use core::marker::PhantomData;

impl<MODE: AesMode, DMA: UsesDma> AesAdv<MODE, DMA> {
    pub fn to_cbc(self, iv: [u32; 4]) -> AesAdv<ModeCBC, DMA> {
        self._aes.aesadv_ctrl().write(|w| {
            w.save_cntxt().no_effect();
            w.cbc().enable();
            w.keysize().k128();
            w.dir().encrypt()
        });

        self._aes.aesadv_iv0().write(|w| unsafe { w.bits(iv[0]) });
        self._aes.aesadv_iv1().write(|w| unsafe { w.bits(iv[1]) });
        self._aes.aesadv_iv2().write(|w| unsafe { w.bits(iv[2]) });
        self._aes.aesadv_iv3().write(|w| unsafe { w.bits(iv[3]) });
        AesAdv::<ModeCBC, DMA> {
            _aes: self._aes,
            _chan0: self._chan0,
            _chan1: self._chan1,
            _mode: PhantomData,
            _dma_mode: PhantomData,
        }
    }
}
