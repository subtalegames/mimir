#[cfg(feature = "float")]
use float_cmp::approx_eq;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait Evaluator<T> {
    fn evaluate(self, value: T) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(feature = "float")]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum FloatRangeBound {
    Exclusive(f64),
    Inclusive(f64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(feature = "float")]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum FloatEvaluator {
    EqualTo(f64),
    NotEqualTo(f64),
    LessThan(FloatRangeBound),
    GreaterThan(FloatRangeBound),
    InRange(FloatRangeBound, FloatRangeBound),
}

#[cfg(feature = "float")]
impl Evaluator<f64> for FloatEvaluator {
    fn evaluate(self, value: f64) -> bool {
        match self {
            Self::EqualTo(x) => approx_eq!(f64, x, value),
            Self::NotEqualTo(x) => !approx_eq!(f64, x, value),
            Self::LessThan(upper) => match upper {
                FloatRangeBound::Exclusive(x) => value < x,
                FloatRangeBound::Inclusive(x) => value <= x,
            },
            Self::GreaterThan(lower) => match lower {
                FloatRangeBound::Exclusive(x) => value > x,
                FloatRangeBound::Inclusive(x) => value >= x,
            },
            Self::InRange(lower, upper) => match (lower, upper) {
                (FloatRangeBound::Exclusive(x), FloatRangeBound::Exclusive(y)) => value > x && value < y,
                (FloatRangeBound::Exclusive(x), FloatRangeBound::Inclusive(y)) => value > x && value <= y,
                (FloatRangeBound::Inclusive(x), FloatRangeBound::Exclusive(y)) => value >= x && value < y,
                (FloatRangeBound::Inclusive(x), FloatRangeBound::Inclusive(y)) => value >= x && value <= y,
            },
        }
    }
}

#[cfg(feature = "float")]
impl FloatEvaluator {
    pub fn lt(value: f64) -> FloatEvaluator {
        Self::LessThan(FloatRangeBound::Exclusive(value))
    }

    pub fn lte(value: f64) -> FloatEvaluator {
        Self::LessThan(FloatRangeBound::Inclusive(value))
    }

    pub fn gt(value: f64) -> FloatEvaluator {
        Self::GreaterThan(FloatRangeBound::Exclusive(value))
    }

    pub fn gte(value: f64) -> FloatEvaluator {
        Self::GreaterThan(FloatRangeBound::Inclusive(value))
    }

    pub fn range(lower: f64, upper: f64) -> FloatEvaluator {
        Self::InRange(FloatRangeBound::Inclusive(lower), FloatRangeBound::Exclusive(upper))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "float")]
    use super::{Evaluator, FloatEvaluator, FloatRangeBound};

    #[cfg(feature = "float")]
    #[test]
    fn in_range() {
        let evaluator =
            FloatEvaluator::InRange(FloatRangeBound::Exclusive(5.), FloatRangeBound::Inclusive(25.));
        assert!(evaluator.evaluate(6.));
        assert!(evaluator.evaluate(10.));
        assert!(!evaluator.evaluate(5.));
    }

    #[cfg(feature = "float")]
    #[test]
    fn equal_to() {
        let evaluator = FloatEvaluator::EqualTo(5.);
        assert!(evaluator.evaluate(5.));
        assert!(evaluator.evaluate(1. + 1.5 + 2.5));
        assert!(!evaluator.evaluate(1.005 + 1.5 + 2.5));
    }

    #[cfg(feature = "float")]
    #[test]
    fn not_equal_to() {
        let evaluator = FloatEvaluator::NotEqualTo(5.);
        assert!(!evaluator.evaluate(5.));
        assert!(!evaluator.evaluate(1. + 1.5 + 2.5));
        assert!(evaluator.evaluate(1.005 + 1.5 + 2.5));
    }

    #[cfg(feature = "float")]
    #[test]
    fn less_than_exclusive() {
        let evaluator = FloatEvaluator::LessThan(FloatRangeBound::Exclusive(5.));
        assert!(!evaluator.evaluate(5.));
        assert!(evaluator.evaluate(1. + 1. + 2.5));
        assert!(!evaluator.evaluate(6.));
        assert!(evaluator.evaluate(-1.));
    }

    #[cfg(feature = "float")]
    #[test]
    fn less_than_inclusive() {
        let evaluator = FloatEvaluator::LessThan(FloatRangeBound::Inclusive(5.));
        assert!(evaluator.evaluate(5.));
        assert!(evaluator.evaluate(1. + 1. + 2.5));
        assert!(!evaluator.evaluate(6.));
        assert!(evaluator.evaluate(-1.));
    }

    #[cfg(feature = "float")]
    #[test]
    fn greater_than_exclusive() {
        let evaluator = FloatEvaluator::GreaterThan(FloatRangeBound::Exclusive(5.));
        assert!(!evaluator.evaluate(5.));
        assert!(!evaluator.evaluate(1. + 1. + 2.5));
        assert!(evaluator.evaluate(6.));
        assert!(!evaluator.evaluate(-1.));
    }

    #[cfg(feature = "float")]
    #[test]
    fn greater_than_inclusive() {
        let evaluator = FloatEvaluator::GreaterThan(FloatRangeBound::Inclusive(5.));
        assert!(evaluator.evaluate(5.));
        assert!(!evaluator.evaluate(1. + 1. + 2.5));
        assert!(evaluator.evaluate(6.));
        assert!(!evaluator.evaluate(-1.));
    }

    #[cfg(feature = "float")]
    #[test]
    fn lt_helper() {
        let evaluator = FloatEvaluator::lt(5.);
        assert_eq!(
                evaluator,
            FloatEvaluator::LessThan(FloatRangeBound::Exclusive(5.))
        );
    }

    #[cfg(feature = "float")]
    #[test]
    fn lte_helper() {
        let evaluator = FloatEvaluator::lte(5.);
        assert_eq!(
                evaluator,
            FloatEvaluator::LessThan(FloatRangeBound::Inclusive(5.))
        );
    }

    #[cfg(feature = "float")]
    #[test]
    fn gt_helper() {
        let evaluator = FloatEvaluator::gt(5.);
        assert_eq!(
                evaluator,
            FloatEvaluator::GreaterThan(FloatRangeBound::Exclusive(5.))
        );
    }

    #[cfg(feature = "float")]
    #[test]
    fn gte_helper() {
        let evaluator = FloatEvaluator::gte(5.);
        assert_eq!(
                evaluator,
            FloatEvaluator::GreaterThan(FloatRangeBound::Inclusive(5.))
        );
    }

    #[cfg(feature = "float")]
    #[test]
    fn range_helper() {
        let evaluator = FloatEvaluator::range(5., 25.);
        assert_eq!(
                evaluator,
            FloatEvaluator::InRange(FloatRangeBound::Inclusive(5.), FloatRangeBound::Exclusive(25.))
        );
    }
}
