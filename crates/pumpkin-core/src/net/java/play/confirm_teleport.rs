#[allow(clippy::wildcard_imports)]
use super::*;

impl JavaClient {
    pub fn handle_confirm_teleport(&self, player: &Player, confirm_teleport: &SConfirmTeleport) {
        enum TeleportResult {
            Success,
            WrongId,
            NotTeleporting,
        }

        let result = {
            let mut awaiting_teleport = player
                .awaiting_teleport
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some((id, position)) = awaiting_teleport.as_ref() {
                if id == &confirm_teleport.teleport_id {
                    // We should set the position now to what we requested in the teleport packet.
                    // This may fix issues when the client sends the position while being teleported.
                    player.get_entity().set_pos(*position);
                    *awaiting_teleport = None;
                    TeleportResult::Success
                } else {
                    TeleportResult::WrongId
                }
            } else {
                TeleportResult::NotTeleporting
            }
        };

        match result {
            // WrongId: a confirm for an older teleport arrives when a newer one was sent before the client
            // answered (two plugins teleporting at once). Vanilla ignores it and waits for the newest.
            TeleportResult::Success | TeleportResult::WrongId => {}
            // Two teleports in a row can reach the client newest-first; the older confirm then arrives
            // after the newest cleared the wait. Like vanilla, only a repeated confirm of the last id kicks.
            TeleportResult::NotTeleporting
                if confirm_teleport.teleport_id.0
                    != player.teleport_id_count.load(std::sync::atomic::Ordering::Relaxed) => {}
            TeleportResult::NotTeleporting => {
                self.try_kick(&TextComponent::text(
                    "Send Teleport confirm, but we did not teleport",
                ));
            }
        }
    }
}
