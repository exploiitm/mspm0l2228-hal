#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1020],
    sysctl_iidx: SysctlIidx,
    _reserved1: [u8; 0x04],
    sysctl_imask: SysctlImask,
    _reserved2: [u8; 0x04],
    sysctl_ris: SysctlRis,
    _reserved3: [u8; 0x04],
    sysctl_mis: SysctlMis,
    _reserved4: [u8; 0x04],
    sysctl_iset: SysctlIset,
    _reserved5: [u8; 0x04],
    sysctl_iclr: SysctlIclr,
    _reserved6: [u8; 0x04],
    sysctl_nmiiidx: SysctlNmiiidx,
    _reserved7: [u8; 0x0c],
    sysctl_nmiris: SysctlNmiris,
    _reserved8: [u8; 0x0c],
    sysctl_nmiiset: SysctlNmiiset,
    _reserved9: [u8; 0x04],
    sysctl_nmiiclr: SysctlNmiiclr,
    _reserved10: [u8; 0x84],
    sysctl_sysosccfg: SysctlSysosccfg,
    sysctl_mclkcfg: SysctlMclkcfg,
    sysctl_hsclken: SysctlHsclken,
    sysctl_hsclkcfg: SysctlHsclkcfg,
    sysctl_hfclkclkcfg: SysctlHfclkclkcfg,
    sysctl_lfclkcfg: SysctlLfclkcfg,
    _reserved16: [u8; 0x20],
    sysctl_genclkcfg: SysctlGenclkcfg,
    sysctl_genclken: SysctlGenclken,
    sysctl_pmodecfg: SysctlPmodecfg,
    _reserved19: [u8; 0x0c],
    sysctl_fcc: SysctlFcc,
    _reserved20: [u8; 0x1c],
    sysctl_sysosctrimuser: SysctlSysosctrimuser,
    _reserved21: [u8; 0x04],
    sysctl_sramboundary: SysctlSramboundary,
    _reserved22: [u8; 0x04],
    sysctl_systemcfg: SysctlSystemcfg,
    _reserved23: [u8; 0x7c],
    sysctl_writelock: SysctlWritelock,
    sysctl_clkstatus: SysctlClkstatus,
    sysctl_sysstatus: SysctlSysstatus,
    sysctl_dederraddr: SysctlDederraddr,
    _reserved27: [u8; 0x10],
    sysctl_rstcause: SysctlRstcause,
    _reserved28: [u8; 0xdc],
    sysctl_resetlevel: SysctlResetlevel,
    sysctl_resetcmd: SysctlResetcmd,
    sysctl_borthreshold: SysctlBorthreshold,
    sysctl_borclrcmd: SysctlBorclrcmd,
    sysctl_sysoscfclctl: SysctlSysoscfclctl,
    sysctl_lfxtctl: SysctlLfxtctl,
    sysctl_exlfctl: SysctlExlfctl,
    sysctl_shdniorel: SysctlShdniorel,
    sysctl_exrstpin: SysctlExrstpin,
    sysctl_sysstatusclr: SysctlSysstatusclr,
    sysctl_swdcfg: SysctlSwdcfg,
    sysctl_fcccmd: SysctlFcccmd,
    _reserved40: [u8; 0xd0],
    sysctl_shutdnstore0: SysctlShutdnstore0,
    sysctl_shutdnstore1: SysctlShutdnstore1,
    sysctl_shutdnstore2: SysctlShutdnstore2,
    sysctl_shutdnstore3: SysctlShutdnstore3,
    _reserved44: [u8; 0x1bf0],
    sysctl_fweprotmain: SysctlFweprotmain,
    _reserved45: [u8; 0x14],
    sysctl_frxprotmainstart: SysctlFrxprotmainstart,
    sysctl_frxprotmainend: SysctlFrxprotmainend,
    sysctl_fipprotmainstart: SysctlFipprotmainstart,
    sysctl_fipprotmainend: SysctlFipprotmainend,
    _reserved49: [u8; 0x10],
    sysctl_flbankswppolicy: SysctlFlbankswppolicy,
    sysctl_flbankswp: SysctlFlbankswp,
    _reserved51: [u8; 0x04],
    sysctl_fwenable: SysctlFwenable,
    sysctl_secstatus: SysctlSecstatus,
    _reserved53: [u8; 0x14],
    sysctl_initdone: SysctlInitdone,
    _reserved54: [u8; 0x179c],
    sysctl_mgmt_adc12b1msps0_pwren: SysctlMgmtAdc12b1msps0Pwren,
    sysctl_mgmt_adc12b1msps0_rstctl: SysctlMgmtAdc12b1msps0Rstctl,
    sysctl_mgmt_adc12b1msps0_clkcfg: SysctlMgmtAdc12b1msps0Clkcfg,
    _reserved57: [u8; 0x08],
    sysctl_mgmt_adc12b1msps0_stat: SysctlMgmtAdc12b1msps0Stat,
    _reserved58: [u8; 0x3fe8],
    sysctl_mgmt_anacomp0_pwren: SysctlMgmtAnacomp0Pwren,
    sysctl_mgmt_anacomp0_rstctl: SysctlMgmtAnacomp0Rstctl,
    sysctl_mgmt_anacomp0_clkcfg: SysctlMgmtAnacomp0Clkcfg,
    _reserved61: [u8; 0x08],
    sysctl_mgmt_anacomp0_stat: SysctlMgmtAnacomp0Stat,
    _reserved62: [u8; 0x0002_7fe8],
    sysctl_mgmt_vref_pwren: SysctlMgmtVrefPwren,
    sysctl_mgmt_vref_rstctl: SysctlMgmtVrefRstctl,
    _reserved64: [u8; 0x0c],
    sysctl_mgmt_vref_stat: SysctlMgmtVrefStat,
    _reserved65: [u8; 0x0003_ffe8],
    sysctl_mgmt_lcd_pwren: SysctlMgmtLcdPwren,
    sysctl_mgmt_lcd_rstctl: SysctlMgmtLcdRstctl,
    _reserved67: [u8; 0x0c],
    sysctl_mgmt_lcd_stat: SysctlMgmtLcdStat,
    _reserved68: [u8; 0xffe8],
    sysctl_mgmt_wwdtlp0_pwren: SysctlMgmtWwdtlp0Pwren,
    sysctl_mgmt_wwdtlp0_rstctl: SysctlMgmtWwdtlp0Rstctl,
    _reserved70: [u8; 0x0c],
    sysctl_mgmt_wwdtlp0_stat: SysctlMgmtWwdtlp0Stat,
    _reserved71: [u8; 0x3fe8],
    sysctl_mgmt_gptimer16b2cclp0_pwren: SysctlMgmtGptimer16b2cclp0Pwren,
    sysctl_mgmt_gptimer16b2cclp0_rstctl: SysctlMgmtGptimer16b2cclp0Rstctl,
    _reserved73: [u8; 0x0c],
    sysctl_mgmt_gptimer16b2cclp0_stat: SysctlMgmtGptimer16b2cclp0Stat,
    _reserved74: [u8; 0x7fe8],
    sysctl_mgmt_gptimer16b2ccsplp0_pwren: SysctlMgmtGptimer16b2ccsplp0Pwren,
    sysctl_mgmt_gptimer16b2ccsplp0_rstctl: SysctlMgmtGptimer16b2ccsplp0Rstctl,
    _reserved76: [u8; 0x0c],
    sysctl_mgmt_gptimer16b2ccsplp0_stat: SysctlMgmtGptimer16b2ccsplp0Stat,
    _reserved77: [u8; 0x1fe8],
    sysctl_mgmt_gptimer16b2ccsplp1_pwren: SysctlMgmtGptimer16b2ccsplp1Pwren,
    sysctl_mgmt_gptimer16b2ccsplp1_rstctl: SysctlMgmtGptimer16b2ccsplp1Rstctl,
    _reserved79: [u8; 0x0c],
    sysctl_mgmt_gptimer16b2ccsplp1_stat: SysctlMgmtGptimer16b2ccsplp1Stat,
    _reserved80: [u8; 0x1fe8],
    sysctl_mgmt_gptimer16b2ccqeilp0_pwren: SysctlMgmtGptimer16b2ccqeilp0Pwren,
    sysctl_mgmt_gptimer16b2ccqeilp0_rstctl: SysctlMgmtGptimer16b2ccqeilp0Rstctl,
    _reserved82: [u8; 0x0c],
    sysctl_mgmt_gptimer16b2ccqeilp0_stat: SysctlMgmtGptimer16b2ccqeilp0Stat,
    _reserved83: [u8; 0x3ff0],
    sysctl_mgmt_rtc_clkcfg: SysctlMgmtRtcClkcfg,
    _reserved84: [u8; 0xbff4],
    sysctl_mgmt_gpio0_pwren: SysctlMgmtGpio0Pwren,
    sysctl_mgmt_gpio0_rstctl: SysctlMgmtGpio0Rstctl,
    _reserved86: [u8; 0x0c],
    sysctl_mgmt_gpio0_stat: SysctlMgmtGpio0Stat,
    _reserved87: [u8; 0x1fe8],
    sysctl_mgmt_gpio1_pwren: SysctlMgmtGpio1Pwren,
    sysctl_mgmt_gpio1_rstctl: SysctlMgmtGpio1Rstctl,
    _reserved89: [u8; 0x0c],
    sysctl_mgmt_gpio1_stat: SysctlMgmtGpio1Stat,
    _reserved90: [u8; 0x1fe8],
    sysctl_mgmt_gpio2_pwren: SysctlMgmtGpio2Pwren,
    sysctl_mgmt_gpio2_rstctl: SysctlMgmtGpio2Rstctl,
    _reserved92: [u8; 0x0c],
    sysctl_mgmt_gpio2_stat: SysctlMgmtGpio2Stat,
    _reserved93: [u8; 0x0004_bfe8],
    sysctl_mgmt_i2c0_pwren: SysctlMgmtI2c0Pwren,
    sysctl_mgmt_i2c0_rstctl: SysctlMgmtI2c0Rstctl,
    sysctl_mgmt_i2c0_clkcfg: SysctlMgmtI2c0Clkcfg,
    _reserved96: [u8; 0x08],
    sysctl_mgmt_i2c0_stat: SysctlMgmtI2c0Stat,
    _reserved97: [u8; 0x1fe8],
    sysctl_mgmt_i2c1_pwren: SysctlMgmtI2c1Pwren,
    sysctl_mgmt_i2c1_rstctl: SysctlMgmtI2c1Rstctl,
    sysctl_mgmt_i2c1_clkcfg: SysctlMgmtI2c1Clkcfg,
    _reserved100: [u8; 0x08],
    sysctl_mgmt_i2c1_stat: SysctlMgmtI2c1Stat,
    _reserved101: [u8; 0x1fe8],
    sysctl_mgmt_i2c2_pwren: SysctlMgmtI2c2Pwren,
    sysctl_mgmt_i2c2_rstctl: SysctlMgmtI2c2Rstctl,
    sysctl_mgmt_i2c2_clkcfg: SysctlMgmtI2c2Clkcfg,
    _reserved104: [u8; 0x08],
    sysctl_mgmt_i2c2_stat: SysctlMgmtI2c2Stat,
    _reserved105: [u8; 0xbfe8],
    sysctl_mgmt_uartlp0_pwren: SysctlMgmtUartlp0Pwren,
    sysctl_mgmt_uartlp0_rstctl: SysctlMgmtUartlp0Rstctl,
    sysctl_mgmt_uartlp0_clkcfg: SysctlMgmtUartlp0Clkcfg,
    _reserved108: [u8; 0x08],
    sysctl_mgmt_uartlp0_stat: SysctlMgmtUartlp0Stat,
    _reserved109: [u8; 0x1fe8],
    sysctl_mgmt_uartlp1_pwren: SysctlMgmtUartlp1Pwren,
    sysctl_mgmt_uartlp1_rstctl: SysctlMgmtUartlp1Rstctl,
    sysctl_mgmt_uartlp1_clkcfg: SysctlMgmtUartlp1Clkcfg,
    _reserved112: [u8; 0x08],
    sysctl_mgmt_uartlp1_stat: SysctlMgmtUartlp1Stat,
    _reserved113: [u8; 0x1fe8],
    sysctl_mgmt_uartlp2_pwren: SysctlMgmtUartlp2Pwren,
    sysctl_mgmt_uartlp2_rstctl: SysctlMgmtUartlp2Rstctl,
    sysctl_mgmt_uartlp2_clkcfg: SysctlMgmtUartlp2Clkcfg,
    _reserved116: [u8; 0x08],
    sysctl_mgmt_uartlp2_stat: SysctlMgmtUartlp2Stat,
    _reserved117: [u8; 0x3fe8],
    sysctl_mgmt_uartadvlp0_pwren: SysctlMgmtUartadvlp0Pwren,
    sysctl_mgmt_uartadvlp0_rstctl: SysctlMgmtUartadvlp0Rstctl,
    sysctl_mgmt_uartadvlp0_clkcfg: SysctlMgmtUartadvlp0Clkcfg,
    _reserved120: [u8; 0x08],
    sysctl_mgmt_uartadvlp0_stat: SysctlMgmtUartadvlp0Stat,
    _reserved121: [u8; 0x1fe8],
    sysctl_mgmt_uartadvlp1_pwren: SysctlMgmtUartadvlp1Pwren,
    sysctl_mgmt_uartadvlp1_rstctl: SysctlMgmtUartadvlp1Rstctl,
    sysctl_mgmt_uartadvlp1_clkcfg: SysctlMgmtUartadvlp1Clkcfg,
    _reserved124: [u8; 0x08],
    sysctl_mgmt_uartadvlp1_stat: SysctlMgmtUartadvlp1Stat,
    _reserved125: [u8; 0x0033_5fe8],
    sysctl_mgmt_crc0_pwren: SysctlMgmtCrc0Pwren,
    sysctl_mgmt_crc0_rstctl: SysctlMgmtCrc0Rstctl,
    _reserved127: [u8; 0x0c],
    sysctl_mgmt_crc0_stat: SysctlMgmtCrc0Stat,
    _reserved128: [u8; 0x1fe8],
    sysctl_mgmt_aes_pwren: SysctlMgmtAesPwren,
    sysctl_mgmt_aes_rstctl: SysctlMgmtAesRstctl,
    _reserved130: [u8; 0x0c],
    sysctl_mgmt_aes_stat: SysctlMgmtAesStat,
    _reserved131: [u8; 0x1fe8],
    sysctl_mgmt_trng_pwren: SysctlMgmtTrngPwren,
    sysctl_mgmt_trng_rstctl: SysctlMgmtTrngRstctl,
    _reserved133: [u8; 0x0c],
    sysctl_mgmt_trng_stat: SysctlMgmtTrngStat,
    _reserved134: [u8; 0x0002_3fe8],
    sysctl_mgmt_spi0_pwren: SysctlMgmtSpi0Pwren,
    sysctl_mgmt_spi0_rstctl: SysctlMgmtSpi0Rstctl,
    sysctl_mgmt_spi0_clkcfg: SysctlMgmtSpi0Clkcfg,
    _reserved137: [u8; 0x08],
    sysctl_mgmt_spi0_stat: SysctlMgmtSpi0Stat,
    _reserved138: [u8; 0x1fe8],
    sysctl_mgmt_spi1_pwren: SysctlMgmtSpi1Pwren,
    sysctl_mgmt_spi1_rstctl: SysctlMgmtSpi1Rstctl,
    sysctl_mgmt_spi1_clkcfg: SysctlMgmtSpi1Clkcfg,
    _reserved141: [u8; 0x08],
    sysctl_mgmt_spi1_stat: SysctlMgmtSpi1Stat,
    _reserved142: [u8; 0x003f_5fe8],
    sysctl_mgmt_gptimer16badv4cc0_pwren: SysctlMgmtGptimer16badv4cc0Pwren,
    sysctl_mgmt_gptimer16badv4cc0_rstctl: SysctlMgmtGptimer16badv4cc0Rstctl,
    _reserved144: [u8; 0x0c],
    sysctl_mgmt_gptimer16badv4cc0_stat: SysctlMgmtGptimer16badv4cc0Stat,
    _reserved145: [u8; 0xffe8],
    sysctl_mgmt_gptimer32b2cc0_pwren: SysctlMgmtGptimer32b2cc0Pwren,
    sysctl_mgmt_gptimer32b2cc0_rstctl: SysctlMgmtGptimer32b2cc0Rstctl,
    _reserved147: [u8; 0x0c],
    sysctl_mgmt_gptimer32b2cc0_stat: SysctlMgmtGptimer32b2cc0Stat,
}
impl RegisterBlock {
    #[doc = "0x1020 - SYSCTL interrupt index"]
    #[inline(always)]
    pub const fn sysctl_iidx(&self) -> &SysctlIidx {
        &self.sysctl_iidx
    }
    #[doc = "0x1028 - SYSCTL interrupt mask"]
    #[inline(always)]
    pub const fn sysctl_imask(&self) -> &SysctlImask {
        &self.sysctl_imask
    }
    #[doc = "0x1030 - SYSCTL raw interrupt status"]
    #[inline(always)]
    pub const fn sysctl_ris(&self) -> &SysctlRis {
        &self.sysctl_ris
    }
    #[doc = "0x1038 - SYSCTL masked interrupt status"]
    #[inline(always)]
    pub const fn sysctl_mis(&self) -> &SysctlMis {
        &self.sysctl_mis
    }
    #[doc = "0x1040 - SYSCTL interrupt set"]
    #[inline(always)]
    pub const fn sysctl_iset(&self) -> &SysctlIset {
        &self.sysctl_iset
    }
    #[doc = "0x1048 - SYSCTL interrupt clear"]
    #[inline(always)]
    pub const fn sysctl_iclr(&self) -> &SysctlIclr {
        &self.sysctl_iclr
    }
    #[doc = "0x1050 - NMI interrupt index"]
    #[inline(always)]
    pub const fn sysctl_nmiiidx(&self) -> &SysctlNmiiidx {
        &self.sysctl_nmiiidx
    }
    #[doc = "0x1060 - NMI raw interrupt status"]
    #[inline(always)]
    pub const fn sysctl_nmiris(&self) -> &SysctlNmiris {
        &self.sysctl_nmiris
    }
    #[doc = "0x1070 - NMI interrupt set"]
    #[inline(always)]
    pub const fn sysctl_nmiiset(&self) -> &SysctlNmiiset {
        &self.sysctl_nmiiset
    }
    #[doc = "0x1078 - NMI interrupt clear"]
    #[inline(always)]
    pub const fn sysctl_nmiiclr(&self) -> &SysctlNmiiclr {
        &self.sysctl_nmiiclr
    }
    #[doc = "0x1100 - SYSOSC configuration"]
    #[inline(always)]
    pub const fn sysctl_sysosccfg(&self) -> &SysctlSysosccfg {
        &self.sysctl_sysosccfg
    }
    #[doc = "0x1104 - Main clock (MCLK) configuration"]
    #[inline(always)]
    pub const fn sysctl_mclkcfg(&self) -> &SysctlMclkcfg {
        &self.sysctl_mclkcfg
    }
    #[doc = "0x1108 - High-speed clock (HSCLK) source enable/disable"]
    #[inline(always)]
    pub const fn sysctl_hsclken(&self) -> &SysctlHsclken {
        &self.sysctl_hsclken
    }
    #[doc = "0x110c - High-speed clock (HSCLK) source selection"]
    #[inline(always)]
    pub const fn sysctl_hsclkcfg(&self) -> &SysctlHsclkcfg {
        &self.sysctl_hsclkcfg
    }
    #[doc = "0x1110 - High-frequency clock (HFCLK) configuration"]
    #[inline(always)]
    pub const fn sysctl_hfclkclkcfg(&self) -> &SysctlHfclkclkcfg {
        &self.sysctl_hfclkclkcfg
    }
    #[doc = "0x1114 - Low frequency crystal oscillator (LFXT) configuration"]
    #[inline(always)]
    pub const fn sysctl_lfclkcfg(&self) -> &SysctlLfclkcfg {
        &self.sysctl_lfclkcfg
    }
    #[doc = "0x1138 - General clock configuration"]
    #[inline(always)]
    pub const fn sysctl_genclkcfg(&self) -> &SysctlGenclkcfg {
        &self.sysctl_genclkcfg
    }
    #[doc = "0x113c - General clock enable control"]
    #[inline(always)]
    pub const fn sysctl_genclken(&self) -> &SysctlGenclken {
        &self.sysctl_genclken
    }
    #[doc = "0x1140 - Power mode configuration"]
    #[inline(always)]
    pub const fn sysctl_pmodecfg(&self) -> &SysctlPmodecfg {
        &self.sysctl_pmodecfg
    }
    #[doc = "0x1150 - Frequency clock counter (FCC) count"]
    #[inline(always)]
    pub const fn sysctl_fcc(&self) -> &SysctlFcc {
        &self.sysctl_fcc
    }
    #[doc = "0x1170 - SYSOSC user-specified trim"]
    #[inline(always)]
    pub const fn sysctl_sysosctrimuser(&self) -> &SysctlSysosctrimuser {
        &self.sysctl_sysosctrimuser
    }
    #[doc = "0x1178 - SRAM Write Boundary"]
    #[inline(always)]
    pub const fn sysctl_sramboundary(&self) -> &SysctlSramboundary {
        &self.sysctl_sramboundary
    }
    #[doc = "0x1180 - System configuration"]
    #[inline(always)]
    pub const fn sysctl_systemcfg(&self) -> &SysctlSystemcfg {
        &self.sysctl_systemcfg
    }
    #[doc = "0x1200 - SYSCTL register write lockout"]
    #[inline(always)]
    pub const fn sysctl_writelock(&self) -> &SysctlWritelock {
        &self.sysctl_writelock
    }
    #[doc = "0x1204 - Clock module (CKM) status"]
    #[inline(always)]
    pub const fn sysctl_clkstatus(&self) -> &SysctlClkstatus {
        &self.sysctl_clkstatus
    }
    #[doc = "0x1208 - System status information"]
    #[inline(always)]
    pub const fn sysctl_sysstatus(&self) -> &SysctlSysstatus {
        &self.sysctl_sysstatus
    }
    #[doc = "0x120c - Memory DED Address"]
    #[inline(always)]
    pub const fn sysctl_dederraddr(&self) -> &SysctlDederraddr {
        &self.sysctl_dederraddr
    }
    #[doc = "0x1220 - Reset cause"]
    #[inline(always)]
    pub const fn sysctl_rstcause(&self) -> &SysctlRstcause {
        &self.sysctl_rstcause
    }
    #[doc = "0x1300 - Reset level for application-triggered reset command"]
    #[inline(always)]
    pub const fn sysctl_resetlevel(&self) -> &SysctlResetlevel {
        &self.sysctl_resetlevel
    }
    #[doc = "0x1304 - Execute an application-triggered reset command"]
    #[inline(always)]
    pub const fn sysctl_resetcmd(&self) -> &SysctlResetcmd {
        &self.sysctl_resetcmd
    }
    #[doc = "0x1308 - BOR threshold selection"]
    #[inline(always)]
    pub const fn sysctl_borthreshold(&self) -> &SysctlBorthreshold {
        &self.sysctl_borthreshold
    }
    #[doc = "0x130c - Set the BOR threshold"]
    #[inline(always)]
    pub const fn sysctl_borclrcmd(&self) -> &SysctlBorclrcmd {
        &self.sysctl_borclrcmd
    }
    #[doc = "0x1310 - SYSOSC frequency correction loop (FCL) ROSC enable"]
    #[inline(always)]
    pub const fn sysctl_sysoscfclctl(&self) -> &SysctlSysoscfclctl {
        &self.sysctl_sysoscfclctl
    }
    #[doc = "0x1314 - LFXT and LFCLK control"]
    #[inline(always)]
    pub const fn sysctl_lfxtctl(&self) -> &SysctlLfxtctl {
        &self.sysctl_lfxtctl
    }
    #[doc = "0x1318 - LFCLK_IN and LFCLK control"]
    #[inline(always)]
    pub const fn sysctl_exlfctl(&self) -> &SysctlExlfctl {
        &self.sysctl_exlfctl
    }
    #[doc = "0x131c - SHUTDOWN IO release control"]
    #[inline(always)]
    pub const fn sysctl_shdniorel(&self) -> &SysctlShdniorel {
        &self.sysctl_shdniorel
    }
    #[doc = "0x1320 - Disable the reset function of the NRST pin"]
    #[inline(always)]
    pub const fn sysctl_exrstpin(&self) -> &SysctlExrstpin {
        &self.sysctl_exrstpin
    }
    #[doc = "0x1324 - Clear sticky bits of SYSSTATUS"]
    #[inline(always)]
    pub const fn sysctl_sysstatusclr(&self) -> &SysctlSysstatusclr {
        &self.sysctl_sysstatusclr
    }
    #[doc = "0x1328 - Disable the SWD function on the SWD pins"]
    #[inline(always)]
    pub const fn sysctl_swdcfg(&self) -> &SysctlSwdcfg {
        &self.sysctl_swdcfg
    }
    #[doc = "0x132c - Frequency clock counter start capture"]
    #[inline(always)]
    pub const fn sysctl_fcccmd(&self) -> &SysctlFcccmd {
        &self.sysctl_fcccmd
    }
    #[doc = "0x1400 - Shutdown storage memory (byte 0)"]
    #[inline(always)]
    pub const fn sysctl_shutdnstore0(&self) -> &SysctlShutdnstore0 {
        &self.sysctl_shutdnstore0
    }
    #[doc = "0x1404 - Shutdown storage memory (byte 1)"]
    #[inline(always)]
    pub const fn sysctl_shutdnstore1(&self) -> &SysctlShutdnstore1 {
        &self.sysctl_shutdnstore1
    }
    #[doc = "0x1408 - Shutdown storage memory (byte 2)"]
    #[inline(always)]
    pub const fn sysctl_shutdnstore2(&self) -> &SysctlShutdnstore2 {
        &self.sysctl_shutdnstore2
    }
    #[doc = "0x140c - Shutdown storage memory (byte 3)"]
    #[inline(always)]
    pub const fn sysctl_shutdnstore3(&self) -> &SysctlShutdnstore3 {
        &self.sysctl_shutdnstore3
    }
    #[doc = "0x3000 - 1 Sector Write-Erase per bit starting at address 0x0 of flash"]
    #[inline(always)]
    pub const fn sysctl_fweprotmain(&self) -> &SysctlFweprotmain {
        &self.sysctl_fweprotmain
    }
    #[doc = "0x3018 - Flash RX Protection Start Address"]
    #[inline(always)]
    pub const fn sysctl_frxprotmainstart(&self) -> &SysctlFrxprotmainstart {
        &self.sysctl_frxprotmainstart
    }
    #[doc = "0x301c - Flash RX Protection End Address"]
    #[inline(always)]
    pub const fn sysctl_frxprotmainend(&self) -> &SysctlFrxprotmainend {
        &self.sysctl_frxprotmainend
    }
    #[doc = "0x3020 - Flash IP Protection Start Address"]
    #[inline(always)]
    pub const fn sysctl_fipprotmainstart(&self) -> &SysctlFipprotmainstart {
        &self.sysctl_fipprotmainstart
    }
    #[doc = "0x3024 - Flash IP Protection End Address"]
    #[inline(always)]
    pub const fn sysctl_fipprotmainend(&self) -> &SysctlFipprotmainend {
        &self.sysctl_fipprotmainend
    }
    #[doc = "0x3038 - Flash Bank Swap Policy"]
    #[inline(always)]
    pub const fn sysctl_flbankswppolicy(&self) -> &SysctlFlbankswppolicy {
        &self.sysctl_flbankswppolicy
    }
    #[doc = "0x303c - Flash MAIN bank address swap"]
    #[inline(always)]
    pub const fn sysctl_flbankswp(&self) -> &SysctlFlbankswp {
        &self.sysctl_flbankswp
    }
    #[doc = "0x3044 - Security Firewall Enable Register"]
    #[inline(always)]
    pub const fn sysctl_fwenable(&self) -> &SysctlFwenable {
        &self.sysctl_fwenable
    }
    #[doc = "0x3048 - Security Configuration status"]
    #[inline(always)]
    pub const fn sysctl_secstatus(&self) -> &SysctlSecstatus {
        &self.sysctl_secstatus
    }
    #[doc = "0x3060 - INITCODE PASS"]
    #[inline(always)]
    pub const fn sysctl_initdone(&self) -> &SysctlInitdone {
        &self.sysctl_initdone
    }
    #[doc = "0x4800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_adc12b1msps0_pwren(&self) -> &SysctlMgmtAdc12b1msps0Pwren {
        &self.sysctl_mgmt_adc12b1msps0_pwren
    }
    #[doc = "0x4804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_adc12b1msps0_rstctl(&self) -> &SysctlMgmtAdc12b1msps0Rstctl {
        &self.sysctl_mgmt_adc12b1msps0_rstctl
    }
    #[doc = "0x4808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_adc12b1msps0_clkcfg(&self) -> &SysctlMgmtAdc12b1msps0Clkcfg {
        &self.sysctl_mgmt_adc12b1msps0_clkcfg
    }
    #[doc = "0x4814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_adc12b1msps0_stat(&self) -> &SysctlMgmtAdc12b1msps0Stat {
        &self.sysctl_mgmt_adc12b1msps0_stat
    }
    #[doc = "0x8800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_anacomp0_pwren(&self) -> &SysctlMgmtAnacomp0Pwren {
        &self.sysctl_mgmt_anacomp0_pwren
    }
    #[doc = "0x8804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_anacomp0_rstctl(&self) -> &SysctlMgmtAnacomp0Rstctl {
        &self.sysctl_mgmt_anacomp0_rstctl
    }
    #[doc = "0x8808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_anacomp0_clkcfg(&self) -> &SysctlMgmtAnacomp0Clkcfg {
        &self.sysctl_mgmt_anacomp0_clkcfg
    }
    #[doc = "0x8814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_anacomp0_stat(&self) -> &SysctlMgmtAnacomp0Stat {
        &self.sysctl_mgmt_anacomp0_stat
    }
    #[doc = "0x30800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_vref_pwren(&self) -> &SysctlMgmtVrefPwren {
        &self.sysctl_mgmt_vref_pwren
    }
    #[doc = "0x30804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_vref_rstctl(&self) -> &SysctlMgmtVrefRstctl {
        &self.sysctl_mgmt_vref_rstctl
    }
    #[doc = "0x30814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_vref_stat(&self) -> &SysctlMgmtVrefStat {
        &self.sysctl_mgmt_vref_stat
    }
    #[doc = "0x70800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_lcd_pwren(&self) -> &SysctlMgmtLcdPwren {
        &self.sysctl_mgmt_lcd_pwren
    }
    #[doc = "0x70804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_lcd_rstctl(&self) -> &SysctlMgmtLcdRstctl {
        &self.sysctl_mgmt_lcd_rstctl
    }
    #[doc = "0x70814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_lcd_stat(&self) -> &SysctlMgmtLcdStat {
        &self.sysctl_mgmt_lcd_stat
    }
    #[doc = "0x80800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_wwdtlp0_pwren(&self) -> &SysctlMgmtWwdtlp0Pwren {
        &self.sysctl_mgmt_wwdtlp0_pwren
    }
    #[doc = "0x80804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_wwdtlp0_rstctl(&self) -> &SysctlMgmtWwdtlp0Rstctl {
        &self.sysctl_mgmt_wwdtlp0_rstctl
    }
    #[doc = "0x80814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_wwdtlp0_stat(&self) -> &SysctlMgmtWwdtlp0Stat {
        &self.sysctl_mgmt_wwdtlp0_stat
    }
    #[doc = "0x84800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2cclp0_pwren(&self) -> &SysctlMgmtGptimer16b2cclp0Pwren {
        &self.sysctl_mgmt_gptimer16b2cclp0_pwren
    }
    #[doc = "0x84804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2cclp0_rstctl(&self) -> &SysctlMgmtGptimer16b2cclp0Rstctl {
        &self.sysctl_mgmt_gptimer16b2cclp0_rstctl
    }
    #[doc = "0x84814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2cclp0_stat(&self) -> &SysctlMgmtGptimer16b2cclp0Stat {
        &self.sysctl_mgmt_gptimer16b2cclp0_stat
    }
    #[doc = "0x8c800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2ccsplp0_pwren(&self) -> &SysctlMgmtGptimer16b2ccsplp0Pwren {
        &self.sysctl_mgmt_gptimer16b2ccsplp0_pwren
    }
    #[doc = "0x8c804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2ccsplp0_rstctl(
        &self,
    ) -> &SysctlMgmtGptimer16b2ccsplp0Rstctl {
        &self.sysctl_mgmt_gptimer16b2ccsplp0_rstctl
    }
    #[doc = "0x8c814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2ccsplp0_stat(&self) -> &SysctlMgmtGptimer16b2ccsplp0Stat {
        &self.sysctl_mgmt_gptimer16b2ccsplp0_stat
    }
    #[doc = "0x8e800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2ccsplp1_pwren(&self) -> &SysctlMgmtGptimer16b2ccsplp1Pwren {
        &self.sysctl_mgmt_gptimer16b2ccsplp1_pwren
    }
    #[doc = "0x8e804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2ccsplp1_rstctl(
        &self,
    ) -> &SysctlMgmtGptimer16b2ccsplp1Rstctl {
        &self.sysctl_mgmt_gptimer16b2ccsplp1_rstctl
    }
    #[doc = "0x8e814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2ccsplp1_stat(&self) -> &SysctlMgmtGptimer16b2ccsplp1Stat {
        &self.sysctl_mgmt_gptimer16b2ccsplp1_stat
    }
    #[doc = "0x90800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2ccqeilp0_pwren(
        &self,
    ) -> &SysctlMgmtGptimer16b2ccqeilp0Pwren {
        &self.sysctl_mgmt_gptimer16b2ccqeilp0_pwren
    }
    #[doc = "0x90804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2ccqeilp0_rstctl(
        &self,
    ) -> &SysctlMgmtGptimer16b2ccqeilp0Rstctl {
        &self.sysctl_mgmt_gptimer16b2ccqeilp0_rstctl
    }
    #[doc = "0x90814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16b2ccqeilp0_stat(&self) -> &SysctlMgmtGptimer16b2ccqeilp0Stat {
        &self.sysctl_mgmt_gptimer16b2ccqeilp0_stat
    }
    #[doc = "0x94808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_rtc_clkcfg(&self) -> &SysctlMgmtRtcClkcfg {
        &self.sysctl_mgmt_rtc_clkcfg
    }
    #[doc = "0xa0800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gpio0_pwren(&self) -> &SysctlMgmtGpio0Pwren {
        &self.sysctl_mgmt_gpio0_pwren
    }
    #[doc = "0xa0804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gpio0_rstctl(&self) -> &SysctlMgmtGpio0Rstctl {
        &self.sysctl_mgmt_gpio0_rstctl
    }
    #[doc = "0xa0814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gpio0_stat(&self) -> &SysctlMgmtGpio0Stat {
        &self.sysctl_mgmt_gpio0_stat
    }
    #[doc = "0xa2800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gpio1_pwren(&self) -> &SysctlMgmtGpio1Pwren {
        &self.sysctl_mgmt_gpio1_pwren
    }
    #[doc = "0xa2804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gpio1_rstctl(&self) -> &SysctlMgmtGpio1Rstctl {
        &self.sysctl_mgmt_gpio1_rstctl
    }
    #[doc = "0xa2814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gpio1_stat(&self) -> &SysctlMgmtGpio1Stat {
        &self.sysctl_mgmt_gpio1_stat
    }
    #[doc = "0xa4800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gpio2_pwren(&self) -> &SysctlMgmtGpio2Pwren {
        &self.sysctl_mgmt_gpio2_pwren
    }
    #[doc = "0xa4804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gpio2_rstctl(&self) -> &SysctlMgmtGpio2Rstctl {
        &self.sysctl_mgmt_gpio2_rstctl
    }
    #[doc = "0xa4814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gpio2_stat(&self) -> &SysctlMgmtGpio2Stat {
        &self.sysctl_mgmt_gpio2_stat
    }
    #[doc = "0xf0800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c0_pwren(&self) -> &SysctlMgmtI2c0Pwren {
        &self.sysctl_mgmt_i2c0_pwren
    }
    #[doc = "0xf0804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c0_rstctl(&self) -> &SysctlMgmtI2c0Rstctl {
        &self.sysctl_mgmt_i2c0_rstctl
    }
    #[doc = "0xf0808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c0_clkcfg(&self) -> &SysctlMgmtI2c0Clkcfg {
        &self.sysctl_mgmt_i2c0_clkcfg
    }
    #[doc = "0xf0814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c0_stat(&self) -> &SysctlMgmtI2c0Stat {
        &self.sysctl_mgmt_i2c0_stat
    }
    #[doc = "0xf2800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c1_pwren(&self) -> &SysctlMgmtI2c1Pwren {
        &self.sysctl_mgmt_i2c1_pwren
    }
    #[doc = "0xf2804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c1_rstctl(&self) -> &SysctlMgmtI2c1Rstctl {
        &self.sysctl_mgmt_i2c1_rstctl
    }
    #[doc = "0xf2808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c1_clkcfg(&self) -> &SysctlMgmtI2c1Clkcfg {
        &self.sysctl_mgmt_i2c1_clkcfg
    }
    #[doc = "0xf2814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c1_stat(&self) -> &SysctlMgmtI2c1Stat {
        &self.sysctl_mgmt_i2c1_stat
    }
    #[doc = "0xf4800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c2_pwren(&self) -> &SysctlMgmtI2c2Pwren {
        &self.sysctl_mgmt_i2c2_pwren
    }
    #[doc = "0xf4804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c2_rstctl(&self) -> &SysctlMgmtI2c2Rstctl {
        &self.sysctl_mgmt_i2c2_rstctl
    }
    #[doc = "0xf4808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c2_clkcfg(&self) -> &SysctlMgmtI2c2Clkcfg {
        &self.sysctl_mgmt_i2c2_clkcfg
    }
    #[doc = "0xf4814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_i2c2_stat(&self) -> &SysctlMgmtI2c2Stat {
        &self.sysctl_mgmt_i2c2_stat
    }
    #[doc = "0x100800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp0_pwren(&self) -> &SysctlMgmtUartlp0Pwren {
        &self.sysctl_mgmt_uartlp0_pwren
    }
    #[doc = "0x100804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp0_rstctl(&self) -> &SysctlMgmtUartlp0Rstctl {
        &self.sysctl_mgmt_uartlp0_rstctl
    }
    #[doc = "0x100808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp0_clkcfg(&self) -> &SysctlMgmtUartlp0Clkcfg {
        &self.sysctl_mgmt_uartlp0_clkcfg
    }
    #[doc = "0x100814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp0_stat(&self) -> &SysctlMgmtUartlp0Stat {
        &self.sysctl_mgmt_uartlp0_stat
    }
    #[doc = "0x102800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp1_pwren(&self) -> &SysctlMgmtUartlp1Pwren {
        &self.sysctl_mgmt_uartlp1_pwren
    }
    #[doc = "0x102804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp1_rstctl(&self) -> &SysctlMgmtUartlp1Rstctl {
        &self.sysctl_mgmt_uartlp1_rstctl
    }
    #[doc = "0x102808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp1_clkcfg(&self) -> &SysctlMgmtUartlp1Clkcfg {
        &self.sysctl_mgmt_uartlp1_clkcfg
    }
    #[doc = "0x102814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp1_stat(&self) -> &SysctlMgmtUartlp1Stat {
        &self.sysctl_mgmt_uartlp1_stat
    }
    #[doc = "0x104800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp2_pwren(&self) -> &SysctlMgmtUartlp2Pwren {
        &self.sysctl_mgmt_uartlp2_pwren
    }
    #[doc = "0x104804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp2_rstctl(&self) -> &SysctlMgmtUartlp2Rstctl {
        &self.sysctl_mgmt_uartlp2_rstctl
    }
    #[doc = "0x104808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp2_clkcfg(&self) -> &SysctlMgmtUartlp2Clkcfg {
        &self.sysctl_mgmt_uartlp2_clkcfg
    }
    #[doc = "0x104814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartlp2_stat(&self) -> &SysctlMgmtUartlp2Stat {
        &self.sysctl_mgmt_uartlp2_stat
    }
    #[doc = "0x108800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartadvlp0_pwren(&self) -> &SysctlMgmtUartadvlp0Pwren {
        &self.sysctl_mgmt_uartadvlp0_pwren
    }
    #[doc = "0x108804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartadvlp0_rstctl(&self) -> &SysctlMgmtUartadvlp0Rstctl {
        &self.sysctl_mgmt_uartadvlp0_rstctl
    }
    #[doc = "0x108808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartadvlp0_clkcfg(&self) -> &SysctlMgmtUartadvlp0Clkcfg {
        &self.sysctl_mgmt_uartadvlp0_clkcfg
    }
    #[doc = "0x108814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartadvlp0_stat(&self) -> &SysctlMgmtUartadvlp0Stat {
        &self.sysctl_mgmt_uartadvlp0_stat
    }
    #[doc = "0x10a800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartadvlp1_pwren(&self) -> &SysctlMgmtUartadvlp1Pwren {
        &self.sysctl_mgmt_uartadvlp1_pwren
    }
    #[doc = "0x10a804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartadvlp1_rstctl(&self) -> &SysctlMgmtUartadvlp1Rstctl {
        &self.sysctl_mgmt_uartadvlp1_rstctl
    }
    #[doc = "0x10a808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartadvlp1_clkcfg(&self) -> &SysctlMgmtUartadvlp1Clkcfg {
        &self.sysctl_mgmt_uartadvlp1_clkcfg
    }
    #[doc = "0x10a814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_uartadvlp1_stat(&self) -> &SysctlMgmtUartadvlp1Stat {
        &self.sysctl_mgmt_uartadvlp1_stat
    }
    #[doc = "0x440800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_crc0_pwren(&self) -> &SysctlMgmtCrc0Pwren {
        &self.sysctl_mgmt_crc0_pwren
    }
    #[doc = "0x440804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_crc0_rstctl(&self) -> &SysctlMgmtCrc0Rstctl {
        &self.sysctl_mgmt_crc0_rstctl
    }
    #[doc = "0x440814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_crc0_stat(&self) -> &SysctlMgmtCrc0Stat {
        &self.sysctl_mgmt_crc0_stat
    }
    #[doc = "0x442800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_aes_pwren(&self) -> &SysctlMgmtAesPwren {
        &self.sysctl_mgmt_aes_pwren
    }
    #[doc = "0x442804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_aes_rstctl(&self) -> &SysctlMgmtAesRstctl {
        &self.sysctl_mgmt_aes_rstctl
    }
    #[doc = "0x442814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_aes_stat(&self) -> &SysctlMgmtAesStat {
        &self.sysctl_mgmt_aes_stat
    }
    #[doc = "0x444800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_trng_pwren(&self) -> &SysctlMgmtTrngPwren {
        &self.sysctl_mgmt_trng_pwren
    }
    #[doc = "0x444804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_trng_rstctl(&self) -> &SysctlMgmtTrngRstctl {
        &self.sysctl_mgmt_trng_rstctl
    }
    #[doc = "0x444814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_trng_stat(&self) -> &SysctlMgmtTrngStat {
        &self.sysctl_mgmt_trng_stat
    }
    #[doc = "0x468800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_spi0_pwren(&self) -> &SysctlMgmtSpi0Pwren {
        &self.sysctl_mgmt_spi0_pwren
    }
    #[doc = "0x468804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_spi0_rstctl(&self) -> &SysctlMgmtSpi0Rstctl {
        &self.sysctl_mgmt_spi0_rstctl
    }
    #[doc = "0x468808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_spi0_clkcfg(&self) -> &SysctlMgmtSpi0Clkcfg {
        &self.sysctl_mgmt_spi0_clkcfg
    }
    #[doc = "0x468814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_spi0_stat(&self) -> &SysctlMgmtSpi0Stat {
        &self.sysctl_mgmt_spi0_stat
    }
    #[doc = "0x46a800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_spi1_pwren(&self) -> &SysctlMgmtSpi1Pwren {
        &self.sysctl_mgmt_spi1_pwren
    }
    #[doc = "0x46a804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_spi1_rstctl(&self) -> &SysctlMgmtSpi1Rstctl {
        &self.sysctl_mgmt_spi1_rstctl
    }
    #[doc = "0x46a808 - IP Clock Configuration Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_spi1_clkcfg(&self) -> &SysctlMgmtSpi1Clkcfg {
        &self.sysctl_mgmt_spi1_clkcfg
    }
    #[doc = "0x46a814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_spi1_stat(&self) -> &SysctlMgmtSpi1Stat {
        &self.sysctl_mgmt_spi1_stat
    }
    #[doc = "0x860800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16badv4cc0_pwren(&self) -> &SysctlMgmtGptimer16badv4cc0Pwren {
        &self.sysctl_mgmt_gptimer16badv4cc0_pwren
    }
    #[doc = "0x860804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16badv4cc0_rstctl(&self) -> &SysctlMgmtGptimer16badv4cc0Rstctl {
        &self.sysctl_mgmt_gptimer16badv4cc0_rstctl
    }
    #[doc = "0x860814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer16badv4cc0_stat(&self) -> &SysctlMgmtGptimer16badv4cc0Stat {
        &self.sysctl_mgmt_gptimer16badv4cc0_stat
    }
    #[doc = "0x870800 - IP Enable Register"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer32b2cc0_pwren(&self) -> &SysctlMgmtGptimer32b2cc0Pwren {
        &self.sysctl_mgmt_gptimer32b2cc0_pwren
    }
    #[doc = "0x870804 - Power Control Register - Write Only Register, Always Read as 0"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer32b2cc0_rstctl(&self) -> &SysctlMgmtGptimer32b2cc0Rstctl {
        &self.sysctl_mgmt_gptimer32b2cc0_rstctl
    }
    #[doc = "0x870814 - IP State Register - Read Only"]
    #[inline(always)]
    pub const fn sysctl_mgmt_gptimer32b2cc0_stat(&self) -> &SysctlMgmtGptimer32b2cc0Stat {
        &self.sysctl_mgmt_gptimer32b2cc0_stat
    }
}
#[doc = "SYSCTL_IIDX (r) register accessor: SYSCTL interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_iidx`] module"]
#[doc(alias = "SYSCTL_IIDX")]
pub type SysctlIidx = crate::Reg<sysctl_iidx::SysctlIidxSpec>;
#[doc = "SYSCTL interrupt index"]
pub mod sysctl_iidx;
#[doc = "SYSCTL_IMASK (rw) register accessor: SYSCTL interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_imask`] module"]
#[doc(alias = "SYSCTL_IMASK")]
pub type SysctlImask = crate::Reg<sysctl_imask::SysctlImaskSpec>;
#[doc = "SYSCTL interrupt mask"]
pub mod sysctl_imask;
#[doc = "SYSCTL_RIS (r) register accessor: SYSCTL raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_ris`] module"]
#[doc(alias = "SYSCTL_RIS")]
pub type SysctlRis = crate::Reg<sysctl_ris::SysctlRisSpec>;
#[doc = "SYSCTL raw interrupt status"]
pub mod sysctl_ris;
#[doc = "SYSCTL_MIS (r) register accessor: SYSCTL masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mis`] module"]
#[doc(alias = "SYSCTL_MIS")]
pub type SysctlMis = crate::Reg<sysctl_mis::SysctlMisSpec>;
#[doc = "SYSCTL masked interrupt status"]
pub mod sysctl_mis;
#[doc = "SYSCTL_ISET (w) register accessor: SYSCTL interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_iset`] module"]
#[doc(alias = "SYSCTL_ISET")]
pub type SysctlIset = crate::Reg<sysctl_iset::SysctlIsetSpec>;
#[doc = "SYSCTL interrupt set"]
pub mod sysctl_iset;
#[doc = "SYSCTL_ICLR (w) register accessor: SYSCTL interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_iclr`] module"]
#[doc(alias = "SYSCTL_ICLR")]
pub type SysctlIclr = crate::Reg<sysctl_iclr::SysctlIclrSpec>;
#[doc = "SYSCTL interrupt clear"]
pub mod sysctl_iclr;
#[doc = "SYSCTL_NMIIIDX (r) register accessor: NMI interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_nmiiidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_nmiiidx`] module"]
#[doc(alias = "SYSCTL_NMIIIDX")]
pub type SysctlNmiiidx = crate::Reg<sysctl_nmiiidx::SysctlNmiiidxSpec>;
#[doc = "NMI interrupt index"]
pub mod sysctl_nmiiidx;
#[doc = "SYSCTL_NMIRIS (r) register accessor: NMI raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_nmiris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_nmiris`] module"]
#[doc(alias = "SYSCTL_NMIRIS")]
pub type SysctlNmiris = crate::Reg<sysctl_nmiris::SysctlNmirisSpec>;
#[doc = "NMI raw interrupt status"]
pub mod sysctl_nmiris;
#[doc = "SYSCTL_NMIISET (w) register accessor: NMI interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_nmiiset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_nmiiset`] module"]
#[doc(alias = "SYSCTL_NMIISET")]
pub type SysctlNmiiset = crate::Reg<sysctl_nmiiset::SysctlNmiisetSpec>;
#[doc = "NMI interrupt set"]
pub mod sysctl_nmiiset;
#[doc = "SYSCTL_NMIICLR (w) register accessor: NMI interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_nmiiclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_nmiiclr`] module"]
#[doc(alias = "SYSCTL_NMIICLR")]
pub type SysctlNmiiclr = crate::Reg<sysctl_nmiiclr::SysctlNmiiclrSpec>;
#[doc = "NMI interrupt clear"]
pub mod sysctl_nmiiclr;
#[doc = "SYSCTL_SYSOSCCFG (rw) register accessor: SYSOSC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_sysosccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_sysosccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_sysosccfg`] module"]
#[doc(alias = "SYSCTL_SYSOSCCFG")]
pub type SysctlSysosccfg = crate::Reg<sysctl_sysosccfg::SysctlSysosccfgSpec>;
#[doc = "SYSOSC configuration"]
pub mod sysctl_sysosccfg;
#[doc = "SYSCTL_MCLKCFG (rw) register accessor: Main clock (MCLK) configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mclkcfg`] module"]
#[doc(alias = "SYSCTL_MCLKCFG")]
pub type SysctlMclkcfg = crate::Reg<sysctl_mclkcfg::SysctlMclkcfgSpec>;
#[doc = "Main clock (MCLK) configuration"]
pub mod sysctl_mclkcfg;
#[doc = "SYSCTL_HSCLKEN (rw) register accessor: High-speed clock (HSCLK) source enable/disable\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_hsclken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_hsclken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_hsclken`] module"]
#[doc(alias = "SYSCTL_HSCLKEN")]
pub type SysctlHsclken = crate::Reg<sysctl_hsclken::SysctlHsclkenSpec>;
#[doc = "High-speed clock (HSCLK) source enable/disable"]
pub mod sysctl_hsclken;
#[doc = "SYSCTL_HSCLKCFG (rw) register accessor: High-speed clock (HSCLK) source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_hsclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_hsclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_hsclkcfg`] module"]
#[doc(alias = "SYSCTL_HSCLKCFG")]
pub type SysctlHsclkcfg = crate::Reg<sysctl_hsclkcfg::SysctlHsclkcfgSpec>;
#[doc = "High-speed clock (HSCLK) source selection"]
pub mod sysctl_hsclkcfg;
#[doc = "SYSCTL_HFCLKCLKCFG (rw) register accessor: High-frequency clock (HFCLK) configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_hfclkclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_hfclkclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_hfclkclkcfg`] module"]
#[doc(alias = "SYSCTL_HFCLKCLKCFG")]
pub type SysctlHfclkclkcfg = crate::Reg<sysctl_hfclkclkcfg::SysctlHfclkclkcfgSpec>;
#[doc = "High-frequency clock (HFCLK) configuration"]
pub mod sysctl_hfclkclkcfg;
#[doc = "SYSCTL_LFCLKCFG (rw) register accessor: Low frequency crystal oscillator (LFXT) configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_lfclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_lfclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_lfclkcfg`] module"]
#[doc(alias = "SYSCTL_LFCLKCFG")]
pub type SysctlLfclkcfg = crate::Reg<sysctl_lfclkcfg::SysctlLfclkcfgSpec>;
#[doc = "Low frequency crystal oscillator (LFXT) configuration"]
pub mod sysctl_lfclkcfg;
#[doc = "SYSCTL_GENCLKCFG (rw) register accessor: General clock configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_genclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_genclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_genclkcfg`] module"]
#[doc(alias = "SYSCTL_GENCLKCFG")]
pub type SysctlGenclkcfg = crate::Reg<sysctl_genclkcfg::SysctlGenclkcfgSpec>;
#[doc = "General clock configuration"]
pub mod sysctl_genclkcfg;
#[doc = "SYSCTL_GENCLKEN (rw) register accessor: General clock enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_genclken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_genclken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_genclken`] module"]
#[doc(alias = "SYSCTL_GENCLKEN")]
pub type SysctlGenclken = crate::Reg<sysctl_genclken::SysctlGenclkenSpec>;
#[doc = "General clock enable control"]
pub mod sysctl_genclken;
#[doc = "SYSCTL_PMODECFG (rw) register accessor: Power mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_pmodecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_pmodecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_pmodecfg`] module"]
#[doc(alias = "SYSCTL_PMODECFG")]
pub type SysctlPmodecfg = crate::Reg<sysctl_pmodecfg::SysctlPmodecfgSpec>;
#[doc = "Power mode configuration"]
pub mod sysctl_pmodecfg;
#[doc = "SYSCTL_FCC (r) register accessor: Frequency clock counter (FCC) count\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_fcc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_fcc`] module"]
#[doc(alias = "SYSCTL_FCC")]
pub type SysctlFcc = crate::Reg<sysctl_fcc::SysctlFccSpec>;
#[doc = "Frequency clock counter (FCC) count"]
pub mod sysctl_fcc;
#[doc = "SYSCTL_SYSOSCTRIMUSER (rw) register accessor: SYSOSC user-specified trim\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_sysosctrimuser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_sysosctrimuser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_sysosctrimuser`] module"]
#[doc(alias = "SYSCTL_SYSOSCTRIMUSER")]
pub type SysctlSysosctrimuser = crate::Reg<sysctl_sysosctrimuser::SysctlSysosctrimuserSpec>;
#[doc = "SYSOSC user-specified trim"]
pub mod sysctl_sysosctrimuser;
#[doc = "SYSCTL_SRAMBOUNDARY (rw) register accessor: SRAM Write Boundary\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_sramboundary::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_sramboundary::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_sramboundary`] module"]
#[doc(alias = "SYSCTL_SRAMBOUNDARY")]
pub type SysctlSramboundary = crate::Reg<sysctl_sramboundary::SysctlSramboundarySpec>;
#[doc = "SRAM Write Boundary"]
pub mod sysctl_sramboundary;
#[doc = "SYSCTL_SYSTEMCFG (rw) register accessor: System configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_systemcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_systemcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_systemcfg`] module"]
#[doc(alias = "SYSCTL_SYSTEMCFG")]
pub type SysctlSystemcfg = crate::Reg<sysctl_systemcfg::SysctlSystemcfgSpec>;
#[doc = "System configuration"]
pub mod sysctl_systemcfg;
#[doc = "SYSCTL_WRITELOCK (rw) register accessor: SYSCTL register write lockout\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_writelock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_writelock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_writelock`] module"]
#[doc(alias = "SYSCTL_WRITELOCK")]
pub type SysctlWritelock = crate::Reg<sysctl_writelock::SysctlWritelockSpec>;
#[doc = "SYSCTL register write lockout"]
pub mod sysctl_writelock;
#[doc = "SYSCTL_CLKSTATUS (r) register accessor: Clock module (CKM) status\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_clkstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_clkstatus`] module"]
#[doc(alias = "SYSCTL_CLKSTATUS")]
pub type SysctlClkstatus = crate::Reg<sysctl_clkstatus::SysctlClkstatusSpec>;
#[doc = "Clock module (CKM) status"]
pub mod sysctl_clkstatus;
#[doc = "SYSCTL_SYSSTATUS (r) register accessor: System status information\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_sysstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_sysstatus`] module"]
#[doc(alias = "SYSCTL_SYSSTATUS")]
pub type SysctlSysstatus = crate::Reg<sysctl_sysstatus::SysctlSysstatusSpec>;
#[doc = "System status information"]
pub mod sysctl_sysstatus;
#[doc = "SYSCTL_DEDERRADDR (r) register accessor: Memory DED Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_dederraddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_dederraddr`] module"]
#[doc(alias = "SYSCTL_DEDERRADDR")]
pub type SysctlDederraddr = crate::Reg<sysctl_dederraddr::SysctlDederraddrSpec>;
#[doc = "Memory DED Address"]
pub mod sysctl_dederraddr;
#[doc = "SYSCTL_RSTCAUSE (r) register accessor: Reset cause\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_rstcause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_rstcause`] module"]
#[doc(alias = "SYSCTL_RSTCAUSE")]
pub type SysctlRstcause = crate::Reg<sysctl_rstcause::SysctlRstcauseSpec>;
#[doc = "Reset cause"]
pub mod sysctl_rstcause;
#[doc = "SYSCTL_RESETLEVEL (rw) register accessor: Reset level for application-triggered reset command\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_resetlevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_resetlevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_resetlevel`] module"]
#[doc(alias = "SYSCTL_RESETLEVEL")]
pub type SysctlResetlevel = crate::Reg<sysctl_resetlevel::SysctlResetlevelSpec>;
#[doc = "Reset level for application-triggered reset command"]
pub mod sysctl_resetlevel;
#[doc = "SYSCTL_RESETCMD (w) register accessor: Execute an application-triggered reset command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_resetcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_resetcmd`] module"]
#[doc(alias = "SYSCTL_RESETCMD")]
pub type SysctlResetcmd = crate::Reg<sysctl_resetcmd::SysctlResetcmdSpec>;
#[doc = "Execute an application-triggered reset command"]
pub mod sysctl_resetcmd;
#[doc = "SYSCTL_BORTHRESHOLD (rw) register accessor: BOR threshold selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_borthreshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_borthreshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_borthreshold`] module"]
#[doc(alias = "SYSCTL_BORTHRESHOLD")]
pub type SysctlBorthreshold = crate::Reg<sysctl_borthreshold::SysctlBorthresholdSpec>;
#[doc = "BOR threshold selection"]
pub mod sysctl_borthreshold;
#[doc = "SYSCTL_BORCLRCMD (w) register accessor: Set the BOR threshold\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_borclrcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_borclrcmd`] module"]
#[doc(alias = "SYSCTL_BORCLRCMD")]
pub type SysctlBorclrcmd = crate::Reg<sysctl_borclrcmd::SysctlBorclrcmdSpec>;
#[doc = "Set the BOR threshold"]
pub mod sysctl_borclrcmd;
#[doc = "SYSCTL_SYSOSCFCLCTL (w) register accessor: SYSOSC frequency correction loop (FCL) ROSC enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_sysoscfclctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_sysoscfclctl`] module"]
#[doc(alias = "SYSCTL_SYSOSCFCLCTL")]
pub type SysctlSysoscfclctl = crate::Reg<sysctl_sysoscfclctl::SysctlSysoscfclctlSpec>;
#[doc = "SYSOSC frequency correction loop (FCL) ROSC enable"]
pub mod sysctl_sysoscfclctl;
#[doc = "SYSCTL_LFXTCTL (w) register accessor: LFXT and LFCLK control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_lfxtctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_lfxtctl`] module"]
#[doc(alias = "SYSCTL_LFXTCTL")]
pub type SysctlLfxtctl = crate::Reg<sysctl_lfxtctl::SysctlLfxtctlSpec>;
#[doc = "LFXT and LFCLK control"]
pub mod sysctl_lfxtctl;
#[doc = "SYSCTL_EXLFCTL (w) register accessor: LFCLK_IN and LFCLK control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_exlfctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_exlfctl`] module"]
#[doc(alias = "SYSCTL_EXLFCTL")]
pub type SysctlExlfctl = crate::Reg<sysctl_exlfctl::SysctlExlfctlSpec>;
#[doc = "LFCLK_IN and LFCLK control"]
pub mod sysctl_exlfctl;
#[doc = "SYSCTL_SHDNIOREL (w) register accessor: SHUTDOWN IO release control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_shdniorel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_shdniorel`] module"]
#[doc(alias = "SYSCTL_SHDNIOREL")]
pub type SysctlShdniorel = crate::Reg<sysctl_shdniorel::SysctlShdniorelSpec>;
#[doc = "SHUTDOWN IO release control"]
pub mod sysctl_shdniorel;
#[doc = "SYSCTL_EXRSTPIN (w) register accessor: Disable the reset function of the NRST pin\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_exrstpin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_exrstpin`] module"]
#[doc(alias = "SYSCTL_EXRSTPIN")]
pub type SysctlExrstpin = crate::Reg<sysctl_exrstpin::SysctlExrstpinSpec>;
#[doc = "Disable the reset function of the NRST pin"]
pub mod sysctl_exrstpin;
#[doc = "SYSCTL_SYSSTATUSCLR (w) register accessor: Clear sticky bits of SYSSTATUS\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_sysstatusclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_sysstatusclr`] module"]
#[doc(alias = "SYSCTL_SYSSTATUSCLR")]
pub type SysctlSysstatusclr = crate::Reg<sysctl_sysstatusclr::SysctlSysstatusclrSpec>;
#[doc = "Clear sticky bits of SYSSTATUS"]
pub mod sysctl_sysstatusclr;
#[doc = "SYSCTL_SWDCFG (w) register accessor: Disable the SWD function on the SWD pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_swdcfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_swdcfg`] module"]
#[doc(alias = "SYSCTL_SWDCFG")]
pub type SysctlSwdcfg = crate::Reg<sysctl_swdcfg::SysctlSwdcfgSpec>;
#[doc = "Disable the SWD function on the SWD pins"]
pub mod sysctl_swdcfg;
#[doc = "SYSCTL_FCCCMD (w) register accessor: Frequency clock counter start capture\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_fcccmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_fcccmd`] module"]
#[doc(alias = "SYSCTL_FCCCMD")]
pub type SysctlFcccmd = crate::Reg<sysctl_fcccmd::SysctlFcccmdSpec>;
#[doc = "Frequency clock counter start capture"]
pub mod sysctl_fcccmd;
#[doc = "SYSCTL_SHUTDNSTORE0 (rw) register accessor: Shutdown storage memory (byte 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_shutdnstore0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_shutdnstore0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_shutdnstore0`] module"]
#[doc(alias = "SYSCTL_SHUTDNSTORE0")]
pub type SysctlShutdnstore0 = crate::Reg<sysctl_shutdnstore0::SysctlShutdnstore0Spec>;
#[doc = "Shutdown storage memory (byte 0)"]
pub mod sysctl_shutdnstore0;
#[doc = "SYSCTL_SHUTDNSTORE1 (rw) register accessor: Shutdown storage memory (byte 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_shutdnstore1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_shutdnstore1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_shutdnstore1`] module"]
#[doc(alias = "SYSCTL_SHUTDNSTORE1")]
pub type SysctlShutdnstore1 = crate::Reg<sysctl_shutdnstore1::SysctlShutdnstore1Spec>;
#[doc = "Shutdown storage memory (byte 1)"]
pub mod sysctl_shutdnstore1;
#[doc = "SYSCTL_SHUTDNSTORE2 (rw) register accessor: Shutdown storage memory (byte 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_shutdnstore2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_shutdnstore2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_shutdnstore2`] module"]
#[doc(alias = "SYSCTL_SHUTDNSTORE2")]
pub type SysctlShutdnstore2 = crate::Reg<sysctl_shutdnstore2::SysctlShutdnstore2Spec>;
#[doc = "Shutdown storage memory (byte 2)"]
pub mod sysctl_shutdnstore2;
#[doc = "SYSCTL_SHUTDNSTORE3 (rw) register accessor: Shutdown storage memory (byte 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_shutdnstore3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_shutdnstore3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_shutdnstore3`] module"]
#[doc(alias = "SYSCTL_SHUTDNSTORE3")]
pub type SysctlShutdnstore3 = crate::Reg<sysctl_shutdnstore3::SysctlShutdnstore3Spec>;
#[doc = "Shutdown storage memory (byte 3)"]
pub mod sysctl_shutdnstore3;
#[doc = "SYSCTL_FWEPROTMAIN (rw) register accessor: 1 Sector Write-Erase per bit starting at address 0x0 of flash\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_fweprotmain::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_fweprotmain::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_fweprotmain`] module"]
#[doc(alias = "SYSCTL_FWEPROTMAIN")]
pub type SysctlFweprotmain = crate::Reg<sysctl_fweprotmain::SysctlFweprotmainSpec>;
#[doc = "1 Sector Write-Erase per bit starting at address 0x0 of flash"]
pub mod sysctl_fweprotmain;
#[doc = "SYSCTL_FRXPROTMAINSTART (rw) register accessor: Flash RX Protection Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_frxprotmainstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_frxprotmainstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_frxprotmainstart`] module"]
#[doc(alias = "SYSCTL_FRXPROTMAINSTART")]
pub type SysctlFrxprotmainstart = crate::Reg<sysctl_frxprotmainstart::SysctlFrxprotmainstartSpec>;
#[doc = "Flash RX Protection Start Address"]
pub mod sysctl_frxprotmainstart;
#[doc = "SYSCTL_FRXPROTMAINEND (rw) register accessor: Flash RX Protection End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_frxprotmainend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_frxprotmainend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_frxprotmainend`] module"]
#[doc(alias = "SYSCTL_FRXPROTMAINEND")]
pub type SysctlFrxprotmainend = crate::Reg<sysctl_frxprotmainend::SysctlFrxprotmainendSpec>;
#[doc = "Flash RX Protection End Address"]
pub mod sysctl_frxprotmainend;
#[doc = "SYSCTL_FIPPROTMAINSTART (rw) register accessor: Flash IP Protection Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_fipprotmainstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_fipprotmainstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_fipprotmainstart`] module"]
#[doc(alias = "SYSCTL_FIPPROTMAINSTART")]
pub type SysctlFipprotmainstart = crate::Reg<sysctl_fipprotmainstart::SysctlFipprotmainstartSpec>;
#[doc = "Flash IP Protection Start Address"]
pub mod sysctl_fipprotmainstart;
#[doc = "SYSCTL_FIPPROTMAINEND (rw) register accessor: Flash IP Protection End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_fipprotmainend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_fipprotmainend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_fipprotmainend`] module"]
#[doc(alias = "SYSCTL_FIPPROTMAINEND")]
pub type SysctlFipprotmainend = crate::Reg<sysctl_fipprotmainend::SysctlFipprotmainendSpec>;
#[doc = "Flash IP Protection End Address"]
pub mod sysctl_fipprotmainend;
#[doc = "SYSCTL_FLBANKSWPPOLICY (w) register accessor: Flash Bank Swap Policy\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_flbankswppolicy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_flbankswppolicy`] module"]
#[doc(alias = "SYSCTL_FLBANKSWPPOLICY")]
pub type SysctlFlbankswppolicy = crate::Reg<sysctl_flbankswppolicy::SysctlFlbankswppolicySpec>;
#[doc = "Flash Bank Swap Policy"]
pub mod sysctl_flbankswppolicy;
#[doc = "SYSCTL_FLBANKSWP (w) register accessor: Flash MAIN bank address swap\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_flbankswp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_flbankswp`] module"]
#[doc(alias = "SYSCTL_FLBANKSWP")]
pub type SysctlFlbankswp = crate::Reg<sysctl_flbankswp::SysctlFlbankswpSpec>;
#[doc = "Flash MAIN bank address swap"]
pub mod sysctl_flbankswp;
#[doc = "SYSCTL_FWENABLE (w) register accessor: Security Firewall Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_fwenable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_fwenable`] module"]
#[doc(alias = "SYSCTL_FWENABLE")]
pub type SysctlFwenable = crate::Reg<sysctl_fwenable::SysctlFwenableSpec>;
#[doc = "Security Firewall Enable Register"]
pub mod sysctl_fwenable;
#[doc = "SYSCTL_SECSTATUS (r) register accessor: Security Configuration status\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_secstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_secstatus`] module"]
#[doc(alias = "SYSCTL_SECSTATUS")]
pub type SysctlSecstatus = crate::Reg<sysctl_secstatus::SysctlSecstatusSpec>;
#[doc = "Security Configuration status"]
pub mod sysctl_secstatus;
#[doc = "SYSCTL_INITDONE (w) register accessor: INITCODE PASS\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_initdone::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_initdone`] module"]
#[doc(alias = "SYSCTL_INITDONE")]
pub type SysctlInitdone = crate::Reg<sysctl_initdone::SysctlInitdoneSpec>;
#[doc = "INITCODE PASS"]
pub mod sysctl_initdone;
#[doc = "SYSCTL_MGMT_ADC12B1MSPS0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_adc12b1msps0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_adc12b1msps0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_adc12b1msps0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_ADC12B1MSPS0_PWREN")]
pub type SysctlMgmtAdc12b1msps0Pwren =
    crate::Reg<sysctl_mgmt_adc12b1msps0_pwren::SysctlMgmtAdc12b1msps0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_adc12b1msps0_pwren;
