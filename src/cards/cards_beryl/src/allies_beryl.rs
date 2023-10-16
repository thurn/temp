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

use card_helpers::abilities::ActivatedConfig;
use card_helpers::effects::Effects;
use card_helpers::{abilities, costs, in_play, show_prompt, text, this};
use core_ui::design;
use core_ui::design::TimedEffectDataExt;
use game_data::card_definition::{CardConfig, CardDefinition};
use game_data::card_name::{CardMetadata, CardName};
use game_data::card_set_name::CardSetName;
use game_data::game_actions::{ButtonPromptContext, PromptChoice};
use game_data::game_effect::GameEffect;
use game_data::game_state::GameState;
use game_data::primitives::{CardSubtype, CardType, GameObjectId, Rarity, School, Side};
use game_data::special_effects::{SoundEffect, TimedEffect, TimedEffectData};
use game_data::text::TextElement;
use game_data::text::TextToken::*;
use rules::curses;

pub fn stalwart_protector(meta: CardMetadata) -> CardDefinition {
    fn update(game: &mut GameState) {
        Effects::new()
            .timed_effect(
                GameObjectId::Character(Side::Champion),
                TimedEffectData::new(TimedEffect::MagicCircles1(7))
                    .scale(2.0)
                    .sound(SoundEffect::LightMagic("RPG3_LightMagicEpic_HealingWing_P1"))
                    .effect_color(design::YELLOW_900),
            )
            .apply(game);
    }

    CardDefinition {
        name: CardName::StalwartProtector,
        sets: vec![CardSetName::Beryl],
        cost: costs::mana(meta.upgrade(1, 0)),
        image: assets::champion_card(meta, "stalwart_protector"),
        card_type: CardType::Ally,
        subtypes: vec![CardSubtype::Warrior],
        side: Side::Champion,
        school: School::Law,
        rarity: Rarity::Common,
        abilities: vec![
            abilities::standard(
                text![TextElement::Activated {
                    cost: text![SacrificeCost],
                    effect: text!["Prevent receiving a", Curse]
                }],
                in_play::on_will_receive_curses(|g, s, _| {
                    show_prompt::with_context_and_choices(
                        g,
                        s,
                        ButtonPromptContext::SacrificeToPreventCurses(s.card_id(), 1),
                        vec![
                            PromptChoice::new()
                                .effect(GameEffect::SacrificeCard(s.card_id()))
                                .effect(GameEffect::PreventCurses(1)),
                            PromptChoice::new().effect(GameEffect::Continue),
                        ],
                    );
                    Ok(())
                }),
            ),
            abilities::activated_with_config(
                text!["Remove a curse"],
                costs::sacrifice(),
                ActivatedConfig::new().can_activate(|g, _| g.champion.curses > 0),
                this::on_activated(|g, _, _| {
                    update(g);
                    curses::remove_curses(g, 1)
                }),
            ),
        ],
        config: CardConfig::default(),
    }
}
