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

use adventure_data::adventure::{CardChoice, Coins, ShopData, TileEntity};
use game_data::card_name::CardName;
use game_data::primitives::Side;
use test_utils::client_interface::{self};
use test_utils::test_adventure::TestAdventure;
use test_utils::*;

const BUY_COST: Coins = Coins(75);
const EXAMPLE_CARD: CardName = CardName::TestChampionSpell;

#[test]
fn test_visit_shop() {
    let mut adventure = TestAdventure::new(Side::Champion).build();

    let shop = adventure.insert_tile(TileEntity::Shop(ShopData {
        choices: vec![CardChoice { quantity: 2, card: EXAMPLE_CARD, cost: BUY_COST, sold: false }],
    }));

    adventure.visit_tile(shop);

    assert!(adventure.has_text(BUY_COST.to_string()));
}

#[test]
fn test_buy_card() {
    let mut adventure = TestAdventure::new(Side::Champion).build();

    let shop = adventure.insert_tile(TileEntity::Shop(ShopData {
        choices: vec![CardChoice { quantity: 2, card: EXAMPLE_CARD, cost: BUY_COST, sold: false }],
    }));

    adventure.visit_tile(shop);

    assert!(adventure.has_text(test_constants::STARTING_COINS.to_string()));
    adventure.click_on(adventure.user_id(), BUY_COST.to_string());
    assert!(adventure.has_text((test_constants::STARTING_COINS - BUY_COST).to_string()));

    adventure.click(Buttons::CloseIcon);
    adventure.click(Buttons::ShowDeck);

    client_interface::assert_has_element_name(
        adventure.user.interface.top_panel(),
        element_names::deck_card(EXAMPLE_CARD),
    );
}
