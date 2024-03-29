use serde::Serialize;

#[test]
fn can_serialize_simple_struct() {
    #[derive(Serialize, Clone, Debug)]
    struct Input {
        something: String,
    }

    let input = Input {
        something: "Something".into(),
    };

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(r#"{something:"Something"}"#, output.as_str())
}

#[test]
fn can_handle_multiple_values() {
    #[derive(Serialize, Clone, Debug)]
    struct Input {
        something: String,
        #[serde(rename = "somethingElse")]
        something_else: String,
    }

    let input = Input {
        something: "Something".into(),
        something_else: "else".into(),
    };

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(
        r#"{something:"Something",somethingElse:"else"}"#,
        output.as_str()
    )
}

#[test]
fn can_handle_embedded_structs() {
    #[derive(Serialize, Clone, Debug)]
    struct Input {
        nested: Option<Box<Input>>,
        item: String,
    }

    let input = Input {
        nested: Some(Box::new(Input {
            nested: None,
            item: "some nested item".into(),
        })),
        item: "some item".into(),
    };

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(
        r#"{nested:{nested:null,item:"some nested item"},item:"some item"}"#,
        output.as_str()
    )
}

#[test]
fn can_handle_embedded_structs_omit() {
    #[derive(Serialize, Clone, Debug)]
    struct Input {
        #[serde(skip_serializing_if = "Option::is_none")]
        nested: Option<Box<Input>>,
        item: String,
    }

    let input = Input {
        nested: Some(Box::new(Input {
            nested: None,
            item: "some nested item".into(),
        })),
        item: "some item".into(),
    };

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(
        r#"{nested:{item:"some nested item"},item:"some item"}"#,
        output.as_str()
    )
}

#[test]
fn can_handle_array() {
    #[derive(Serialize, Clone, Debug)]
    struct Input {
        items: Vec<String>,
    }

    let input = Input {
        items: vec!["one".into(), "two".into(), "three".into(), "four".into()],
    };

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(r#"{items:["one","two","three","four"]}"#, output.as_str())
}

#[test]
fn can_handle_mixed_items() {
    #[derive(Serialize, Clone, Debug)]
    #[serde(untagged)]
    enum VariantItem {
        String(String),
        Item { item: String },
    }

    #[derive(Serialize, Clone, Debug)]
    struct Input {
        items: Vec<VariantItem>,
    }

    let input = Input {
        items: vec![
            VariantItem::String("something".into()),
            VariantItem::Item {
                item: "something".into(),
            },
        ],
    };

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(
        r#"{items:["something",{item:"something"}]}"#,
        output.as_str()
    )
}

#[test]
fn can_handle_enums() {
    #[derive(Serialize, Clone, Debug)]
    enum VariantEnum {
        ItemA,
        ItemB,
    }

    #[derive(Serialize, Clone, Debug)]
    struct Input {
        items: Vec<VariantEnum>,
    }

    let input = Input {
        items: vec![VariantEnum::ItemA, VariantEnum::ItemB],
    };

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(r#"{items:[ItemA,ItemB]}"#, output.as_str())
}

#[test]
fn can_handle_tuples() {
    #[derive(Serialize, Clone, Debug)]
    struct Input {
        items: (String, String),
    }

    let input = Input {
        items: ("one".into(), "two".into()),
    };

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(r#"{items:["one","two"]}"#, output.as_str())
}

#[test]
fn can_handle_newtype_struct() {
    #[derive(Serialize, Clone, Debug)]
    struct Input(String);

    let input = Input("something".into());

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(r#""something""#, output.as_str())
}

#[test]
fn can_handle_i64() {
    let input = 42_i64;

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(r#"42"#, output.as_str())
}

#[test]
fn can_handle_bool() {
    let input = true;

    let output = serde_graphql_input::to_string_pretty(&input).unwrap();

    assert_eq!(r#"true"#, output.as_str())
}
