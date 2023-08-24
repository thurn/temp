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
use game_data::game::{GamePhase, GameState, MulliganDecision};
use game_data::game_actions::{ButtonPrompt, GamePrompt, PromptAction};
use game_data::primitives::Side;
use prompts::prompts;
use protos::spelldawn::InterfaceMainControls;
use rules::flags;

use crate::card_browser;

/// Returns a [InterfaceMainControls] to render the interface state for the
/// provided `game`.
pub fn render(game: &GameState, side: Side) -> Result<Option<InterfaceMainControls>> {
    let current_prompt = &game.player(side).prompt_queue.get(0);
    if let Some(GamePrompt::ButtonPrompt(prompt)) = current_prompt {
        return prompts::action_prompt(game, side, prompt);
    } else if let Some(GamePrompt::CardBrowserPrompt(prompt)) = current_prompt {
        return Ok(card_browser::controls(prompt));
    } else if let Some(prompt) = raids::current_prompt(game, side)? {
        return prompts::action_prompt(game, side, &prompt);
    } else if let GamePhase::ResolveMulligans(_) = &game.info.phase {
        if flags::can_make_mulligan_decision(game, side) {
            return prompts::action_prompt(
                game,
                side,
                &ButtonPrompt {
                    context: None,
                    responses: vec![
                        PromptAction::MulliganDecision(MulliganDecision::Keep),
                        PromptAction::MulliganDecision(MulliganDecision::Mulligan),
                    ],
                },
            );
        }
    } else if flags::can_take_start_turn_action(game, side) {
        return prompts::action_prompt(
            game,
            side,
            &ButtonPrompt { context: None, responses: vec![PromptAction::StartTurnAction] },
        );
    } else if flags::can_take_end_turn_action(game, side) {
        return prompts::action_prompt(
            game,
            side,
            &ButtonPrompt { context: None, responses: vec![PromptAction::EndTurnAction] },
        );
    }

    Ok(None)
}
