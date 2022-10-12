use crate::score::ScoreTrait;
use crate::solver::{EventListener, Solution, SolverEventListener, SolverScope, SolverTerminationCondition, TerminationCondition};

struct TimeBasedTermination {
    terminate_time: u64,
    current_run_length: u64,
}
impl TerminationCondition for TimeBasedTermination {
    fn should_terminate(&self) -> bool {
        self.current_run_length > self.terminate_time
    }
}

impl<SolutionType, ScoreType> SolverEventListener<SolutionType, ScoreType> for TimeBasedTermination where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType> + Clone {
    fn solving_started(&self, solver_scope: &mut SolverScope<SolutionType, ScoreType>) {
        todo!()
    }

    fn solving_ended(&self, solver_scope: &mut SolverScope<SolutionType, ScoreType>) {
        todo!()
    }
}

impl<SolutionType, ScoreType> SolverTerminationCondition<SolutionType, ScoreType>  for TimeBasedTermination where ScoreType: ScoreTrait, SolutionType: Solution<ScoreType>+ Clone{

}

impl EventListener for TimeBasedTermination {}