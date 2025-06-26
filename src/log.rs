pub trait Log {
    fn lines<T>(&self) -> Vec<T>;
}
