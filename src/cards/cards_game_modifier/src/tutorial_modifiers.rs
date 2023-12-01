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

use assets::rexard_images;
use card_helpers::requirements::always;
use card_helpers::*;
use core_data::game_primitives::{CardType, Rarity, RoomId, School, Side};
use game_data::card_definition::{Ability, CardConfig, CardDefinition, Cost};
use game_data::card_name::{CardMetadata, CardName};
use game_data::card_set_name::CardSetName;
use game_data::delegate_data::{Delegate, EventDelegate, QueryDelegate};

fn tutorial_modifier(name: CardName, ability: Ability) -> CardDefinition {
    CardDefinition {
        name,
        sets: vec![CardSetName::TutorialEffects],
        cost: Cost::default(),
        image: rexard_images::spell(1, "SpellBook01_01"),
        card_type: CardType::GameModifier,
        subtypes: vec![],
        side: Side::Overlord,
        school: School::Neutral,
        rarity: Rarity::Common,
        abilities: vec![ability],
        config: CardConfig::default(),
    }
}

pub fn overlord_empty_modifier(_: CardMetadata) -> CardDefinition {
    tutorial_modifier(CardName::OverlordEmptyModifier, Ability::new(text!["No effect"]))
}

pub fn tutorial_disable_draw_action(_: CardMetadata) -> CardDefinition {
    tutorial_modifier(
        CardName::TutorialDisableDrawAction,
        Ability::new_with_delegate(
            text!["The Champion cannot take the 'draw card' action"],
            Delegate::CanTakeDrawCardAction(QueryDelegate {
                requirement: side_is_champion,
                transformation: delegates::disallow,
            }),
        ),
    )
}

pub fn tutorial_disable_gain_mana(_: CardMetadata) -> CardDefinition {
    tutorial_modifier(
        CardName::TutorialDisableGainMana,
        Ability::new_with_delegate(
            text!["The Champion cannot take the 'gain mana' action"],
            Delegate::CanTakeGainManaAction(QueryDelegate {
                requirement: side_is_champion,
                transformation: delegates::disallow,
            }),
        ),
    )
}

pub fn tutorial_disable_raid_sanctum(_: CardMetadata) -> CardDefinition {
    tutorial_modifier(
        CardName::TutorialDisableRaidSanctum,
        Ability::new_with_delegate(
            text!["The Champion cannot raid the Sanctum"],
            Delegate::CanInitiateRaid(QueryDelegate {
                requirement: room_is_sanctum,
                transformation: delegates::disallow,
            }),
        ),
    )
}

pub fn tutorial_disable_raid_vault(_: CardMetadata) -> CardDefinition {
    tutorial_modifier(
        CardName::TutorialDisableRaidVault,
        Ability::new_with_delegate(
            text!["The Champion cannot raid the Vault"],
            Delegate::CanInitiateRaid(QueryDelegate {
                requirement: room_is_vault,
                transformation: delegates::disallow,
            }),
        ),
    )
}

pub fn tutorial_disable_raid_crypt(_: CardMetadata) -> CardDefinition {
    tutorial_modifier(
        CardName::TutorialDisableRaidCrypt,
        Ability::new_with_delegate(
            text!["The Champion cannot raid the Crypt"],
            Delegate::CanInitiateRaid(QueryDelegate {
                requirement: room_is_crypt,
                transformation: delegates::disallow,
            }),
        ),
    )
}

pub fn tutorial_disable_raid_outer(_: CardMetadata) -> CardDefinition {
    tutorial_modifier(
        CardName::TutorialDisableRaidOuter,
        Ability::new_with_delegate(
            text!["The Champion cannot raid outer rooms"],
            Delegate::CanInitiateRaid(QueryDelegate {
                requirement: |_, _, room_id| room_id.is_outer_room(),
                transformation: delegates::disallow,
            }),
        ),
    )
}

pub fn tutorial_disable_raid_continue(_: CardMetadata) -> CardDefinition {
    tutorial_modifier(
        CardName::TutorialDisableRaidContinue,
        Ability::new_with_delegate(
            text!["The Champion must use a weapon during raid_state"],
            Delegate::CanUseNoWeapon(QueryDelegate {
                requirement: always,
                transformation: delegates::disallow,
            }),
        ),
    )
}

pub fn tutorial_disable_end_raid(_: CardMetadata) -> CardDefinition {
    tutorial_modifier(
        CardName::TutorialDisableEndRaid,
        Ability::new_with_delegate(
            text!["The Champion cannot end the access phase of raid_state"],
            Delegate::CanEndRaidAccessPhase(QueryDelegate {
                requirement: always,
                transformation: delegates::disallow,
            }),
        ),
    )
}

pub fn tutorial_force_sanctum_score(_: CardMetadata) -> CardDefinition {
    tutorial_modifier(
        CardName::TutorialForceSanctumScore,
        Ability::new_with_delegate(
            text!["The Champion always accesses a scheme card when raiding the Sanctum"],
            Delegate::RaidAccessSelected(EventDelegate {
                requirement: |_, _, event| event.target == RoomId::Sanctum,
                mutation: |g, _, _| {
                    let scheme = g
                        .hand(Side::Overlord)
                        .find(|card| rules::get(card.variant).card_type == CardType::Scheme)
                        .map(|c| c.id);
                    if let Some(id) = scheme {
                        g.raid_mut()?.accessed = vec![id];
                    }
                    Ok(())
                },
            }),
        ),
    )
}
