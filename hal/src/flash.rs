use crate::pac;

pub struct Flash<'a> {
    _flash: pac::Flashctl,
    _cpuss: &'a pac::Cpuss,
    _factoryregion: &'a pac::Factoryregion,
    _sysctl: &'a pac::Sysctl,
}

#[derive(Copy, Clone)]
pub enum RegionSelect {
    Main,
    NonMain,
}

#[derive(Debug)]
pub enum FlashError {
    InvalidData,
    Misc,
    VerifyFailure,
    IllegalAddress,
    WriteEraseFailure,
    ModeFailure,
}

#[repr(u32)]
enum FlashCommand {
    Program64WithEcc = 0x1ff,
}

#[repr(u32)]
#[derive(PartialEq)]
pub enum CommandSize {
    OneWord = 0x00000000,
    TwoWord = 0x00000010,
    FourWord = 0x00000020,
    EightWord = 0x00000030,
    Sector = 0x00000040,
    Bank = 0x00000050,
}

impl<'a> Flash<'a> {
    pub fn new(
        flash: pac::Flashctl,
        cpuss: &'a mut pac::Cpuss,
        factoryregion: &'a pac::Factoryregion,
        sysctl: &'a pac::Sysctl,
    ) -> Self {
        Self {
            _flash: flash,
            _cpuss: cpuss,
            _factoryregion: factoryregion,
            _sysctl: sysctl,
        }
    }

    fn execute_command(&mut self) -> Result<(), FlashError> {
        self._flash.flashctl_cmdexec().write(|w| w.val().execute());

        while self._flash.flashctl_statcmd().read().cmdinprogress().bit() {}

        if self._flash.flashctl_statcmd().read().cmdpass().bit_is_set() {
            Ok(())
        } else if self._flash.flashctl_statcmd().read().failinvdata().bit() {
            Err(FlashError::InvalidData)
        } else if self._flash.flashctl_statcmd().read().failmode().bit() {
            Err(FlashError::ModeFailure)
        } else if self._flash.flashctl_statcmd().read().failmisc().bit() {
            Err(FlashError::Misc)
        } else if self._flash.flashctl_statcmd().read().failverify().bit() {
            Err(FlashError::VerifyFailure)
        } else if self._flash.flashctl_statcmd().read().faililladdr().bit() {
            Err(FlashError::IllegalAddress)
        } else if self._flash.flashctl_statcmd().read().failweprot().bit() {
            Err(FlashError::WriteEraseFailure)
        } else {
            unreachable!()
        }
    }

    pub fn clear_status(&mut self) {
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

    #[inline(always)]
    fn blank_verify(&mut self, addr: u32) {
        self._flash
            .flashctl_cmdtype()
            .write(|w| w.command().blankverify());

        self._flash
            .flashctl_cmdaddr()
            .write(|w| unsafe { w.bits(addr) });
    }

    pub fn execute_blank_verify_from_ram(
        &mut self,
        addr: u32,
    ) -> Result<(), FlashError> {
        self.blank_verify(addr);
        self.execute_command()
    }

    #[inline(always)]
    fn erase_memory(&mut self, addr: u32, size: CommandSize) {
        assert!(size == CommandSize::Bank || size == CommandSize::Sector);
        self._flash.flashctl_cmdtype().write(|w| unsafe {
            w.bits(size as u32);
            w.command().erase()
        });

        self._flash
            .flashctl_cmdaddr()
            .write(|w| unsafe { w.bits(addr) });
    }

    pub fn execute_erase_memory_from_ram(
        &mut self,
        addr: u32,
        size: CommandSize,
    ) -> Result<(), FlashError> {
        self.erase_memory(addr, size);
        self.execute_command()
    }

    pub fn program_memory_from_ram<T>(
        &mut self,
        addr: u32,
        data: &[T],
        region_select: RegionSelect,
    ) -> Result<(), FlashError>
    where
        u32: From<T>,
        u64: From<T>,
        T: Copy,
    {
        let data_size = size_of::<T>();
        assert!(data_size == 4 || data_size == 8);
        if data.len() == 0 {
            return Err(FlashError::Misc);
        }

        let mut status = Ok(());
        let mut addr = addr;

        for item in data.iter() {
            self.unprotect_sector(addr, region_select);

            self._flash.flashctl_cmdtype().write(|w| {
                w.size().oneword();
                w.command().program()
            });
            if data_size == 4 {
                self._flash.flashctl_cmdbyten().write(|w| unsafe {
                    w.bits(0xf) // Program 32 bits without ecc
                });
                addr += 4;
            } else if data_size == 8 {
                self._flash.flashctl_cmdbyten().write(|w| unsafe {
                    w.bits(0xff) // Program 64 bits without ecc
                });
                addr += 8;
            }
            self._flash
                .flashctl_cmddata0()
                .write(|w| unsafe { w.bits((*item).into()) });
            status = status.and(self.execute_command());
        }

        status
    }

    fn set_program_memory_config(&mut self, addr: u32, command: FlashCommand) {
        self._flash.flashctl_cmdtype().write(|w| {
            w.size().oneword();
            w.command().program()
        });

        self._flash
            .flashctl_cmdbyten()
            .write(|w| unsafe { w.val().bits(command as u32) });

        self._flash
            .flashctl_cmdaddr()
            .write(|w| unsafe { w.bits(addr) });
    }

    fn program_memory_from_ram_with_ecc_generated(
        &mut self,
        addr: u32,
        buf: &[u32],
        region_select: RegionSelect,
    ) -> Result<(), FlashError> {
        if buf.len() == 0 || buf.len() & 1 == 1 {
            return Err(FlashError::Misc);
        }

        let status = Ok(());

        for (index, (word0, word1)) in
            buf.chunks(2).map(|v| (v[0], v[1])).enumerate()
        {
            let addr = addr + 8 * index as u32;
            self.clear_status();
            self.unprotect_sector(addr, region_select);

            self.set_program_memory_config(
                addr,
                FlashCommand::Program64WithEcc,
            );

            self._flash
                .flashctl_cmddata0()
                .write(|w| unsafe { w.bits(word0) });
            self._flash
                .flashctl_cmddata1()
                .write(|w| unsafe { w.bits(word1) });

            // status = status.and(self.execute_command());
            self.execute_command()?;
        }

        status
    }

    fn wait_for_command_done(&self) -> Result<(), FlashError> {
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
            Err(FlashError::Misc)
        }
    }

