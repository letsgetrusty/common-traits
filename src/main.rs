use std::{rc::Rc, sync::Arc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Role {
    Admin,
    Standard,
    #[default]
    Guest,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct DB {}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct User {
    id: u32,
    name: String,
    role: Role,
    #[cfg_attr(feature = "serde", serde(skip))]
    db: Arc<DB>,
}

fn main() {
    let user = User {
        id: 123,
        name: "Bogdan".to_owned(),
        role: Role::Admin,
        db: Arc::new(DB {}),
    };

    println!("{:?}", user);

    let user2 = user.clone();

    println!("{:?}", user2);

    let guest = User::default();

    let guest2 = User::default();

    assert_eq!(guest, guest2);

    let user_str = "{ \"id\": 123, \"name\": \"Bogdan\", \"role\": \"Admin\" }";

    #[cfg(feature = "serde")]
    let user: User = serde_json::from_str(&user_str).unwrap();

    #[cfg(feature = "serde")]
    println!("{:?}", user);
}

fn is_normal<T: Sized + Send + Sync + Unpin>() {}

#[test]
fn normal_types() {
    is_normal::<User>();
}
