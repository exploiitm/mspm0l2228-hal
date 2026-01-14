#[repr(u32)]
#[allow(dead_code)]
pub enum I2cTxFifoLevel {
    LevelEmpty,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
}

#[repr(u32)]
#[allow(dead_code)]
pub enum I2cRxFifoLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
}
