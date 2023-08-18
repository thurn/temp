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

use adventure_data::adventure::{
    AdventureConfiguration, CardChoice, Coins, DraftContext, DraftData, ShopData,
};
use game_data::card_name::CardName;
use game_data::card_set_name::CardSetName;
use game_data::primitives::{CardType, Rarity, School, Side, STANDARD_SCHOOLS};

/// Generates options for drafting a card during an adventure
pub fn draft_choices(config: &mut AdventureConfiguration) -> DraftData {
    draft_data(None, config.choose_multiple(3, common_cards(config.side)))
}

/// Generates sigil draft options from 3 randomly chosen schools
pub fn sigil_choices(config: &mut AdventureConfiguration) -> DraftData {
    let schools = config.choose_multiple(3, STANDARD_SCHOOLS.iter());
    let cards = schools.into_iter().filter_map(|school| random_sigil(config, config.side, *school));
    draft_data(Some(DraftContext::StartingSigil), cards.collect())
}

fn draft_data(context: Option<DraftContext>, cards: Vec<CardName>) -> DraftData {
    DraftData {
        context,
        choices: cards
            .into_iter()
            .map(|name| CardChoice { quantity: 1, card: name, cost: Coins(0), sold: false })
            .collect(),
    }
}

/// Generates options for buying from a shop during an adventure
pub fn shop_options(config: &mut AdventureConfiguration) -> ShopData {
    ShopData {
        choices: config
            .choose_multiple(5, common_cards(config.side))
            .into_iter()
            .map(|name| CardChoice {
                quantity: config.gen_range(1..=3),
                card: name,
                cost: Coins(config.gen_range(1..=4) * 25),
                sold: false,
            })
            .collect(),
    }
}

fn random_sigil(
    config: &mut AdventureConfiguration,
    side: Side,
    school: School,
) -> Option<CardName> {
    config.choose(
        rules::all_cards()
            .filter(move |definition| {
                definition.sets.contains(&CardSetName::Amethyst)
                    && definition.side == side
                    && definition.school == school
                    && definition.card_type == CardType::Sigil
            })
            .map(|definition| definition.name),
    )
}

fn common_cards(side: Side) -> impl Iterator<Item = CardName> {
    rules::all_cards()
        .filter(move |definition| {
            definition.sets.contains(&CardSetName::Amethyst)
                && definition.rarity == Rarity::Common
                && definition.side == side
        })
        .map(|definition| definition.name)
}
