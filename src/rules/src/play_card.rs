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

use std::iter;

use anyhow::Result;
use constants::game_constants;
use game_data::animation_tracker::{GameAnimation, InitiatedBy};
use game_data::card_state::{CardPosition, CardState};
use game_data::delegate_data::{CardPlayed, PlayCardEvent};
use game_data::game_actions::{
    ButtonPrompt, ButtonPromptContext, CardTarget, GamePrompt, PromptChoice, PromptChoiceLabel,
    UnplayedAction,
};
use game_data::game_effect::GameEffect;
use game_data::game_history::HistoryEvent;
use game_data::game_state::{GamePhase, GameState};
use game_data::primitives::{CardId, CardSubtype, CardType, Side};
use game_data::state_machines::{PlayCardData, PlayCardStep};
use with_error::{verify, WithError};

use crate::mana::ManaPurpose;
use crate::{dispatch, flags, mana, mutations, queries, CardDefinitionExt};

/// Starts a new play card action, either as a result the explicit game action
/// or as an effect of another card.
pub fn initiate(game: &mut GameState, card_id: CardId, target: CardTarget) -> Result<()> {
    verify!(game.state_machines.play_card.is_none(), "An action is already being resolved!");

    let initiated_by = if let Some(GamePrompt::PlayCardBrowser(prompt)) =
        game.player(card_id.side).prompt_queue.get(0)
    {
        InitiatedBy::Ability(prompt.initiated_by)
    } else {
        InitiatedBy::GameAction
    };

    game.state_machines.play_card =
        Some(PlayCardData { card_id, initiated_by, target, step: PlayCardStep::Begin });

    run(game)
}

/// Run the play card state machine, if needed.
///
/// This will advance the state machine through its steps. The state machine
/// pauses if a player is presented with a prompt to respond to, and aborts if
/// the action is aborted. If no play action action is currently active or the
/// state machine cannot currently advance, this function silently ignores the
/// run request.
pub fn run(game: &mut GameState) -> Result<()> {
    loop {
        if has_non_play_prompt(&game.overlord.prompt_queue)
            || has_non_play_prompt(&game.champion.prompt_queue)
        {
            // We pause the state machine if a player has a prompt. We do *not* pause for
            // the PlayCardBrowser prompt since this would prevent anyone from
            // being able to play cards from that browser.
            break;
        }

        if game.info.phase != GamePhase::Play {
            break;
        }

        if let Some(play_card) = game.state_machines.play_card {
            let step = evaluate_play_step(game, play_card)?;
            if let Some(play) = &mut game.state_machines.play_card {
                play.step = step;
            }
        } else {
            break;
        }
    }
    Ok(())
}

/// Returns true if the provided prompt queue currently contains a prompt which
/// is *not* the PlayCardBrowser prompt.
fn has_non_play_prompt(queue: &[GamePrompt]) -> bool {
    if !queue.is_empty() {
        !matches!(queue.get(0), Some(GamePrompt::PlayCardBrowser(_)))
    } else {
        false
    }
}

fn evaluate_play_step(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    match play_card.step {
        PlayCardStep::Begin => Ok(PlayCardStep::CheckLimits),
        PlayCardStep::CheckLimits => check_limits(game, play_card),
        PlayCardStep::AddToHistory => add_to_history(game, play_card),
        PlayCardStep::MoveToPlayedPosition => move_to_played_position(game, play_card),
        PlayCardStep::PayActionPoints => pay_action_points(game, play_card),
        PlayCardStep::ApplyPlayCardBrowser => apply_play_card_browser(game, play_card),
        PlayCardStep::PayManaCost => pay_mana_cost(game, play_card),
        PlayCardStep::PayCustomCost => pay_custom_cost(game, play_card),
        PlayCardStep::TurnFaceUp => turn_face_up(game, play_card),
        PlayCardStep::MoveToTargetPosition => move_to_target_position(game, play_card),
        PlayCardStep::Finish => finish(game, play_card),
    }
}

fn check_limits(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    let definition = game.card(play_card.card_id).definition();
    let prompt = match play_card.target {
        CardTarget::None => match definition.card_type {
            CardType::Artifact
                if definition.subtypes.contains(&CardSubtype::Weapon)
                    && game_weapons(game).count() >= game_constants::MAXIMUM_WEAPONS_IN_PLAY =>
            {
                Some(card_limit_prompt(
                    game_weapons(game),
                    ButtonPromptContext::CardLimit(CardType::Artifact, Some(CardSubtype::Weapon)),
                ))
            }
            CardType::Artifact
                if game.artifacts().count() >= game_constants::MAXIMUM_ARTIFACTS_IN_PLAY =>
            {
                Some(card_limit_prompt(
                    game.artifacts(),
                    ButtonPromptContext::CardLimit(CardType::Artifact, None),
                ))
            }
            CardType::Evocation
                if game.evocations().count() >= game_constants::MAXIMUM_EVOCATIONS_IN_PLAY =>
            {
                Some(card_limit_prompt(
                    game.evocations(),
                    ButtonPromptContext::CardLimit(CardType::Evocation, None),
                ))
            }
            CardType::Ally if game.allies().count() >= game_constants::MAXIMUM_ALLIES_IN_PLAY => {
                Some(card_limit_prompt(
                    game.allies(),
                    ButtonPromptContext::CardLimit(CardType::Ally, None),
                ))
            }
            _ => None,
        },
        CardTarget::Room(room_id) => match definition.card_type {
            CardType::Minion
                if game.defenders_unordered(room_id).count()
                    >= game_constants::MAXIMUM_MINIONS_IN_ROOM =>
            {
                Some(card_limit_prompt(
                    game.defenders_unordered(room_id),
                    ButtonPromptContext::CardLimit(CardType::Minion, None),
                ))
            }
            CardType::Project | CardType::Scheme
                if game.occupants(room_id).count() >= game_constants::MAXIMUM_OCCUPANTS_IN_ROOM =>
            {
                Some(card_limit_prompt(
                    game.occupants(room_id),
                    ButtonPromptContext::CardLimit(definition.card_type, None),
                ))
            }
            _ => None,
        },
    };

    if let Some(p) = prompt {
        game.player_mut(play_card.card_id.side).prompt_queue.push(p);
    }

    Ok(PlayCardStep::AddToHistory)
}