    pub fn simple_write(
        &mut self,
        addr: u32,
        buf: &[u32],
    ) -> Result<(), FlashError> {
        self.clear_status();
        self.unprotect_sector(addr, RegionSelect::Main);

        let rounded_write_buffer_size = if buf.len() % 2 == 0 {
            buf.len()
        } else {
            buf.len() + 1
        };

        // NOTE: Statically allocates 10kB to make sure the maximum file size of 8kB can be
        // reasonably stored using this buffer, might have implications for the stack size/ram
        // usage. Important to keep in mind and change if there is a better way
        let mut writebuf = [!(0u32); 1024];
        writebuf[..rounded_write_buffer_size].copy_from_slice(buf);

        self.program_memory_from_ram_with_ecc_generated(
            addr,
            &writebuf,
            RegionSelect::Main,
        )?;

        self.wait_for_command_done()
    }

    #[inline(always)]
    fn get_flash_sector_number(addr: u32) -> u32 {
        addr >> 10
    }

    #[inline(always)]
    fn get_flash_sector_number_in_bank(&self, addr: u32) -> u32 {
        let ctl_temp = self._cpuss.cpuss_ctl().read().bits() & (0b111);
        self._cpuss
            .cpuss_ctl()
            .write(|w| unsafe { w.bits(1 | 0 | 4) });

        let num_banks =
            self._factoryregion.sramflash().read().mainnumbanks().bits() as u32
                + 1;

        self._cpuss
            .cpuss_ctl()
            .write(|w| unsafe { w.bits(ctl_temp) });

        let main_flash_size =
            self._factoryregion.sramflash().read().mainflash_sz().bits() as u32;
        let sector = Self::get_flash_sector_number(addr);
        if num_banks > 1 {
            let bank_sectors = main_flash_size / num_banks;

            sector % bank_sectors
        } else {
            sector
        }
    }

    pub fn unprotect_sector(&mut self, addr: u32, region_select: RegionSelect) {
        let sector_number = Self::get_flash_sector_number(addr);
        let sector_in_bank = self.get_flash_sector_number_in_bank(addr);

        let num_banks =
            self._factoryregion.sramflash().read().mainnumbanks().bits() as u32
                + 1;
        let main_flash_size =
            self._factoryregion.sramflash().read().mainflash_sz().bits() as u32;

        match region_select {
            RegionSelect::Main => {
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
                    self._flash.flashctl_cmdweprota().modify(|r, w| unsafe {
                        w.bits(r.bits() & !sectormask)
                    });
                } else {
                    if sector_in_bank < 256 {
                        let sectormask = if num_banks == 1 {
                            1 << ((sector_in_bank - 32) / 8)
                        } else {
                            1 << (sector_in_bank / 8)
                        };

                        self._flash.flashctl_cmdweprotb().modify(
                            |r, w| unsafe { w.bits(r.bits() & !sectormask) },
                        );
                    } else if sector_in_bank < 511 {
                        let sectormask = 1 << ((sector_in_bank - 256) / 8);

                        self._flash.flashctl_cmdweprotc().modify(
                            |r, w| unsafe { w.bits(r.bits() & !sectormask) },
                        );
                    } else {
                        unreachable!()
                    }
                }
            }
            RegionSelect::NonMain => {
                let sectormask = 1 << (sector_number % 32);
                self._flash
                    .flashctl_cmdweprotnm()
                    .modify(|r, w| unsafe { w.bits(r.bits() & !sectormask) });
            }
        }
    }
}
