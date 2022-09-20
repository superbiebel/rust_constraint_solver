use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use uuid::Uuid;
use rust_constraint_solver::rsolver::{ExecutableMove, MoveSelector, Phase, PhaseConfig, PhaseScope, Selector, Solution, SolverScope, StepScope};
use rust_constraint_solver::score_impl::hard_soft_int_score::HardSoftIntScore;

#[test]
fn knapsack_test() {

    struct HillClimbingPhase {
        phase_config: PhaseConfig<KnapSackSolution, HardSoftIntScore>
    }
    impl Phase<KnapSackSolution> for HillClimbingPhase {
        fn do_phase(&self, solution: &mut KnapSackSolution) {
            let phase_scope = PhaseScope {
                solver_scope: &mut SolverScope {}
            };
            let moves = self.phase_config.move_selector.get_vec(10, solution);
        }
    }

    let item1 = KnapsackItem {
        weight: 1,
        worth: 5,
        in_knapsack: false
    };
    let item2 = KnapsackItem {
        weight: 5,
        worth: 10,
        in_knapsack: false
    };
    let item3 = KnapsackItem {
        weight: 1,
        worth: 5,
        in_knapsack: false
    };
    let item4 = KnapsackItem {
        weight: 1,
        worth: 5,
        in_knapsack: false
    };

    struct KnapsackMoveSelector;

    impl Selector<KnapSackSolution, dyn ExecutableMove<KnapSackSolution, HardSoftIntScore>, HardSoftIntScore> for KnapsackMoveSelector {
        fn get_vec(&self, max_amount: u32, solution: &KnapSackSolution, step_scope: &mut StepScope<KnapSackSolution, HardSoftIntScore>) -> Vec<Box<dyn ExecutableMove<KnapSackSolution, HardSoftIntScore>>> {
            let mut vector: Vec<Box<dyn ExecutableMove<KnapSackSolution, HardSoftIntScore>>> = Vec::new();
            let element = Box::new(RandomFlipKnapsackMove {
                index: step_scope.phase_scope.solver_scope.thread_rng.gen_range(0..u32::try_from(solution.items.len())),
            });
            vector.push(element);
            vector
        }
    }

    impl MoveSelector<KnapSackSolution, HardSoftIntScore> for KnapsackMoveSelector {}
    struct KnapsackItem {
        weight:u32,
        worth:u32,
        in_knapsack: bool,
    }
    struct KnapSackSolution {
        uuid: Uuid,
        max_weight:u32,
        items: Vec<KnapsackItem> //planning variable (changeable)
    }
    impl Solution<HardSoftIntScore> for KnapSackSolution {
        fn get_solution_uuid(&self) -> Uuid {
            todo!()
        }
    }
    struct RandomFlipKnapsackMove {
        index: u32,
    }
    impl ExecutableMove<KnapSackSolution, HardSoftIntScore> for RandomFlipKnapsackMove {
        fn get_undo_move(&self, solution: &KnapSackSolution) -> Box<dyn ExecutableMove<KnapSackSolution, HardSoftIntScore>> {
            Box::new(RandomFlipKnapsackMove {
                index: self.index,
            })
        }

        fn is_doable(&self, solution: &KnapSackSolution) -> bool {
            solution.items.get(self.index as usize).is_some()
        }

        fn do_move_unchecked(&self, solution: &mut KnapSackSolution) -> Box<dyn ExecutableMove<KnapSackSolution, HardSoftIntScore>> {
            let item:&mut KnapsackItem = solution.items.get_mut(self.index as usize).unwrap();
            if item.in_knapsack {
                item.in_knapsack = false;
            } else {
                item.in_knapsack = true;
            }
            self.get_undo_move(solution)
        }
    }
}