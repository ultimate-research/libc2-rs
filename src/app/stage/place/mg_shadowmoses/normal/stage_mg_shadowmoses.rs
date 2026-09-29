#[repr(C)]
pub struct StageMGShadowmoses {
    _0: [u8; 0x880],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMGShadowmoses>(), 0x880);
        assert_eq!(offset_of!(StageMGShadowmoses, _0), 0x0);
    }
}