#[doc = "SYSCTL_MGMT_ADC12B1MSPS0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_adc12b1msps0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_adc12b1msps0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_ADC12B1MSPS0_RSTCTL")]
pub type SysctlMgmtAdc12b1msps0Rstctl =
    crate::Reg<sysctl_mgmt_adc12b1msps0_rstctl::SysctlMgmtAdc12b1msps0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_adc12b1msps0_rstctl;
#[doc = "SYSCTL_MGMT_ADC12B1MSPS0_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_adc12b1msps0_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_adc12b1msps0_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_adc12b1msps0_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_ADC12B1MSPS0_CLKCFG")]
pub type SysctlMgmtAdc12b1msps0Clkcfg =
    crate::Reg<sysctl_mgmt_adc12b1msps0_clkcfg::SysctlMgmtAdc12b1msps0ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_adc12b1msps0_clkcfg;
#[doc = "SYSCTL_MGMT_ADC12B1MSPS0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_adc12b1msps0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_adc12b1msps0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_ADC12B1MSPS0_STAT")]
pub type SysctlMgmtAdc12b1msps0Stat =
    crate::Reg<sysctl_mgmt_adc12b1msps0_stat::SysctlMgmtAdc12b1msps0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_adc12b1msps0_stat;
#[doc = "SYSCTL_MGMT_ANACOMP0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_anacomp0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_anacomp0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_anacomp0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_ANACOMP0_PWREN")]
pub type SysctlMgmtAnacomp0Pwren =
    crate::Reg<sysctl_mgmt_anacomp0_pwren::SysctlMgmtAnacomp0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_anacomp0_pwren;
