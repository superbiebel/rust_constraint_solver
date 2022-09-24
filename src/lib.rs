mod score;
pub mod score_impl;

pub mod rsolver {
    use rand_chacha::ChaCha8Rng;
    use rand_chacha::rand_core::SeedableRng;
    use uuid::Uuid;
    use crate::score::ScoreTrait;

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

    pub trait ScoreDirector<SolutionType, ScoreType> where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> {
        fn get_current_score(&self) -> ScoreType;
        fn get_current_solution(&self) -> SolutionType;
        fn set_current_score(&self, score: ScoreType) -> ScoreType;
        fn set_current_solution(&self, solution: SolutionType) -> SolutionType;
        fn calculate_score(&self, solution: &SolutionType) -> ScoreType;
    }

    ///The selector is the object that gives an iterator full of moves the phase can do.
    pub trait Selector<SolutionType, InnerType: ?Sized, ScoreType> where SolutionType: Solution<ScoreType>, ScoreType: ScoreTrait {
        fn get_vec(&self, max_amount: u32, solution: &SolutionType, step_scope: StepScope<SolutionType, ScoreType>) -> Vec<Box<InnerType>>;
    }

    pub trait MoveSelector<SolutionType, ScoreType>: Selector<SolutionType, dyn ExecutableMove<SolutionType, ScoreType>, ScoreType>
        where SolutionType: Solution<ScoreType>, ScoreType: ScoreTrait {}

    pub trait EntitySelector<SolutionType, EntityType, ScoreType>: Selector<SolutionType, EntityType, ScoreType>
        where SolutionType: Solution<ScoreType>, EntityType: Entity, ScoreType: ScoreTrait {}

    pub trait ValueSelector<SolutionType, ValueType, ScoreType>: Selector<SolutionType, ValueType, ScoreType>
        where SolutionType: Solution<ScoreType>, ScoreType: ScoreTrait {}

    pub trait TerminationCondition {
        fn should_terminate(&self) -> bool;
    }

    pub trait SolverTerminationCondition<SolutionType, ScoreType>: TerminationCondition + SolverEventListener<SolutionType, ScoreType> {}

    pub trait PhaseTerminationCondition<SolutionType, ScoreType>: TerminationCondition + PhaseEventListener<SolutionType, ScoreType> {}

    pub trait EventListener {}

    pub trait SolverEventListener<SolutionType, ScoreType>: EventListener {
        fn solving_started(&self, solver_scope: &mut SolverScope<SolutionType, ScoreType>);
        fn solving_ended(&self, solver_scope: &mut SolverScope<SolutionType, ScoreType>);
    }

    pub trait PhaseEventListener<SolutionType, ScoreType>: EventListener {
        fn phase_started(&self, phase_scope:&mut PhaseScope<SolutionType, ScoreType>);
        fn phase_ended(&self, phase_scope:&mut PhaseScope<SolutionType, ScoreType>);
    }

    pub trait StepEventListener: EventListener {}

    pub trait MoveEventListener: EventListener {}

    pub struct SolverScope<'a, SolutionType, ScoreType> {
        pub thread_rng: ChaCha8Rng,
        pub config: &'a SolverConfig<SolutionType, ScoreType>,
        pub score_director: Box<dyn ScoreDirector<SolutionType, ScoreType>>
    }

    pub struct PhaseScope<'a, 'b, SolutionType, ScoreType> {
        pub solver_scope: &'a mut SolverScope<'b, SolutionType, ScoreType>,
    }

    pub struct StepScope<'a, 'b, 'c, SolutionType, ScoreType> {
        pub phase_scope: &'a mut PhaseScope<'b, 'c, SolutionType, ScoreType>,
    }

    pub struct MoveScope<'a, 'b, 'c, 'd, SolutionType, ScoreType> {
        step_scope:&'a mut StepScope<'b, 'c, 'd, SolutionType, ScoreType>,
    }

    pub trait Phase<SolutionType, ScoreType> {
        fn do_phase(&self, solution: &mut SolutionType, solver_scope: &mut SolverScope< SolutionType, ScoreType>);
    }

    pub struct SolverConfig<SolutionType, ScoreType> {
        phase_list: Vec<Box<dyn Phase<SolutionType, ScoreType>>>,
        termination: Box<dyn SolverTerminationCondition<SolutionType, ScoreType>>,
        solver_listeners: Vec<Box<dyn SolverEventListener<SolutionType, ScoreType>>>,
        score_director: Box<dyn ScoreDirector<SolutionType, ScoreType>>,
        random_seed: u64,
    }

    pub struct PhaseConfig<SolutionType, ScoreType> {
        pub move_selector: Box<dyn MoveSelector<SolutionType, ScoreType>>,
        termination: Box<dyn PhaseTerminationCondition<SolutionType, ScoreType>>,
        phase_listeners: Vec<Box<dyn SolverEventListener<SolutionType, ScoreType>>>,
    }

    pub struct Solver<SolutionType, ScoreType> {
        solver_config: SolverConfig<SolutionType, ScoreType>
    }

    impl<SolutionType, ScoreType> Solver<SolutionType, ScoreType> {
        fn solve(&mut self, solution: &mut SolutionType) {
            let mut solver_scope: SolverScope<SolutionType, ScoreType> = SolverScope {
                thread_rng: ChaCha8Rng::seed_from_u64(self.solver_config.random_seed),
                config: &self.solver_config
            };
            for phase in &self.solver_config.phase_list {
                if solver_scope.config.termination.should_terminate() {
                    break;
                }
                phase.do_phase(solution, &mut solver_scope);
            }
        }
    }
}
#[cfg(test)]
mod tests {
}