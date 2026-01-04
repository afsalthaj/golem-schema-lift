#[cfg(test)]
mod tests {
    use golem_rust::agentic::{Schema, StructuredSchema, StructuredValue};
    use golem_rust::golem_wasm::WitValue;
    use golem_schema_lift::RecursiveSchema;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    #[derive(RecursiveSchema, Debug)]
    pub struct Tree {
        pub value: String,
        pub left: Option<Box<Tree>>,
        pub right: Vec<Tree>,
    }

    impl Schema for Tree {
        fn get_type() -> StructuredSchema {
            TreeNonRecursive::get_type()
        }

        fn to_structured_value(self) -> Result<StructuredValue, String> {

            let mut tree_arena = TreeArena::new();

            let _ = self.to_arena(&mut tree_arena);

            tree_arena.to_structured_value()

        }

        fn from_structured_value(value: StructuredValue, schema: StructuredSchema) -> Result<Self, String>
        where
            Self: Sized
        {
            let tree_arena = TreeArena::from_structured_value(value, schema)?;

            Ok(Tree::from_arena(0, &tree_arena))
        }

        fn from_wit_value(wit_value: WitValue, schema: StructuredSchema) -> Result<Self, String>
        where
            Self: Sized
        {
            let structured_value = StructuredValue::from_wit_value(wit_value, schema)?;
            Self::from_structured_value(structured_value, schema)
        }

        fn to_wit_value(self) -> Result<WitValue, String>
        where
            Self: Sized
        {
            todo!()
        }
    }

    #[test]
    fn test_recursion() {
        let tree = Tree {
            value: "root".to_string(),
            left: Some(Box::new(Tree {
                value: "left".to_string(),
                left: Some(Box::new(Tree {
                    value: "left.left".to_string(),
                    left: None,
                    right: vec![],
                })),
                right: vec![],
            })),
            right: vec![Tree {
                value: "right".to_string(),
                left: None,
                right: vec![],
            }],
        };

        let schema = TreeNonRecursive::get_type();
    }
}