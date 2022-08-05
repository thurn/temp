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

use std::time::Instant;

use ai_core::game_state_node::GameStateNode;
use ai_core::selection_algorithm::SelectionAlgorithm;
use ai_core::state_evaluator::StateEvaluator;
use anyhow::Result;

use crate::scored_action::ScoredAction;

/// The Minimax search algorithm, one of the simplest tree search algorithms.
///
/// See <https://en.wikipedia.org/wiki/Minimax>
pub struct MinimaxAlgorithm {
    pub search_depth: u32,
}

impl SelectionAlgorithm for MinimaxAlgorithm {
    fn pick_action<N, E>(
        &self,
        deadline: Instant,
        node: &N,
        evaluator: &E,
        player: N::PlayerName,
    ) -> Result<N::Action>
    where
        N: GameStateNode,
        E: StateEvaluator<N>,
    {
        run_internal(deadline, node, evaluator, self.search_depth, player)?.action()
    }
}

fn run_internal<N, E>(
    deadline: Instant,
    node: &N,
    evaluator: &E,
    depth: u32,
    player: N::PlayerName,
) -> Result<ScoredAction<N::Action>>
where
    N: GameStateNode,
    E: StateEvaluator<N>,
{
    Ok(match node.current_turn() {
        _ if depth == 0 => ScoredAction::new(evaluator.evaluate(node, player)),
        None => ScoredAction::new(evaluator.evaluate(node, player)),
        Some(current) if current == player => {
            let mut result = ScoredAction::new(i64::MIN);
            // I was worried about creating a ScoredAction and tracking the action
            // unnecessarily for children, but it makes no performance
            // difference in benchmark tests.
            for action in node.legal_actions()? {
                if deadline_exceeded(deadline, depth) {
                    return Ok(result.with_fallback_action(action));
                }
                let mut child = node.make_copy();
                child.execute_action(current, action)?;
                result.insert_max(
                    action,
                    run_internal(deadline, &child, evaluator, depth - 1, player)?.score(),
                );
            }
            result
        }
        Some(current) => {
            let mut result = ScoredAction::new(i64::MAX);
            for action in node.legal_actions()? {
                if deadline_exceeded(deadline, depth) {
                    return Ok(result.with_fallback_action(action));
                }
                let mut child = node.make_copy();
                child.execute_action(current, action)?;
                result.insert_min(
                    action,
                    run_internal(deadline, &child, evaluator, depth - 1, player)?.score(),
                );
            }
            result
        }
    })
}

/// Check whether `deadline` has been exceeded. Only checks deadlines for higher
/// parts of the tree to avoid excessive calls to Instant::now().
fn deadline_exceeded(deadline: Instant, depth: u32) -> bool {
    depth > 1 && deadline < Instant::now()
}
