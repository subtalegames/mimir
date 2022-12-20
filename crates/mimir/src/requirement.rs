use float_cmp::approx_eq;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum RangeBound {
    Exclusive(f64),
    Inclusive(f64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum Requirement {
    EqualTo(f64),
    NotEqualTo(f64),
    LessThan(RangeBound),
    GreaterThan(RangeBound),
    InRange(RangeBound, RangeBound),
}

impl Requirement {
    pub fn evaluate(self, value: f64) -> bool {
        match self {
            Self::EqualTo(x) => approx_eq!(f64, x, value),
            Self::NotEqualTo(x) => !approx_eq!(f64, x, value),
            Self::LessThan(upper) => match upper {
                RangeBound::Exclusive(x) => value < x,
                RangeBound::Inclusive(x) => value <= x,
            },
            Self::GreaterThan(lower) => match lower {
                RangeBound::Exclusive(x) => value > x,
                RangeBound::Inclusive(x) => value >= x,
            },
            Self::InRange(lower, upper) => match (lower, upper) {
                (RangeBound::Exclusive(x), RangeBound::Exclusive(y)) => {
                    value > x && value < y
                }
                (RangeBound::Exclusive(x), RangeBound::Inclusive(y)) => {
                    value > x && value <= y
                }
                (RangeBound::Inclusive(x), RangeBound::Exclusive(y)) => {
                    value >= x && value < y
                }
                (RangeBound::Inclusive(x), RangeBound::Inclusive(y)) => {
                    value >= x && value <= y
                }
            },
        }
    }

    pub fn lt(value: f64) -> Requirement {
        Self::LessThan(RangeBound::Exclusive(value))
    }

    pub fn lte(value: f64) -> Requirement {
        Self::LessThan(RangeBound::Inclusive(value))
    }

    pub fn gt(value: f64) -> Requirement {
        Self::GreaterThan(RangeBound::Exclusive(value))
    }

    pub fn gte(value: f64) -> Requirement {
        Self::GreaterThan(RangeBound::Inclusive(value))
    }

    pub fn range(lower: f64, upper: f64) -> Requirement {
        Self::InRange(
            RangeBound::Inclusive(lower),
            RangeBound::Exclusive(upper),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_range() {
        let requirement = Requirement::InRange(
            RangeBound::Exclusive(5.),
            RangeBound::Inclusive(25.),
        );
        assert!(requirement.evaluate(6.));
        assert!(requirement.evaluate(10.));
        assert!(!requirement.evaluate(5.));
    }

    #[test]
    fn equal_to() {
        let requirement = Requirement::EqualTo(5.);
        assert!(requirement.evaluate(5.));
        assert!(requirement.evaluate(1. + 1.5 + 2.5));
        assert!(!requirement.evaluate(1.005 + 1.5 + 2.5));
    }

    #[test]
    fn not_equal_to() {
        let requirement = Requirement::NotEqualTo(5.);
        assert!(!requirement.evaluate(5.));
        assert!(!requirement.evaluate(1. + 1.5 + 2.5));
        assert!(requirement.evaluate(1.005 + 1.5 + 2.5));
    }

    #[test]
    fn less_than_exclusive() {
        let requirement = Requirement::LessThan(RangeBound::Exclusive(5.));
        assert!(!requirement.evaluate(5.));
        assert!(requirement.evaluate(1. + 1. + 2.5));
        assert!(!requirement.evaluate(6.));
        assert!(requirement.evaluate(-1.));
    }

    #[test]
    fn less_than_inclusive() {
        let requirement = Requirement::LessThan(RangeBound::Inclusive(5.));
        assert!(requirement.evaluate(5.));
        assert!(requirement.evaluate(1. + 1. + 2.5));
        assert!(!requirement.evaluate(6.));
        assert!(requirement.evaluate(-1.));
    }

    #[test]
    fn greater_than_exclusive() {
        let requirement = Requirement::GreaterThan(RangeBound::Exclusive(5.));
        assert!(!requirement.evaluate(5.));
        assert!(!requirement.evaluate(1. + 1. + 2.5));
        assert!(requirement.evaluate(6.));
        assert!(!requirement.evaluate(-1.));
    }

    #[test]
    fn greater_than_inclusive() {
        let requirement = Requirement::GreaterThan(RangeBound::Inclusive(5.));
        assert!(requirement.evaluate(5.));
        assert!(!requirement.evaluate(1. + 1. + 2.5));
        assert!(requirement.evaluate(6.));
        assert!(!requirement.evaluate(-1.));
    }

    #[test]
    fn lt_helper() {
        let requirement = Requirement::lt(5.);
        assert_eq!(
            requirement,
            Requirement::LessThan(RangeBound::Exclusive(5.))
        );
    }

    #[test]
    fn lte_helper() {
        let requirement = Requirement::lte(5.);
        assert_eq!(
            requirement,
            Requirement::LessThan(RangeBound::Inclusive(5.))
        );
    }

    #[test]
    fn gt_helper() {
        let requirement = Requirement::gt(5.);
        assert_eq!(
            requirement,
            Requirement::GreaterThan(RangeBound::Exclusive(5.))
        );
    }

    #[test]
    fn gte_helper() {
        let requirement = Requirement::gte(5.);
        assert_eq!(
            requirement,
            Requirement::GreaterThan(RangeBound::Inclusive(5.))
        );
    }

    #[test]
    fn range_helper() {
        let requirement = Requirement::range(5., 25.);
        assert_eq!(
            requirement,
            Requirement::InRange(
                RangeBound::Inclusive(5.),
                RangeBound::Exclusive(25.)
            )
        );
    }
}
