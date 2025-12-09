#[repr(u32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum UartOversamplingRate {
    Rate16x = 0x0000_0000, // Set oversampling rate to 16x
    Rate8x = 0x0000_8000,  // Set oversampling rate to 8x
    Rate3x = 0x0001_0000,  // Set oversampling rate to 3x
                           // Note: IrDA, Manchester, and DALI are not supported when 3x oversampling is enabled.
}
