#[cfg(feature = "impl")]
use crate::app::BattleObject;

#[repr(C)]
pub struct BattleObjectManager {}

#[cfg(feature = "impl")]
impl BattleObjectManager {
    pub fn find_battle_object(id: u32) -> *mut BattleObject {
        unsafe {
            #[skyline::from_offset(0x3AC560)]
            fn inner(id: u32) -> *mut BattleObject;

            inner(id)
        }
    }
}
