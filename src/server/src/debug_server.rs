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

use std::collections::HashMap;
use std::mem;
use std::sync::{Mutex, OnceLock};

use ::panels::add_to_hand_panel::AddToHandPanel;
use anyhow::Result;
use core_ui::actions::InterfaceAction;
use core_ui::panels;
use database::Database;
use game_data::game::GameState;
use game_data::player_name::{AIPlayer, PlayerId};
use game_data::primitives::{GameId, Side};
use panel_address::Panel;
use player_data::PlayerStatus;
use protos::spelldawn::client_debug_command::DebugCommand;
use protos::spelldawn::game_command::Command;
use protos::spelldawn::{ClientAction, ClientDebugCommand, LoadSceneCommand, SceneLoadMode};
use rules::{mana, mutations};
use ulid::Ulid;
use user_action_data::{
    DebugAction, NamedDeck, NewGameAction, NewGameDebugOptions, NewGameDeck, UserAction,
};
use with_error::WithError;

use crate::server_data::{ClientData, GameResponse, RequestData};
use crate::{adventure_server, game_server, requests};

fn current_game_id() -> &'static Mutex<Option<GameId>> {
    static GAME_ID: OnceLock<Mutex<Option<GameId>>> = OnceLock::new();
    GAME_ID.get_or_init(|| Mutex::new(None))
}

pub async fn handle_debug_action(
    database: &impl Database,
    data: &RequestData,
    action: &DebugAction,
    request_fields: &HashMap<String, String>,
) -> Result<GameResponse> {
    match action {
        DebugAction::NewGame(side) => create_debug_game(data, *side),
        DebugAction::JoinGame(side) => {
            let game_id = current_game_id().lock().unwrap().expect("game_id");
            let mut game = requests::fetch_game(database, Some(game_id)).await?;
            match side {
                Side::Overlord => game.overlord.id = data.player_id,
                Side::Champion => game.champion.id = data.player_id,
            }
            let result = reload_scene(data, &game);
            database.write_game(&game).await?;
            let mut player = requests::fetch_player(database, data.player_id).await?;
            player.status = Some(PlayerStatus::Playing(game_id));
            database.write_player(&player).await?;
            result
        }
        DebugAction::FlipViewpoint => {
            requests::with_game(database, data, |game| {
                mem::swap(&mut game.champion.id, &mut game.overlord.id);
                reload_scene(data, game)
            })
            .await
        }
        DebugAction::AddMana(amount) => {
            game_server::update_game(database, data, |game, user_side| {
                mana::gain(game, user_side, *amount);
                Ok(())
            })
            .await
        }
        DebugAction::AddActionPoints(amount) => {
            game_server::update_game(database, data, |game, user_side| {
                game.player_mut(user_side).actions += amount;
                Ok(())
            })
            .await
        }
        DebugAction::AddScore(amount) => {
            game_server::update_game(database, data, |game, user_side| {
                mutations::score_points(game, user_side, *amount)?;
                Ok(())
            })
            .await
        }
        DebugAction::SaveGameState(index) => {
            let mut game = requests::fetch_game(database, data.game_id).await?;
            let game_id = GameId::new_from_u128(100 + index);
            game.id = game_id;
            database.write_game(&game).await?;
            Ok(GameResponse::new(ClientData::propagate(data)))
        }
        DebugAction::LoadGameState(index) => {
            let game_id = data.game_id.with_error(|| "Expected game_id")?;
            let saved_id = GameId::new_from_u128(100 + index);
            let mut game = requests::fetch_game(database, Some(saved_id)).await?;
            game.id = game_id;
            let result = reload_scene(data, &game);
            database.write_game(&game).await?;
            result
        }
        DebugAction::SavePlayerState(index) => {
            let mut player = requests::fetch_player(database, data.player_id).await?;
            let player_id = PlayerId::Database(Ulid(*index));
            player.id = player_id;
            database.write_player(&player).await?;
            Ok(GameResponse::new(ClientData::propagate(data)))
        }
        DebugAction::LoadPlayerState(index) => {
            let saved_id = PlayerId::Database(Ulid(*index));
            let mut player = requests::fetch_player(database, saved_id).await?;
            player.id = data.player_id;
            let result = reload_world_scene(data);
            database.write_player(&player).await?;
            Ok(result)
        }
        DebugAction::SetNamedPlayer(side, name) => {
            game_server::update_game(database, data, |game, _| {
                game.player_mut(*side).id = PlayerId::AI(*name);
                Ok(())
            })
            .await
        }
        DebugAction::AddCoins(coins) => {
            adventure_server::update_adventure(database, data, |state| {
                state.coins += *coins;
                Ok(())
            })
            .await
        }
        DebugAction::FilterCardList => {
            let input = request_fields.get("CardName").with_error(|| "Expected CardName")?;
            Ok(GameResponse::new(ClientData::propagate(data))
                .command(panels::update(AddToHandPanel::new(input).build_panel().unwrap())))
        }
        DebugAction::AddToHand(card_name) => {
            game_server::update_game(database, data, |game, user_side| {
                if let Some(top_of_deck) =
                    mutations::realize_top_of_deck(game, user_side, 1)?.get(0)
                {
                    mutations::overwrite_card(game, *top_of_deck, *card_name)?;
                    mutations::draw_cards(game, user_side, 1)?;
                }
                Ok(())
            })
            .await
        }
    }
}

fn create_debug_game(data: &RequestData, side: Side) -> Result<GameResponse> {
    let id = GameId::new(Ulid::new());
    let _ = current_game_id().lock().unwrap().insert(id);
    Ok(GameResponse::new(ClientData::propagate(data)).commands(vec![Command::Debug(
        ClientDebugCommand {
            debug_command: Some(DebugCommand::InvokeAction(ClientAction {
                action: Some(
                    UserAction::NewGame(NewGameAction {
                        opponent: PlayerId::AI(match side {
                            Side::Overlord => AIPlayer::DebugChampion,
                            Side::Champion => AIPlayer::DebugOverlord,
                        }),
                        deck: match side {
                            Side::Overlord => NewGameDeck::NamedDeck(NamedDeck::CanonicalOverlord),
                            Side::Champion => NewGameDeck::NamedDeck(NamedDeck::CanonicalChampion),
                        },
                        debug_options: Some(NewGameDebugOptions {
                            deterministic: false,
                            override_game_id: Some(id),
                        }),
                        tutorial: false,
                    })
                    .as_client_action(),
                ),
            })),
        },
    )]))
}

fn reload_scene(data: &RequestData, game: &GameState) -> Result<GameResponse> {
    let command = Command::LoadScene(LoadSceneCommand {
        scene_name: "Game".to_string(),
        mode: SceneLoadMode::Single as i32,
        skip_if_current: false,
    });
    let user_side = game.player_side(data.player_id)?;
    let opponent_id = game.player(user_side.opponent()).id;
    Ok(GameResponse::new(ClientData::with_game_id(data, Some(game.id)))
        .command(command.clone())
        .opponent_response(opponent_id, vec![command]))
}

fn reload_world_scene(data: &RequestData) -> GameResponse {
    let command = Command::LoadScene(LoadSceneCommand {
        scene_name: "World".to_string(),
        mode: SceneLoadMode::Single as i32,
        skip_if_current: false,
    });
    GameResponse::new(ClientData::propagate(data)).command(command)
}
