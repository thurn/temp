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

use std::cmp;

use anyhow::Result;
use card_helpers::card_selector_prompt_builder::CardSelectorPromptBuilder;
use card_helpers::{costs, delegates, history, in_play, requirements, text, this};
use core_data::game_primitives::{CardSubtype, CardType, GameObjectId, Rarity, School, Side};
use core_ui::design;
use core_ui::design::TimedEffectDataExt;
use game_data::card_definition::{Ability, ActivatedAbility, CardConfigBuilder, CardDefinition};
use game_data::card_name::{CardMetadata, CardName};
use game_data::card_set_name::CardSetName;
use game_data::card_state::CardIdsExt;
use game_data::delegate_data::Scope;
use game_data::game_state::GameState;
use game_data::history_data::HistoryCounters;
use game_data::prompt_data::{BrowserPromptTarget, BrowserPromptValidation, PromptContext};
use game_data::special_effects::{SoundEffect, TimedEffect, TimedEffectData};
use game_data::text::TextToken::*;
use rules::visual_effects::{ShowAlert, VisualEffects};
use rules::{damage, draw_cards, mutations, prompts, visual_effects};
use with_error::fail;

pub fn magistrates_thronehall(meta: CardMetadata) -> CardDefinition {
    CardDefinition {
        name: CardName::MagistratesThronehall,
        sets: vec![CardSetName::Beryl],
        cost: costs::mana(meta.upgrade(1, 0)),
        image: assets::covenant_card(meta, "magistrates_thronehall"),
        card_type: CardType::Project,
        subtypes: vec![CardSubtype::Nightbound, CardSubtype::Dictate],
        side: Side::Covenant,
        school: School::Law,
        rarity: Rarity::Rare,
        abilities: vec![Ability::new_with_delegate(
            text!["The Riftcaller cannot draw more than", 2, "cards during their turn"],
            in_play::on_will_draw_cards(|g, s, _| {
                let drawn_this_turn = history::counters(g, Side::Riftcaller).cards_drawn;
                let mut show_vfx = false;
                let Some(state) = g.state_machines.draw_cards.last_mut() else {
                    fail!("Expected active draw_cards state machine");
                };
                if state.side == Side::Riftcaller {
                    let new = cmp::min(2u32.saturating_sub(drawn_this_turn), state.quantity);
                    if new < state.quantity {
                        show_vfx = true;
                    }
                    state.quantity = new;
                }

                if show_vfx {
                    VisualEffects::new()
                        .ability_alert(s)
                        .timed_effect(
                            GameObjectId::CardId(s.card_id()),
                            TimedEffectData::new(TimedEffect::MagicCircles1(6))
                                .scale(2.0)
                                .sound(SoundEffect::LightMagic("RPG3_LightMagic_Buff01"))
                                .effect_color(design::YELLOW_900),
                        )
                        .apply(g);
                }
                Ok(())
            }),
        )],
        config: CardConfigBuilder::new().raze_cost(5).build(),
    }
}

pub fn living_stone(meta: CardMetadata) -> CardDefinition {
    CardDefinition {
        name: CardName::LivingStone,
        sets: vec![CardSetName::Beryl],
        cost: costs::mana(meta.upgrade(5, 3)),
        image: assets::covenant_card(meta, "living_stone"),
        card_type: CardType::Project,
        subtypes: vec![CardSubtype::Roombound],
        side: Side::Covenant,
        school: School::Law,
        rarity: Rarity::Common,
        abilities: vec![Ability::new_with_delegate(
            text!["When the Riftcaller uses a", RazeAbility, ", deal", 1, "damage"],
            delegates::on_card_razed(
                |g, s, event| {
                    requirements::face_up_in_play(g, s, &()) || *event.data() == s.card_id()
                },
                |g, s, _| {
                    VisualEffects::new()
                        .ability_alert(s)
                        .timed_effect(
                            GameObjectId::CardId(s.card_id()),
                            TimedEffectData::new(TimedEffect::MagicCircles1(8))
                                .scale(2.0)
                                .sound(SoundEffect::LightMagic("RPG3_LightMagic_Buff01"))
                                .effect_color(design::YELLOW_900),
                        )
                        .apply(g);
                    damage::deal(g, s, 1)
                },
            ),
        )],
        config: CardConfigBuilder::new().raze_cost(5).build(),
    }
}

