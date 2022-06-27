
use uuid::Uuid;
use reindeer::{Db, Serialize, Deserialize, Entity, Error};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Up,
    Down
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    x: i32, // east/west
    y: i32, // north/south
    z: i32, // up/down
}

impl Location {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z}
    }

    pub fn go(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North =>        Self { x: self.x,     y: self.y + 1,  z: self.z },
            Direction::NorthEast =>    Self { x: self.x + 1, y: self.y+1,    z: self.z },
            Direction::East =>         Self { x: self.x + 1, y: self.y,      z: self.z },
            Direction::SouthEast =>    Self { x: self.x + 1, y: self.y - 1,  z: self.z },
            Direction::South =>        Self { x: self.x,     y: self.y - 1,  z: self.z },
            Direction::SouthWest =>    Self { x: self.x - 1, y: self.y - 1,  z: self.z },
            Direction::West =>         Self { x: self.x - 1, y: self.y,      z: self.z },
            Direction::NorthWest =>    Self { x: self.x -1,  y: self.y + 1,  z: self.z },
            Direction::Up =>           Self { x: self.z,     y: self.y,      z: self.z + 1},
            Direction::Down  =>        Self { x: self.z,     y: self.y,      z: self.z - 1},
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    id: String,
    location: Location,
    name: String,
    description: String,
    exits: Vec<Direction>,
}


impl Room {
    pub fn new(x: i32, y: i32, z:i32, name: &str, description: &str) -> Self {
        Self { 
            id: Uuid::new_v4().to_string(), 
            location: Location::new(x,y,z), 
            name: name.to_owned(), 
            description: description.to_owned(),
            exits: vec![], 
        }
    }

    pub fn go(&self, direciton: &Direction) -> &Room {
        // look in exits for valid directon
        // if not available return this room

        // get new location self.location.go(direction)

        // find room in database based on direction:
            // check local cache first -> room
            // check database
            // return room
        self
    }
}

