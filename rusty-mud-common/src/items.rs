use uuid::Uuid;
use reindeer::{Db, Serialize, Deserialize, Entity, Error};
use std::fmt;

pub enum ItemCategories {
    Weapon,
    Armor,
    Tool,
    Clothing,
    Food,
    Drink,
    Resource(ResourceTypes, ResourceCategories),
}

pub enum ResourceTypes {
    Solid,
    Liquid,
}

pub enum ResourceCategories {
    Mineral,
    Material
}

