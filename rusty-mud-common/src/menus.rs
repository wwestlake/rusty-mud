use std::num::ParseIntError;


pub struct MenuItem {
    text: String,
    item: i32,
    func: fn () -> (),
}

impl MenuItem {
    pub fn new(text: String, item: i32, func: fn () -> ()) -> Self {
        Self {
            text, item, func
        }
    }
}

pub struct Menu {
    title: String,
    prompt: String,
    items: Vec<MenuItem>,
    out: fn (String) -> (),
    read: fn () -> String,
}

impl Menu {
    pub fn new(title: String, prompt: String, out: fn (String) -> (), read: fn () -> String) -> Self {
        Self {
            title, prompt, items: vec![], out, read
        }
    }

    pub fn add(&mut self, text: String, item: i32, func: fn () -> ()) -> &mut Self {
        self.items.push(MenuItem {
            text, item, func
        });
        self
    }

    pub fn show(&mut self) {
        self.items.sort_by(|a,b| a.item.cmp(&b.item) );
        for item in &self.items {
            (self.out)(std::format!("{}) {}", item.item, item.text));
        }
    }

    pub fn run(&mut self) {
        let input: Result<i32, ParseIntError> = (self.read)().parse();
        match input {
            Ok(num) => {
                let action = self.items.iter().find(|x| x.item == num);
                match action {
                    Some(item) => (item.func)(),
                    None => ()
                }
            },
            Err(_) => {}
        }
    }

}

