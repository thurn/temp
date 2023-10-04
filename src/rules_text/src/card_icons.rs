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

use assets;
use assets::CardIconType;
use game_data::card_view_context::CardViewContext;
use game_data::primitives::ManaValue;
use protos::spelldawn::{CardIcon, CardIcons};
use rules::queries;

pub fn build(context: &CardViewContext, revealed: bool) -> CardIcons {
    let definition = context.definition();
    let mut icons = CardIcons::default();

    match context.card_data() {
        Some(data) if data.progress > 0 => {
            icons.arena_icon = Some(CardIcon {
                background: Some(assets::card_icon(CardIconType::LevelCounter)),
                text: Some(data.progress.to_string()),
                background_scale: assets::icon_background_scale(CardIconType::LevelCounter),
            })
        }
        _ => {}
    }

    match context.card_data() {
        Some(data) if data.stored_mana > 0 => {
            icons.arena_icon = Some(CardIcon {
                background: Some(assets::card_icon(CardIconType::Mana)),
                text: Some(data.stored_mana.to_string()),
                background_scale: assets::icon_background_scale(CardIconType::Mana),
            })
        }
        _ => {}
    }

    if revealed {
        icons.top_left_icon = if let Some(mana_cost) =
            context.query_id_or(definition.cost.mana, queries::mana_cost)
        {
            Some(mana_card_icon(mana_cost))
        } else {
            definition.config.stats.scheme_points.map(|points| CardIcon {
                background: Some(assets::card_icon(CardIconType::LevelRequirement)),
                text: Some(points.level_requirement.to_string()),
                background_scale: assets::icon_background_scale(CardIconType::LevelRequirement),
            })
        };

        icons.bottom_right_icon = if let Some(attack) = definition.config.stats.base_attack {
            Some(CardIcon {
                background: Some(assets::card_icon(CardIconType::Attack)),
                text: Some(context.query_id_or(attack, queries::base_attack).to_string()),
                background_scale: assets::icon_background_scale(CardIconType::Attack),
            })
        } else if let Some(health) = definition.config.stats.health {
            Some(CardIcon {
                background: Some(assets::card_icon(CardIconType::Health)),
                text: Some(context.query_id_or(health, queries::health).to_string()),
                background_scale: assets::icon_background_scale(CardIconType::Health),
            })
        } else if let Some(raze) = definition.config.stats.raze_cost {
            Some(CardIcon {
                background: Some(assets::card_icon(CardIconType::Raze)),
                text: Some(context.query_id_or(raze, queries::raze_cost).to_string()),
                background_scale: assets::icon_background_scale(CardIconType::Raze),
            })
        } else {
            definition.config.stats.scheme_points.map(|points| CardIcon {
                background: Some(assets::card_icon(CardIconType::Points)),
                text: Some(points.points.to_string()),
                background_scale: assets::icon_background_scale(CardIconType::Points),
            })
        };

        let shield = context
            .query_id_or(definition.config.stats.shield.unwrap_or_default(), queries::shield);
        icons.bottom_left_icon = if shield > 0 {
            Some(CardIcon {
                background: Some(assets::card_icon(CardIconType::Shield)),
                text: Some(shield.to_string()),
                background_scale: assets::icon_background_scale(CardIconType::Shield),
            })
        } else {
            None
        };
    }

    icons
}

pub fn mana_card_icon(value: ManaValue) -> CardIcon {
    CardIcon {
        background: Some(assets::card_icon(CardIconType::Mana)),
        text: Some(value.to_string()),
        background_scale: assets::icon_background_scale(CardIconType::Mana),
    }
}
