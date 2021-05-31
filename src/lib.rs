#[derive(Debug)]
pub enum Async<T, Err> {
    Uninitialized,
    Loading,
    Error(Err),
    Success(T),
}
