#[cfg(feature = "float")]
use float_cmp::approx_eq;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::evaluator::Evaluator;

/// Represents a bound of a range used during float comparisons made by `FloatEvaluator`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(feature = "float")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FloatRangeBound {
    /// A bound that's exclusive of the contained value.
    Exclusive(f64),
    /// A bound that's exclusive of the contained value.
    Inclusive(f64),
}

/// A reference implementation of the `Evaluator` trait that allows for comparisons against
/// facts with a value type of `f64`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(feature = "float")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FloatEvaluator {
    /// Checks if a fact has a specific `f64` value (using the `float_cmp` crate for approximate
    /// equality).
    EqualTo(f64),
    /// Checks if a fact does not have a specific `f64` value (using the `float_cmp` crate for
    /// approximate equality).
    NotEqualTo(f64),
    /// Checks if a fact's value is less than a given `f64` (see `FloatRangeBound` for guidance
    /// on inclusive/exclusive bounds).
    LessThan(FloatRangeBound),
    /// Checks if a fact's value is greater than a given `f64` (see `FloatRangeBound` for
    /// guidance on inclusive/exclusive bounds).
    GreaterThan(FloatRangeBound),
    /// Checks if a fact's value is within a given range of `f64` values (see `FloatRangeBound`
    /// for guidance on inclusive/exclusive bounds).
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
                (FloatRangeBound::Exclusive(x), FloatRangeBound::Exclusive(y)) => {
                    value > x && value < y
                }
                (FloatRangeBound::Exclusive(x), FloatRangeBound::Inclusive(y)) => {
                    value > x && value <= y
                }
                (FloatRangeBound::Inclusive(x), FloatRangeBound::Exclusive(y)) => {
                    value >= x && value < y
                }
                (FloatRangeBound::Inclusive(x), FloatRangeBound::Inclusive(y)) => {
                    value >= x && value <= y
                }
            },
        }
    }
}

#[cfg(feature = "float")]
impl FloatEvaluator {
    /// Utility function for composing an instance of `FloatEvaluator` that checks
    /// for values less than `value`.
    pub fn lt(value: f64) -> FloatEvaluator {
        Self::LessThan(FloatRangeBound::Exclusive(value))
    }

    /// Utility function for composing an instance of `FloatEvaluator` that checks
    /// for values less than or equal to `value`.
    pub fn lte(value: f64) -> FloatEvaluator {
        Self::LessThan(FloatRangeBound::Inclusive(value))
    }

    /// Utility function for composing an instance of `FloatEvaluator` that checks
    /// for values greater than `value`.
    pub fn gt(value: f64) -> FloatEvaluator {
        Self::GreaterThan(FloatRangeBound::Exclusive(value))
    }

    /// Utility function for composing an instance of `FloatEvaluator` that checks
    /// for values greater than or equal to `value`.
    pub fn gte(value: f64) -> FloatEvaluator {
        Self::GreaterThan(FloatRangeBound::Inclusive(value))
    }

    /// Utility function for composing an instance of `FloatEvaluator` that checks
    /// for values such that `lower` <= `value` < `upper`.
    pub fn range(lower: f64, upper: f64) -> FloatEvaluator {
        Self::InRange(
            FloatRangeBound::Inclusive(lower),
            FloatRangeBound::Exclusive(upper),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Evaluator, FloatEvaluator, FloatRangeBound};

    #[test]

    fn in_range() {
        let evaluator = FloatEvaluator::InRange(
            FloatRangeBound::Exclusive(5.),
            FloatRangeBound::Inclusive(25.),
        );
        assert!(evaluator.evaluate(6.));
        assert!(evaluator.evaluate(10.));
        assert!(!evaluator.evaluate(5.));
    }

    #[test]
    fn equal_to() {
        let evaluator = FloatEvaluator::EqualTo(5.);
        assert!(evaluator.evaluate(5.));
        assert!(evaluator.evaluate(1. + 1.5 + 2.5));
        assert!(!evaluator.evaluate(1.005 + 1.5 + 2.5));
    }

    #[test]
    fn not_equal_to() {
        let evaluator = FloatEvaluator::NotEqualTo(5.);
        assert!(!evaluator.evaluate(5.));
        assert!(!evaluator.evaluate(1. + 1.5 + 2.5));
        assert!(evaluator.evaluate(1.005 + 1.5 + 2.5));
    }

    #[test]
    fn less_than_exclusive() {
        let evaluator = FloatEvaluator::LessThan(FloatRangeBound::Exclusive(5.));
        assert!(!evaluator.evaluate(5.));
        assert!(evaluator.evaluate(1. + 1. + 2.5));
        assert!(!evaluator.evaluate(6.));
        assert!(evaluator.evaluate(-1.));
    }

    #[test]
    fn less_than_inclusive() {
        let evaluator = FloatEvaluator::LessThan(FloatRangeBound::Inclusive(5.));
        assert!(evaluator.evaluate(5.));
        assert!(evaluator.evaluate(1. + 1. + 2.5));
        assert!(!evaluator.evaluate(6.));
        assert!(evaluator.evaluate(-1.));
    }

    #[test]
    fn greater_than_exclusive() {
        let evaluator = FloatEvaluator::GreaterThan(FloatRangeBound::Exclusive(5.));
        assert!(!evaluator.evaluate(5.));
        assert!(!evaluator.evaluate(1. + 1. + 2.5));
        assert!(evaluator.evaluate(6.));
        assert!(!evaluator.evaluate(-1.));
    }

    #[test]
    fn greater_than_inclusive() {
        let evaluator = FloatEvaluator::GreaterThan(FloatRangeBound::Inclusive(5.));
        assert!(evaluator.evaluate(5.));
        assert!(!evaluator.evaluate(1. + 1. + 2.5));
        assert!(evaluator.evaluate(6.));
        assert!(!evaluator.evaluate(-1.));
    }

    #[test]
    fn lt_helper() {
        let evaluator = FloatEvaluator::lt(5.);
        assert_eq!(
            evaluator,
            FloatEvaluator::LessThan(FloatRangeBound::Exclusive(5.))
        );
    }

    #[test]
    fn lte_helper() {
        let evaluator = FloatEvaluator::lte(5.);
        assert_eq!(
            evaluator,
            FloatEvaluator::LessThan(FloatRangeBound::Inclusive(5.))
        );
    }

    #[test]
    fn gt_helper() {
        let evaluator = FloatEvaluator::gt(5.);
        assert_eq!(
            evaluator,
            FloatEvaluator::GreaterThan(FloatRangeBound::Exclusive(5.))
        );
    }

    #[test]
    fn gte_helper() {
        let evaluator = FloatEvaluator::gte(5.);
        assert_eq!(
            evaluator,
            FloatEvaluator::GreaterThan(FloatRangeBound::Inclusive(5.))
        );
    }

    #[test]
    fn range_helper() {
        let evaluator = FloatEvaluator::range(5., 25.);
        assert_eq!(
            evaluator,
            FloatEvaluator::InRange(
                FloatRangeBound::Inclusive(5.),
                FloatRangeBound::Exclusive(25.)
            )
        );
    }
}
