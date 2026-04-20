use crate::pac;
use core::ptr;
mod flash_config;

pub use flash_config::FlashError;

pub struct Flash {
    _flash: pac::Flashctl,
    _cpuss: pac::Cpuss,
    _factoryregion: pac::Factoryregion,
    _sysctl: pac::Sysctl,
}

impl Flash {
    pub fn new(
        flash: pac::Flashctl,
        cpuss: pac::Cpuss,
        factoryregion: pac::Factoryregion,
        sysctl: pac::Sysctl,
    ) -> Self {
        Self {
            _flash: flash,
            _cpuss: cpuss,
            _factoryregion: factoryregion,
            _sysctl: sysctl,
        }
    }

    pub fn simple_erase_page(&self, addr: &u32) -> Result<(), flash_config::FlashError> {
        self.clear_status();
        self.unprotect_sector(addr, flash_config::RegionSelect::Main);
        self.execute_erase_memory(addr, flash_config::CommandSize::Sector)
    }

    pub fn simple_read(&self, addr: u32, buf: &mut [u8]) {
        let src = addr as *const u8;
        unsafe {
            ptr::copy_nonoverlapping(src, buf.as_mut_ptr(), buf.len());
        }
    }

    pub fn simple_write(&self, addr: &u32, buf: &[u32]) -> Result<(), flash_config::FlashError> {
        self.clear_status();
        self.unprotect_sector(addr, flash_config::RegionSelect::Main);
        let size = buf.len();

        // Expects an even number of 32 bit words
        // since flash word size is 64 bit
        if size % 2 != 0 {
            return Err(flash_config::FlashError::InvalidSize);
        }

        self.program_memory_blocking(addr, buf)?;
        self.wait_for_command_done()
    }

    fn program_memory_blocking(
        &self,
        addr: &u32,
        buf: &[u32],
    ) -> Result<(), flash_config::FlashError> {
        let mut addr = *addr;
        for chunk in buf.chunks(2) {
            let chunk_arr = match chunk {
                [a, b] => Ok([a, b]),
                _ => Err(flash_config::FlashError::InvalidSize),
            }?;
            self.program_memory_word(&addr, chunk_arr[0], chunk_arr[1])?;
            addr += 8;
        }

        Ok(())
    }

    fn program_memory_word(
        &self,
        addr: &u32,
        dat0: &u32,
        dat1: &u32,
    ) -> Result<(), flash_config::FlashError> {
        self.clear_status();
        self.unprotect_sector(addr, flash_config::RegionSelect::Main);
        self._flash.flashctl_cmdtype().write(|w| {
            w.size().oneword();
            w.command().program()
        });
        self._flash
            .flashctl_cmdbyten()
            .write(|w| w.val().program_64_with_ecc());
        self._flash
            .flashctl_cmdaddr()
            .write(|w| unsafe { w.bits(*addr) });
        self._flash
            .flashctl_cmddata0()
            .write(|w| unsafe { w.val().bits(*dat0) });
        self._flash
            .flashctl_cmddata1()
            .write(|w| unsafe { w.val().bits(*dat1) });

        self.execute_command()
    }

    fn execute_command(&self) -> Result<(), flash_config::FlashError> {
        self._flash.flashctl_cmdexec().write(|w| w.val().execute());

        while self
            ._flash
            .flashctl_statcmd()
            .read()
            .cmdinprogress()
            .bit_is_set()
        {}

        if self._flash.flashctl_statcmd().read().cmdpass().bit_is_set() {
            Ok(())
        } else if self._flash.flashctl_statcmd().read().failinvdata().bit() {
            Err(flash_config::FlashError::InvalidData)
        } else if self._flash.flashctl_statcmd().read().failmode().bit() {
            Err(flash_config::FlashError::ModeFailure)
        } else if self._flash.flashctl_statcmd().read().failmisc().bit() {
            Err(flash_config::FlashError::Misc)
        } else if self._flash.flashctl_statcmd().read().failverify().bit() {
            Err(flash_config::FlashError::VerifyFailure)
        } else if self._flash.flashctl_statcmd().read().faililladdr().bit() {
            Err(flash_config::FlashError::IllegalAddress)
        } else if self._flash.flashctl_statcmd().read().failweprot().bit() {
            Err(flash_config::FlashError::WriteEraseFailure)
        } else {
            unreachable!()
        }
    }

