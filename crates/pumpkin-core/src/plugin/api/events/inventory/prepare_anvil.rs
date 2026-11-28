use crate::entity::player::Player;
use pumpkin_data::item_stack::ItemStack;
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

    /// Set by a handler to change the cost at or above which the result is "Too Expensive!" (vanilla 40).
    /// `None` leaves the anvil's current maximum unchanged.
    pub maximum_repair_cost: Option<u32>,

    /// True when the second input slot is empty.
    pub second_input_empty: bool,

    /// The first input slot.
    pub first_input: ItemStack,

    /// The second input slot.
    pub second_input: ItemStack,

    /// The result the anvil prepared (before the "Too Expensive!" check).
    pub result: ItemStack,
}

impl PrepareAnvilEvent {
    #[must_use]
    pub const fn new(
        player: Arc<Player>,
        rename_text: String,
        repair_cost: u32,
        first_input: ItemStack,
        second_input: ItemStack,
        result: ItemStack,
    ) -> Self {
        Self {
            player,
            rename_text,
            repair_cost,
            maximum_repair_cost: None,
            second_input_empty: second_input.is_empty(),
            first_input,
            second_input,
            result,
        }
    }
}
