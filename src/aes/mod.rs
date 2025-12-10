use crate::pac;
use paste::paste;

pub struct AesAdv {
    _aes: pac::Aesadv,
}

impl AesAdv {
    pub fn new(aes: pac::Aesadv) -> Self {
        Self::reset(&aes);
        Self::pwren(&aes);

        Self { _aes: aes }
    }

    pub fn test(&self) -> [u32; 4] {
        while self._aes.aesadv_ctrl().read().cntxt_rdy().is_notready() {}
        while self._aes.aesadv_ctrl().read().input_rdy().is_empty() {}

        // VERY NOT SECURE PLS USE KEYSTORE MEM LEAK WILL RUIN YOU
        self._aes
            .aesadv_key0()
            .write(|w| unsafe { w.bits(0x00000000) });
        self._aes
            .aesadv_key1()
            .write(|w| unsafe { w.bits(0x00000000) });
        self._aes
            .aesadv_key2()
            .write(|w| unsafe { w.bits(0x00000000) });
        self._aes
            .aesadv_key3()
            .write(|w| unsafe { w.bits(0x00000000) });

        self._aes.aesadv_ctrl().write(|w| {
            w.save_cntxt().no_effect();
            w.keysize().k128();
            w.dir().encrypt()
        });

        self._aes
            .aesadv_data0()
            .write(|w| unsafe { w.bits(0x11111111) });
        self._aes
            .aesadv_data1()
            .write(|w| unsafe { w.bits(0x00000000) });
        self._aes
            .aesadv_data2()
            .write(|w| unsafe { w.bits(0x00000000) });
        self._aes
            .aesadv_data3()
            .write(|w| unsafe { w.bits(0x00000000) });

        while self._aes.aesadv_ctrl().read().output_rdy().is_notready() {}

        [
            self._aes.aesadv_data0().read().bits(),
            self._aes.aesadv_data1().read().bits(),
            self._aes.aesadv_data2().read().bits(),
            self._aes.aesadv_data3().read().bits(),
        ]
    }
    pub fn test2(&self) -> [u32; 4] {
        while self._aes.aesadv_ctrl().read().cntxt_rdy().is_notready() {}

        // Key should be stored in register -_- imagine...


        self._aes.aesadv_ctrl().write(|w| {
            w.save_cntxt().no_effect();
            w.keysize().k128();
            w.dir().encrypt()
        });

        self._aes
            .aesadv_data0()
            .write(|w| unsafe { w.bits(0x2) });
        self._aes
            .aesadv_data1()
            .write(|w| unsafe { w.bits(0x00000000) });
        self._aes
            .aesadv_data2()
            .write(|w| unsafe { w.bits(0x00000000) });
        self._aes
            .aesadv_data3()
            .write(|w| unsafe { w.bits(0x00000000) });

        while self._aes.aesadv_ctrl().read().output_rdy().is_notready() {}

        [
            self._aes.aesadv_data0().read().bits(),
            self._aes.aesadv_data1().read().bits(),
            self._aes.aesadv_data2().read().bits(),
            self._aes.aesadv_data3().read().bits(),
        ]
    }

    fn pwren(aes: &pac::Aesadv) {
        aes.aesadv_gprcm(0).aesadv_pwren().write(|w| {
            w.enable().enable();
            w.key_unlock().unlock()
        });
    }

    fn reset(aes: &pac::Aesadv) {
        aes.aesadv_gprcm(0).aesadv_rstctl().write(|w| {
            w.resetassert().assert();
            w.resetstkyclr().clr();
            w.key_unlock().unlock()
        });
    }
}
