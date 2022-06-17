mod entity_registry;

trait Phase{
    fn do_phase(self);
}

struct SolverScope{}
struct PhaseScope{
    solver_scope: SolverScope
}
impl PhaseScope {
    fn new(scope: SolverScope) -> PhaseScope{
        PhaseScope{solver_scope: scope}
    }
}
struct StepScope{
    phase_scope: PhaseScope
}
impl StepScope {
    fn new(scope: PhaseScope) -> StepScope{
        StepScope{phase_scope: scope}
    }
}
struct MoveScope{
    step_scope: StepScope
}
impl MoveScope {
    fn new(scope: StepScope) -> MoveScope{
        MoveScope{step_scope: scope}
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

    }
}
