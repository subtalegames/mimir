use float_cmp::approx_eq;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Criterion {
    pub gt: f64,
    pub gt_inclusive: bool,
    pub lt: f64,
    pub lt_inclusive: bool,
}

impl Criterion {
    pub fn evaluate(&self, value: f64) -> bool {
        if self.gt_inclusive && self.lt_inclusive && self.gt == self.lt {
            return approx_eq!(f64, self.gt, value);
        }

        (if self.gt_inclusive {
            value >= self.gt
        } else {
            value > self.gt
        } && if self.lt_inclusive {
            value <= self.lt
        } else {
            value < self.lt
        })
    }

    pub fn lt(value: f64) -> Criterion {
        Self {
            gt: f64::NEG_INFINITY,
            gt_inclusive: true,
            lt: value,
            ..Default::default()
        }
    }

    pub fn lte(value: f64) -> Criterion {
        Self {
            gt: f64::NEG_INFINITY,
            gt_inclusive: true,
            lt: value,
            lt_inclusive: true,
        }
    }

    pub fn gt(value: f64) -> Criterion {
        Self {
            gt: value,
            lt: f64::INFINITY,
            lt_inclusive: true,
            ..Default::default()
        }
    }

    pub fn gte(value: f64) -> Criterion {
        Self {
            gt: value,
            gt_inclusive: true,
            lt: f64::INFINITY,
            lt_inclusive: true,
        }
    }

    pub fn eq(value: f64) -> Criterion {
        Self {
            gt: value,
            gt_inclusive: true,
            lt: value,
            lt_inclusive: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison() {
        let criterion = Criterion {
            gt: 5.,
            gt_inclusive: false,
            lt: 10.,
            lt_inclusive: true,
        };
        assert!(criterion.evaluate(6.));
        assert!(criterion.evaluate(10.));
        assert!(!criterion.evaluate(5.));
    }

    #[test]
    fn eq() {
        let criterion = Criterion::eq(5.);
        assert!(criterion.evaluate(5.));
        assert!(criterion.evaluate(1. + 1.5 + 2.5));
        assert!(!criterion.evaluate(1.005 + 1.5 + 2.5));
    }
}
