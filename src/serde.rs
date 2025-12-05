use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Id;

impl<T, I: Serialize> Serialize for Id<T, I> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.id.serialize(serializer)
    }
}

impl<'de, T, I: Deserialize<'de>> Deserialize<'de> for Id<T, I> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        I::deserialize(deserializer).map(|i| i.into())
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use serde::{Deserialize, Serialize};

    use crate::Id;

    #[test]
    fn serialize() {
        let expected = r#"{"id":1,"name":"admin"}"#;
        let user = User {
            id: 1.into(),
            name: "admin".to_string(),
        };

        let result = serde_json::to_string(&user).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn deserialize() {
        let json = r#"{ "id": 1, "name": "admin" }"#;

        let user = serde_json::from_str::<User>(json).unwrap();

        assert_eq!(user.id, Id::<User, u32>::new(1));
        assert_eq!(user.name, "admin");
    }

    #[derive(Serialize, Deserialize)]
    struct User {
        id: Id<Self>,
        name: String,
    }
}
