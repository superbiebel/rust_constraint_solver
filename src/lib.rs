mod score;
pub mod score_impl;
mod termination;

pub mod solver {
    use rand_chacha::ChaCha8Rng;
    use rand_chacha::rand_core::SeedableRng;
    use uuid::Uuid;
    use crate::score::{ScoreCalculator, ScoreTrait};

    pub trait Solution<ScoreType: ScoreTrait> {
        fn get_solution_uuid(&self) -> Uuid;
    }

    pub trait Entity {}

    pub trait ExecutableMove<SolutionType, ScoreType> where SolutionType: Solution<ScoreType>, ScoreType: ScoreTrait {
        fn do_move_unchecked(&self, solution: &mut SolutionType) -> Box<dyn ExecutableMove<SolutionType, ScoreType>>;
        fn get_undo_move(&self, solution: &SolutionType) -> Box<dyn ExecutableMove<SolutionType, ScoreType>>;
        fn is_doable(&self, solution: &SolutionType) -> bool;
        fn do_move(&self, solution: &mut SolutionType) -> Option<Box<dyn ExecutableMove<SolutionType, ScoreType>>> {
            if !self.is_doable(solution) {
                return None
            }
            Some(self.do_move_unchecked(solution))
        }
    }
    pub struct SolutionTracker<SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone {
        score_calculator: Box<dyn ScoreCalculator<SolutionType, ScoreType>>,
        best_solution: Option<SolutionType>,
        best_solution_score: Option<ScoreType>,
        current_solution: SolutionType,
        current_solution_score: Option<ScoreType>,
    }
    impl<SolutionType, ScoreType> Clone for SolutionTracker<SolutionType, ScoreType> where SolutionType: Solution<ScoreType> + Clone, ScoreType: ScoreTrait{
        fn clone(&self) -> Self {
            SolutionTracker {
                score_calculator: self.score_calculator.clone(),
                best_solution: self.best_solution.clone(),
                best_solution_score: self.best_solution.clone(),
                current_solution: self.current_solution.clone(),
                current_solution_score: None
            }

        }
    }

    impl<SolutionType, ScoreType> SolutionTracker<SolutionType, ScoreType>
        where ScoreType: ScoreTrait + Clone, SolutionType: Solution<ScoreType> + Clone{
        fn get_current_solution_score(&self) -> ScoreType {
            match &self.current_solution_score {
                None => {
                    self.calculate_score(&self.current_solution)}
                Some(score) => {score.clone()}
            }
        }
        fn borrow_current_solution(&self) -> &SolutionType {
            &self.current_solution
        }
        fn set_current_solution(&mut self, solution: SolutionType, precalculated_score: Option<ScoreType>) {
            self.current_solution = solution;
            match precalculated_score {
                None => {
                    self.current_solution_score = Some(self.score_calculator.calculate_score(&self.current_solution));
                }
                Some(precalc_score) => {
                    self.current_solution_score = Some(precalc_score);
                }
            }
        }
        fn set_current_and_best(&mut self, solution: SolutionType) -> bool {
            self.current_solution = solution;

            self.current_solution_score = Some(self.calculate_score(&self.current_solution));

            if self.current_solution_score > self.best_solution_score {
                self.best_solution = Some(self.current_solution.clone());
                self.best_solution_score = self.current_solution_score.clone();
                return true
            }
            return false
        }
        fn calculate_score(&self, solution: &SolutionType) -> ScoreType {
            self.score_calculator.calculate_score(solution)
        }
    }

    ///The selector is the object that gives an iterator full of moves the phase can do.
    pub trait Selector<SolutionType, InnerType: ?Sized, ScoreType> where SolutionType: Solution<ScoreType> + Clone, ScoreType: ScoreTrait {
        fn get_vec(&self, max_amount: u32, solution: &SolutionType, step_scope: &StepScope<SolutionType, ScoreType>) -> Vec<Box<InnerType>>;
    }

    pub trait MoveSelector<SolutionType, ScoreType>: Selector<SolutionType, dyn ExecutableMove<SolutionType, ScoreType>, ScoreType>
        where SolutionType: Solution<ScoreType>+ Clone, ScoreType: ScoreTrait {}

    pub trait EntitySelector<SolutionType, EntityType, ScoreType>: Selector<SolutionType, EntityType, ScoreType>
        where SolutionType: Solution<ScoreType>+ Clone, EntityType: Entity, ScoreType: ScoreTrait {}

