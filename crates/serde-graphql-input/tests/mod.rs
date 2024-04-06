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
}
