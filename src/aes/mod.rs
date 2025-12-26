use crate::dma;
use crate::pac;
use core::marker::PhantomData;
mod ecb;
mod cbc;

pub trait AesMode {}
pub struct UnInit;
pub struct ModeECB;
pub struct ModeCBC;

pub trait UsesDma {}
pub struct NoDma;
pub struct WithDma;

impl AesMode for UnInit {}
impl AesMode for ModeECB {}
impl AesMode for ModeCBC {}

impl UsesDma for NoDma {}
impl UsesDma for WithDma {}

pub struct AesAdv<MODE: AesMode = UnInit, DMA: UsesDma = NoDma> {
    // Only handles 128-bit keys cry about it
    // I mean it prolly isn't too hard to add bigger
    _aes: pac::Aesadv,
    _chan0: Option<dma::Channel<0>>,
    _chan1: Option<dma::Channel<1>>,
    _mode: PhantomData<MODE>,
    _dma_mode: PhantomData<DMA>,
}

impl AesAdv<UnInit, NoDma> {
    pub fn new(aes: pac::Aesadv, key: [u32; 4]) -> AesAdv<ModeECB> {
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

        aes.aesadv_ctrl().write(|w| {
            w.save_cntxt().no_effect();
            w.keysize().k128();
            w.dir().encrypt()
        });

        AesAdv::<ModeECB> {
            _aes: aes,
            _chan0: None,
            _chan1: None,
            _mode: PhantomData,
            _dma_mode: PhantomData,
        }
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

// pub trait AesFunction {
//     fn encrypt(&self, data: &[u32], out_buf: &mut [u32]) -> Result<(), AesFunctionError>;
//     // fn decrypt(&self, data: [u32], len: &u32);
// }

#[derive(Debug)]
pub enum AesFunctionError {
    DMABlockSizeError(usize), // Without DMA only single block
    BufferMismatchError(usize, usize),
    BlockSizeError(usize),
}

// impl<T: AesMode> AesFunction for AesAdv<T, NoDma> {
impl<MODE: AesMode> AesAdv<MODE, NoDma> {
    pub fn encrypt(&self, data: &[u32; 4], out_buf: &mut [u32; 4]) {
        while self._aes.aesadv_ctrl().read().cntxt_rdy().is_notready() {}
        self._aes.aesadv_ctrl().modify(|_, w| w.dir().encrypt());

        self._aes
            .aesadv_data0()
            .write(|w| unsafe { w.bits(data[0]) });
        self._aes
            .aesadv_data1()
            .write(|w| unsafe { w.bits(data[1]) });
        self._aes
            .aesadv_data2()
            .write(|w| unsafe { w.bits(data[2]) });
        self._aes
            .aesadv_data3()
            .write(|w| unsafe { w.bits(data[3]) });

        while self._aes.aesadv_ctrl().read().output_rdy().is_notready() {}

        out_buf[0] = self._aes.aesadv_data0().read().bits();
        out_buf[1] = self._aes.aesadv_data1().read().bits();
        out_buf[2] = self._aes.aesadv_data2().read().bits();
        out_buf[3] = self._aes.aesadv_data3().read().bits();
    }
    // fn decrypt(&self, data: [u32], len: &u32);

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

impl<T: AesMode> AesAdv<T, WithDma> {
    pub fn encrypt(&self, dma: &dma::Dma, data: &[u32], out_buf: &mut [u32]) {
        while self._aes.aesadv_ctrl().read().cntxt_rdy().is_notready() {}
        self._aes.aesadv_ctrl().modify(|_, w| w.dir().encrypt());

        let data_out_addr: u32 = out_buf.as_ptr().addr() as u32;
        let aes_data_out_addr: u32 =
            self._aes.aesadv_data_out().as_ptr().addr() as u32;

        let data_in_addr: u32 = data.as_ptr().addr() as u32;
        let aes_data_in_addr: u32 =
            self._aes.aesadv_data_in().as_ptr().addr() as u32;

        let chan0 = self._chan0.as_ref().unwrap();
        let chan1 = self._chan1.as_ref().unwrap();
        let len = data.len() as u16;

        dma.disable(chan0);
        dma.disable(chan1);

        dma.aes_init_0(chan0);
        dma.aes_init_1(chan1);

        dma.aes_set(chan0, data_in_addr, aes_data_in_addr, len);
        dma.enable(chan0);

        dma.aes_set(chan1, aes_data_out_addr, data_out_addr, len);
        dma.enable(chan1);

        self._aes
            .aesadv_c_length_0()
            .write(|w| unsafe { w.bits(4 * len as u32) });
        self._aes
            .aesadv_c_length_1()
            .write(|w| unsafe { w.bits(0) });

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
    // fn decrypt(&self, data: [u32], len: &u32);
}
