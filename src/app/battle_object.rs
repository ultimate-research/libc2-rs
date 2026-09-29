mod battle_object_category;
mod battle_object_manager;
mod battle_object_slow;
mod battle_object_world;

pub use battle_object_category::*;
pub use battle_object_manager::*;
pub use battle_object_slow::*;
pub use battle_object_world::*;

#[repr(C)]
pub struct BattleObject {}
