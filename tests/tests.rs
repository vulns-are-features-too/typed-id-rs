#[cfg(test)]
mod tests {
    use typed_id::*;

    #[test]
    fn comparisons() {
        let id = Id::<String>::new(5);
        assert!(id == Id::new(5));
        assert!(id != Id::new(0));
        assert!(id < Id::new(6));
        assert!(id <= Id::new(6));
        assert!(id > Id::new(4));
        assert!(id >= Id::new(4));
    }

    #[test]
    fn format_strings() {
        let id = Id::<User>::new(5);
        assert_eq!(id.to_string(), "5");
        assert_eq!(
            format!("{:?}", id),
            "Id<owner: tests::tests::User, backing: u32>(5)"
        );
        assert_eq!(id.owner_type(), "tests::tests::User");
        assert_eq!(id.backing_type(), "u32");

        let id = Id::<String, i64>::new(25);
        assert_eq!(id.to_string(), "25");
        assert_eq!(
            format!("{:?}", id),
            "Id<owner: alloc::string::String, backing: i64>(25)"
        );
        assert_eq!(id.owner_type(), "alloc::string::String");
        assert_eq!(id.backing_type(), "i64");
    }

    #[test]
    fn non_default_int_id_type() {
        let id = Id::<String, i64>::new(-10);
        assert_eq!(id, Id::new(-10));
        assert_ne!(id, Id::new(10));
        assert_eq!(id.to_string(), "-10");
    }

    #[test]
    fn non_default_str_id_type() {
        let id = Id::<User, &str>::new("eve");
        assert_eq!(id, Id::new("eve"));
        assert_ne!(id, Id::new("bob"));
        assert_eq!(id.to_string(), "eve");
    }

    #[test]
    fn non_default_int_bool_type() {
        let id1 = Id::<String, bool>::new(true);
        let id0 = Id::<String, bool>::new(false);

        assert_eq!(id1, Id::new(true));
        assert_eq!(id0, Id::new(false));
        assert_ne!(id1, id0);
        assert!(id1 > id0);
        assert_eq!(id1.to_string(), "true");
        assert_eq!(id0.to_string(), "false");
    }

    #[test]
    fn change_owner_and_backing_type() {
        let id1 = Id::<&str, u8>::new(1);
        let id2 = Id::<User, u16>::new(1);

        assert!(id1.change_backing_type().change_owner_type() == id2);
        assert!(id1.change_owner_type().change_backing_type() == id2);
    }

    #[test]
    fn usage_in_struct() {
        let alice = User {
            id: 1.into(),
            name: "alice".to_string(),
        };

        let bob = User {
            id: (alice.id().value() + 2).into(),
            name: "alice".to_string(),
        };

        assert_eq!(alice.id(), Id::<User>::new(1));
        assert_ne!(alice.id(), Id::<User>::new(2));
        assert_eq!(alice.id().to_string(), "1");

        assert_eq!(bob.id(), Id::<User>::new(3));
        assert_ne!(bob.id(), Id::<User>::new(2));
        assert_eq!(bob.id().to_string(), "3");
    }

    #[test]
    fn nested_because_why_not() {
        let id1 = Id::<User, u8>::new(1);
        let id2 = Id::<Id<User>, i64>::new(-2);
        let id3 = Id::<Id<Id<User>>, &str>::new("3");

        assert_eq!(id1.value(), 1);
        assert_eq!(id1.owner_type(), "tests::tests::User");
        assert_eq!(id1.backing_type(), "u8");

        assert_eq!(id2.value(), -2);
        assert_eq!(id2.owner_type(), "typed_id::Id<tests::tests::User>");
        assert_eq!(id2.backing_type(), "i64");

        assert_eq!(id3.value(), "3");
        assert_eq!(
            id3.owner_type(),
            "typed_id::Id<typed_id::Id<tests::tests::User>>"
        );
        assert_eq!(id3.backing_type(), "&str");
    }

    #[derive(Debug, PartialEq)]
    struct User {
        id: Id<Self>,
        name: String,
    }

    impl HasId for User {
        fn id(&self) -> Id<Self> {
            self.id
        }
    }
}
