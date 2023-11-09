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

#nullable enable

using System;

namespace Spelldawn.Game
{
  /// <summary>
  /// Represents the current location and z position of a GameObject.
  /// </summary>
  public enum GameContext
  {
    // Note: Enum numbers are serialized by Unity and cannot be changed.
    //
    // Keep in sync with 'SortOrder' below and with the 'Layers' tab in Unity
    Unspecified = 0,
    Hidden = 17,
    Riftcallers = 21,
    BehindArena = 25,
    Arena = 1,
    Deck = 2,
    DiscardPile = 3,
    GameCharacter = 16,
    ArenaRaidParticipant = 10,
    RaidParticipant = 4,
    HandStorage = 24,    
    Interface = 6,
    Browser = 12,
    Staging = 7,
    RevealedCardsBrowser = 18,
    Hand = 5,
    CardChoiceBrowser = 23,
    BrowserDragTarget = 22,
    Scored = 11,
    Effects = 8,
    Dragging = 9,
    UserMessage = 13,
    LongPressBrowser = 19,
    RewardBrowser = 14,
    InfoZoom = 15,
    SplashScreen = 20,

  }

  public static class GameContextUtil
  {
    public static int SortOrder(this GameContext gameContext) => gameContext switch
    {
      GameContext.Unspecified => 0,
      GameContext.Hidden => 1,
      GameContext.Riftcallers => 2,
      GameContext.BehindArena => 3,
      GameContext.Arena => 4,
      GameContext.Deck => 5,
      GameContext.DiscardPile => 6,
      GameContext.GameCharacter => 7,
      GameContext.ArenaRaidParticipant => 8,
      GameContext.RaidParticipant => 9,
      GameContext.HandStorage => 10,
      GameContext.Interface => 11,
      GameContext.Browser => 12,
      GameContext.Staging => 13,
      GameContext.RevealedCardsBrowser => 14,
      GameContext.Hand => 15,
      GameContext.CardChoiceBrowser => 16,
      GameContext.BrowserDragTarget => 17,
      GameContext.Scored => 18,
      GameContext.Effects => 19,
      GameContext.Dragging => 20,
      GameContext.UserMessage => 21,
      GameContext.LongPressBrowser => 22,
      GameContext.RewardBrowser => 23,
      GameContext.InfoZoom => 24,
      GameContext.SplashScreen => 25,
      _ => throw new ArgumentOutOfRangeException(nameof(gameContext), gameContext, null)
    };

    public static bool ShouldRenderArenaCard(this GameContext gameContext) => gameContext switch
    {
      GameContext.Arena => true,
      GameContext.ArenaRaidParticipant => true,
      GameContext.Riftcallers => true,
      _ => false
    };
  }
}