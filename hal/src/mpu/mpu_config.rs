
/// Memory region properties.
#[derive(Debug, Copy, Clone)]
pub struct Region {
    /// Starting address of the region (lowest address).
    ///
    /// This must be aligned to the region's `size`.
    pub base_addr: usize,
    /// Size of the region.
    pub size: Size,
    /// The subregions to enable or disable.
    pub subregions: Subregions,
    /// Whether to allow instruction fetches from this region.
    ///
    /// If this is `false`, the region will be marked as NX (Never eXecute).
    /// This affects both privileged and unprivileged code, regardless of
    /// other MPU settings.
    pub executable: bool,
    /// Data access permissions for the region.
    pub permissions: AccessPermission,
    /// Memory type and cache policy attributes.
    pub attributes: MemoryAttributes,
}

/// Describes memory type, cache policy, and shareability.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryAttributes {
    /// Shareable, non-cached, strongly-ordered memory region.
    StronglyOrdered,

    /// Non-cached device peripheral region. Always considered shareable.
    Device,

    /// Normal memory region (ie. "actual" memory, such as Flash or SRAM).
    Normal {
        /// Whether the region is accessible by more than one bus master
        /// (eg. a DMA engine or a second MCU core).
        shareable: bool,

        /// Cache policy of the region.
        cache_policy: CachePolicy,
    },
}

impl MemoryAttributes {
    /// Turns `self` into its bit-level representation, in order `0bSCB`.
    pub fn to_bits( self) -> u32 {
        macro_rules! bits {
            ( C=$c:literal, B=$b:literal, S=$s:ident ) => {
                (if $s { 1 } else { 0 } << 2) | ($c << 1) | $b
            };
            ( C=$c:literal, B=$b:literal, S=$s:literal ) => {
                ($s << 2) | ($c << 1) | $b
            };
        }

        match self {
            Self::StronglyOrdered => bits!(C = 0, B = 0, S = 0),
            Self::Device => bits!(C = 0, B = 1, S = 0),
            Self::Normal {
                shareable,
                cache_policy,
            } => match cache_policy {
                CachePolicy::WriteThrough => bits!(C = 1, B = 0, S = shareable),
                CachePolicy::WriteBack => bits!(C = 1, B = 1, S = shareable),
            },
        }
    }
}

/// The caching policy for a "normal" memory region.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CachePolicy {
    /// Write-through, no write allocate.
    WriteThrough,

    /// Write-back cacheable region, no write-allocate.
    WriteBack,
}


/// Data access permissions for a memory region from unprivileged code.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccessPermission {
    /// Any data access (read or write) will generate a fault.
    NoAccess = 0b01,

    /// Any write access will generate a fault.
    ReadOnly = 0b10,

    /// Region unprotected, both reads and writes are allowed.
    ReadWrite = 0b11,
}

/// Subregion Disable (SRD) bits for the 8 subregions in a region.
///
/// Note that some cores do not support subregions for small region sizes. Check the core's User
/// Guide for more information.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Subregions(u8);

impl Subregions {
    /// None of the 8 subregions are enabled. Equivalent to disabling the entire region.
    pub const NONE: Self = Subregions(0xff);

    /// All 8 subregions are enabled.
    pub const ALL: Self = Subregions(0);

    /// Creates a `Subregions` mask from raw Subregion Disable (SRD) bits.
    ///
    /// The least significant bit disables the lowest 1/8th of the region, and so on.
    pub fn from_disable_bits(bits: u8) -> Self {
        Subregions(bits)
    }

    /// Returns the raw 8-bit Subregion Disable Bits value.
    pub fn bits(self) -> u8 {
        self.0
    }
}

/// By default, all subregions are enabled.
impl Default for Subregions {
    fn default() -> Self {
        Self::ALL
    }
}

/// Memory region size value (5 bits).
///
/// Memory regions must have a size that is a power of two, and their base address must be naturally
/// aligned (ie. aligned to their size).
///
/// There is a core-specific minimum size exposed as `Mpu::MIN_REGION_SIZE`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size(u8);

impl Size {
    pub const S32B: Self = Size(4);
    pub const S64B: Self = Size(5);
    pub const S128B: Self = Size(6);
    pub const S256B: Self = Size(7);
    pub const S512B: Self = Size(8);

    pub const S1K: Self = Size(9);
    pub const S2K: Self = Size(10);
    pub const S4K: Self = Size(11);
    pub const S8K: Self = Size(12);
    pub const S16K: Self = Size(13);
    pub const S32K: Self = Size(14);
    pub const S64K: Self = Size(15);
    pub const S128K: Self = Size(16);
    pub const S256K: Self = Size(17);
    pub const S512K: Self = Size(18);

    pub const S1M: Self = Size(19);
    pub const S2M: Self = Size(20);
    pub const S4M: Self = Size(21);
    pub const S8M: Self = Size(22);
    pub const S16M: Self = Size(23);
    pub const S32M: Self = Size(24);
    pub const S64M: Self = Size(25);
    pub const S128M: Self = Size(26);
    pub const S256M: Self = Size(27);
    pub const S512M: Self = Size(28);

    pub const S1G: Self = Size(29);
    pub const S2G: Self = Size(30);

    /// The entire 4 GiB memory space.
    pub const S4G: Self = Size(31);

    /// Creates a `Size` from a raw 5-bit value.
    ///
    /// The `bits` encode a region size of `2^(bits + 1)`. For example, a 1 KiB region would use
    /// `0b01001` (9): `2^(9+1) = 2^10 = 1024`.
    pub const fn from_raw_bits(bits: u8) -> Self {
        Size(bits)
    }

    /// Returns the raw 5-bit value encoding the region size.
    pub const fn bits(self) -> u8 {
        self.0
    }
}
