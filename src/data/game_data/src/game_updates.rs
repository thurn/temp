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

use crate::game::GameState;
use crate::primitives::{AbilityId, CardId, GameObjectId, RoomId, Side};
use crate::special_effects::SpecialEffect;

/// Indicates one game object targeted another with an effect.
///
/// Typically represented in animation as a projectile being fired.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TargetedInteraction {
    pub source: GameObjectId,
    pub target: GameObjectId,
}

/// Identifies whether some game update was caused by a player taking an
/// explicit game action such as the 'initiate raid' action, or by a card
/// effect.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InitiatedBy {
    GameAction,
    Card,
}

/// Represents a change to the state of the game which should be translated
/// into a client animation
#[derive(Debug, Clone)]
pub enum GameUpdate {
    /// Indicates that a player's turn has started
    StartTurn(Side),
    /// A player has played a card face-up.
    PlayCard(Side, CardId),
    /// A request to play one or more visual or audio effects
    CustomEffects(Vec<SpecialEffect>),
    /// A player has activated an ability of a card
    AbilityActivated(Side, AbilityId),
    /// An ability of a card has triggered an effect
    AbilityTriggered(AbilityId, Vec<SpecialEffect>),
    /// One or more cards have been drawn by the [Side] player.
    DrawCards(Side, Vec<CardId>),
    /// A player has shuffled cards into their deck
    ShuffleIntoDeck,
    /// A card has been turned face-up via the unveil mechanic, typically a
    /// project card
    UnveilCard(CardId),
    /// A minion card has been turned face-up.
    SummonMinion(CardId),
    /// The Overlord has leveled up a room
    LevelUpRoom(RoomId, InitiatedBy),
    /// The Champion has initiated a raid on a room
    InitiateRaid(RoomId, InitiatedBy),
    /// Interaction between two cards during raid combat.
    CombatInteraction(TargetedInteraction),
    /// A player has scored a card
    ScoreCard(Side, CardId),
    /// The game has ended and the indicated player has won
    GameOver(Side),
    /// Card selection browser has completed
    BrowserSubmitted,
    /// Show a 'play card' browser to play one of the indicated cards.
    ShowPlayCardBrowser(Vec<CardId>),
}

/// A step in the animation process
#[derive(Debug, Clone)]
pub struct UpdateStep {
    pub snapshot: GameState,
    pub update: GameUpdate,
}

/// Standard enum used by APIs to configure their update tracking behavior.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UpdateState {
    /// Game updates should not be tracked by the receiver
    Ignore,
    /// Game updates should be tracked by the receiver
    Push,
}

/// Tracks game mutations for a game action.
///
/// Some game state changes in Spelldawn require custom animations in the UI in
/// order to communicate their effects clearly. In order to implement the
/// animation system, code which mutates game state can also call
/// [GameState::record_update] and provide a [GameUpdate] to record the action
/// they took. The way this process works is that a snapshot of the game state
/// is stored (to capture any mutations that occurred *before* the animation),
/// and then the update is stored. During the animation process, the
/// stored snapshots and [GameUpdate]s are played back sequentially.
///
/// Many types of state changes are handled automatically by the game state
/// snapshot system, so appending an update is only needed for custom
/// animations. For example the system will correctly detect and animate a card
/// which has moved to a new position.
#[derive(Debug, Clone)]
pub struct UpdateTracker {
    /// Used to globally disable or enable update tracking
    pub state: UpdateState,
    /// List of update steps with snapshots of the game state
    pub steps: Vec<UpdateStep>,
}

impl Default for UpdateTracker {
    fn default() -> Self {
        Self { state: UpdateState::Ignore, steps: vec![] }
    }
}

impl UpdateTracker {
    pub fn new(updates: UpdateState) -> Self {
        Self { state: updates, steps: vec![] }
    }
}
