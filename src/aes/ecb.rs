use crate::aes::AesAdv;
use crate::aes::ModeECB;
use crate::aes::UsesDma;

impl<DMA: UsesDma> AesAdv<ModeECB, DMA> {}