#[doc = "SYSCTL_MGMT_ANACOMP0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_anacomp0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_anacomp0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_ANACOMP0_RSTCTL")]
pub type SysctlMgmtAnacomp0Rstctl =
    crate::Reg<sysctl_mgmt_anacomp0_rstctl::SysctlMgmtAnacomp0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_anacomp0_rstctl;
#[doc = "SYSCTL_MGMT_ANACOMP0_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_anacomp0_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_anacomp0_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_anacomp0_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_ANACOMP0_CLKCFG")]
pub type SysctlMgmtAnacomp0Clkcfg =
    crate::Reg<sysctl_mgmt_anacomp0_clkcfg::SysctlMgmtAnacomp0ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_anacomp0_clkcfg;
#[doc = "SYSCTL_MGMT_ANACOMP0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_anacomp0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_anacomp0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_ANACOMP0_STAT")]
pub type SysctlMgmtAnacomp0Stat = crate::Reg<sysctl_mgmt_anacomp0_stat::SysctlMgmtAnacomp0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_anacomp0_stat;
#[doc = "SYSCTL_MGMT_VREF_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_vref_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_vref_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_vref_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_VREF_PWREN")]
pub type SysctlMgmtVrefPwren = crate::Reg<sysctl_mgmt_vref_pwren::SysctlMgmtVrefPwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_vref_pwren;
#[doc = "SYSCTL_MGMT_VREF_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_vref_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_vref_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_VREF_RSTCTL")]
pub type SysctlMgmtVrefRstctl = crate::Reg<sysctl_mgmt_vref_rstctl::SysctlMgmtVrefRstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_vref_rstctl;
#[doc = "SYSCTL_MGMT_VREF_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_vref_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_vref_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_VREF_STAT")]
pub type SysctlMgmtVrefStat = crate::Reg<sysctl_mgmt_vref_stat::SysctlMgmtVrefStatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_vref_stat;
#[doc = "SYSCTL_MGMT_LCD_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_lcd_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_lcd_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_lcd_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_LCD_PWREN")]
pub type SysctlMgmtLcdPwren = crate::Reg<sysctl_mgmt_lcd_pwren::SysctlMgmtLcdPwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_lcd_pwren;
#[doc = "SYSCTL_MGMT_LCD_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_lcd_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_lcd_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_LCD_RSTCTL")]
pub type SysctlMgmtLcdRstctl = crate::Reg<sysctl_mgmt_lcd_rstctl::SysctlMgmtLcdRstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_lcd_rstctl;
#[doc = "SYSCTL_MGMT_LCD_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_lcd_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_lcd_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_LCD_STAT")]
pub type SysctlMgmtLcdStat = crate::Reg<sysctl_mgmt_lcd_stat::SysctlMgmtLcdStatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_lcd_stat;
#[doc = "SYSCTL_MGMT_WWDTLP0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_wwdtlp0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_wwdtlp0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_wwdtlp0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_WWDTLP0_PWREN")]
pub type SysctlMgmtWwdtlp0Pwren = crate::Reg<sysctl_mgmt_wwdtlp0_pwren::SysctlMgmtWwdtlp0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_wwdtlp0_pwren;
#[doc = "SYSCTL_MGMT_WWDTLP0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_wwdtlp0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_wwdtlp0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_WWDTLP0_RSTCTL")]
pub type SysctlMgmtWwdtlp0Rstctl =
    crate::Reg<sysctl_mgmt_wwdtlp0_rstctl::SysctlMgmtWwdtlp0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_wwdtlp0_rstctl;
