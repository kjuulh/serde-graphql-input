mod integer {
    use serde::Serialize;

    #[test]
    fn can_serialize_i8() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: i8,
        }

        let input = Input { something: 123 };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:123}"#, output.as_str())
    }

    #[test]
    fn can_serialize_i16() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: i16,
        }

        let input = Input { something: 123 };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:123}"#, output.as_str())
    }

    #[test]
    fn can_serialize_i32() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: i32,
        }

        let input = Input { something: 123 };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:123}"#, output.as_str())
    }

    #[test]
    fn can_serialize_i64() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: i64,
        }

        let input = Input { something: 123 };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:123}"#, output.as_str())
    }
}

mod unsigned_integer {
    use serde::Serialize;

    #[test]
    fn can_serialize_u8() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: u8,
        }

        let input = Input { something: 123 };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:123}"#, output.as_str())
    }

    #[test]
    fn can_serialize_u16() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: u16,
        }

        let input = Input { something: 123 };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:123}"#, output.as_str())
    }

    #[test]
    fn can_serialize_u32() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: u32,
        }

        let input = Input { something: 123 };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:123}"#, output.as_str())
    }

    #[test]
    fn can_serialize_u64() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: u64,
        }

        let input = Input { something: 123 };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:123}"#, output.as_str())
    }
}

mod float {
    use serde::Serialize;

    #[test]
    fn can_serialize_f32() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: f32,
        }

        let input = Input { something: 123.5 };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:123.5}"#, output.as_str())
    }

    #[test]
    fn can_serialize_f64() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: f64,
        }

        let input = Input { something: 123.5 };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:123.5}"#, output.as_str())
    }
}

mod char {
    use serde::Serialize;

    #[test]
    fn can_serialize_char() {
        #[derive(Serialize, Clone, Debug)]
        struct Input {
            something: char,
        }

        let input = Input { something: 'a' };

        let output = serde_graphql_input::to_string_pretty(&input).unwrap();

        assert_eq!(r#"{something:'a'}"#, output.as_str())
    }
}
