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

use assets::rexard_images::RexardPack;
use assets::{rexard_images, EnvironmentType};
use card_helpers::*;
use game_data::card_definition::{
    Ability, AbilityType, CardConfig, CardDefinition, CardStats, SchemePoints,
};
use game_data::card_name::CardName;
use game_data::card_set_name::CardSetName;
use game_data::primitives::{CardType, Lineage, Rarity, School, Side};
use game_data::text2::trigger;
use rules::mutations::OnZeroStored;
use rules::{mana, mutations};

pub fn tutorial_overlord_leader() -> CardDefinition {
    CardDefinition {
        name: CardName::TutorialOverlordLeader,
        sets: vec![CardSetName::Basics],
        cost: cost(0),
        image: assets::fantasy_class_image("Warlock", "Male"),
        card_type: CardType::Leader,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![],
        config: CardConfig {
            player_portrait: Some(assets::fantasy_class_portrait(Side::Champion, "Warlock_M")),
            image_background: Some(assets::environments(
                EnvironmentType::CastlesTowersKeeps,
                "BoneKeep/SceneryBKeep_outside_2",
            )),
            ..CardConfig::default()
        },
    }
}

pub fn conspire() -> CardDefinition {
    CardDefinition {
        name: CardName::Conspire,
        sets: vec![CardSetName::Basics],
        cost: scheme_cost(),
        image: rexard_images::spell(2, "SpellBook02_17"),
        card_type: CardType::Scheme,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![],
        config: CardConfig {
            stats: scheme_points(SchemePoints { level_requirement: 3, points: 15 }),
            ..CardConfig::default()
        },
    }
}

pub fn devise() -> CardDefinition {
    CardDefinition {
        name: CardName::Devise,
        sets: vec![CardSetName::Basics],
        cost: scheme_cost(),
        image: rexard_images::spell(2, "SpellBook02_27"),
        card_type: CardType::Scheme,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![],
        config: CardConfig {
            stats: scheme_points(SchemePoints { level_requirement: 4, points: 30 }),
            ..CardConfig::default()
        },
    }
}

pub fn machinate() -> CardDefinition {
    CardDefinition {
        name: CardName::Machinate,
        sets: vec![CardSetName::Basics],
        cost: scheme_cost(),
        image: rexard_images::spell(2, "SpellBook02_29"),
        card_type: CardType::Scheme,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![],
        config: CardConfig {
            stats: scheme_points(SchemePoints { level_requirement: 5, points: 45 }),
            ..CardConfig::default()
        },
    }
}

pub fn gathering_dark() -> CardDefinition {
    CardDefinition {
        name: CardName::GatheringDark,
        sets: vec![CardSetName::Basics],
        cost: cost(5),
        image: rexard_images::spell(1, "SpellBook01_88"),
        card_type: CardType::OverlordSpell,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![simple_ability(
            text2![Gain, Mana(9)],
            on_cast(|g, s, _| {
                mana::gain(g, s.side(), 9);
                Ok(())
            }),
        )],
        config: CardConfig::default(),
    }
}

pub fn coinery() -> CardDefinition {
    CardDefinition {
        name: CardName::Coinery,
        sets: vec![CardSetName::Basics],
        cost: cost(2),
        image: rexard_images::get(RexardPack::LootIcons, "coins_b_03"),
        card_type: CardType::Project,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![
            text_only_ability(text2![text2![Unveil, "when activated, then", StoreMana(15)]]),
            Ability {
                ability_type: activate_for_action(),
                text: text2![TakeMana(3)],
                delegates: vec![
                    activate_while_face_down(),
                    face_down_ability_cost(),
                    on_activated(|g, s, _| {
                        if mutations::unveil_project_ignoring_costs(g, s.card_id())? {
                            add_stored_mana(g, s.card_id(), 15);
                        }
                        mutations::take_stored_mana(g, s.card_id(), 3, OnZeroStored::Sacrifice)
                            .map(|_| ())
                    }),
                ],
            },
        ],
        config: CardConfig::default(),
    }
}

