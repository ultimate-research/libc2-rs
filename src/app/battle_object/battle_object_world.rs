use crate::{cpp, phx::Vector3f};

#[repr(C)]
pub struct BattleObjectWorld {
    pub _0: f32,
    pub gravity_speed_coefficient: f32,
    pub scale_z: f32,
    pub _c: bool,
    pub gravity_pos: Vector3f,
    pub move_speed: Vector3f,
    pub _30: f32,
    _34: [u8; 0x4],
    pub _38: BattleObjectWorldSub1,
    pub _58: bool,
    pub is_gravity_normal: bool,
    pub is_move: bool,
    pub _5b: bool,
    pub _5c: bool,
    pub is_disable_reverse: bool,
    pub _5e: bool,
    _5f: [u8; 0x1],
    pub _60: cpp::UniquePtr<BattleObjectWorldSub2>,
    _68: [u8; 0x8],
}

#[cfg(feature = "impl")]
impl BattleObjectWorld {
    pub fn instance() -> Option<&'static Self> {
        unsafe { impl_::INSTANCE.as_ref() }
    }

    pub fn instance_mut() -> Option<&'static mut Self> {
        unsafe { impl_::INSTANCE.as_mut() }
    }
}

#[repr(C)]
pub struct BattleObjectWorldSub1 {
    _0: u32,
    _8: usize,
    _10: usize,
    _18: u32,
    _1c: u32,
}

#[repr(C)]
pub struct BattleObjectWorldSub2 {
    _0: bool,
    _1: bool,
}

#[cfg(feature = "impl")]
mod impl_ {
    use crate::*;

    unsafe extern "C" {
        #[link_name = "_ZN3lib9SingletonIN3app17BattleObjectWorldEE9instance_E"]
        pub(super) static INSTANCE: *mut app::BattleObjectWorld;
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<BattleObjectWorld>(), 0x70);
        assert_eq!(offset_of!(BattleObjectWorld, _0), 0x0);
        assert_eq!(offset_of!(BattleObjectWorld, gravity_speed_coefficient), 0x4);
        assert_eq!(offset_of!(BattleObjectWorld, scale_z), 0x8);
        assert_eq!(offset_of!(BattleObjectWorld, _c), 0xC);
        assert_eq!(offset_of!(BattleObjectWorld, gravity_pos), 0x10);
        assert_eq!(offset_of!(BattleObjectWorld, move_speed), 0x20);
        assert_eq!(offset_of!(BattleObjectWorld, _30), 0x30);
        assert_eq!(offset_of!(BattleObjectWorld, _38), 0x38);
        assert_eq!(offset_of!(BattleObjectWorld, _58), 0x58);
        assert_eq!(offset_of!(BattleObjectWorld, is_gravity_normal), 0x59);
        assert_eq!(offset_of!(BattleObjectWorld, is_move), 0x5A);
        assert_eq!(offset_of!(BattleObjectWorld, _5b), 0x5B);
        assert_eq!(offset_of!(BattleObjectWorld, _5c), 0x5C);
        assert_eq!(offset_of!(BattleObjectWorld, is_disable_reverse), 0x5D);
        assert_eq!(offset_of!(BattleObjectWorld, _5e), 0x5E);
        assert_eq!(offset_of!(BattleObjectWorld, _60), 0x60);
    }
}
