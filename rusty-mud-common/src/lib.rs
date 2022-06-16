use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: i32,
    message: String
}

impl Message {
    pub fn new(id: i32, message: String) -> Self {
        Self { id, message }
    }

    pub fn serialize(&self) -> Result<String> {
        
        serde_json::to_string(&self)
    }

    pub fn deserialize(msg: &str) -> Result<Message> {
        serde_json::from_str(msg)
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