    // #[inline(always)]
    // fn blank_verify(&mut self, addr: u32) {
    //     self._flash
    //         .flashctl_cmdtype()
    //         .write(|w| w.command().blankverify());
    //
    //     self._flash
    //         .flashctl_cmdaddr()
    //         .write(|w| unsafe { w.bits(addr) });
    // }

    // pub fn execute_blank_verify_from_ram(
    //     &mut self,
    //     addr: u32,
    // ) -> Result<(), FlashError> {
    //     self.blank_verify(addr);
    //     self.execute_command()
    // }

    fn execute_erase_memory(
        &self,
        addr: &u32,
        size: flash_config::CommandSize,
    ) -> Result<(), flash_config::FlashError> {
        self._flash.flashctl_cmdtype().write(|w| {
            match size {
                flash_config::CommandSize::Bank => w.size().bank(),
                flash_config::CommandSize::Sector => w.size().sector(),
                _ => panic!(),
            };
            w.command().erase()
        });

        self._flash
            .flashctl_cmdaddr()
            .write(|w| unsafe { w.bits(*addr) });

        self.execute_command()
    }

    // pub fn program_memory_from_ram<T>(
    //     &mut self,
    //     addr: u32,
    //     data: &[T],
    //     region_select: RegionSelect,
    // ) -> Result<(), FlashError>
    // where
    //     u32: From<T>,
    //     u64: From<T>,
    //     T: Copy,
    // {
    //     let data_size = size_of::<T>();
    //     assert!(data_size == 4 || data_size == 8);
    //     if data.len() == 0 {
    //         return Err(flash_config::FlashError::Misc);
    //     }
    //
    //     let mut status = Ok(());
    //     let mut addr = addr;
    //
    //     for item in data.iter() {
    //         self.unprotect_sector(addr, region_select);
    //
    //         self._flash.flashctl_cmdtype().write(|w| {
    //             w.size().oneword();
    //             w.command().program()
    //         });
    //         if data_size == 4 {
    //             self._flash.flashctl_cmdbyten().write(|w| unsafe {
    //                 w.bits(0xf) // Program 32 bits without ecc
    //             });
    //             addr += 4;
    //         } else if data_size == 8 {
    //             self._flash.flashctl_cmdbyten().write(|w| unsafe {
    //                 w.bits(0xff) // Program 64 bits without ecc
    //             });
    //             addr += 8;
    //         }
    //         self._flash
    //             .flashctl_cmddata0()
    //             .write(|w| unsafe { w.bits((*item).into()) });
    //         status = status.and(self.execute_command());
    //     }
    //
    //     status
    // }

    // fn set_program_memory_config(&mut self, addr: u32, command: FlashCommand) {
    //     self._flash.flashctl_cmdtype().write(|w| {
    //         w.size().oneword();
    //         w.command().program()
    //     });
    //
    //     self._flash
    //         .flashctl_cmdbyten()
    //         .write(|w| unsafe { w.val().bits(command as u32) });
    //
    //     self._flash
    //         .flashctl_cmdaddr()
    //         .write(|w| unsafe { w.bits(addr) });
    // }

    // fn program_memory_from_ram_with_ecc_generated(
    //     &mut self,
    //     addr: u32,
    //     buf: &[u32],
    //     region_select: RegionSelect,
    // ) -> Result<(), FlashError> {
    //     if buf.len() == 0 || buf.len() & 1 == 1 {
    //         return Err(flash_config::FlashError::Misc);
    //     }
    //
    //     let status = Ok(());
    //
    //     for (index, (word0, word1)) in
    //         buf.chunks(2).map(|v| (v[0], v[1])).enumerate()
    //     {
    //         let addr = addr + 8 * index as u32;
    //         self.clear_status();
    //         self.unprotect_sector(addr, region_select);
    //
    //         self.set_program_memory_config(
    //             addr,
    //             FlashCommand::Program64WithEcc,
    //         );
    //
    //         self._flash
    //             .flashctl_cmddata0()
    //             .write(|w| unsafe { w.bits(word0) });
    //         self._flash
    //             .flashctl_cmddata1()
    //             .write(|w| unsafe { w.bits(word1) });
    //
    //         // status = status.and(self.execute_command());
    //         self.execute_command()?;
    //     }
    //
    //     status
    // }

