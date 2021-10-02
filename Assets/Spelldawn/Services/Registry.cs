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

using System;
using Spelldawn.Game;
using Spelldawn.Protos;
using UnityEngine;
using UnityEngine.UIElements;

#nullable enable

namespace Spelldawn.Services
{
  public sealed class Registry : MonoBehaviour
  {
    public Camera MainCamera => _mainCamera;
    [SerializeField] Camera _mainCamera = null!;

    /// <summary>'Scaled with Screen Size' document for rendering gameplay</summary>
    public UIDocument GameDocument => _gameDocument;
    [SerializeField] UIDocument _gameDocument = null!;

    /// <summary>'Constant Physical Size' document for rendering gameplay</summary>
    public UIDocument InterfaceDocument => _interfaceDocument;
    [SerializeField] UIDocument _interfaceDocument = null!;

    public AssetService AssetService => _assetService;
    [SerializeField] AssetService _assetService = null!;

    public ActionService ActionService => _actionService;
    [SerializeField] ActionService _actionService = null!;

    public CardService CardService => _cardService;
    [SerializeField] CardService _cardService = null!;

    public CommandService CommandService => _commandService;
    [SerializeField] CommandService _commandService = null!;

    public DocumentService DocumentService => _documentService;
    [SerializeField] DocumentService _documentService = null!;

    public SampleData SampleData => _sampleData;
    [SerializeField] SampleData _sampleData = null!;

    public ArenaService ArenaService => _arenaService;
    [SerializeField] ArenaService _arenaService = null!;

    [SerializeField] Hand _userHand = null!;
    [SerializeField] Hand _opponentHand = null!;
    public Hand PlayerHand(PlayerName playerName) => playerName == PlayerName.User ? _userHand : _opponentHand;
  }
}