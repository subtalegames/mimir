use float_cmp::approx_eq;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum CriterionBound {
    Exclusive(f64),
    Inclusive(f64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum Criterion {
    EqualTo(f64),
    NotEqualTo(f64),
    LessThan(CriterionBound),
    GreaterThan(CriterionBound),
    InRange(CriterionBound, CriterionBound),
}

impl Criterion {
    pub fn evaluate(self, value: f64) -> bool {
        match self {
            Self::EqualTo(x) => approx_eq!(f64, x, value),
            Self::NotEqualTo(x) => !approx_eq!(f64, x, value),
            Self::LessThan(upper) => match upper {
                CriterionBound::Exclusive(x) => value < x,
                CriterionBound::Inclusive(x) => value <= x,
            },
            Self::GreaterThan(lower) => match lower {
                CriterionBound::Exclusive(x) => value > x,
                CriterionBound::Inclusive(x) => value >= x,
            },
            Self::InRange(lower, upper) => match (lower, upper) {
                (CriterionBound::Exclusive(x), CriterionBound::Exclusive(y)) => {
                    value > x && value < y
                }
                (CriterionBound::Exclusive(x), CriterionBound::Inclusive(y)) => {
                    value > x && value <= y
                }
                (CriterionBound::Inclusive(x), CriterionBound::Exclusive(y)) => {
                    value >= x && value < y
                }
                (CriterionBound::Inclusive(x), CriterionBound::Inclusive(y)) => {
                    value >= x && value <= y
                }
            },
        }
    }

    pub fn lt(value: f64) -> Criterion {
        Self::LessThan(CriterionBound::Exclusive(value))
    }

    pub fn lte(value: f64) -> Criterion {
        Self::LessThan(CriterionBound::Inclusive(value))
    }

    pub fn gt(value: f64) -> Criterion {
        Self::GreaterThan(CriterionBound::Exclusive(value))
    }

    pub fn gte(value: f64) -> Criterion {
        Self::GreaterThan(CriterionBound::Inclusive(value))
    }

    pub fn range(lower: f64, upper: f64) -> Criterion {
        Self::InRange(
            CriterionBound::Inclusive(lower),
            CriterionBound::Exclusive(upper),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_range() {
        let criterion = Criterion::InRange(
            CriterionBound::Exclusive(5.),
            CriterionBound::Inclusive(25.),
        );
        assert!(criterion.evaluate(6.));
        assert!(criterion.evaluate(10.));
        assert!(!criterion.evaluate(5.));
    }

    #[test]
    fn equal_to() {
        let criterion = Criterion::EqualTo(5.);
        assert!(criterion.evaluate(5.));
        assert!(criterion.evaluate(1. + 1.5 + 2.5));
        assert!(!criterion.evaluate(1.005 + 1.5 + 2.5));
    }

    #[test]
    fn not_equal_to() {
        let criterion = Criterion::NotEqualTo(5.);
        assert!(!criterion.evaluate(5.));
        assert!(!criterion.evaluate(1. + 1.5 + 2.5));
        assert!(criterion.evaluate(1.005 + 1.5 + 2.5));
    }

    #[test]
    fn less_than_exclusive() {
        let criterion = Criterion::LessThan(CriterionBound::Exclusive(5.));
        assert!(!criterion.evaluate(5.));
        assert!(criterion.evaluate(1. + 1. + 2.5));
        assert!(!criterion.evaluate(6.));
        assert!(criterion.evaluate(-1.));
    }

    #[test]
    fn less_than_inclusive() {
        let criterion = Criterion::LessThan(CriterionBound::Inclusive(5.));
        assert!(criterion.evaluate(5.));
        assert!(criterion.evaluate(1. + 1. + 2.5));
        assert!(!criterion.evaluate(6.));
        assert!(criterion.evaluate(-1.));
    }

    #[test]
    fn greater_than_exclusive() {
        let criterion = Criterion::GreaterThan(CriterionBound::Exclusive(5.));
        assert!(!criterion.evaluate(5.));
        assert!(!criterion.evaluate(1. + 1. + 2.5));
        assert!(criterion.evaluate(6.));
        assert!(!criterion.evaluate(-1.));
    }

    #[test]
    fn greater_than_inclusive() {
        let criterion = Criterion::GreaterThan(CriterionBound::Inclusive(5.));
        assert!(criterion.evaluate(5.));
        assert!(!criterion.evaluate(1. + 1. + 2.5));
        assert!(criterion.evaluate(6.));
        assert!(!criterion.evaluate(-1.));
    }

    #[test]
    fn lt_helper() {
        let criterion = Criterion::lt(5.);
        assert_eq!(
            criterion,
            Criterion::LessThan(CriterionBound::Exclusive(5.))
        );
    }

    #[test]
    fn lte_helper() {
        let criterion = Criterion::lte(5.);
        assert_eq!(
            criterion,
            Criterion::LessThan(CriterionBound::Inclusive(5.))
        );
    }

    #[test]
    fn gt_helper() {
        let criterion = Criterion::gt(5.);
        assert_eq!(
            criterion,
            Criterion::GreaterThan(CriterionBound::Exclusive(5.))
        );
    }

    #[test]
    fn gte_helper() {
        let criterion = Criterion::gte(5.);
        assert_eq!(
            criterion,
            Criterion::GreaterThan(CriterionBound::Inclusive(5.))
        );
    }

    #[test]
    fn range_helper() {
        let criterion = Criterion::range(5., 25.);
        assert_eq!(
            criterion,
            Criterion::InRange(
                CriterionBound::Inclusive(5.),
                CriterionBound::Exclusive(25.)
            )
        );
    }
}
