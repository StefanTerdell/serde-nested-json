# Serde nested JSON

## Summary and basic use

This is a small utility crate to help deal with nested JSON in structs.

Here's a basic example:

```rust
use serde::{Serialize, Deserialize};
use serde_json::{json, from_value, to_string_pretty};
use serde_nested_json;

let as_json = json!({
  "someData": {
    "foo": "bar",
    "baz": 123
  },
  "nestedJson": "{\"foo\":\"bar\",\"baz\":123}"
});

#[derive(Serialize, Deserialize)]
struct SomeData {
  foo: String,
  baz: u32
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MyStruct {
  some_data: SomeData,
  #[serde(with = "serde_nested_json")]
  nested_json: SomeData
}
# println!("{}", to_string_pretty(&from_value::<MyStruct>(as_json).unwrap()).unwrap());
```

## Optional values

Optional values deserialize just fine from `"null"`, but if the value may be undefined please add `default` field attribute:

```rust

use serde::{Serialize, Deserialize};
use serde_json::{json, from_value, to_string_pretty};
use serde_nested_json;

let as_json = json!({
  "here": "\"hello!\"",
  "null": "null",
  // "undefined": _
});

#[derive(Serialize, Deserialize)]
struct MyStruct {
  #[serde(with = "serde_nested_json")]
  here: Option<String>,
  #[serde(with = "serde_nested_json")]
  null: Option<String>,
  #[serde(with = "serde_nested_json", default)] // <-- default saves the day
  undefined: Option<String>,
}
# println!("{}", to_string_pretty(&from_value::<MyStruct>(as_json).unwrap()).unwrap());
```

## Vecs

There's also a helper module for vecs containing nested items.
To use it, Just add `::vec` to the `with` field attribute:

```rust
use serde::{Serialize, Deserialize};
use serde_json::{json, from_value, to_string_pretty};
use serde_nested_json;

let as_json = json!({
  "array": [
    "{\"foo\":\"bar\",\"baz\":123}",
    "{\"foo\":\"baz\",\"baz\":54321}"
  ]
});

#[derive(Serialize, Deserialize)]
struct SomeData {
  foo: String,
  baz: u32
}

#[derive(Serialize, Deserialize)]
struct MyStruct {
  #[serde(with = "serde_nested_json::vec")]
  array: Vec<SomeData>,
}
# println!("{}", to_string_pretty(&from_value::<MyStruct>(as_json).unwrap()).unwrap());
```

## `NestedJson<T>`

The main helper type of this crate is `NestedJson<T>` which
can be used without field annotation. You will however need
to extract and insert the inner value on your own:

```rust
use serde::{Serialize, Deserialize};
use serde_json::{json, from_value, to_string_pretty};
use serde_nested_json::NestedJson;

let as_json = json!({
  "nestedJson": "{\"baz\":123}"
});

#[derive(Serialize, Deserialize)]
struct SomeData {
  baz: u32
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MyStruct {
  nested_json: NestedJson<SomeData>
}

// NestedJson<T> implements From<T>, so you can use t.into()
// as well as NestedJson::from(t)
impl From<SomeData> for MyStruct {
  fn from(value: SomeData) -> MyStruct {
    MyStruct {
      nested_json: value.into()
    }
  }
}

// There's also an associated `into` fn which returns T
// NestedJson::from is also available, but I havent found
// a way to associate it with with the std::conversion
// traits using stable features
impl From<MyStruct> for SomeData {
  fn from(value: MyStruct) -> SomeData {
    value.nested_json.into()
  }
}

// NestedJson<T> also implements AsRef<T>:
impl AsRef<SomeData> for MyStruct {
    fn as_ref(&self) -> &SomeData {
        self.nested_json.as_ref()
    }
}

let a = SomeData { baz: 32 };
let b: MyStruct = a.into();
let c: SomeData = b.into();
let d: NestedJson<SomeData> = c.into();
let e: SomeData = d.into();

let a = SomeData { baz: 32 };
let b = MyStruct::from(a);
let c = SomeData::from(b);
let d = NestedJson::<SomeData>::from(c);
// Sadly not possible without manual implementation:
// let e = SomeData::from(d);
# println!("{}", to_string_pretty(&from_value::<MyStruct>(as_json).unwrap()).unwrap());
```

### Forwarded trait implementations

`NestedJson<T>` implements the following traits for any type
`T` that also implements them, but does not require any of them:
1. Debug
1. Clone
1. PartialEq (and Eq)
1. PartialOrd
1. Ord

These should be enough at least for basic tests etc., but
just open an issue if you need anything else.

```rust
# use serde_nested_json::NestedJson;
let stuff = NestedJson::from(vec!["hello"]);
let mut clone = stuff.clone();

assert_eq!(stuff, clone);
println!("{:?}", clone); // 'NestedJson(["hello"])'

clone.into().sort();
```