#[doc = "SYSCTL_MGMT_WWDTLP0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_wwdtlp0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_wwdtlp0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_WWDTLP0_STAT")]
pub type SysctlMgmtWwdtlp0Stat = crate::Reg<sysctl_mgmt_wwdtlp0_stat::SysctlMgmtWwdtlp0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_wwdtlp0_stat;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCLP0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16b2cclp0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer16b2cclp0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2cclp0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCLP0_PWREN")]
pub type SysctlMgmtGptimer16b2cclp0Pwren =
    crate::Reg<sysctl_mgmt_gptimer16b2cclp0_pwren::SysctlMgmtGptimer16b2cclp0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_gptimer16b2cclp0_pwren;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCLP0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer16b2cclp0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2cclp0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCLP0_RSTCTL")]
pub type SysctlMgmtGptimer16b2cclp0Rstctl =
    crate::Reg<sysctl_mgmt_gptimer16b2cclp0_rstctl::SysctlMgmtGptimer16b2cclp0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_gptimer16b2cclp0_rstctl;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCLP0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16b2cclp0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2cclp0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCLP0_STAT")]
pub type SysctlMgmtGptimer16b2cclp0Stat =
    crate::Reg<sysctl_mgmt_gptimer16b2cclp0_stat::SysctlMgmtGptimer16b2cclp0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_gptimer16b2cclp0_stat;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCSPLP0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16b2ccsplp0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer16b2ccsplp0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2ccsplp0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCSPLP0_PWREN")]
