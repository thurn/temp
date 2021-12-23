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

//! Helpers for defining card behaviors. This file is intended be be used via
//! wildcard import in card definition files.

use data::card_definition::{
    Ability, AbilityText, AbilityType, AttackBoost, CardStats, Cost, Keyword, NumericOperator,
    SchemePoints, TextToken,
};
use data::delegates::{Delegate, EventDelegate, MutationFn, Scope};
use data::game::GameState;
use data::primitives::{
    AbilityId, AttackValue, BoostData, CardId, HealthValue, ManaValue, Sprite, TurnNumber,
};

/// Provides the rules text for a card
pub fn text(text: impl Into<String>) -> TextToken {
    TextToken::Literal(text.into())
}

pub fn number(number: impl Into<u32>) -> TextToken {
    TextToken::Number(NumericOperator::None, number.into())
}

pub fn add_number(number: impl Into<u32>) -> TextToken {
    TextToken::Number(NumericOperator::Add, number.into())
}

pub fn mana_symbol(value: ManaValue) -> TextToken {
    TextToken::Mana(value)
}

pub fn mana_cost_text(value: ManaValue) -> TextToken {
    TextToken::Cost(vec![mana_symbol(value)])
}

pub fn keyword(keyword: Keyword) -> TextToken {
    TextToken::Keyword(keyword)
}

/// Provides the cost for a card
pub fn cost(mana: ManaValue) -> Cost {
    Cost { mana: Some(mana), actions: 1 }
}

/// Provides an image for a card
pub fn sprite(text: &str) -> Sprite {
    Sprite::new(text.to_string())
}

/// RequirementFn which always returns true
pub fn always<T>(_: &GameState, _: Scope, _: T) -> bool {
    true
}

/// RequirementFn that this delegate's card is currently in play
pub fn in_play<T>(game: &GameState, scope: Scope, _: T) -> bool {
    game.card(scope.card_id()).position.in_play()
}

/// A RequirementFn which restricts delegates to only listen to events for their
/// own card.
pub fn this_card(game: &GameState, scope: Scope, card_id: impl Into<CardId>) -> bool {
    scope.card_id() == card_id.into()
}

/// A RequirementFn which restricts delegates to only listen to events for their
/// own ability.
pub fn this_ability(game: &GameState, scope: Scope, ability_id: impl Into<AbilityId>) -> bool {
    scope.ability_id() == ability_id.into()
}

/// A RequirementFn which restricts delegates to only listen to [BoostData]
/// events matching their card.
pub fn this_boost(game: &GameState, scope: Scope, boost_data: BoostData) -> bool {
    scope.card_id() == boost_data.card_id
}

/// An ability which triggers when a card is cast
pub fn on_cast(rules: AbilityText, mutation: MutationFn<CardId>) -> Ability {
    Ability {
        text: rules,
        ability_type: AbilityType::Standard,
        delegates: vec![Delegate::OnCastCard(EventDelegate { requirement: this_card, mutation })],
    }
}

/// An ability which triggers when a card is played
pub fn on_play(rules: AbilityText, mutation: MutationFn<CardId>) -> Ability {
    Ability {
        text: rules,
        ability_type: AbilityType::Standard,
        delegates: vec![Delegate::OnPlayCard(EventDelegate { requirement: this_card, mutation })],
    }
}

/// An ability which triggers at dawn if a card is in play
pub fn at_dawn(rules: AbilityText, mutation: MutationFn<TurnNumber>) -> Ability {
    Ability {
        text: rules,
        ability_type: AbilityType::Standard,
        delegates: vec![Delegate::OnDawn(EventDelegate { requirement: in_play, mutation })],
    }
}

/// An ability which triggers at dusk if a card is in play
pub fn at_dusk(rules: AbilityText, mutation: MutationFn<TurnNumber>) -> Ability {
    Ability {
        text: rules,
        ability_type: AbilityType::Standard,
        delegates: vec![Delegate::OnDusk(EventDelegate { requirement: in_play, mutation })],
    }
}

/// A minion combat ability
pub fn combat(rules: AbilityText, mutation: MutationFn<CardId>) -> Ability {
    Ability {
        text: rules,
        ability_type: AbilityType::Standard,
        delegates: vec![Delegate::OnMinionCombatAbility(EventDelegate {
            requirement: this_card,
            mutation,
        })],
    }
}

/// An ability when a card is scored
pub fn on_score(rules: AbilityText, mutation: MutationFn<CardId>) -> Ability {
    Ability {
        text: rules,
        ability_type: AbilityType::Standard,
        delegates: vec![Delegate::OnScoreScheme(EventDelegate {
            requirement: this_card,
            mutation,
        })],
    }
}

/// Helper to create a [CardStats] with the given `base_attack` and
/// [AttackBoost]
pub fn attack(base_attack: AttackValue, boost: AttackBoost) -> CardStats {
    CardStats { base_attack: Some(base_attack), attack_boost: Some(boost), ..CardStats::default() }
}

pub fn health(health: HealthValue) -> CardStats {
    CardStats { health: Some(health), ..CardStats::default() }
}

pub fn scheme_points(points: SchemePoints) -> CardStats {
    CardStats { scheme_points: Some(points), ..CardStats::default() }
}
