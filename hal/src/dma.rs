use crate::pac;

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

    pub fn disable<const ID: u8>(&self, _: &Channel<ID>) {
        self._dma
            .dma_dmachan(ID.into())
            .dma_dmactl()
            .write(|w| w.dmaen().disable());
    }

    pub fn aes_init_0<const ID: u8>(&self, _: &Channel<ID>) {
        self._dma.dma_dmachan(ID.into()).dma_dmactl().write(|w| {
            w.dmasrcincr().increment();
            w.dmasrcwdth().word();
            w.dmadstincr().unchanged();
            w.dmadstwdth().word();
            w.dmaem().normal();
            w.dmatm().single()
        });

        self._dma.dma_dmatrig(ID.into()).dma_dmatctl().write(|w| {
            w.dmatint().external();
            unsafe { w.dmatsel().bits(3) }
        });
    }

    pub fn aes_init_1<const ID: u8>(&self, _: &Channel<ID>) {
        self._dma.dma_dmachan(ID.into()).dma_dmactl().write(|w| {
            w.dmasrcincr().unchanged();
            w.dmadstincr().increment();
            w.dmasrcwdth().word();
            w.dmadstwdth().word();
            w.dmaem().normal();
            w.dmatm().single()
        });
        self._dma.dma_dmatrig(ID.into()).dma_dmatctl().write(|w| {
            w.dmatint().external();
            unsafe { w.dmatsel().bits(4) }
        });
    }

    pub fn aes_set<const ID: u8>(
        &self,
        _: &Channel<ID>,
        a: u32,
        b: u32,
        c: u16,
    ) {
        self._dma
            .dma_dmachan(ID.into())
            .dma_dmasa()
            .write(|w| unsafe { w.addr().bits(a) });
        self._dma
            .dma_dmachan(ID.into())
            .dma_dmada()
            .write(|w| unsafe { w.addr().bits(b) });
        self._dma
            .dma_dmachan(ID.into())
            .dma_dmasz()
            .write(|w| unsafe { w.size().bits(c) });
    }

    pub fn aes_wait<const ID: u8>(&self, _: &Channel<ID>) {
        while self
            ._dma
            .dma_cpu_int(ID.into())
            .dma_cpu_int_ris()
            .read()
            .dmach1()
            .is_clr()
        {}
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
