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

//! Card definitions for the Minion card type

use data::card_definition::{Ability, AbilityType, CardConfig, CardDefinition, CardStats};
use data::card_name::CardName;
use data::card_state::CardPosition;
use data::delegates::{Delegate, EventDelegate, RaidOutcome};
use data::game_actions::CardPromptAction;
use data::primitives::{
    CardType, ColdDamage, DamageType, Faction, Rarity, RoomLocation, School, Side,
};
use data::text::{DamageWord, Keyword};
use data::with_error::WithError;
use display2::rexard_images;
use display2::rexard_images::RexardPack;
use rules::helpers::*;
use rules::mana::ManaPurpose;
use rules::mutations::SummonMinion;
use rules::{abilities, mana, mutations, queries, text};

pub fn ice_dragon() -> CardDefinition {
    CardDefinition {
        name: CardName::IceDragon,
        cost: cost(3),
        image: sprite("Rexard/SpellBookPage01/SpellBookPage01_png/SpellBook01_44"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Time,
        rarity: Rarity::Common,
        abilities: vec![abilities::combat_deal_damage::<ColdDamage, 1>(), abilities::end_raid()],
        config: CardConfig {
            stats: CardStats { health: Some(5), shield: Some(1), ..CardStats::default() },
            faction: Some(Faction::Infernal),
            ..CardConfig::default()
        },
    }
}

pub fn time_golem() -> CardDefinition {
    CardDefinition {
        name: CardName::TimeGolem,
        cost: cost(2),
        image: rexard_images::get(RexardPack::MonstersAvatars, "10"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Time,
        rarity: Rarity::Common,
        abilities: vec![
            abilities::construct(),
            simple_ability(
                text![
                    Keyword::Encounter,
                    "End the raid unless the Champion pays",
                    mana_text(5),
                    "or",
                    actions_text(2)
                ],
                on_encountered(|g, _s, _| {
                    set_card_prompt(
                        g,
                        Side::Champion,
                        vec![
                            Some(CardPromptAction::EndRaid),
                            lose_mana_prompt(g, Side::Champion, 5),
                            lose_actions_prompt(g, Side::Champion, 2),
                        ],
                    )
                }),
            ),
        ],
        config: CardConfig {
            stats: health(3),
            faction: Some(Faction::Construct),
            ..CardConfig::default()
        },
    }
}

pub fn temporal_vortex() -> CardDefinition {
    CardDefinition {
        name: CardName::TemporalVortex,
        cost: cost(6),
        image: rexard_images::get(RexardPack::MonstersAvatars, "87"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Time,
        rarity: Rarity::Common,
        abilities: vec![
            simple_ability(
                text![Keyword::Combat, "End the raid unless the Champion pays", actions_text(2)],
                minion_combat_actions(|g, _, _, _| {
                    vec![Some(CardPromptAction::EndRaid), lose_actions_prompt(g, Side::Champion, 2)]
                }),
            ),
            simple_ability(
                text![Keyword::Combat, "Summon a minion from the Sanctum or Crypts for free.",],
                combat(|g, s, _| {
                    let cards = g.hand(Side::Overlord).chain(g.discard_pile(Side::Overlord));
                    if let Some(minion_id) = queries::highest_cost(cards) {
                        let (room_id, index) = queries::minion_position(g, s.card_id())
                            .with_error(|| "Minion not found")?;
                        mutations::turn_face_down(g, minion_id)?; // Card may be face-up in Crypt
                        mutations::move_card(
                            g,
                            minion_id,
                            CardPosition::Room(room_id, RoomLocation::Defender),
                        )?;
                        g.move_card_to_index(minion_id, index);
                        mutations::summon_minion(g, minion_id, SummonMinion::IgnoreCosts)?;
                        mutations::set_raid_encountering_minion(g, s.card_id())?;
                    }
                    Ok(())
                }),
            ),
        ],
        config: CardConfig {
            stats: CardStats { health: Some(6), shield: Some(3), ..CardStats::default() },
            faction: Some(Faction::Abyssal),
            ..CardConfig::default()
        },
    }
}

pub fn shadow_lurker() -> CardDefinition {
    CardDefinition {
        name: CardName::ShadowLurker,
        cost: cost(3),
        image: rexard_images::get(RexardPack::MonstersAvatars, "80"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Time,
        rarity: Rarity::Common,
        abilities: vec![
            simple_ability(
                text!["While this minion is in an outer room, it has +2 health"],
                on_calculate_health(|g, s, _, current| match g.card(s.card_id()).position() {
                    CardPosition::Room(room_id, _) if !is_inner_room(room_id) => current + 2,
                    _ => current,
                }),
            ),
            abilities::end_raid(),
        ],
        config: CardConfig {
            stats: CardStats { health: Some(2), shield: Some(1), ..CardStats::default() },
            faction: Some(Faction::Abyssal),
            ..CardConfig::default()
        },
    }
}

pub fn sphinx_of_winters_breath() -> CardDefinition {
    CardDefinition {
        name: CardName::SphinxOfWintersBreath,
        cost: cost(2),
        image: rexard_images::get(RexardPack::MonstersAvatars, "17"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Time,
        rarity: Rarity::Common,
        abilities: vec![Ability {
            text: text![
                Keyword::Combat,
                Keyword::DealDamage(DamageWord::DealStart, 1, DamageType::Cold),
                ".",
                "If a card with an odd mana cost is discarded, end the raid."
            ],
            ability_type: AbilityType::Standard,
            delegates: vec![
                combat(|g, s, _| mutations::deal_damage(g, s, DamageType::Cold, 1)),
                Delegate::DealtDamage(EventDelegate {
                    requirement: |g, s, data| {
                        s.ability_id() == data.source
                            && data.discarded.iter().any(|card_id| {
                                queries::mana_cost(g, *card_id).unwrap_or(0) % 2 != 0
                            })
                    },
                    mutation: |g, _, _| mutations::end_raid(g, RaidOutcome::Failure),
                }),
            ],
        }],
        config: CardConfig {
            stats: CardStats { health: Some(3), shield: Some(1), ..CardStats::default() },
            faction: Some(Faction::Mortal),
            ..CardConfig::default()
        },
    }
}

pub fn bridge_troll() -> CardDefinition {
    CardDefinition {
        name: CardName::BridgeTroll,
        cost: cost(2),
        image: rexard_images::get(RexardPack::MonstersAvatars, "29"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Time,
        rarity: Rarity::Common,
        abilities: vec![simple_ability(
            text![
                Keyword::Combat,
                "The Champion loses",
                mana_text(3),
                ".",
                "If they have",
                mana_text(6),
                "or less, end the raid."
            ],
            combat(|g, _, _| {
                mana::lose_upto(g, Side::Champion, ManaPurpose::PayForTriggeredAbility, 3);
                if mana::get(g, Side::Champion, ManaPurpose::BaseMana) <= 6 {
                    mutations::end_raid(g, RaidOutcome::Failure)?;
                }
                Ok(())
            }),
        )],
        config: CardConfig {
            stats: CardStats { health: Some(0), shield: Some(2), ..CardStats::default() },
            faction: Some(Faction::Mortal),
            ..CardConfig::default()
        },
    }
}

pub fn stormcaller() -> CardDefinition {
    CardDefinition {
        name: CardName::Stormcaller,
        cost: cost(4),
        image: rexard_images::get(RexardPack::MonstersAvatars, "53"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Time,
        rarity: Rarity::Common,
        abilities: vec![simple_ability(
            text![
                Keyword::Combat,
                "The Champion must end the raid and",
                Keyword::DealDamage(DamageWord::TakeInternal, 2, DamageType::Lightning),
                "or",
                Keyword::DealDamage(DamageWord::TakeInternal, 4, DamageType::Lightning),
                "."
            ],
            minion_combat_actions(|g, s, _, _| {
                vec![
                    Some(CardPromptAction::TakeDamageEndRaid(
                        s.ability_id(),
                        DamageType::Lightning,
                        2,
                    )),
                    take_damage_prompt(g, s.ability_id(), DamageType::Lightning, 4),
                ]
            }),
        )],
        config: CardConfig {
            stats: CardStats { health: Some(3), shield: Some(2), ..CardStats::default() },
            faction: Some(Faction::Infernal),
            ..CardConfig::default()
        },
    }
}

pub fn fire_goblin() -> CardDefinition {
    CardDefinition {
        name: CardName::FireGoblin,
        cost: cost(1),
        image: rexard_images::get(RexardPack::MonstersAvatars, "70"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Time,
        rarity: Rarity::Common,
        abilities: vec![simple_ability(
            text![
                Keyword::Combat,
                Keyword::DealDamage(DamageWord::DealStart, 1, DamageType::Fire),
                ".",
                "Gain",
                mana_text(1),
                "."
            ],
            combat(|g, s, _| {
                mutations::deal_damage(g, s, DamageType::Fire, 1)?;
                mana::gain(g, Side::Overlord, 1);
                Ok(())
            }),
        )],
        config: CardConfig {
            stats: CardStats { health: Some(1), shield: Some(2), ..CardStats::default() },
            faction: Some(Faction::Infernal),
            ..CardConfig::default()
        },
    }
}
