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

use data::card_definition::{AttackBoost, CardConfig, CardDefinition};
use data::card_name::CardName;
use data::primitives::{CardType, Faction, Rarity, School, Side};

use crate::abilities;
use crate::helpers::*;

pub fn greataxe() -> CardDefinition {
    CardDefinition {
        name: CardName::Greataxe,
        cost: cost(3),
        image: sprite("Rexard/SpellBookPage01/SpellBookPage01_png/SpellBook01_42"),
        card_type: CardType::Weapon,
        side: Side::Champion,
        school: School::Time,
        rarity: Rarity::Common,
        abilities: vec![abilities::encounter_boost()],
        config: CardConfig {
            stats: attack(3, AttackBoost { cost: 2, bonus: 1 }),
            faction: Some(Faction::Infernal),
            ..CardConfig::default()
        },
    }
}
