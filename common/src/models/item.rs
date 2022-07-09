use crate::schema::items;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: i32,
    pub path: String,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}

impl std::hash::Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub path: &'a str,
}
