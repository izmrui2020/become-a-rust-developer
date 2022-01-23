
pub trait Reader {
    fn decode(&self);
    fn reset(&self);
}