pub fn leyline() -> CardDefinition {
    CardDefinition {
        name: CardName::Leyline,
        sets: vec![CardSetName::Basics],
        cost: cost(2),
        image: rexard_images::spell(2, "SpellBook02_78"),
        card_type: CardType::Project,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![
            unveil_at_dusk_ability(),
            simple_ability(
                trigger(Dusk, text2!["Gain", Mana(1)]),
                at_dusk(|g, s, _| {
                    mana::gain(g, s.side(), 1);
                    Ok(())
                }),
            ),
        ],
        config: CardConfig::default(),
    }
}

pub fn ore_refinery() -> CardDefinition {
    CardDefinition {
        name: CardName::OreRefinery,
        sets: vec![CardSetName::Basics],
        cost: cost(4),
        image: rexard_images::get(RexardPack::MiningIcons, "MiningIcons_06_b"),
        card_type: CardType::Project,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![
            Ability {
                ability_type: AbilityType::Standard,
                text: text2![Unveil, "at", Dusk, ", then", StoreMana(12)],
                delegates: vec![unveil_at_dusk(), store_mana_on_unveil::<12>()],
            },
            simple_ability(
                trigger(Dusk, text2![TakeMana(3)]),
                at_dusk(|g, s, _| {
                    mutations::take_stored_mana(g, s.card_id(), 3, OnZeroStored::Sacrifice)?;
                    Ok(())
                }),
            ),
        ],
        config: CardConfig::default(),
    }
}

pub fn crab() -> CardDefinition {
    CardDefinition {
        name: CardName::Crab,
        sets: vec![CardSetName::Basics],
        cost: cost(4),
        image: rexard_images::get(RexardPack::MonstersAvatars, "64"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![abilities::combat_end_raid()],
        config: CardConfig {
            stats: CardStats { health: Some(2), ..CardStats::default() },
            lineage: Some(Lineage::Infernal),
            ..CardConfig::default()
        },
    }
}

pub fn fire_goblin() -> CardDefinition {
    CardDefinition {
        name: CardName::FireGoblin,
        sets: vec![CardSetName::Basics],
        cost: cost(1),
        image: rexard_images::get(RexardPack::MonstersAvatars, "70"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![abilities::combat_deal_damage::<1>(), abilities::combat_gain_mana::<1>()],
        config: CardConfig {
            stats: CardStats { health: Some(1), shield: Some(2), ..CardStats::default() },
            lineage: Some(Lineage::Infernal),
            ..CardConfig::default()
        },
    }
}

pub fn toucan() -> CardDefinition {
    CardDefinition {
        name: CardName::Toucan,
        sets: vec![CardSetName::Basics],
        cost: cost(3),
        image: rexard_images::get(RexardPack::MonstersAvatars, "65"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![abilities::combat_end_raid()],
        config: CardConfig {
            stats: CardStats { health: Some(3), ..CardStats::default() },
            lineage: Some(Lineage::Abyssal),
            ..CardConfig::default()
        },
    }
}

pub fn frog() -> CardDefinition {
    CardDefinition {
        name: CardName::Frog,
        sets: vec![CardSetName::Basics],
        cost: cost(4),
        image: rexard_images::get(RexardPack::MonstersAvatars, "66"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![abilities::combat_end_raid()],
        config: CardConfig {
            stats: CardStats { health: Some(4), ..CardStats::default() },
            lineage: Some(Lineage::Abyssal),
            ..CardConfig::default()
        },
    }
}

pub fn captain() -> CardDefinition {
    CardDefinition {
        name: CardName::Captain,
        sets: vec![CardSetName::Basics],
        cost: cost(3),
        image: rexard_images::get(RexardPack::MonstersAvatars, "103"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![abilities::combat_end_raid(), abilities::remove_actions_if_able::<1>()],
        config: CardConfig {
            stats: CardStats { health: Some(2), ..CardStats::default() },
            lineage: Some(Lineage::Mortal),
            ..CardConfig::default()
        },
    }
}

pub fn scout() -> CardDefinition {
    CardDefinition {
        name: CardName::Scout,
        sets: vec![CardSetName::Basics],
        cost: cost(5),
        image: rexard_images::get(RexardPack::MonstersAvatars, "19"),
        card_type: CardType::Minion,
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![abilities::combat_end_raid()],
        config: CardConfig {
            stats: CardStats { health: Some(4), ..CardStats::default() },
            lineage: Some(Lineage::Mortal),
            ..CardConfig::default()
        },
    }
}