pub type SysctlMgmtGptimer16b2ccsplp0Pwren =
    crate::Reg<sysctl_mgmt_gptimer16b2ccsplp0_pwren::SysctlMgmtGptimer16b2ccsplp0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_gptimer16b2ccsplp0_pwren;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCSPLP0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer16b2ccsplp0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2ccsplp0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCSPLP0_RSTCTL")]
pub type SysctlMgmtGptimer16b2ccsplp0Rstctl =
    crate::Reg<sysctl_mgmt_gptimer16b2ccsplp0_rstctl::SysctlMgmtGptimer16b2ccsplp0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_gptimer16b2ccsplp0_rstctl;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCSPLP0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16b2ccsplp0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2ccsplp0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCSPLP0_STAT")]
pub type SysctlMgmtGptimer16b2ccsplp0Stat =
    crate::Reg<sysctl_mgmt_gptimer16b2ccsplp0_stat::SysctlMgmtGptimer16b2ccsplp0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_gptimer16b2ccsplp0_stat;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCSPLP1_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16b2ccsplp1_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer16b2ccsplp1_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2ccsplp1_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCSPLP1_PWREN")]
pub type SysctlMgmtGptimer16b2ccsplp1Pwren =
    crate::Reg<sysctl_mgmt_gptimer16b2ccsplp1_pwren::SysctlMgmtGptimer16b2ccsplp1PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_gptimer16b2ccsplp1_pwren;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCSPLP1_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer16b2ccsplp1_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2ccsplp1_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCSPLP1_RSTCTL")]
