#[repr(C)]
pub struct BattleObjectSlow {
    pub _0: i32,
    pub rate: f32,
    pub _8: bool,
}

#[cfg(feature = "impl")]
impl BattleObjectSlow {
    pub fn instance() -> Option<&'static Self> {
        unsafe { impl_::INSTANCE.as_ref() }
    }

    pub fn instance_mut() -> Option<&'static mut Self> {
        unsafe { impl_::INSTANCE.as_mut() }
    }

    pub fn rate(&self) -> f32 {
        self.rate
    }

    // pub fn rate_request(&self) -> f32;

    pub fn is_estimate(&self) -> bool {
        if self._8 {
            return self._0 == 0;
        }

        true
    }

    pub fn is_adjust(&self) -> bool {
        if self._8 {
            return self._0 == 2;
        }

        true
    }
}

#[cfg(feature = "impl")]
mod impl_ {
    use crate::*;

    unsafe extern "C" {
        #[link_name = "_ZN3lib9SingletonIN3app16BattleObjectSlowEE9instance_E"]
        pub(super) static INSTANCE: *mut app::BattleObjectSlow;
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<BattleObjectSlow>(), 0xC);
        assert_eq!(offset_of!(BattleObjectSlow, _0), 0x0);
        assert_eq!(offset_of!(BattleObjectSlow, rate), 0x4);
        assert_eq!(offset_of!(BattleObjectSlow, _8), 0x8);
    }
}
