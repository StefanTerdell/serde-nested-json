#![doc = include_str!("../README.md")]

pub mod de;
pub mod ser;

pub use de::unnest as deserialize;
pub use ser::nest as serialize;

/// Wraps an instance of T for nested serialization and deserializion.
/// You will however need to extract and insert the inner value on, so
/// Its recommended to use the entire module at once instead - see the
/// main crate docs or README.md for that.
///
/// ```rust  
/// use serde::{Serialize, Deserialize};
/// use serde_json::{json, from_value, to_string_pretty};
/// use serde_nested_json::NestedJson;
///
/// let as_json = json!({
///   "nestedJson": "{\"baz\":123}"
/// });
///
/// #[derive(Serialize, Deserialize)]
/// struct SomeData {
///   baz: u32
/// }
///
/// #[derive(Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// struct MyStruct {
///   nested_json: NestedJson<SomeData>
/// }
///
/// // NestedJson<T> implements From<T>, so you can use t.into()
/// // as well as NestedJson::from(t)
/// impl From<SomeData> for MyStruct {
///   fn from(value: SomeData) -> MyStruct {
///     MyStruct {
///       nested_json: value.into()
///     }
///   }
/// }
///
/// // There's also an associated `into` fn which returns T
/// // NestedJson::from is also available, but I havent found
/// // a way to associate it with with the std::conversion
/// // traits using stable features
/// impl From<MyStruct> for SomeData {
///   fn from(value: MyStruct) -> SomeData {
///     value.nested_json.into()
///   }
/// }
///
/// // NestedJson<T> also implements AsRef<T>:
/// impl AsRef<SomeData> for MyStruct {
///     fn as_ref(&self) -> &SomeData {
///         self.nested_json.as_ref()
///     }
/// }
///
/// let a = SomeData { baz: 32 };
/// let b: MyStruct = a.into();
/// let c: SomeData = b.into();
/// let d: NestedJson<SomeData> = c.into();
/// let e: SomeData = d.into();
///
/// let a = SomeData { baz: 32 };
/// let b = MyStruct::from(a);
/// let c = SomeData::from(b);
/// let d = NestedJson::<SomeData>::from(c);
/// // Sadly not possible without manual implementation:
/// // let e = SomeData::from(d);
///
/// println!("{}", to_string_pretty(&from_value::<MyStruct>(as_json).unwrap()).unwrap());
/// ```
pub struct NestedJson<T>(T);

impl<T> NestedJson<T> {
    /// Stand-in for `impl<T> From<NestedJson<T>> for T`
    /// which is not possible to implement without
    /// non-standard language features.
    ///
    /// Makes it possible to use for instance `.map(NestedJson::from)`,
    /// but sadly not as `std::convert::From`
    pub fn from(inner: T) -> Self {
        NestedJson(inner)
    }

    /// Stand-in for `impl<T> Into<T> for NestedJson<T>`
    /// which is not possible to implement without
    /// non-standard language features.
    ///
    /// Makes it possible to use for instance `.map(NestedJson::into)`
    /// but sadly not as `std::convert::Into`
    pub fn into(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for NestedJson<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for NestedJson<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

pub mod vec {
    pub use super::de::unnest_vec as deserialize;
    pub use super::ser::nest_iter as serialize;
}

#[cfg(test)]
mod tests;