pub type SysctlMgmtGptimer16b2ccsplp1Rstctl =
    crate::Reg<sysctl_mgmt_gptimer16b2ccsplp1_rstctl::SysctlMgmtGptimer16b2ccsplp1RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_gptimer16b2ccsplp1_rstctl;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCSPLP1_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16b2ccsplp1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2ccsplp1_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCSPLP1_STAT")]
pub type SysctlMgmtGptimer16b2ccsplp1Stat =
    crate::Reg<sysctl_mgmt_gptimer16b2ccsplp1_stat::SysctlMgmtGptimer16b2ccsplp1StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_gptimer16b2ccsplp1_stat;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCQEILP0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16b2ccqeilp0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer16b2ccqeilp0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2ccqeilp0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCQEILP0_PWREN")]
pub type SysctlMgmtGptimer16b2ccqeilp0Pwren =
    crate::Reg<sysctl_mgmt_gptimer16b2ccqeilp0_pwren::SysctlMgmtGptimer16b2ccqeilp0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_gptimer16b2ccqeilp0_pwren;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCQEILP0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer16b2ccqeilp0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2ccqeilp0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCQEILP0_RSTCTL")]
pub type SysctlMgmtGptimer16b2ccqeilp0Rstctl =
    crate::Reg<sysctl_mgmt_gptimer16b2ccqeilp0_rstctl::SysctlMgmtGptimer16b2ccqeilp0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_gptimer16b2ccqeilp0_rstctl;
