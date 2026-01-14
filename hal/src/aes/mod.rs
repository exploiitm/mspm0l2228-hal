use crate::dma;
use crate::pac;
use core::marker::PhantomData;
mod cbc;
mod ecb;

pub trait AesMode {}
pub struct KeyLoad;
pub struct UnInit;
pub struct ModeECB;
pub struct ModeCBC;

pub trait UsesDma {}
pub struct NoDma;
pub struct WithDma;

impl AesMode for KeyLoad {}
impl AesMode for UnInit {}
impl AesMode for ModeECB {}
impl AesMode for ModeCBC {}

impl UsesDma for NoDma {}
impl UsesDma for WithDma {}

pub struct AesAdv<MODE: AesMode = KeyLoad, DMA: UsesDma = NoDma> {
    // Only handles 128-bit keys cry about it
    // I mean it prolly isn't too hard to add bigger
    // maybe later
    _aes: pac::Aesadv,
    _chan0: Option<dma::Channel<0>>,
    _chan1: Option<dma::Channel<1>>,
    _mode: PhantomData<MODE>,
    _dma_mode: PhantomData<DMA>,
}

impl<MODE: AesMode, DMA: UsesDma> AesAdv<MODE, DMA> {
    pub fn periph_reset(
        self,
    ) -> (
        AesAdv<KeyLoad, NoDma>,
        Option<dma::Channel<0>>,
        Option<dma::Channel<1>>,
    ) {
        // Wipes KEYSTORE
        Self::reset(&self._aes);
        Self::pwren(&self._aes);
        (
            AesAdv::<KeyLoad, NoDma> {
                _aes: self._aes,
                _chan0: None,
                _chan1: None,
                _mode: PhantomData,
                _dma_mode: PhantomData,
            },
            self._chan0,
            self._chan1,
        )
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

impl AesAdv<KeyLoad, NoDma> {
    pub fn new(aes: pac::Aesadv, key: [u32; 4]) -> AesAdv<UnInit> {
        Self::reset(&aes);
        Self::pwren(&aes);

        while aes.aesadv_ctrl().read().cntxt_rdy().is_notready() {}
        while aes.aesadv_ctrl().read().input_rdy().is_empty() {}

        // TODO: Verify physical memory gets cleaned and
        // look into keystore, binary embedding
        aes.aesadv_key0().write(|w| unsafe { w.bits(key[0]) });
        aes.aesadv_key1().write(|w| unsafe { w.bits(key[1]) });
        aes.aesadv_key2().write(|w| unsafe { w.bits(key[2]) });
        aes.aesadv_key3().write(|w| unsafe { w.bits(key[3]) });

        AesAdv::<UnInit> {
            _aes: aes,
            _chan0: None,
            _chan1: None,
            _mode: PhantomData,
            _dma_mode: PhantomData,
        }
    }
}

impl AesAdv<UnInit, NoDma> {
    pub fn to_ecb(self) -> AesAdv<ModeECB, NoDma> {
        self._aes.aesadv_ctrl().write(|w| {
            w.save_cntxt().no_effect();
            w.keysize().k128();
            w.dir().encrypt()
        });

        AesAdv::<ModeECB> {
            _aes: self._aes,
            _chan0: None,
            _chan1: None,
            _mode: PhantomData,
            _dma_mode: PhantomData,
        }
    }

    pub fn to_cbc(self) -> AesAdv<ModeCBC, NoDma> {
        self._aes.aesadv_ctrl().write(|w| {
            w.save_cntxt().no_effect();
            w.cbc().enable();
            w.keysize().k128();
            w.dir().encrypt()
        });

        AesAdv::<ModeCBC, NoDma> {
            _aes: self._aes,
            _chan0: self._chan0,
            _chan1: self._chan1,
            _mode: PhantomData,
            _dma_mode: PhantomData,
        }
    }
}

impl<MODE: AesMode> AesAdv<MODE, NoDma> {
    pub fn use_dma(
        self,
        chan0: dma::Channel<0>,
        chan1: dma::Channel<1>,
    ) -> AesAdv<MODE, WithDma> {
        AesAdv::<MODE, WithDma> {
            _aes: self._aes,
            _chan0: Some(chan0),
            _chan1: Some(chan1),
            _mode: PhantomData,
            _dma_mode: PhantomData,
        }
    }
}

impl<MODE: AesMode> AesAdv<MODE, WithDma> {
    pub fn dma_preconfig(
        &self,
        dma: &dma::Dma,
        chan0: &dma::Channel<0>,
        chan1: &dma::Channel<1>,
    ) {
        self._aes
            .aesadv_dma_hs()
            .write(|w| w.dma_data_ack().dma_disable());
        self._aes
            .aesadv_int_event1(0)
            .aesadv_int_event1_imask()
            .write(|w| w.trig0().clr());
        self._aes
            .aesadv_int_event2(0)
            .aesadv_int_event2_imask()
            .write(|w| w.trig1().clr());
        dma.disable(chan0);
        dma.disable(chan1);

        dma.aes_init_0(chan0);
        dma.aes_init_1(chan1);
    }

    pub fn dma_postconfig(&self, dma: &dma::Dma, chan0: &dma::Channel<0>) {
        self._aes
            .aesadv_dma_hs()
            .write(|w| w.dma_data_ack().dma_enable());

        self._aes
            .aesadv_int_event1(0)
            .aesadv_int_event1_imask()
            .write(|w| w.trig0().set_());

        self._aes
            .aesadv_int_event2(0)
            .aesadv_int_event2_imask()
            .write(|w| w.trig1().set_());

        dma.aes_wait(chan0);
    }
}

#[derive(Debug)]
pub enum AesFunctionError {
    BufferMismatchError(usize, usize),
    BlockSizeError(usize),
}
