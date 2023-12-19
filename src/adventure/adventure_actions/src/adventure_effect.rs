// Copyright © Riftcaller 2021-present

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//    https://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use adventure_data::adventure::{AdventureScreen, AdventureState};
use adventure_data::adventure_effect_data::AdventureEffect;
use adventure_generator::{battle_generator, card_selector, narrative_event_generator};
use anyhow::Result;
use game_data::card_name::CardVariant;

pub fn apply(
    state: &mut AdventureState,
    effect: AdventureEffect,
    _known_card: Option<CardVariant>,
) -> Result<()> {
    match effect {
        AdventureEffect::Draft(selector) => {
            let data = card_selector::draft_choices(state, selector);
            state.screens.push(AdventureScreen::Draft(data));
        }
        AdventureEffect::Shop(selector) => {
            let data = card_selector::shop_choices(state, selector);
            state.screens.push(AdventureScreen::Shop(data));
        }
        AdventureEffect::NarrativeEvent(_) => {
            let data = narrative_event_generator::generate();
            state.screens.push(AdventureScreen::NarrativeEvent(data));
        }
        AdventureEffect::Battle => state
            .screens
            .push(AdventureScreen::Battle(battle_generator::create(state.side.opponent()))),
        AdventureEffect::PickCardForEffect(selector, effect) => {
            state.screens.push(AdventureScreen::ApplyDeckEffect(selector, effect))
        }
        _ => {
            panic!("Not implemented {effect:?}")
        }
    }
    Ok(())
}