#[doc = "SYSCTL_MGMT_GPTIMER16B2CCQEILP0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16b2ccqeilp0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16b2ccqeilp0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16B2CCQEILP0_STAT")]
pub type SysctlMgmtGptimer16b2ccqeilp0Stat =
    crate::Reg<sysctl_mgmt_gptimer16b2ccqeilp0_stat::SysctlMgmtGptimer16b2ccqeilp0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_gptimer16b2ccqeilp0_stat;
#[doc = "SYSCTL_MGMT_RTC_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_rtc_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_rtc_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_rtc_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_RTC_CLKCFG")]
pub type SysctlMgmtRtcClkcfg = crate::Reg<sysctl_mgmt_rtc_clkcfg::SysctlMgmtRtcClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_rtc_clkcfg;
#[doc = "SYSCTL_MGMT_GPIO0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gpio0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gpio0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gpio0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_GPIO0_PWREN")]
pub type SysctlMgmtGpio0Pwren = crate::Reg<sysctl_mgmt_gpio0_pwren::SysctlMgmtGpio0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_gpio0_pwren;
#[doc = "SYSCTL_MGMT_GPIO0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gpio0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gpio0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_GPIO0_RSTCTL")]
pub type SysctlMgmtGpio0Rstctl = crate::Reg<sysctl_mgmt_gpio0_rstctl::SysctlMgmtGpio0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_gpio0_rstctl;
#[doc = "SYSCTL_MGMT_GPIO0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gpio0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gpio0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_GPIO0_STAT")]
pub type SysctlMgmtGpio0Stat = crate::Reg<sysctl_mgmt_gpio0_stat::SysctlMgmtGpio0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_gpio0_stat;
#[doc = "SYSCTL_MGMT_GPIO1_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gpio1_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gpio1_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gpio1_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_GPIO1_PWREN")]
pub type SysctlMgmtGpio1Pwren = crate::Reg<sysctl_mgmt_gpio1_pwren::SysctlMgmtGpio1PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_gpio1_pwren;
#[doc = "SYSCTL_MGMT_GPIO1_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gpio1_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gpio1_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_GPIO1_RSTCTL")]
pub type SysctlMgmtGpio1Rstctl = crate::Reg<sysctl_mgmt_gpio1_rstctl::SysctlMgmtGpio1RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_gpio1_rstctl;
#[doc = "SYSCTL_MGMT_GPIO1_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gpio1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gpio1_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_GPIO1_STAT")]
pub type SysctlMgmtGpio1Stat = crate::Reg<sysctl_mgmt_gpio1_stat::SysctlMgmtGpio1StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_gpio1_stat;
#[doc = "SYSCTL_MGMT_GPIO2_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gpio2_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gpio2_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gpio2_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_GPIO2_PWREN")]
pub type SysctlMgmtGpio2Pwren = crate::Reg<sysctl_mgmt_gpio2_pwren::SysctlMgmtGpio2PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_gpio2_pwren;
#[doc = "SYSCTL_MGMT_GPIO2_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gpio2_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gpio2_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_GPIO2_RSTCTL")]
pub type SysctlMgmtGpio2Rstctl = crate::Reg<sysctl_mgmt_gpio2_rstctl::SysctlMgmtGpio2RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_gpio2_rstctl;
#[doc = "SYSCTL_MGMT_GPIO2_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gpio2_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gpio2_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_GPIO2_STAT")]
pub type SysctlMgmtGpio2Stat = crate::Reg<sysctl_mgmt_gpio2_stat::SysctlMgmtGpio2StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_gpio2_stat;
#[doc = "SYSCTL_MGMT_I2C0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_i2c0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_i2c0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C0_PWREN")]
pub type SysctlMgmtI2c0Pwren = crate::Reg<sysctl_mgmt_i2c0_pwren::SysctlMgmtI2c0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_i2c0_pwren;
#[doc = "SYSCTL_MGMT_I2C0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_i2c0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C0_RSTCTL")]
pub type SysctlMgmtI2c0Rstctl = crate::Reg<sysctl_mgmt_i2c0_rstctl::SysctlMgmtI2c0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_i2c0_rstctl;
#[doc = "SYSCTL_MGMT_I2C0_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_i2c0_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_i2c0_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c0_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C0_CLKCFG")]
pub type SysctlMgmtI2c0Clkcfg = crate::Reg<sysctl_mgmt_i2c0_clkcfg::SysctlMgmtI2c0ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_i2c0_clkcfg;
#[doc = "SYSCTL_MGMT_I2C0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_i2c0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C0_STAT")]
pub type SysctlMgmtI2c0Stat = crate::Reg<sysctl_mgmt_i2c0_stat::SysctlMgmtI2c0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_i2c0_stat;
#[doc = "SYSCTL_MGMT_I2C1_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_i2c1_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_i2c1_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c1_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C1_PWREN")]
pub type SysctlMgmtI2c1Pwren = crate::Reg<sysctl_mgmt_i2c1_pwren::SysctlMgmtI2c1PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_i2c1_pwren;
#[doc = "SYSCTL_MGMT_I2C1_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_i2c1_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c1_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C1_RSTCTL")]
pub type SysctlMgmtI2c1Rstctl = crate::Reg<sysctl_mgmt_i2c1_rstctl::SysctlMgmtI2c1RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_i2c1_rstctl;
#[doc = "SYSCTL_MGMT_I2C1_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_i2c1_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_i2c1_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c1_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C1_CLKCFG")]
pub type SysctlMgmtI2c1Clkcfg = crate::Reg<sysctl_mgmt_i2c1_clkcfg::SysctlMgmtI2c1ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_i2c1_clkcfg;
#[doc = "SYSCTL_MGMT_I2C1_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_i2c1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c1_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C1_STAT")]
pub type SysctlMgmtI2c1Stat = crate::Reg<sysctl_mgmt_i2c1_stat::SysctlMgmtI2c1StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_i2c1_stat;
#[doc = "SYSCTL_MGMT_I2C2_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_i2c2_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_i2c2_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c2_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C2_PWREN")]
pub type SysctlMgmtI2c2Pwren = crate::Reg<sysctl_mgmt_i2c2_pwren::SysctlMgmtI2c2PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_i2c2_pwren;
#[doc = "SYSCTL_MGMT_I2C2_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_i2c2_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c2_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C2_RSTCTL")]
pub type SysctlMgmtI2c2Rstctl = crate::Reg<sysctl_mgmt_i2c2_rstctl::SysctlMgmtI2c2RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_i2c2_rstctl;
#[doc = "SYSCTL_MGMT_I2C2_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_i2c2_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_i2c2_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c2_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C2_CLKCFG")]
pub type SysctlMgmtI2c2Clkcfg = crate::Reg<sysctl_mgmt_i2c2_clkcfg::SysctlMgmtI2c2ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_i2c2_clkcfg;
#[doc = "SYSCTL_MGMT_I2C2_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_i2c2_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_i2c2_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_I2C2_STAT")]
pub type SysctlMgmtI2c2Stat = crate::Reg<sysctl_mgmt_i2c2_stat::SysctlMgmtI2c2StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_i2c2_stat;
#[doc = "SYSCTL_MGMT_UARTLP0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartlp0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartlp0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP0_PWREN")]
pub type SysctlMgmtUartlp0Pwren = crate::Reg<sysctl_mgmt_uartlp0_pwren::SysctlMgmtUartlp0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_uartlp0_pwren;
#[doc = "SYSCTL_MGMT_UARTLP0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartlp0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP0_RSTCTL")]
pub type SysctlMgmtUartlp0Rstctl =
    crate::Reg<sysctl_mgmt_uartlp0_rstctl::SysctlMgmtUartlp0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_uartlp0_rstctl;
#[doc = "SYSCTL_MGMT_UARTLP0_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartlp0_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartlp0_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp0_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP0_CLKCFG")]
pub type SysctlMgmtUartlp0Clkcfg =
    crate::Reg<sysctl_mgmt_uartlp0_clkcfg::SysctlMgmtUartlp0ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_uartlp0_clkcfg;
#[doc = "SYSCTL_MGMT_UARTLP0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartlp0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP0_STAT")]
pub type SysctlMgmtUartlp0Stat = crate::Reg<sysctl_mgmt_uartlp0_stat::SysctlMgmtUartlp0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_uartlp0_stat;
#[doc = "SYSCTL_MGMT_UARTLP1_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartlp1_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartlp1_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp1_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP1_PWREN")]
pub type SysctlMgmtUartlp1Pwren = crate::Reg<sysctl_mgmt_uartlp1_pwren::SysctlMgmtUartlp1PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_uartlp1_pwren;
#[doc = "SYSCTL_MGMT_UARTLP1_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartlp1_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp1_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP1_RSTCTL")]
pub type SysctlMgmtUartlp1Rstctl =
    crate::Reg<sysctl_mgmt_uartlp1_rstctl::SysctlMgmtUartlp1RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_uartlp1_rstctl;
#[doc = "SYSCTL_MGMT_UARTLP1_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartlp1_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartlp1_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp1_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP1_CLKCFG")]
pub type SysctlMgmtUartlp1Clkcfg =
    crate::Reg<sysctl_mgmt_uartlp1_clkcfg::SysctlMgmtUartlp1ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_uartlp1_clkcfg;