    pub trait ValueSelector<SolutionType, ValueType, ScoreType>: Selector<SolutionType, ValueType, ScoreType>
        where SolutionType: Solution<ScoreType>+ Clone, ScoreType: ScoreTrait {}

    pub trait TerminationCondition {
        fn should_terminate(&self) -> bool;
    }

    pub trait SolverTerminationCondition<SolutionType, ScoreType>: TerminationCondition + SolverEventListener<SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType>+ Clone {}

    pub trait PhaseTerminationCondition<SolutionType, ScoreType>: TerminationCondition + PhaseEventListener<SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType>+ Clone {}

    pub trait EventListener {}

    pub trait SolverEventListener<SolutionType, ScoreType>: EventListener where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone {
        fn solving_started(&self, solver_scope: &mut SolverScope<SolutionType, ScoreType>);
        fn solving_ended(&self, solver_scope: &mut SolverScope<SolutionType, ScoreType>);
    }

    pub trait PhaseEventListener<SolutionType, ScoreType>: EventListener where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone{
        fn phase_started(&self, phase_scope:&mut PhaseScope<SolutionType, ScoreType>);
        fn phase_ended(&self, phase_scope:&mut PhaseScope<SolutionType, ScoreType>);
    }

    pub trait StepEventListener: EventListener {}

    pub trait MoveEventListener: EventListener {}

    pub struct SolverScope<SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone {
        pub thread_rng: ChaCha8Rng,
        pub solution_tracker: SolutionTracker<SolutionType, ScoreType>
    }

    pub struct PhaseScope<'a,SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone {
        pub solver_scope: &'a mut SolverScope<SolutionType, ScoreType>,
    }

    pub struct StepScope<'a, 'b, SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone {
        pub phase_scope: &'a mut PhaseScope<'b, SolutionType, ScoreType>,
    }

    pub struct MoveScope<'a, 'b, 'c, SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType>+ Clone{
        step_scope:&'a mut StepScope<'b, 'c, SolutionType, ScoreType>,
    }

    pub trait Phase<SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone{
        fn do_phase(&self, solver_scope: &mut SolverScope<SolutionType, ScoreType>);
    }
    impl<SolutionType, ScoreType> Solver<SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone{
        fn new(phases: Vec<Box<dyn Phase<SolutionType, ScoreType>>>,
               termination: Box<dyn SolverTerminationCondition<SolutionType, ScoreType>>,
               solver_listeners: Vec<Box<dyn SolverEventListener<SolutionType, ScoreType>>>,
               solution_tracker: SolutionTracker<SolutionType, ScoreType>, random_seed: u64) -> Solver<SolutionType, ScoreType> {
            Solver {
                phase_list: phases,
                termination,
                solver_listeners,
                score_director: solution_tracker,
                random_seed
            }
        }
    }

    pub struct PhaseConfig<SolutionType, ScoreType> {
        pub move_selector: Box<dyn MoveSelector<SolutionType, ScoreType>>,
        termination: Box<dyn PhaseTerminationCondition<SolutionType, ScoreType>>,
        phase_listeners: Vec<Box<dyn SolverEventListener<SolutionType, ScoreType>>>,
    }

    pub struct Solver<SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone{
        pub phase_list: Vec<Box<dyn Phase<SolutionType, ScoreType>>>,
        pub termination: Box<dyn SolverTerminationCondition<SolutionType, ScoreType>>,
        pub solver_listeners: Vec<Box<dyn SolverEventListener<SolutionType, ScoreType>>>,
        pub score_director: SolutionTracker<SolutionType, ScoreType>,
        pub random_seed: u64,
    }

    impl<SolutionType, ScoreType> Solver<SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone{
        fn solve(&mut self, solution: &mut SolutionType) {
            let mut solver_scope: SolverScope<SolutionType, ScoreType> = SolverScope {
                thread_rng: ChaCha8Rng::seed_from_u64(self.random_seed.clone()),
                solution_tracker: self.score_director.clone(),
            };
            for phase in &mut self.phase_list {
                if self.termination.should_terminate() {
                    break;
                }
                phase.do_phase(&mut solver_scope);
            }
        }
        fn fire_solver_begin(&mut self, solver_scope: &mut SolverScope<SolutionType, ScoreType>) {
            for listener_id in 0..self.solver_listeners.len() {
                self.solver_listeners.get(listener_id).unwrap().solving_started(solver_scope)
            }
        }
    }
}
#[cfg(test)]
mod tests {
}