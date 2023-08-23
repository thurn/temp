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

use anyhow::Result;
use game_data::card_state::CardPosition;
use game_data::delegates::{
    CardAccessEvent, ChampionScoreCardEvent, RaidAccessSelectedEvent, RaidAccessStartEvent,
    RaidEvent, RaidOutcome, ScoreCard, ScoreCardEvent,
};
use game_data::game::{GameState, InternalRaidPhase};
use game_data::game_actions::{AccessPhaseAction, PromptAction, RazeCardActionType};
use game_data::primitives::{CardId, CardType, RoomId, Side};
use game_data::random;
use game_data::updates::GameUpdate;
use rules::mana::ManaPurpose;
use rules::{dispatch, flags, mana, mutations, queries};
use with_error::{fail, WithError};

use crate::traits::{RaidDisplayState, RaidPhaseImpl};

/// Final step of a raid, in which cards are accessed by the Champion
#[derive(Debug, Clone, Copy)]
pub struct AccessPhase {}

impl RaidPhaseImpl for AccessPhase {
    type Action = AccessPhaseAction;

    fn unwrap(action: PromptAction) -> Result<AccessPhaseAction> {
        match action {
            PromptAction::AccessPhaseAction(action) => Ok(action),
            _ => fail!("Expected AccessPhaseAction"),
        }
    }

    fn wrap(action: AccessPhaseAction) -> Result<PromptAction> {
        Ok(PromptAction::AccessPhaseAction(action))
    }

    fn enter(self, game: &mut GameState) -> Result<Option<InternalRaidPhase>> {
        dispatch::invoke_event(game, RaidAccessStartEvent(game.raid()?.raid_id))?;
        if game.info.raid.is_none() {
            return Ok(None);
        }

        game.raid_mut()?.accessed = select_accessed_cards(game)?;

        dispatch::invoke_event(
            game,
            RaidAccessSelectedEvent(RaidEvent {
                raid_id: game.raid()?.raid_id,
                target: game.raid()?.target,
            }),
        )?;

        let accessed = game.raid()?.accessed.clone();
        for card_id in &accessed {
            game.card_mut(*card_id).set_revealed_to(Side::Champion, true);
        }

        for card_id in &accessed {
            dispatch::invoke_event(game, CardAccessEvent(*card_id))?;
        }

        Ok(None)
    }

    fn actions(self, game: &GameState) -> Result<Vec<AccessPhaseAction>> {
        let raid = game.raid()?;
        let can_end = flags::can_take_end_raid_access_phase_action(game, raid.raid_id);
        Ok(raid
            .accessed
            .iter()
            .filter_map(|card_id| access_action_for_card(game, *card_id))
            .chain(can_end.then_some(AccessPhaseAction::EndRaid).into_iter())
            .collect())
    }

    fn handle_action(
        self,
        game: &mut GameState,
        action: AccessPhaseAction,
    ) -> Result<Option<InternalRaidPhase>> {
        match action {
            AccessPhaseAction::ScoreCard(card_id) => handle_score_card(game, card_id),
            AccessPhaseAction::RazeCard(card_id, _, _) => handle_raze_card(game, card_id),
            AccessPhaseAction::EndRaid => mutations::end_raid(game, RaidOutcome::Success),
        }?;

        Ok(None)
    }

    fn display_state(self, _: &GameState) -> Result<RaidDisplayState> {
        Ok(RaidDisplayState::Access)
    }
}

/// Returns a vector of the cards accessed for the current raid target, mutating
/// the [GameState] to store the results of random zone selections.
fn select_accessed_cards(game: &mut GameState) -> Result<Vec<CardId>> {
    let target = game.raid()?.target;

    let accessed = match target {
        RoomId::Vault => mutations::realize_top_of_deck(
            game,
            Side::Overlord,
            queries::vault_access_count(game)?,
        )?,
        RoomId::Sanctum => {
            let count = queries::sanctum_access_count(game)?;

            random::cards_in_position(
                game,
                Side::Overlord,
                CardPosition::Hand(Side::Overlord),
                count as usize,
            )
        }
        RoomId::Crypts => {
            game.card_list_for_position(Side::Overlord, CardPosition::DiscardPile(Side::Overlord))
        }
        _ => game.occupants(target).map(|c| c.id).collect(),
    };

    Ok(accessed)
}

/// Returns an [AccessPhaseAction] for the Champion to access the provided
/// `card_id`, if any action can be taken.
fn access_action_for_card(game: &GameState, card_id: CardId) -> Option<AccessPhaseAction> {
    let definition = rules::card_definition(game, card_id);
    match definition.card_type {
        CardType::Scheme if can_score_card(game, card_id) => {
            Some(AccessPhaseAction::ScoreCard(card_id))
        }
        CardType::Project if can_raze_project(game, card_id) => {
            let raze_type = if game.card(card_id).position().in_play() {
                RazeCardActionType::Destroy
            } else {
                RazeCardActionType::Discard
            };
            Some(AccessPhaseAction::RazeCard(card_id, raze_type, queries::raze_cost(game, card_id)))
        }
        _ => None,
    }
}

/// Can the Champion player score the `card_id` card when accessed during a
/// raid?
fn can_score_card(game: &GameState, card_id: CardId) -> bool {
    let raid = match &game.info.raid {
        Some(r) => r,
        None => return false,
    };

    raid.accessed.contains(&card_id)
        && rules::card_definition(game, card_id).config.stats.scheme_points.is_some()
}

/// Can the Champion player raze the `card_id` project when accessed during a
/// raid?
fn can_raze_project(game: &GameState, card_id: CardId) -> bool {
    !game.card(card_id).position().in_discard_pile()
        && queries::raze_cost(game, card_id)
            <= mana::get(game, Side::Champion, ManaPurpose::RazeCard(card_id))
}

fn handle_score_card(game: &mut GameState, card_id: CardId) -> Result<()> {
    game.card_mut(card_id).turn_face_up();
    mutations::move_card(game, card_id, CardPosition::Scoring)?;
    game.raid_mut()?.accessed.retain(|c| *c != card_id);

    game.record_update(|| GameUpdate::ScoreCard(Side::Champion, card_id));

    dispatch::invoke_event(game, ChampionScoreCardEvent(card_id))?;
    dispatch::invoke_event(game, ScoreCardEvent(ScoreCard { player: Side::Champion, card_id }))?;

    let scheme_points = rules::card_definition(game, card_id)
        .config
        .stats
        .scheme_points
        .with_error(|| format!("Expected SchemePoints for {card_id:?}"))?;
    mutations::score_points(game, Side::Champion, scheme_points.points)?;

    mutations::move_card(game, card_id, CardPosition::Scored(Side::Champion))?;
    Ok(())
}

fn handle_raze_card(game: &mut GameState, card_id: CardId) -> Result<()> {
    mana::spend(
        game,
        Side::Champion,
        ManaPurpose::RazeCard(card_id),
        queries::raze_cost(game, card_id),
    )?;
    mutations::move_card(game, card_id, CardPosition::DiscardPile(Side::Overlord))?;
    game.raid_mut()?.accessed.retain(|c| *c != card_id);
    Ok(())
}
