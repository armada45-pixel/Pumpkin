use crate::entity::player::Player;
use pumpkin_macros::Event;
use std::sync::Arc;

/// An event that occurs when an item is prepared in an anvil.
#[derive(Event, Clone)]
pub struct PrepareAnvilEvent {
    /// The player using the anvil.
    pub player: Arc<Player>,

    /// The name prepared in the anvil input text field.
    pub rename_text: String,

    /// The repair cost in experience levels.
    pub repair_cost: u32,

    /// The cost at or above which the result is "Too Expensive!". Vanilla default 40; handlers may change it.
    pub maximum_repair_cost: u32,

    /// True when the second input slot is empty.
    pub second_input_empty: bool,
}

impl PrepareAnvilEvent {
    #[must_use]
    pub const fn new(
        player: Arc<Player>,
        rename_text: String,
        repair_cost: u32,
        maximum_repair_cost: u32,
        second_input_empty: bool,
    ) -> Self {
        Self {
            player,
            rename_text,
            repair_cost,
            maximum_repair_cost,
            second_input_empty,
        }
    }
}
