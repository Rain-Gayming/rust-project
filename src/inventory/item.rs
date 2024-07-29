use bevy::prelude::*;

use super::item_type::item_types;


#[derive(Component)]
pub struct ItemInfo{
    pub(crate) item_name: String,
    pub(crate) item_type: item_types
}
