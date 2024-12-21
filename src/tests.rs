use crate as serde_nested_json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    foo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Unnested {
    full: Item,
    empty: Item,
    null: Option<Item>,
    array: Vec<Option<Item>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Nested {
    #[serde(with = "serde_nested_json")]
    full: Item,
    #[serde(with = "serde_nested_json")]
    empty: Item,
    #[serde(with = "serde_nested_json")]
    null: Option<Item>,
    #[serde(with = "serde_nested_json::vec")]
    array: Vec<Option<Item>>,
}

#[test]
fn it_should_handle_options_without_values_as_null() {
    let nested = r#"
        {
            "some": "\"here\"",
            "null": "null"
        }
    "#;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct NullTest {
        #[serde(with = "serde_nested_json")]
        some: Option<String>,
        #[serde(with = "serde_nested_json")]
        null: Option<String>,
        #[serde(with = "serde_nested_json", default)]
        undefined: Option<String>,
    }

    assert_eq!(
        NullTest {
            some: Some("here".into()),
            null: None,
            undefined: None
        },
        serde_json::from_str(nested).unwrap(),
    )
}

#[test]
fn it_should_work() {
    let unnested_json = r#"
            {
              "full": { "foo": "bar" },
              "empty": {},
              "null": null,
              "array": [
                { "foo": "bar"},
                {},
                null
              ]
            }
        "#;

    let nested_json = r#"
            {
              "full": "{\"foo\":\"bar\"}",
              "empty": "{}",
              "null": "null",
              "array": [
                "{\"foo\":\"bar\"}",
                "{}",
                "null"
              ]
            }

        "#;

    let de_unnested: Unnested = serde_json::from_str(unnested_json).unwrap();
    let de_nested: Nested = serde_json::from_str(nested_json).unwrap();

    assert_eq!(de_unnested.full, de_nested.full);
    assert_eq!(de_unnested.empty, de_nested.empty);
    assert_eq!(de_unnested.null, de_nested.null);
    assert_eq!(de_unnested.array, de_nested.array);

    let ser_unnested = serde_json::to_string(&de_unnested).unwrap();
    let ser_nested = serde_json::to_string(&de_nested).unwrap();

    assert_eq!(
        serde_json::from_str::<Value>(&ser_unnested).unwrap(),
        serde_json::from_str::<Value>(unnested_json).unwrap(),
    );

    assert_eq!(
        serde_json::from_str::<Value>(&ser_nested).unwrap(),
        serde_json::from_str::<Value>(nested_json).unwrap(),
    );
}
