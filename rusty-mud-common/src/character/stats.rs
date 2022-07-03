
use crate::random::*;
use prettytable::*;

pub struct CharStats {
    strength: i32,
    dexterity: i32,
    endurance: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,

    armor_class: i32,
    hit_points: i32,
    experience: i32,
    level: i32
}

impl CharStats {

    pub fn new() -> Self {
        Self {
            strength: dice(6,3),
            dexterity:  dice(6,3),
            endurance:  dice(6,3),
            intelligence:  dice(6,3),
            wisdom:  dice(6,3),
            charisma:  dice(6,3),
        
            armor_class: 1,
            hit_points:  dice(6,3) + 20,
            experience: 0,
            level: 1
        }
    }

    pub fn as_table(&self) -> Table {
        let mut table = Table::new();

        table.add_row(Row::new(vec![Cell::new("General")]));
        table.add_row(Row::new(vec![Cell::new("Experience"), Cell::new(&self.experience.to_string())]));
        table.add_row(Row::new(vec![Cell::new("Level"), Cell::new(&self.level.to_string())]));
        table.add_row(Row::new(vec![Cell::new("Hit Points"), Cell::new(&self.hit_points.to_string())]));
        table.add_row(Row::new(vec![Cell::new("Armor Class"), Cell::new(&self.armor_class.to_string())]));

        table.add_row(Row::new(vec![Cell::new("Stats")]));

        table.add_row(Row::new(vec![Cell::new("Strength"), Cell::new(&self.strength.to_string())]));
        table.add_row(Row::new(vec![Cell::new("Dexterity"), Cell::new(&self.dexterity.to_string())]));
        table.add_row(Row::new(vec![Cell::new("Endurance"), Cell::new(&self.endurance.to_string())]));
        table.add_row(Row::new(vec![Cell::new("Intelligence"), Cell::new(&self.intelligence.to_string())]));
        table.add_row(Row::new(vec![Cell::new("Wisdom"), Cell::new(&self.wisdom.to_string())]));
        table.add_row(Row::new(vec![Cell::new("Charisma"), Cell::new(&self.charisma.to_string())]));
        table.add_row(Row::new(vec![Cell::new("Armor Class"), Cell::new(&self.armor_class.to_string())]));

        table    
    }

}

