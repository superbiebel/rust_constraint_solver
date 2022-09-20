pub trait ScoreTrait {
    fn feasible(&self) -> bool;
    fn is_zero(&self) -> bool;
}
///Gives a NEW instance in return!
pub trait ScoreModifier<ScoreType> where ScoreType: ScoreTrait {
    fn add(&self, first: ScoreType, second: ScoreType) -> ScoreType;
    fn subtract(&self, first: ScoreType, second: ScoreType) -> ScoreType;
    fn multiply(&self, first: ScoreType, second: ScoreType) -> ScoreType;
    fn divide(&self, first: ScoreType, second: ScoreType) -> ScoreType;
    fn power(&self, first: ScoreType, other: u32) -> ScoreType;
    fn negate(&self, first: ScoreType) -> ScoreType;

    fn zero() -> ScoreType;
}
pub trait ScoreCalculator<SolutionType> {
    fn calculate_score(&self, solution: &SolutionType) -> Box<dyn ScoreTrait>;
}
