// Copyright © Spelldawn 2021-present

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//    https://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Implements state machine model for handling raids"

pub mod traits;

mod access;
pub mod approach_room;
mod begin;
mod defenders;
mod encounter;
mod summon;

use anyhow::Result;
use game_data::game::{
    GamePhase, GameState, HistoryEntry, HistoryEvent, InternalRaidPhase, RaidData, RaidJumpRequest,
};
use game_data::game_actions::{GamePrompt, PromptAction};
use game_data::primitives::{RaidId, RoomId, Side};
use game_data::updates::{GameUpdate, InitiatedBy};
use rules::{flags, mutations, queries};
use tracing::info;
use with_error::{verify, WithError};

use crate::access::AccessPhase;
use crate::approach_room::ApproachRoomPhase;
use crate::begin::BeginPhase;
use crate::encounter::EncounterPhase;
use crate::summon::SummonPhase;
use crate::traits::RaidPhase;

/// Extension trait to add the `phase` method to [RaidData] without introducing
/// cyclical crate dependencies.
pub trait RaidDataExt {
    fn phase(&self) -> Box<dyn RaidPhase>;
}

impl RaidDataExt for RaidData {
    fn phase(&self) -> Box<dyn RaidPhase> {
        match self.internal_phase {
            InternalRaidPhase::Begin => Box::new(BeginPhase {}),
            InternalRaidPhase::Summon => Box::new(SummonPhase {}),
            InternalRaidPhase::Encounter => Box::new(EncounterPhase {}),
            InternalRaidPhase::ApproachRoom => Box::new(ApproachRoomPhase {}),
            InternalRaidPhase::Access => Box::new(AccessPhase {}),
        }
    }
}

/// Handle a client request to initiate a new raid. Deducts action points and
/// then invokes [initiate].
pub fn handle_initiate_action(
    game: &mut GameState,
    user_side: Side,
    target_room: RoomId,
) -> Result<()> {
    verify!(
        flags::can_take_initiate_raid_action(game, user_side, target_room),
        "Cannot initiate raid for {:?}",
        user_side
    );
    mutations::spend_action_points(game, user_side, 1)?;
    initiate(game, target_room, InitiatedBy::GameAction, |_, _| {})
}

/// Starts a new raid, either as a result of an explicit game action or via a
/// card effect (as differentiated by the [InitiatedBy] prop). Invokes the
/// `on_begin` function immediately with the [RaidId] that will be used for this
/// raid, before any other game logic runs.
pub fn initiate(
    game: &mut GameState,
    target_room: RoomId,
    initiated_by: InitiatedBy,
    on_begin: impl Fn(&mut GameState, RaidId),
) -> Result<()> {
    let raid_id = RaidId(game.info.next_raid_id);
    let phase = InternalRaidPhase::Begin;
    let raid = RaidData {
        target: target_room,
        raid_id,
        internal_phase: phase,
        encounter: None,
        accessed: vec![],
        jump_request: None,
    };

    game.info.next_raid_id += 1;
    game.info.raid = Some(raid);
    on_begin(game, raid_id);
    game.record_update(|| GameUpdate::InitiateRaid(target_room, initiated_by));
    enter_phase(game, Some(phase))?;
    game.history
        .push(HistoryEntry { turn: game.info.turn, event: HistoryEvent::RaidBegan(target_room) });

    Ok(())
}

/// Handles a [PromptAction] supplied by a user during a raid. Returns an error
/// if no raid is currently active or if this action was not expected from this
/// player.
pub fn handle_action(game: &mut GameState, user_side: Side, action: PromptAction) -> Result<()> {
    let raid = game.raid()?;
    let phase = raid.phase();
    verify!(raid.internal_phase.active_side() == user_side, "Unexpected side");
    verify!(phase.prompts(game)?.iter().any(|c| c == &action), "Unexpected action");

    info!(?user_side, ?action, "Handling raid action");
    let mut new_state = phase.handle_prompt(game, action)?;
    new_state = apply_jump(game)?.or(new_state);

    if game.info.raid.is_some() {
        enter_phase(game, new_state)
    } else {
        Ok(())
    }
}

/// Returns a list of the user actions which are possible in the current raid
/// state for the `side` player, or `None` if no such actions are possible.
pub fn current_actions(game: &GameState, user_side: Side) -> Result<Option<Vec<PromptAction>>> {
    if game.info.phase != GamePhase::Play {
        return Ok(None);
    }

    if let Some(raid) = &game.info.raid {
        if raid.internal_phase.active_side() == user_side {
            let prompts = raid.phase().prompts(game)?;
            if !prompts.is_empty() {
                return Ok(Some(prompts));
            }
        }
    }

    Ok(None)
}

/// Builds a [GamePrompt] representing the possible actions for the `side` user,
/// as determined by the [current_actions] function.
pub fn current_prompt(game: &GameState, user_side: Side) -> Result<Option<GamePrompt>> {
    if let Some(actions) = current_actions(game, user_side)? {
        Ok(Some(GamePrompt { context: game.raid()?.phase().prompt_context(), responses: actions }))
    } else {
        Ok(None)
    }
}

/// Sets the game to a new raid phase and invokes callbacks as needed.
fn enter_phase(game: &mut GameState, mut phase: Option<InternalRaidPhase>) -> Result<()> {
    loop {
        if let Some(s) = phase {
            game.raid_mut()?.internal_phase = s;
            phase = game.raid()?.phase().enter(game)?;
            phase = apply_jump(game)?.or(phase);
        } else {
            return Ok(());
        }
    }
}

/// Implements a [RaidJumpRequest], if one has been specified for the current
/// raid.
fn apply_jump(game: &mut GameState) -> Result<Option<InternalRaidPhase>> {
    if let Some(raid) = &game.info.raid {
        if let Some(RaidJumpRequest::EncounterMinion(card_id)) = raid.jump_request {
            let (room_id, index) =
                queries::minion_position(game, card_id).with_error(|| "Minion not found")?;
            let raid = game.raid_mut()?;
            raid.target = room_id;
            raid.encounter = Some(index);
            raid.jump_request = None;
            return Ok(Some(InternalRaidPhase::Encounter));
        }
    }

    Ok(None)
}
