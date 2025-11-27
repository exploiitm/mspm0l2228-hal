macro_rules! update_reg {
    ($r:ident, $w:ident, $threshold:expr, $mask:expr) => {
        $w.bits(($r.bits() & !($mask)) | (($threshold) & ($mask)))
    };
}
pub(crate) use update_reg;
