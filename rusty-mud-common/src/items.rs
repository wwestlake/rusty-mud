use uuid::Uuid;
use reindeer::{Db, Serialize, Deserialize, Entity, Error};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ItemCategories {
    Money,
    Weapon,
    Armor,
    Tool,
    Clothing,
    Food,
    Drink,
    Resource,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoneyType {
    id: String,
    denomination: String,
    value: i32,
    based_on: Option<String> // the ID of the convert to for value
}

impl MoneyType {
    pub fn new(denomination: &str, value: i32, based_on: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            denomination: denomination.to_owned(),
            value,
            based_on
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetarySystem {
    id: String,
    denominations: Vec<MoneyType>,        
}

impl MonetarySystem {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4().to_string(), denominations: vec![] }
    }

    pub fn add(&mut self, moneyType: MoneyType) -> &mut Self {
        self.denominations.push(moneyType);
        self
    }



}






#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceTypes {
    Solid,
    Liquid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceCategories {
    Mineral,
    Material
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeaponType {
    damage: i32,
    durability: i32,
    repairable: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolType {
    durability: i32,
    repairable: bool
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArmorType {
    protection: i32,
    durability: i32,
    repairable: bool
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemTypes {
    id: String,
    category: ItemCategories,
    name: String,
    description: String,
    weight: i32,
    value: MoneyType
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemInstance {
    id: String,
    item: ItemTypes,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inventory {

}