pub fn sealed_necropolis(meta: CardMetadata) -> CardDefinition {
    CardDefinition {
        name: CardName::SealedNecropolis,
        sets: vec![CardSetName::Beryl],
        cost: costs::mana(0),
        image: assets::covenant_card(meta, "sealed_necropolis"),
        card_type: CardType::Project,
        subtypes: vec![CardSubtype::Roombound, CardSubtype::Nightbound],
        side: Side::Covenant,
        school: School::Law,
        rarity: Rarity::Common,
        abilities: vec![
            ActivatedAbility::new(costs::actions(1), text!["Draw", 2, "cards"])
                .delegate(this::on_activated(|g, s, _| {
                    draw_cards::run(g, Side::Covenant, 2, s.initiated_by())
                }))
                .build(),
            ActivatedAbility::new(
                costs::banish(),
                text!["Shuffle up to", 3, "cards from the crypt into the vault"],
            )
            .delegate(this::on_activated(|g, s, _| {
                prompts::push(g, Side::Covenant, s.ability_id());
                Ok(())
            }))
            .delegate(this::prompt(|g, s, _, _| {
                CardSelectorPromptBuilder::new(s, BrowserPromptTarget::DeckShuffled)
                    .subjects(g.discard_pile(Side::Covenant).card_ids())
                    .context(PromptContext::ShuffleIntoVault)
                    .validation(BrowserPromptValidation::LessThanOrEqualTo(3))
                    .build()
            }))
            .build(),
        ],
        config: CardConfigBuilder::new().raze_cost(meta.upgrade(3, 5)).build(),
    }
}

pub fn haste_resonator(meta: CardMetadata) -> CardDefinition {
    fn gain_action_if_3x(
        game: &mut GameState,
        scope: Scope,
        function: impl Fn(&HistoryCounters) -> u32,
    ) -> Result<()> {
        if function(game.current_history_counters(Side::Covenant)) == 3 {
            visual_effects::show(game, scope, scope.card_id(), ShowAlert::Yes);
            mutations::gain_action_points(game, Side::Covenant, 1)?;
        }
        Ok(())
    }

    CardDefinition {
        name: CardName::HasteResonator,
        sets: vec![CardSetName::Beryl],
        cost: costs::mana(meta.upgrade(2, 0)),
        image: assets::covenant_card(meta, "haste_resonator"),
        card_type: CardType::Project,
        subtypes: vec![CardSubtype::Nightbound],
        side: Side::Covenant,
        school: School::Beyond,
        rarity: Rarity::Rare,
        abilities: vec![Ability::new(text![
            "When you take the gain mana, progress card, play card, or draw card basic action",
            3,
            "times in a turn,",
            GainActions(1)
        ])
        .delegate(in_play::on_gain_mana_action(|g, s, _| {
            gain_action_if_3x(g, s, |counters| counters.gain_mana_actions)
        }))
        .delegate(in_play::on_progress_card_action(|g, s, _| {
            gain_action_if_3x(g, s, |counters| counters.progress_card_actions)
        }))
        .delegate(in_play::on_card_played(|g, s, _| {
            gain_action_if_3x(g, s, |counters| counters.play_card_actions)
        }))
        .delegate(in_play::on_draw_card_action(|g, s, _| {
            gain_action_if_3x(g, s, |counters| counters.draw_card_actions)
        }))],
        config: CardConfigBuilder::new()
            .raze_cost(5)
            .visual_effect(
                TimedEffectData::new(TimedEffect::MagicCircles1(8))
                    .scale(1.5)
                    .effect_color(design::BLUE_500)
                    .sound(SoundEffect::WaterMagic("RPG3_WaterMagic_Cast02")),
            )
            .build(),
    }
}
