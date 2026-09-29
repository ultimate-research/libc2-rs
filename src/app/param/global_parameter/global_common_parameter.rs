use crate::app::ItemKind;

#[repr(C)]
pub struct GlobalCommonParameter {
    _0: u8,
    _1: u8,
    _4: u32,
    _8: u8,
    _9: u8,
    _a: u8,
    _b: u8,
    _c: [u8; 0x4],
    _10: f32,
    _14: ItemKind,
    _18: ItemKind,
    _1c: ItemKind,
    _20: ItemKind,
    _24: ItemKind,
    _28: ItemKind,
    _2c: [u8; 0x4],
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<GlobalCommonParameter>(), 0x30);
        assert_eq!(offset_of!(GlobalCommonParameter, _0), 0x0);
        assert_eq!(offset_of!(GlobalCommonParameter, _1), 0x1);
        assert_eq!(offset_of!(GlobalCommonParameter, _4), 0x4);
        assert_eq!(offset_of!(GlobalCommonParameter, _8), 0x8);
        assert_eq!(offset_of!(GlobalCommonParameter, _9), 0x9);
        assert_eq!(offset_of!(GlobalCommonParameter, _a), 0xA);
        assert_eq!(offset_of!(GlobalCommonParameter, _b), 0xB);
        assert_eq!(offset_of!(GlobalCommonParameter, _10), 0x10);
        assert_eq!(offset_of!(GlobalCommonParameter, _14), 0x14);
        assert_eq!(offset_of!(GlobalCommonParameter, _18), 0x18);
        assert_eq!(offset_of!(GlobalCommonParameter, _1c), 0x1C);
        assert_eq!(offset_of!(GlobalCommonParameter, _20), 0x20);
        assert_eq!(offset_of!(GlobalCommonParameter, _24), 0x24);
        assert_eq!(offset_of!(GlobalCommonParameter, _28), 0x28);
    }
}
