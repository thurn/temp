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
use core_ui::icons;
use game_data::card_name::CardName;
use game_data::primitives::Side;
use test_utils::client_interface::{self, HasText};
use test_utils::test_adventure::{TestAdventure, TestConfig, SHOP_ICON};

const BUY_COST: Coins = Coins(75);
const EXAMPLE_CARD: CardName = CardName::TestChampionSpell;

#[test]
fn test_visit_shop() {
    let mut adventure = TestAdventure::new(Side::Champion, config());
    adventure.visit_tile_with_icon(SHOP_ICON);
    assert!(adventure.interface.top_panel().has_text("Walking through town"));
    adventure.click_on("Continue");
    assert!(adventure.interface.top_panel().has_text(BUY_COST.to_string()));
}

#[test]
fn test_visit_shop_twice() {
    let mut adventure = TestAdventure::new(Side::Champion, config());
    adventure.visit_tile_with_icon(SHOP_ICON);
    adventure.click_on("Continue");
    adventure.click_on_navbar(icons::CLOSE);
    adventure.visit_tile_with_icon(SHOP_ICON);
    assert!(adventure.interface.top_panel().has_text(BUY_COST.to_string()));
}

#[test]
fn test_buy_card() {
    let mut adventure = TestAdventure::new(Side::Champion, config());
    adventure.visit_tile_with_icon(SHOP_ICON);
    adventure.click_on("Continue");

    assert!(adventure
        .interface
        .screen_overlay()
        .has_text(format!("{}", adventure_generator::STARTING_COINS)));
    adventure.click_on(BUY_COST.to_string());
    assert!(adventure
        .interface
        .screen_overlay()
        .has_text(format!("{}", adventure_generator::STARTING_COINS - BUY_COST)));

    adventure.click_on_navbar(icons::CLOSE);
    adventure.click_on_navbar(icons::DECK);

    client_interface::assert_has_element_name(
        adventure.interface.top_panel(),
        element_names::deck_card(EXAMPLE_CARD),
    );
}

fn config() -> TestConfig {
    TestConfig {
        draft: Some(TileEntity::Shop(ShopData {
            visited: false,
            choices: vec![CardChoice {
                quantity: 2,
                card: EXAMPLE_CARD,
                cost: BUY_COST,
                sold: false,
            }],
        })),
        ..TestConfig::default()
    }
}
