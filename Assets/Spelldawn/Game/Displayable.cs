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

using Spelldawn.Utils;
using UnityEngine;
using UnityEngine.Rendering;

#nullable enable

namespace Spelldawn.Game
{
  public abstract class Displayable : Clickable
  {
    [Header("Displayable")] [SerializeField]
    ObjectDisplay? _parent;

    [SerializeField] GameContext _gameContext;

    [SerializeField] SortingGroup? _sortingGroup;

    /// <summary>Provided by the server, used to order items within a display.</summary>
    public uint SortingKey { get; set; }
    
    /// <summary>Tie-breaker key in the case of sorting key ties.</summary>
    public uint SortingSubkey { get; set; }    

    public ObjectDisplay? Parent
    {
      get => _parent;
      set => _parent = value;
    }

    public virtual bool IsContainer() => false;

    public virtual float DefaultScale => 1.0f;

    protected void Start()
    {
      if (_sortingGroup && _gameContext != GameContext.Unspecified)
      {
        SortingOrder.Create(_gameContext, (int)SortingKey, (int)SortingSubkey).ApplyTo(_sortingGroup!);
      }

      OnStart();
    }

    protected virtual void OnStart() {}

    /// <summary>Called on a child container when the parent is repositioned.</summary>
    public virtual void OnUpdateParentContainer()
    {
    }

    public GameContext GameContext => Errors.CheckNotDefault(HasGameContext ? _gameContext : DefaultGameContext());

    public bool HasGameContext => _gameContext != GameContext.Unspecified;

    protected virtual GameContext DefaultGameContext() => GameContext.Unspecified;

    public void SetGameContext(GameContext gameContext)
    {
      Errors.CheckNotDefault(gameContext);
      
      if (_sortingGroup)
      {
        SortingOrder.Create(gameContext, (int)SortingKey, (int)SortingSubkey).ApplyTo(_sortingGroup!);
      }      

      if (_gameContext != gameContext)
      {
        var oldContext = _gameContext;
        _gameContext = gameContext;
        OnSetGameContext(oldContext, gameContext);
      }
    }

    protected abstract void OnSetGameContext(GameContext oldContext, GameContext newContext);
  }
}