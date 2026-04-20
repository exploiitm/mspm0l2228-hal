use crate::pac;
pub mod dma_config;

pub trait UsesDma {}
pub struct NoDma;
pub struct WithDma;

impl UsesDma for NoDma {}
impl UsesDma for WithDma {}

pub struct Dma {
    _dma: pac::Dma,
}

impl Dma {
    pub fn new(dma: pac::Dma) -> Self {
        Self { _dma: dma }
    }

    pub fn chans(&self) -> Channels {
        Channels::new()
    }

    pub fn enable<const ID: u8>(&self, _: &Channel<ID>) {
        self._dma
            .dma_dmachan(ID.into())
            .dma_dmactl()
            .modify(|_, w| w.dmaen().enable());
    }

    pub fn start<const ID: u8>(&self, _: &Channel<ID>) {
        self._dma
            .dma_dmachan(ID.into())
            .dma_dmactl()
            .modify(|_, w| w.dmareq().request());
        self._dma.dma_cpu_int(0).dma_cpu_int_imask().modify(
            |_, w| match ID {
            0 => w.dmach0().set_(),
            1 => w.dmach1().set_(),
            2 => w.dmach2().set_(),
            3 => w.dmach3().set_(),
            4 => w.dmach4().set_(),
            5 => w.dmach5().set_(),
            6 => w.dmach6().set_(),
            _ => panic!(),
        });
    }

    pub fn disable<const ID: u8>(&self, _: &Channel<ID>) {
        self._dma
            .dma_dmachan(ID.into())
            .dma_dmactl()
            .write(|w| w.dmaen().disable());
    }

    pub fn dma_init<const ID: u8>(&self, _: &Channel<ID>, config: &dma_config::DmaConfig) {
        self._dma.dma_dmachan(ID.into()).dma_dmactl().write(|w| {
            match config.src_increment {
                dma_config::Increment::Unchanged => w.dmasrcincr().unchanged(),
                dma_config::Increment::Increment => w.dmasrcincr().increment(),
                dma_config::Increment::Decrement => w.dmasrcincr().decrement(),
                dma_config::Increment::Stride2 => w.dmasrcincr().stride_2(),
                dma_config::Increment::Stride3 => w.dmasrcincr().stride_3(),
                dma_config::Increment::Stride4 => w.dmasrcincr().stride_4(),
                dma_config::Increment::Stride5 => w.dmasrcincr().stride_5(),
                dma_config::Increment::Stride6 => w.dmasrcincr().stride_6(),
                dma_config::Increment::Stride7 => w.dmasrcincr().stride_7(),
                dma_config::Increment::Stride8 => w.dmasrcincr().stride_8(),
                dma_config::Increment::Stride9 => w.dmasrcincr().stride_9(),
            };

            match config.dst_increment {
                dma_config::Increment::Unchanged => w.dmadstincr().unchanged(),
                dma_config::Increment::Increment => w.dmadstincr().increment(),
                dma_config::Increment::Decrement => w.dmadstincr().decrement(),
                dma_config::Increment::Stride2 => w.dmadstincr().stride_2(),
                dma_config::Increment::Stride3 => w.dmadstincr().stride_3(),
                dma_config::Increment::Stride4 => w.dmadstincr().stride_4(),
                dma_config::Increment::Stride5 => w.dmadstincr().stride_5(),
                dma_config::Increment::Stride6 => w.dmadstincr().stride_6(),
                dma_config::Increment::Stride7 => w.dmadstincr().stride_7(),
                dma_config::Increment::Stride8 => w.dmadstincr().stride_8(),
                dma_config::Increment::Stride9 => w.dmadstincr().stride_9(),
            };

            match config.src_width {
                dma_config::Width::Byte => w.dmasrcwdth().byte(),
                dma_config::Width::HalfWord => w.dmasrcwdth().half(),
                dma_config::Width::Word => w.dmasrcwdth().word(),
                dma_config::Width::LongWord => w.dmasrcwdth().long(),
            };

            match config.dst_width {
                dma_config::Width::Byte => w.dmadstwdth().byte(),
                dma_config::Width::HalfWord => w.dmadstwdth().half(),
                dma_config::Width::Word => w.dmadstwdth().word(),
                dma_config::Width::LongWord => w.dmadstwdth().long(),
            };

            w.dmaem().normal();
            w.dmatm().single()
        });

        self._dma.dma_dmatrig(ID.into()).dma_dmatctl().write(|w| {
            w.dmatint().external();
            unsafe {
                w.dmatsel().bits(match config.trigger_map {
                    dma_config::DmaTrigger::Generic => 0,
                    dma_config::DmaTrigger::Aes0 => 3,
                    dma_config::DmaTrigger::Aes1 => 4,
                    dma_config::DmaTrigger::Uart0Rx => 15,
                    dma_config::DmaTrigger::Uart1Rx => 17,
                })
            }
        });
    }

    pub fn dma_set<const ID: u8>(
        &self,
        _: &Channel<ID>,
        src_addr: u32,
        dest_addr: u32,
        data_size: u16,
    ) {
        self._dma
            .dma_dmachan(ID.into())
            .dma_dmasa()
            .write(|w| unsafe { w.addr().bits(src_addr) });
        self._dma
            .dma_dmachan(ID.into())
            .dma_dmada()
            .write(|w| unsafe { w.addr().bits(dest_addr) });
        self._dma
            .dma_dmachan(ID.into())
            .dma_dmasz()
            .write(|w| unsafe { w.size().bits(data_size) });
    }

    pub fn dma_wait<const ID: u8>(&self, _: &Channel<ID>) {
        let x = self._dma.dma_cpu_int(0).dma_cpu_int_ris();
        while match ID {
            0 => x.read().dmach0().is_clr(),
            1 => x.read().dmach1().is_clr(),
            2 => x.read().dmach2().is_clr(),
            3 => x.read().dmach3().is_clr(),
            4 => x.read().dmach4().is_clr(),
            5 => x.read().dmach5().is_clr(),
            6 => x.read().dmach6().is_clr(),
            _ => panic!(),
        } {}

        self._dma
            .dma_cpu_int(0)
            .dma_cpu_int_iclr()
            .write(|w| match ID {
                0 => w.dmach0().clr(),
                1 => w.dmach1().clr(),
                2 => w.dmach2().clr(),
                3 => w.dmach3().clr(),
                4 => w.dmach4().clr(),
                5 => w.dmach5().clr(),
                6 => w.dmach6().clr(),
                _ => panic!(),
            });
    }
}

pub struct Channel<const ID: u8>;

impl<const ID: u8> Channel<ID> {
    fn new() -> Self {
        Self {}
    }
}

pub struct Channels {
    pub chan0: Channel<0>,
    pub chan1: Channel<1>,
    pub chan2: Channel<2>,
    pub chan3: Channel<3>,
    pub chan4: Channel<4>,
    pub chan5: Channel<5>,
    pub chan6: Channel<6>,
}

impl Channels {
    fn new() -> Self {
        Self {
            chan0: Channel::<0>::new(),
            chan1: Channel::<1>::new(),
            chan2: Channel::<2>::new(),
            chan3: Channel::<3>::new(),
            chan4: Channel::<4>::new(),
            chan5: Channel::<5>::new(),
            chan6: Channel::<6>::new(),
        }
    }
}
