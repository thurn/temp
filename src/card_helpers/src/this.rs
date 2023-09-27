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

use game_data::delegate_data::{CardPlayed, Delegate, EventDelegate, MutationFn, Scope};
use game_data::game_state::GameState;
use game_data::primitives::HasCardId;

/// A RequirementFn which restricts delegates to only listen to events for their
/// own card.
pub fn card(_game: &GameState, scope: Scope, card_id: &impl HasCardId) -> bool {
    scope.card_id() == card_id.card_id()
}

/// A delegate which triggers when a card is played
pub fn on_play(mutation: MutationFn<CardPlayed>) -> Delegate {
    Delegate::PlayCard(EventDelegate { requirement: card, mutation })
}
