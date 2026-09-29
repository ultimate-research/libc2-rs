use crate::{
    cpp,
    lib::FixedBaseString64,
    phx::{Hash40, Vector3f},
};

#[repr(C)]
pub struct StageDynamicObject {
    vtable: &'static StageDynamicObjectVtable,

    /// The lowercase name of the dynamic object (e.g., `"dyr_ring_lift_set__villagec__"`).
    pub raw_name: FixedBaseString64,

    /// The lowercase name of the dynamic object excluding the optional unique identifier (e.g., `"dyr_ring_lift_set"`).
    pub name: FixedBaseString64,

    /// The hashed lowercase name of the dynamic object (e.g., `Hash40::new("dyr_ring_lift_set__villagec__")`).
    pub raw_name_hash: Hash40,

    /// The hashed lowercase name of the dynamic object excluding the optional unique identifier (e.g., `Hash40::new("dyr_ring_lift_set")`).
    pub name_hash: Hash40,

    _a8: usize,

    /// The initial position of the dynamic object.
    ///
    /// If two or more dynamic objects are combined into one, the first `dynamic_offset` value will be used.
    pub dynamic_offset: Vector3f,

    /// The index of the joint which a dynamic object is parented to.
    ///
    /// If two or more dynamic objects are combined into one, the first `joint_index` value will be used.
    pub joint_index: i32,

    /// The name of the joint which a dynamic object is parented to.
    ///
    /// If two or more dynamic objects are combined into one, the first `joint_name` will be used.
    /// This field does not get written to if the joint_index is greater than -1.
    pub joint_name: FixedBaseString64,

    /// The pointer to a collection of dynamic ground collision objects and their cliff objects.
    pub ground_edge_collection: cpp::UniquePtr<()>, // StageGroundEdgeCollection

    /// The hashed file path to the LVD file containing the dynamic object.
    pub resource_path: Hash40,

    /// The pointer to a collection of damage collision shapes.
    pub damagee_shape_collection: cpp::UniquePtr<()>, // StageDamageShapeCollection

    /// The pointer to a collection of attack collision shapes.
    pub damager_shape_collection: cpp::UniquePtr<()>, // StageDamageShapeCollection

    /// The pointer to a collection of dynamic item popup regions.
    pub item_popup_region_collection: cpp::UniquePtr<()>, // StageItemPopupRegionCollection
}

#[repr(C)]
struct StageDynamicObjectVtable {
    destroy: extern "C" fn(*mut StageDynamicObject),
    delete: extern "C" fn(*mut StageDynamicObject),
}
/*
#[repr(C)]
pub struct StageDamageShape {
    pub shape: app::CollisionShape,
    pub id: u32,
}
*/
#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageDynamicObject>(), 0x140);
        assert_eq!(offset_of!(StageDynamicObject, vtable), 0x0);
        assert_eq!(offset_of!(StageDynamicObject, raw_name), 0x8);
        assert_eq!(offset_of!(StageDynamicObject, name), 0x50);
        assert_eq!(offset_of!(StageDynamicObject, raw_name_hash), 0x98);
        assert_eq!(offset_of!(StageDynamicObject, name_hash), 0xA0);
        assert_eq!(offset_of!(StageDynamicObject, _a8), 0xA8);
        assert_eq!(offset_of!(StageDynamicObject, dynamic_offset), 0xB0);
        assert_eq!(offset_of!(StageDynamicObject, joint_index), 0xC0);
        assert_eq!(offset_of!(StageDynamicObject, joint_name), 0xC4);
        assert_eq!(offset_of!(StageDynamicObject, ground_edge_collection), 0x110);
        assert_eq!(offset_of!(StageDynamicObject, resource_path), 0x118);
        assert_eq!(offset_of!(StageDynamicObject, damagee_shape_collection), 0x120);
        assert_eq!(offset_of!(StageDynamicObject, damager_shape_collection), 0x128);
        assert_eq!(offset_of!(StageDynamicObject, item_popup_region_collection), 0x130);

        // assert_eq!(size_of::<StageDamageShape>, 0x40);
        // assert_eq!(offset_of!(StageDamageShape, shape), 0x0);
        // assert_eq!(offset_of!(StageDamageShape, id), 0x30);
    }
}
