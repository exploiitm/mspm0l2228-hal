#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TxFifoLevel {
    /// Interrupt triggers when FIFO ≤ 3/4 empty
    ThreeQuartersEmpty = 0x00000001,
    /// Interrupt triggers when FIFO ≤ 1/2 empty
    HalfEmpty = 0x00000002,
    /// Interrupt triggers when FIFO ≤ 1/4 empty
    QuarterEmpty = 0x00000003,
    /// Interrupt triggers when FIFO is empty
    Empty = 0x00000005,
    /// Interrupt triggers when FIFO ≥ 1 entry
    OneEntry = 0x00000007,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum RxFifoLevel {
    /// Interrupt triggers when FIFO ≥ 1 entry available (required for DMA trigger)
    OneEntry = 0x00000070,
    /// Interrupt triggers when FIFO is full
    Full = 0x00000050,
    /// Interrupt triggers when FIFO ≥ 3/4 full
    ThreeQuartersFull = 0x00000030,
    /// Interrupt triggers when FIFO ≥ 1/2 full
    HalfFull = 0x00000020,
    /// Interrupt triggers when FIFO ≥ 1/4 full
    QuarterFull = 0x00000010,
}
