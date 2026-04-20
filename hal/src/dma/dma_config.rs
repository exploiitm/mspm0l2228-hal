pub struct DmaConfig {
    pub src_increment: Increment,
    pub dst_increment: Increment,
    pub src_width: Width,
    pub dst_width: Width,
    pub trigger_map: DmaTrigger,
}

#[allow(dead_code)]
pub enum Increment {
    Unchanged,
    Decrement,
    Increment,
    Stride2,
    Stride3,
    Stride4,
    Stride5,
    Stride6,
    Stride7,
    Stride8,
    Stride9,
}

#[allow(dead_code)]
pub enum Width {
    Byte,
    HalfWord,
    Word,
    LongWord,
}

#[allow(dead_code)]
pub enum DmaTrigger {
    Generic,
    Aes0,
    Aes1,
    Uart0Rx,
    Uart1Rx,
}