#[doc = "SYSCTL_MGMT_UARTLP1_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartlp1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp1_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP1_STAT")]
pub type SysctlMgmtUartlp1Stat = crate::Reg<sysctl_mgmt_uartlp1_stat::SysctlMgmtUartlp1StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_uartlp1_stat;
#[doc = "SYSCTL_MGMT_UARTLP2_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartlp2_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartlp2_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp2_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP2_PWREN")]
pub type SysctlMgmtUartlp2Pwren = crate::Reg<sysctl_mgmt_uartlp2_pwren::SysctlMgmtUartlp2PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_uartlp2_pwren;
#[doc = "SYSCTL_MGMT_UARTLP2_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartlp2_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp2_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP2_RSTCTL")]
pub type SysctlMgmtUartlp2Rstctl =
    crate::Reg<sysctl_mgmt_uartlp2_rstctl::SysctlMgmtUartlp2RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_uartlp2_rstctl;
#[doc = "SYSCTL_MGMT_UARTLP2_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartlp2_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartlp2_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp2_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP2_CLKCFG")]
pub type SysctlMgmtUartlp2Clkcfg =
    crate::Reg<sysctl_mgmt_uartlp2_clkcfg::SysctlMgmtUartlp2ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_uartlp2_clkcfg;
#[doc = "SYSCTL_MGMT_UARTLP2_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartlp2_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartlp2_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTLP2_STAT")]
pub type SysctlMgmtUartlp2Stat = crate::Reg<sysctl_mgmt_uartlp2_stat::SysctlMgmtUartlp2StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_uartlp2_stat;
#[doc = "SYSCTL_MGMT_UARTADVLP0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartadvlp0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartadvlp0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartadvlp0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTADVLP0_PWREN")]
pub type SysctlMgmtUartadvlp0Pwren =
    crate::Reg<sysctl_mgmt_uartadvlp0_pwren::SysctlMgmtUartadvlp0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_uartadvlp0_pwren;
#[doc = "SYSCTL_MGMT_UARTADVLP0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartadvlp0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartadvlp0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTADVLP0_RSTCTL")]
pub type SysctlMgmtUartadvlp0Rstctl =
    crate::Reg<sysctl_mgmt_uartadvlp0_rstctl::SysctlMgmtUartadvlp0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_uartadvlp0_rstctl;
#[doc = "SYSCTL_MGMT_UARTADVLP0_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartadvlp0_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartadvlp0_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartadvlp0_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTADVLP0_CLKCFG")]
pub type SysctlMgmtUartadvlp0Clkcfg =
    crate::Reg<sysctl_mgmt_uartadvlp0_clkcfg::SysctlMgmtUartadvlp0ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_uartadvlp0_clkcfg;
#[doc = "SYSCTL_MGMT_UARTADVLP0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartadvlp0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartadvlp0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTADVLP0_STAT")]
pub type SysctlMgmtUartadvlp0Stat =
    crate::Reg<sysctl_mgmt_uartadvlp0_stat::SysctlMgmtUartadvlp0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_uartadvlp0_stat;
#[doc = "SYSCTL_MGMT_UARTADVLP1_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartadvlp1_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartadvlp1_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartadvlp1_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTADVLP1_PWREN")]
pub type SysctlMgmtUartadvlp1Pwren =
    crate::Reg<sysctl_mgmt_uartadvlp1_pwren::SysctlMgmtUartadvlp1PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_uartadvlp1_pwren;
#[doc = "SYSCTL_MGMT_UARTADVLP1_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartadvlp1_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartadvlp1_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTADVLP1_RSTCTL")]
pub type SysctlMgmtUartadvlp1Rstctl =
    crate::Reg<sysctl_mgmt_uartadvlp1_rstctl::SysctlMgmtUartadvlp1RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_uartadvlp1_rstctl;
#[doc = "SYSCTL_MGMT_UARTADVLP1_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartadvlp1_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_uartadvlp1_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartadvlp1_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTADVLP1_CLKCFG")]
pub type SysctlMgmtUartadvlp1Clkcfg =
    crate::Reg<sysctl_mgmt_uartadvlp1_clkcfg::SysctlMgmtUartadvlp1ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_uartadvlp1_clkcfg;
#[doc = "SYSCTL_MGMT_UARTADVLP1_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_uartadvlp1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_uartadvlp1_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_UARTADVLP1_STAT")]
pub type SysctlMgmtUartadvlp1Stat =
    crate::Reg<sysctl_mgmt_uartadvlp1_stat::SysctlMgmtUartadvlp1StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_uartadvlp1_stat;
#[doc = "SYSCTL_MGMT_CRC0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_crc0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_crc0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_crc0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_CRC0_PWREN")]
pub type SysctlMgmtCrc0Pwren = crate::Reg<sysctl_mgmt_crc0_pwren::SysctlMgmtCrc0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_crc0_pwren;
#[doc = "SYSCTL_MGMT_CRC0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_crc0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_crc0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_CRC0_RSTCTL")]
pub type SysctlMgmtCrc0Rstctl = crate::Reg<sysctl_mgmt_crc0_rstctl::SysctlMgmtCrc0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_crc0_rstctl;
#[doc = "SYSCTL_MGMT_CRC0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_crc0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_crc0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_CRC0_STAT")]
pub type SysctlMgmtCrc0Stat = crate::Reg<sysctl_mgmt_crc0_stat::SysctlMgmtCrc0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_crc0_stat;
#[doc = "SYSCTL_MGMT_AES_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_aes_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_aes_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_aes_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_AES_PWREN")]
pub type SysctlMgmtAesPwren = crate::Reg<sysctl_mgmt_aes_pwren::SysctlMgmtAesPwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_aes_pwren;
#[doc = "SYSCTL_MGMT_AES_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_aes_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_aes_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_AES_RSTCTL")]
pub type SysctlMgmtAesRstctl = crate::Reg<sysctl_mgmt_aes_rstctl::SysctlMgmtAesRstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_aes_rstctl;
#[doc = "SYSCTL_MGMT_AES_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_aes_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_aes_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_AES_STAT")]
pub type SysctlMgmtAesStat = crate::Reg<sysctl_mgmt_aes_stat::SysctlMgmtAesStatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_aes_stat;
#[doc = "SYSCTL_MGMT_TRNG_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_trng_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_trng_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_trng_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_TRNG_PWREN")]
pub type SysctlMgmtTrngPwren = crate::Reg<sysctl_mgmt_trng_pwren::SysctlMgmtTrngPwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_trng_pwren;
#[doc = "SYSCTL_MGMT_TRNG_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_trng_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_trng_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_TRNG_RSTCTL")]
pub type SysctlMgmtTrngRstctl = crate::Reg<sysctl_mgmt_trng_rstctl::SysctlMgmtTrngRstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_trng_rstctl;
#[doc = "SYSCTL_MGMT_TRNG_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_trng_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_trng_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_TRNG_STAT")]
pub type SysctlMgmtTrngStat = crate::Reg<sysctl_mgmt_trng_stat::SysctlMgmtTrngStatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_trng_stat;
#[doc = "SYSCTL_MGMT_SPI0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_spi0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_spi0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_spi0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_SPI0_PWREN")]
pub type SysctlMgmtSpi0Pwren = crate::Reg<sysctl_mgmt_spi0_pwren::SysctlMgmtSpi0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_spi0_pwren;
#[doc = "SYSCTL_MGMT_SPI0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_spi0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_spi0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_SPI0_RSTCTL")]
pub type SysctlMgmtSpi0Rstctl = crate::Reg<sysctl_mgmt_spi0_rstctl::SysctlMgmtSpi0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_spi0_rstctl;
#[doc = "SYSCTL_MGMT_SPI0_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_spi0_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_spi0_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_spi0_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_SPI0_CLKCFG")]
pub type SysctlMgmtSpi0Clkcfg = crate::Reg<sysctl_mgmt_spi0_clkcfg::SysctlMgmtSpi0ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_spi0_clkcfg;
#[doc = "SYSCTL_MGMT_SPI0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_spi0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_spi0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_SPI0_STAT")]
pub type SysctlMgmtSpi0Stat = crate::Reg<sysctl_mgmt_spi0_stat::SysctlMgmtSpi0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_spi0_stat;
#[doc = "SYSCTL_MGMT_SPI1_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_spi1_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_spi1_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_spi1_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_SPI1_PWREN")]
pub type SysctlMgmtSpi1Pwren = crate::Reg<sysctl_mgmt_spi1_pwren::SysctlMgmtSpi1PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_spi1_pwren;
#[doc = "SYSCTL_MGMT_SPI1_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_spi1_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_spi1_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_SPI1_RSTCTL")]
pub type SysctlMgmtSpi1Rstctl = crate::Reg<sysctl_mgmt_spi1_rstctl::SysctlMgmtSpi1RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_spi1_rstctl;
#[doc = "SYSCTL_MGMT_SPI1_CLKCFG (rw) register accessor: IP Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_spi1_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_spi1_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_spi1_clkcfg`] module"]
#[doc(alias = "SYSCTL_MGMT_SPI1_CLKCFG")]
pub type SysctlMgmtSpi1Clkcfg = crate::Reg<sysctl_mgmt_spi1_clkcfg::SysctlMgmtSpi1ClkcfgSpec>;
#[doc = "IP Clock Configuration Register"]
pub mod sysctl_mgmt_spi1_clkcfg;
#[doc = "SYSCTL_MGMT_SPI1_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_spi1_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_spi1_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_SPI1_STAT")]
pub type SysctlMgmtSpi1Stat = crate::Reg<sysctl_mgmt_spi1_stat::SysctlMgmtSpi1StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_spi1_stat;
#[doc = "SYSCTL_MGMT_GPTIMER16BADV4CC0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16badv4cc0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer16badv4cc0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16badv4cc0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16BADV4CC0_PWREN")]
pub type SysctlMgmtGptimer16badv4cc0Pwren =
    crate::Reg<sysctl_mgmt_gptimer16badv4cc0_pwren::SysctlMgmtGptimer16badv4cc0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_gptimer16badv4cc0_pwren;
#[doc = "SYSCTL_MGMT_GPTIMER16BADV4CC0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer16badv4cc0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16badv4cc0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16BADV4CC0_RSTCTL")]
pub type SysctlMgmtGptimer16badv4cc0Rstctl =
    crate::Reg<sysctl_mgmt_gptimer16badv4cc0_rstctl::SysctlMgmtGptimer16badv4cc0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_gptimer16badv4cc0_rstctl;
#[doc = "SYSCTL_MGMT_GPTIMER16BADV4CC0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer16badv4cc0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer16badv4cc0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER16BADV4CC0_STAT")]
pub type SysctlMgmtGptimer16badv4cc0Stat =
    crate::Reg<sysctl_mgmt_gptimer16badv4cc0_stat::SysctlMgmtGptimer16badv4cc0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_gptimer16badv4cc0_stat;
#[doc = "SYSCTL_MGMT_GPTIMER32B2CC0_PWREN (rw) register accessor: IP Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer32b2cc0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer32b2cc0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer32b2cc0_pwren`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER32B2CC0_PWREN")]
pub type SysctlMgmtGptimer32b2cc0Pwren =
    crate::Reg<sysctl_mgmt_gptimer32b2cc0_pwren::SysctlMgmtGptimer32b2cc0PwrenSpec>;
#[doc = "IP Enable Register"]
pub mod sysctl_mgmt_gptimer32b2cc0_pwren;
#[doc = "SYSCTL_MGMT_GPTIMER32B2CC0_RSTCTL (w) register accessor: Power Control Register - Write Only Register, Always Read as 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl_mgmt_gptimer32b2cc0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer32b2cc0_rstctl`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER32B2CC0_RSTCTL")]
pub type SysctlMgmtGptimer32b2cc0Rstctl =
    crate::Reg<sysctl_mgmt_gptimer32b2cc0_rstctl::SysctlMgmtGptimer32b2cc0RstctlSpec>;
#[doc = "Power Control Register - Write Only Register, Always Read as 0"]
pub mod sysctl_mgmt_gptimer32b2cc0_rstctl;
#[doc = "SYSCTL_MGMT_GPTIMER32B2CC0_STAT (r) register accessor: IP State Register - Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl_mgmt_gptimer32b2cc0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl_mgmt_gptimer32b2cc0_stat`] module"]
#[doc(alias = "SYSCTL_MGMT_GPTIMER32B2CC0_STAT")]
pub type SysctlMgmtGptimer32b2cc0Stat =
    crate::Reg<sysctl_mgmt_gptimer32b2cc0_stat::SysctlMgmtGptimer32b2cc0StatSpec>;
#[doc = "IP State Register - Read Only"]
pub mod sysctl_mgmt_gptimer32b2cc0_stat;
