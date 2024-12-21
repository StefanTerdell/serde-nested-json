#[cfg(test)]
mod tests;

pub mod de;
pub mod ser;

pub use de::unnest as deserialize;
pub use ser::nest as serialize;

pub struct NestedJson<T>(T);

pub mod vec {
    pub use super::de::unnest_vec as deserialize;
    pub use super::ser::nest_iter as serialize;
}
