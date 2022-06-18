
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down
}

pub struct Item {
    id: i32,
    name: String,
    description: String,
    value: i32
}

pub struct Inventory {
    items: Vec<Item>
}



pub struct Room {
    name: String,
    description: String,
    exits: Vec<Option<Direction>>
}

pub struct World {
    grid: [[[Room; 10]; 10]; 10]
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
