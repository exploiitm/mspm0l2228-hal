pub mod mpu_config;
use cortex_m::asm;
use crate::mpu::mpu_config::{Region, Size};
use mspm0l2228_pac::MPU;

pub use arrayvec::ArrayVec;

fn update_mpu_unprivileged(mpu: &mut MPU, f: impl FnOnce(&mut MPU)) {
    const CTRL_ENABLE: u32 = 1 << 0;
    const _CTRL_HFNMIENA: u32 = 1 << 1;
    const CTRL_PRIVDEFENA: u32 = 1 << 2;

    // Atomic MPU updates:
    // Turn off interrupts, turn off MPU, reconfigure, turn it back on, reenable interrupts.
    // Turning off interrupts is not needed when the old configuration only applied to
    // unprivileged thread code: The entire operation is interruptible, as long as the
    // processor is never made to run any other thread-mode code.

    // https://developer.arm.com/docs/dui0553/latest/cortex-m4-peripherals/optional-memory-protection-unit/updating-an-mpu-region
    asm::dsb();

    // Disable MPU while we update the regions
    unsafe {
        mpu.ctrl.write(0);
    }

    f(mpu);

    unsafe {
        // Enable MPU, but not for privileged code
        mpu.ctrl.write(CTRL_ENABLE | CTRL_PRIVDEFENA);
    }

    asm::dsb();
    asm::isb();
}

pub struct Mpu(pub MPU);

impl Mpu {
    /// The smallest supported region size.
    pub const MIN_REGION_SIZE: Size = Size::S256B;

    /// Number of supported memory regions.
    pub const REGION_COUNT: u8 = 8;

    const REGION_COUNT_USIZE: usize = Self::REGION_COUNT as usize;

    /// Creates a new MPU wrapper, taking ownership of the `MPU` peripheral.
    ///
    /// # Safety
    ///
    /// This function is safe to call if the processor is a Cortex-M0+ and has an MPU.
    pub unsafe fn new(raw: MPU) -> Self {
        Mpu(raw)
    }

    /// Consumes `self` and returns the raw MPU peripheral.
    pub fn into_inner(self) -> MPU {
        self.0
    }

    /// Configures the MPU to restrict access to software running in unprivileged mode.
    ///
    /// Any violation of the MPU settings will cause a *HardFault* exception. The Cortex-M0+
    /// does not have a dedicated memory management exception.
    ///
    /// Unprivileged code will only be allowed to access memory inside one of the given
    /// `regions`.
    ///
    /// Code running in privileged mode will not be restricted by the MPU, except that regions
    /// that have `executable` set to `false` will be marked as ***N**ever e**X**excute* (`NX`),
    /// which is enforced even for privileged code.
    pub fn configure_unprivileged(
        &mut self,
        regions: &ArrayVec<Region, { Self::REGION_COUNT_USIZE }>,
    ) {
        // Safety: This is safe because it does not affect the privileged code calling it.
        // Unprivileged, untrusted (non-Rust) code is always unsafe to call, so this doesn't
        // have to be unsafe as well. If called by unprivileged code, the register writes will
        // fault, which is also safe.

        update_mpu_unprivileged(&mut self.0, |mpu| {
            for (i, region) in regions.iter().enumerate() {
                unsafe {
                    {
                        let addr = (region.base_addr as u32) & !0b11111;
                        let valid = 1 << 4;
                        let region = i as u32;
                        mpu.rbar.write(addr | valid | region);
                    }

                    {
                        let xn = if region.executable { 0 } else { 1 << 28 };
                        let ap = (region.permissions as u32) << 24;
                        let scb = region.attributes.to_bits() << 16;
                        let srd = u32::from(region.subregions.bits()) << 8;
                        let size = u32::from(region.size.bits()) << 1;
                        let enable = 1;

                        mpu.rasr.write(xn | ap | scb | srd | size | enable);
                    }
                }
            }

            // Disable the remaining regions
            for i in regions.len()..usize::from(Self::REGION_COUNT) {
                unsafe {
                    let addr = 0;
                    let valid = 1 << 4;
                    let region = i as u32;
                    mpu.rbar.write(addr | valid | region);

                    mpu.rasr.write(0); // disable region
                }
            }
        });
    }
}