fn add_to_history(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    game.add_history_event(HistoryEvent::PlayCard(
        play_card.card_id,
        play_card.target,
        play_card.initiated_by,
    ));
    Ok(PlayCardStep::MoveToPlayedPosition)
}

fn move_to_played_position(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    mutations::move_card(
        game,
        play_card.card_id,
        CardPosition::Played(play_card.card_id.side, play_card.target),
    )?;
    Ok(PlayCardStep::PayActionPoints)
}

fn pay_action_points(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    let actions = queries::action_cost(game, play_card.card_id);
    mutations::spend_action_points(game, play_card.card_id.side, actions)?;
    Ok(PlayCardStep::ApplyPlayCardBrowser)
}

fn apply_play_card_browser(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    invoke_play_card_browser(game, play_card.card_id.side, Some(play_card.card_id))?;
    Ok(PlayCardStep::PayManaCost)
}

/// Handles resolution of a [GamePrompt] with a `PlayCardBrowser`. Fires the
/// [UnplayedAction] for this browser and clears the user's prompt queue.
pub fn invoke_play_card_browser(
    game: &mut GameState,
    side: Side,
    card_id: Option<CardId>,
) -> Result<()> {
    if let Some(GamePrompt::PlayCardBrowser(prompt)) = game.player(side).prompt_queue.get(0) {
        if let Some(id) = card_id {
            verify!(prompt.cards.contains(&id), "Unexpected prompt card");
        }

        match prompt.unplayed_action {
            UnplayedAction::None => {}
            UnplayedAction::Discard => {
                let discard = prompt
                    .cards
                    .iter()
                    .copied()
                    .filter(|id| Some(*id) != card_id)
                    .collect::<Vec<_>>();
                for card_id in discard {
                    mutations::discard_card(game, card_id)?;
                }
            }
        }

        game.player_mut(side).prompt_queue.remove(0);
    }
    Ok(())
}

fn pay_mana_cost(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    if flags::enters_play_face_up(game, play_card.card_id) {
        let amount =
            queries::mana_cost(game, play_card.card_id).with_error(|| "Card has no mana cost")?;
        mana::spend(
            game,
            play_card.card_id.side,
            ManaPurpose::PayForCard(play_card.card_id),
            amount,
        )?;

        Ok(PlayCardStep::PayCustomCost)
    } else {
        Ok(PlayCardStep::MoveToTargetPosition)
    }
}

fn pay_custom_cost(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    let definition = game.card(play_card.card_id).definition();
    if let Some(custom_cost) = &definition.cost.custom_cost {
        (custom_cost.pay)(game, play_card.card_id)?;
    }
    Ok(PlayCardStep::TurnFaceUp)
}

fn turn_face_up(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    mutations::turn_face_up(game, play_card.card_id);
    game.add_animation(|| GameAnimation::PlayCard(play_card.card_id.side, play_card.card_id));
    Ok(PlayCardStep::MoveToTargetPosition)
}

fn move_to_target_position(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    mutations::move_card(
        game,
        play_card.card_id,
        queries::played_position(game, play_card.card_id.side, play_card.card_id, play_card.target)
            .with_error(|| "No valid position")?,
    )?;
    Ok(PlayCardStep::Finish)
}

fn finish(game: &mut GameState, play_card: PlayCardData) -> Result<PlayCardStep> {
    dispatch::invoke_event(
        game,
        PlayCardEvent(CardPlayed { card_id: play_card.card_id, target: play_card.target }),
    )?;

    game.state_machines.play_card = None;
    Ok(PlayCardStep::Finish)
}

fn game_weapons(game: &GameState) -> impl Iterator<Item = &CardState> {
    game.artifacts()
        .filter(|card| game.card(card.id).definition().subtypes.contains(&CardSubtype::Weapon))
}

fn card_limit_prompt<'a>(
    cards: impl Iterator<Item = &'a CardState>,
    context: ButtonPromptContext,
) -> GamePrompt {
    GamePrompt::ButtonPrompt(ButtonPrompt {
        context: Some(context),
        choices: cards
            .map(|existing| PromptChoice {
                effects: vec![GameEffect::SacrificeCard(existing.id)],
                anchor_card: Some(existing.id),
                custom_label: Some(PromptChoiceLabel::Sacrifice),
            })
            .chain(iter::once(PromptChoice::new().effect(GameEffect::AbortPlayingCard)))
            .collect(),
    })
}
