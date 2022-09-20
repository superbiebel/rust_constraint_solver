use crate::score::{ScoreModifier, ScoreTrait};

struct HardSoftIntScoreModifier;
pub struct HardSoftIntScore {
    hard_score: i32,
    soft_score: i32,
}

impl ScoreTrait for HardSoftIntScore {
    fn feasible(&self) -> bool {
        self.hard_score < 0
    }

    fn is_zero(&self) -> bool {
        self.hard_score == 0 && self.soft_score == 0
    }
}
impl ScoreModifier<HardSoftIntScore> for HardSoftIntScoreModifier {
    fn add(&self, first: HardSoftIntScore, second: HardSoftIntScore) -> HardSoftIntScore {
        HardSoftIntScore {
            hard_score: first.hard_score + second.hard_score,
            soft_score: first.soft_score + second.hard_score,
        }
    }

    fn subtract(&self, first: HardSoftIntScore, second: HardSoftIntScore) -> HardSoftIntScore {
        HardSoftIntScore {
            hard_score: first.hard_score - second.hard_score,
            soft_score: first.soft_score - second.hard_score,
        }
    }

    fn multiply(&self, first: HardSoftIntScore, second: HardSoftIntScore) -> HardSoftIntScore {
        HardSoftIntScore {
            hard_score: first.hard_score * second.hard_score,
            soft_score: first.soft_score * second.hard_score,
        }
    }

    fn divide(&self, first: HardSoftIntScore, second: HardSoftIntScore) -> HardSoftIntScore {
        HardSoftIntScore {
            hard_score: first.hard_score / second.hard_score,
            soft_score: first.soft_score / second.hard_score,
        }
    }

    fn power(&self, first: HardSoftIntScore, other: u32) -> HardSoftIntScore {
        HardSoftIntScore {
            hard_score: first.hard_score.pow(other),
            soft_score: first.soft_score.pow(other),
        }
    }

    fn negate(&self, first: HardSoftIntScore) -> HardSoftIntScore {
        HardSoftIntScore {
            hard_score: -first.hard_score,
            soft_score: -first.soft_score,
        }
    }

    fn zero() -> HardSoftIntScore {
        HardSoftIntScore {
            hard_score: 0,
            soft_score: 0,
        }
    }
}

