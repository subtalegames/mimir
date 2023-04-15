pub trait Evaluator<T> {
    fn evaluate(self, value: T) -> bool;
}
