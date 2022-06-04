pub trait StorableData where Self: Sized {
    fn load() -> Self;
    fn save(self) -> Self { self }
}