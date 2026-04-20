use crate::aes::{AesAdv, AesFunctionError, ModeCBC};
use crate::dma;
use crate::dma::{NoDma, UsesDma, WithDma};

impl AesAdv<ModeCBC, NoDma> {
    pub fn encrypt(&self, data: &[u32; 4], out_buf: &mut [u32; 4], iv: &[u32; 4]) {
        while self._aes.aesadv_ctrl().read().cntxt_rdy().is_notready() {}
        self.reset_cntxt(iv);
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

    pub fn decrypt(&self, data: &[u32; 4], out_buf: &mut [u32; 4], iv: &[u32; 4]) {
        while self._aes.aesadv_ctrl().read().cntxt_rdy().is_notready() {}
        self.reset_cntxt(iv);
        self._aes.aesadv_ctrl().modify(|_, w| w.dir().decrypt());

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
}

impl<DMA: UsesDma> AesAdv<ModeCBC, DMA> {
    fn reset_cntxt(&self, iv: &[u32; 4]) {
        self._aes.aesadv_ctrl().write(|w| {
            w.save_cntxt().no_effect();
            w.cbc().enable();
            w.keysize().k128()
        });

        self._aes.aesadv_iv0().write(|w| unsafe { w.bits(iv[0]) });
        self._aes.aesadv_iv1().write(|w| unsafe { w.bits(iv[1]) });
        self._aes.aesadv_iv2().write(|w| unsafe { w.bits(iv[2]) });
        self._aes.aesadv_iv3().write(|w| unsafe { w.bits(iv[3]) });
    }
}

impl AesAdv<ModeCBC, WithDma> {
    pub fn encrypt(
        &self,
        dma: &dma::Dma,
        data: &[u32],
        out_buf: &mut [u32],
        iv: &[u32; 4],
    ) -> Result<(), AesFunctionError> {
        let len = data.len();
        let out_len = out_buf.len();

        if len != out_len {
            return Err(AesFunctionError::BufferMismatchError(len, out_len));
        }

        if len % 4 != 0 {
            return Err(AesFunctionError::BlockSizeError(len));
        }

        while self._aes.aesadv_ctrl().read().cntxt_rdy().is_notready() {}
        let data_out_addr: u32 = out_buf.as_ptr().addr() as u32;
        let aes_data_out_addr: u32 = self._aes.aesadv_data_out().as_ptr().addr() as u32;

        let data_in_addr: u32 = data.as_ptr().addr() as u32;
        let aes_data_in_addr: u32 = self._aes.aesadv_data_in().as_ptr().addr() as u32;

        let chan0 = self._chan0.as_ref().unwrap();
        let chan1 = self._chan1.as_ref().unwrap();

        self.dma_preconfig(&dma, &chan0, &chan1);

        dma.dma_set(chan0, data_in_addr, aes_data_in_addr, len as u16);
        dma.enable(chan0);

        dma.dma_set(chan1, aes_data_out_addr, data_out_addr, len as u16);
        dma.enable(chan1);

        self.reset_cntxt(iv);
        self._aes.aesadv_ctrl().modify(|_, w| w.dir().encrypt());
        self._aes
            .aesadv_c_length_0()
            .write(|w| unsafe { w.bits(4 * len as u32) });
        self._aes
            .aesadv_c_length_1()
            .write(|w| unsafe { w.bits(0) });

        self.dma_postconfig(&dma, &chan1);

        Ok(())
    }

    pub fn decrypt(
        &self,
        dma: &dma::Dma,
        data: &[u32],
        out_buf: &mut [u32],
        iv: &[u32; 4],
    ) -> Result<(), AesFunctionError> {
        let len = data.len();
        let out_len = out_buf.len();

        if len != out_len {
            return Err(AesFunctionError::BufferMismatchError(len, out_len));
        }

        if len % 4 != 0 {
            return Err(AesFunctionError::BlockSizeError(len));
        }

        while self._aes.aesadv_ctrl().read().cntxt_rdy().is_notready() {}
        let data_out_addr: u32 = out_buf.as_ptr().addr() as u32;
        let aes_data_out_addr: u32 = self._aes.aesadv_data_out().as_ptr().addr() as u32;

        let data_in_addr: u32 = data.as_ptr().addr() as u32;
        let aes_data_in_addr: u32 = self._aes.aesadv_data_in().as_ptr().addr() as u32;

        let chan0 = self._chan0.as_ref().unwrap();
        let chan1 = self._chan1.as_ref().unwrap();

        self.dma_preconfig(&dma, &chan0, &chan1);

        dma.dma_set(chan0, data_in_addr, aes_data_in_addr, len as u16);
        dma.enable(chan0);

        dma.dma_set(chan1, aes_data_out_addr, data_out_addr, len as u16);
        dma.enable(chan1);

        self.reset_cntxt(iv);
        self._aes.aesadv_ctrl().modify(|_, w| w.dir().decrypt());
        self._aes
            .aesadv_c_length_0()
            .write(|w| unsafe { w.bits(4 * len as u32) });
        self._aes
            .aesadv_c_length_1()
            .write(|w| unsafe { w.bits(0) });

        self.dma_postconfig(&dma, &chan1);
        Ok(())
    }
}
