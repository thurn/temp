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

//! Helpers for defining common card abilities

use game_data::card_definition::{Ability, AbilityType, Cost, TargetRequirement};
use game_data::card_state::CardPosition;
use game_data::delegates::{Delegate, EventDelegate, QueryDelegate, RaidOutcome, Scope};
use game_data::game::GameState;
use game_data::primitives::{AbilityId, AttackValue, CardId, ManaValue};
use game_data::text::{AbilityText, DamageWord, Keyword, RulesTextContext, Sentence, TextToken};
use game_data::text2::Token::*;
use game_data::text2::{activation, trigger};
use rules::mutations::OnZeroStored;
use rules::{mutations, queries};

use crate::text_macro::text;
use crate::text_macro2::text2;
use crate::*;

/// The standard weapon ability; applies an attack boost for the duration of a
/// single encounter.
pub fn encounter_boost() -> Ability {
    let t2 = text2![EncounterBoost];

    Ability {
        ability_type: AbilityType::Encounter,
        text: AbilityText::TextFn(|context| {
            let boost = match context {
                RulesTextContext::Default(definition) => definition.config.stats.attack_boost,
                RulesTextContext::Game(game, card) => queries::attack_boost(game, card.id),
            }
            .unwrap_or_default();

            vec![
                cost(boost.cost).into(),
                add_number(boost.bonus),
                TextToken::Literal("Attack".to_owned()),
            ]
        }),
        delegates: vec![
            Delegate::ActivateBoost(EventDelegate::new(this_card, mutations::write_boost)),
            Delegate::AttackValue(QueryDelegate::new(this_card, add_boost)),
            Delegate::EncounterEnd(EventDelegate::new(always, mutations::clear_boost)),
        ],
    }
}

/// Store `N` mana in this card when played. Move it to the discard pile when
/// the stored mana is depleted.
pub fn store_mana_on_play<const N: ManaValue>() -> Ability {
    let t2 = trigger(Play, text2![StoreMana(N)]);

    Ability {
        ability_type: AbilityType::Standard,
        text: text![Keyword::Play, Keyword::Store(Sentence::Start, N)],
        delegates: vec![
            Delegate::CastCard(EventDelegate::new(this_card, |g, _s, played| {
                g.card_mut(played.card_id).data.stored_mana = N;
                Ok(())
            })),
            Delegate::StoredManaTaken(EventDelegate::new(this_card, |g, s, card_id| {
                if g.card(*card_id).data.stored_mana == 0 {
                    mutations::move_card(g, *card_id, CardPosition::DiscardPile(s.side()))
                } else {
                    Ok(())
                }
            })),
        ],
    }
}

/// Activated ability to take `N` stored mana from this card by paying a cost
pub fn activated_take_mana<const N: ManaValue>(cost: Cost<AbilityId>) -> Ability {
    let t2 = activation(text2![TakeMana(N)]);

    Ability {
        ability_type: AbilityType::Activated(cost, TargetRequirement::None),
        text: text![Keyword::Take(Sentence::Start, N)],
        delegates: vec![on_activated(|g, _s, activated| {
            mutations::take_stored_mana(g, activated.card_id(), N, OnZeroStored::Sacrifice)
                .map(|_| ())
        })],
    }
}

/// Minion combat ability which deals damage to the Champion player during
/// combat, causing them to discard `N` random cards and lose the game if they
/// cannot.
pub fn combat_deal_damage<const N: u32>() -> Ability {
    let t2 = trigger(Combat, text2![DealDamage(N)]);

    Ability {
        ability_type: AbilityType::Standard,
        text: text![Keyword::Combat, Keyword::DealDamage(DamageWord::DealStart, N), "."],
        delegates: vec![combat(|g, s, _| mutations::deal_damage(g, s, N))],
    }
}

/// Minion combat ability which ends the current raid in failure.
pub fn end_raid() -> Ability {
    let t2 = trigger(Combat, text2!["End the raid"]);

    Ability {
        ability_type: AbilityType::Standard,
        text: text![Keyword::Combat, "End the raid."],
        delegates: vec![combat(|g, _, _| mutations::end_raid(g, RaidOutcome::Failure))],
    }
}

/// Minion combat ability which causes the Champion player to lose action
/// points.
pub fn remove_actions_if_able<const N: ActionCount>() -> Ability {
    let t2 = trigger(Combat, text2!["Remove", Actions(1)]);

    Ability {
        ability_type: AbilityType::Standard,
        text: text![Keyword::Combat, "Remove", TextToken::Actions(1)],
        delegates: vec![combat(|g, _s, _| {
            mutations::lose_action_points_if_able(g, Side::Champion, N)
        })],
    }
}

/// Applies this card's `attack_boost` stat a number of times equal to its
/// [CardState::boost_count]. Returns default if this card has no attack boost
/// defined.
fn add_boost(game: &GameState, _: Scope, card_id: &CardId, current: AttackValue) -> AttackValue {
    let boost_count = queries::boost_count(game, *card_id);
    let bonus = queries::attack_boost(game, *card_id).unwrap_or_default().bonus;
    current + (boost_count * bonus)
}

/// An ability which allows a card to have level counters placed on it.
pub fn level_up() -> Ability {
    let t2 = text2![LevelUp];

    Ability {
        ability_type: AbilityType::Standard,
        text: text![Keyword::LevelUp],
        delegates: vec![Delegate::CanLevelUpCard(QueryDelegate {
            requirement: this_card,
            transformation: |_g, _, _, current| current.with_override(true),
        })],
    }
}

pub fn construct() -> Ability {
    let t2 = text2![Construct];

    Ability {
        ability_type: AbilityType::Standard,
        text: text![Keyword::Construct],
        delegates: vec![Delegate::MinionDefeated(EventDelegate {
            requirement: this_card,
            mutation: |g, s, _| {
                mutations::move_card(g, s.card_id(), CardPosition::DiscardPile(s.side()))
            },
        })],
    }
}
