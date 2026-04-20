#[allow(dead_code)]
pub enum RegionSelect {
    Main,
    NonMain,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum FlashError {
    InvalidData,
    Misc,
    VerifyFailure,
    InvalidSize,
    IllegalAddress,
    WriteEraseFailure,
    ModeFailure,
}

#[allow(dead_code)]
enum FlashCommand {
    Program64WithEcc = 0x1ff,
}

#[allow(dead_code)]
pub enum CommandSize {
    OneWord,
    TwoWord,
    FourWord,
    EightWord,
    Sector,
    Bank,
}
