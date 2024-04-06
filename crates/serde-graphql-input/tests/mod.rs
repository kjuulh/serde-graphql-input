mod structs {
    use serde::Serialize;

    #[test]
    fn can_serialize_unit_struct() {
        #[derive(Serialize, Clone, Debug)]
        struct Input;

        let input = Input;

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"null"#, output.as_str())
    }

    #[test]
    fn can_serialize_newtype_struct() {
        #[derive(Serialize, Clone, Debug)]
        struct Input(String);

        let input = Input("something".into());

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#""something""#, output.as_str())
    }

    #[test]
    fn can_serialize_newtype_variant() {
        #[derive(Serialize, Clone, Debug)]
        enum Input {
            Something,
        }

        let input = Input::Something;

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"Something"#, output.as_str())
    }

    #[test]
    fn can_serialize_tuplestruct() {
        #[derive(Serialize, Clone, Debug)]
        struct Input(u64, String);

        let input = Input(123, "something".into());

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"[123,"something"]"#, output.as_str())
    }
}