    fn wait_for_command_done(&self) -> Result<(), flash_config::FlashError> {
        while self
            ._flash
            .flashctl_statcmd()
            .read()
            .cmddone()
            .bit_is_clear()
        {}

        if self._flash.flashctl_statcmd().read().cmdpass().bit_is_set() {
            Ok(())
        } else {
            Err(flash_config::FlashError::Misc)
        }
    }

    #[inline(always)]
    fn get_flash_sector_number(addr: &u32) -> u32 {
        addr >> 10
    }

    #[inline(always)]
    fn get_flash_sector_number_in_bank(&self, addr: &u32) -> u32 {
        let ctl_temp = self._cpuss.cpuss_ctl().read().bits() & (0b111);
        self._cpuss
            .cpuss_ctl()
            .write(|w| unsafe { w.bits(1 | 0 | 4) });

        let num_banks = self._factoryregion.sramflash().read().mainnumbanks().bits() as u32 + 1;

        self._cpuss
            .cpuss_ctl()
            .write(|w| unsafe { w.bits(ctl_temp) });

        let main_flash_size = self._factoryregion.sramflash().read().mainflash_sz().bits() as u32;
        let sector = Self::get_flash_sector_number(addr);
        if num_banks > 1 {
            let bank_sectors = main_flash_size / num_banks;

            sector % bank_sectors
        } else {
            sector
        }
    }

    pub fn unprotect_sector(&self, addr: &u32, region_select: flash_config::RegionSelect) {
        let sector_number = Self::get_flash_sector_number(addr);
        let sector_in_bank = self.get_flash_sector_number_in_bank(addr);

        let num_banks = self._factoryregion.sramflash().read().mainnumbanks().bits() as u32 + 1;
        let main_flash_size = self._factoryregion.sramflash().read().mainflash_sz().bits() as u32;

        match region_select {
            flash_config::RegionSelect::Main => {
                let physical_sector_number = if self
                    ._sysctl
                    .sysctl_secstatus()
                    .read()
                    .flbankswp()
                    .bit_is_set()
                    && num_banks > 1
                {
                    if sector_number >= main_flash_size / 2 {
                        sector_number - (main_flash_size / 2)
                    } else {
                        sector_number + (main_flash_size / 2)
                    }
                } else {
                    sector_number
                };

                if physical_sector_number < 32 {
                    let sectormask = 1 << physical_sector_number;
                    self._flash
                        .flashctl_cmdweprota()
                        .modify(|r, w| unsafe { w.bits(r.bits() & !sectormask) });
                } else {
                    if sector_in_bank < 256 {
                        let sectormask = if num_banks == 1 {
                            1 << ((sector_in_bank - 32) / 8)
                        } else {
                            1 << (sector_in_bank / 8)
                        };

                        self._flash
                            .flashctl_cmdweprotb()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !sectormask) });
                    } else if sector_in_bank < 511 {
                        let sectormask = 1 << ((sector_in_bank - 256) / 8);

                        self._flash
                            .flashctl_cmdweprotc()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !sectormask) });
                    } else {
                        unreachable!()
                    }
                }
            }
            flash_config::RegionSelect::NonMain => {
                let sectormask = 1 << (sector_number % 32);
                self._flash
                    .flashctl_cmdweprotnm()
                    .modify(|r, w| unsafe { w.bits(r.bits() & !sectormask) });
            }
        }
    }

    fn clear_status(&self) {
        self._flash
            .flashctl_cmdtype()
            .write(|w| w.command().clearstatus());
        self._flash.flashctl_cmdexec().write(|w| w.val().execute());

        while self
            ._flash
            .flashctl_statcmd()
            .read()
            .cmdinprogress()
            .bit_is_set()
        {}
    }
}
