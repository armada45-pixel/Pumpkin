use pumpkin_macros::{Event, cancellable};
use pumpkin_util::math::vector3::Vector3;

/// An event that occurs when an entity picks up an item stack.
#[cancellable]
#[derive(Event, Clone)]
pub struct EntityPickupItemEvent {
    /// The ID of the picking entity.
    pub entity_id: i32,

    /// The registry name of the item (`minecraft:diamond`), or the numeric entity type id for an XP orb, arrow or trident.
    pub item_name: String,

    /// The count of items picked up.
    pub count: u8,

    /// The ID of the item entity being picked up.
    pub item_entity_id: i32,

    /// Where the item entity is.
    pub item_position: Vector3<f64>,
}

impl EntityPickupItemEvent {
    #[must_use]
    pub const fn new(
        entity_id: i32,
        item_name: String,
        count: u8,
        item_entity_id: i32,
        item_position: Vector3<f64>,
    ) -> Self {
        Self {
            entity_id,
            item_name,
            count,
            item_entity_id,
            item_position,
            cancelled: false,
        }
    }
}
