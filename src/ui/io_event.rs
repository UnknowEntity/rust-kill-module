pub enum IoEventType {
    Initialize,
    Deleted(usize),
    DeleteError(usize),
    Loaded(usize, u128)
}