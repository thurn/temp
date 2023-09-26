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

use std::collections::HashMap;

use enum_kinds::EnumKind;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::game::TurnData;
use crate::game_actions::CardTarget;
use crate::game_updates::InitiatedBy;
use crate::primitives::{AbilityId, CardId, ProgressValue, RoomId};

/// Records a single event which happened during this game.
#[derive(Debug, Clone, Serialize, Deserialize, EnumKind)]
#[enum_kind(HistoryEventKind)]
pub enum HistoryEvent {
    /// Mana was gained via the standard game action
    GainManaAction,
    /// A card was drawn via the standard game action
    DrawCardAction(CardId),
    /// A card was played, either via the standard game action or initiated by
    /// [an ability of another card.
    PlayCard(CardId, CardTarget, InitiatedBy),
    /// A card ability was activated
    ActivateAbility(AbilityId, CardTarget),
    /// A face-down card has been unveiled.
    UnveilCard(CardId),
    /// A raid was started, either via a card effect or the standard game action
    RaidBegin(RoomId, InitiatedBy),
    /// A raid ended in success.
    RaidSuccess(RoomId),
    /// A raid ended in failure.
    RaidFailure(RoomId),
    /// A card was progressed some number of times, either via a card effect or
    /// the standard game action
    CardProgress(RoomId, ProgressValue, InitiatedBy),
    /// The Champion has been dealt damage
    DealDamage(u32),
}

impl HistoryEvent {
    /// Returns the [HistoryEventKind] for this event
    pub fn kind(&self) -> HistoryEventKind {
        self.into()
    }
}

/// Tuple of [TurnData] and [HistoryEvent].
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HistoryEntry {
    turn: TurnData,
    event: HistoryEvent,
}

/// History of events which have happened during this game.
///
/// This operates via a two-phase system where history entries are collected
/// during action resolution, but are not immediately visible in the general
/// history until they are finalized by calling [Self::write_events], usually as
/// the final step of any game action. This helps avoid confusion where events
/// added during the *current* action appear in history, which is typically not
/// desired.
#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameHistory {
    current: Vec<HistoryEntry>,
    #[serde_as(as = "Vec<(_, _)>")]
    entries: HashMap<TurnData, Vec<HistoryEvent>>,
}

impl GameHistory {
    /// Returns history events in the provided turn, *before* the current game
    /// event.
    pub fn for_turn(&self, turn: TurnData) -> impl Iterator<Item = &HistoryEvent> {
        self.entries.get(&turn).into_iter().flatten()
    }

    /// Adds a new history entry to the 'current events' buffer. Events do
    /// not appear in the [Self::for_turn] history until they are finalized by
    /// calling [Self::write_events], which typically happens as the last step
    /// in processing a game action.
    pub fn add_event(&mut self, turn: TurnData, event: HistoryEvent) {
        self.current.push(HistoryEntry { turn, event })
    }

    /// Writes all stored history events to the game history and clears the
    /// 'current events' buffer.
    pub fn write_events(&mut self) {
        for entry in &self.current {
            self.entries.entry(entry.turn).or_default().push(entry.event.clone());
        }

        self.current.clear();
    }
}
