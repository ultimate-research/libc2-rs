#[repr(C)]
pub struct StageBattleMGShadowmoses {
    _0: [u8; 0x8A0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleMGShadowmoses>(), 0x8A0);
        assert_eq!(offset_of!(StageBattleMGShadowmoses, _0), 0x0);
    }
}
