use serde::{Deserialize, Serialize};

use crate::ListItem;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListItemData {
    completed: bool,
    description: String,
    datetime: String,
}

impl From<ListItemData> for ListItem {
    fn from(val: ListItemData) -> Self {
        ListItem {
            completed: val.completed,
            description: val.description.into(),
            datetime: val.datetime.into(),
        }
    }
}

impl From<ListItem> for ListItemData {
    fn from(value: ListItem) -> Self {
        Self {
            completed: value.completed,
            description: value.description.to_string(),
            datetime: value.datetime.to_string(),
        }
    }
}
