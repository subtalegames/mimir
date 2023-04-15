/// An `Evaluator<T` is a trait that represents a predicate function evaluating against a value (`T`).
///
/// Specifically, in the context of Mímir, an evaluator checks if the value of a fact about the game's
/// current state matches a certain condition.
///
/// You can choose to create your own implementation of the trait, or use the `FloatEvaluator`
/// implementation (provided by the crate's `float` feature) that allows you to evaluate floating-point
/// numbers (Rust's `f64` type).
pub trait Evaluator<T> {
    /// Evaluates against a value of type `T` and returns true or false based on the underlying logic.
    fn evaluate(self, value: T) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Evaluator;

    /// Dummy implementation of the `Evaluator` trait, used for testing purposes.
    pub struct DummyEvaluator {
        threshold: u32,
    }

    impl Evaluator<u32> for DummyEvaluator {
        /// Checks if the provided `value` is greater than or equal to the evaluator's
        /// defined `threshold.
        fn evaluate(self, value: u32) -> bool {
            value >= self.threshold
        }
    }

    #[test]
    fn test_dummy_evaluator_evaluate() {
        let evaluator = DummyEvaluator { threshold: 10 };

        // Test with a value equal to the threshold
        assert_eq!(evaluator.evaluate(10), true);
    }
}
