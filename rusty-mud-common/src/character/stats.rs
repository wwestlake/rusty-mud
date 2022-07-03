
use crate::random::*;
use prettytable::*;

pub struct characterStats {
    strength: i32,
    dexterity: i32,
    endurance: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,

    armor_class: i32,
    hit_points: i32,
    experience: i32,
    level: i32,
}

impl characterStats {

    pub fn new() -> Self {
        Self {
            strength: dice(6, 3) + 10,
            dexterity: dice(6, 3) + 10,
            endurance: dice(6, 3) + 10,
            intelligence: dice(6, 3) + 10,
            wisdom: dice(6, 3) + 10,
            charisma: dice(6, 3) + 10,
        
            armor_class: 1,
            hit_points: dice(6, 3) * 10,
            experience: 0,
            level: 1,
        }
    }

    pub fn to_table(&self) -> Table {
        let mut table = Table::new();

        table.add_row(Row::new(vec![Cell::new("AC"), Cell::new("HP"), Cell::new("Exp"), Cell::new("Lvl")]));
        table.add_row(Row::new(vec![  
                            Cell::new(&self.armor_class.to_string()), 
                            Cell::new(&self.hit_points.to_string()), 
                            Cell::new(&self.experience.to_string()), 
                            Cell::new(&self.level.to_string()), 
                    ]));

        table.add_empty_row();
        table.add_row(Row::new(vec![Cell::new("Str"), Cell::new("Dex"), Cell::new("End"), Cell::new("Int"), Cell::new("Wis"), Cell::new("Chr")]));
        table.add_row(Row::new(vec![ Cell::new(&self.strength.to_string()), 
                            Cell::new(&self.dexterity.to_string()), 
                            Cell::new(&self.endurance.to_string()), 
                            Cell::new(&self.intelligence.to_string()), 
                            Cell::new(&self.wisdom.to_string()), 
                            Cell::new(&self.charisma.to_string())
                    ]));

        table

    }

}



