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

using System.Collections;
using UnityEngine;

namespace Spelldawn.Game
{
  public sealed class RewardChest : MonoBehaviour
  {
    [SerializeField] GameObject? _buildupGlow;
    [SerializeField] bool _autoOpen;
    
    public RewardDisplay? RewardDisplay { get; set; }

    IEnumerator AutoOpen()
    {
      yield return new WaitForSeconds(1f);
      OnOpened();
    }

    // ReSharper disable once UnusedMember.Local (Called by Animator)
    void OnOpened()
    {
      RewardDisplay!.OnOpened();
    }

    public void SetGlowEnabled(bool glowEnabled)
    {
      if (_buildupGlow)
      {
        _buildupGlow!.SetActive(glowEnabled);
      }
      
      if (_autoOpen && glowEnabled)
      {
        StartCoroutine(AutoOpen());
      }      
    }
  }
}