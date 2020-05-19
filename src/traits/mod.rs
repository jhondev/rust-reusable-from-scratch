// monomorphization
pub trait Talker {
    fn talk(&self, message: &str);
}

pub struct Banker {
    name: String,
}

impl Talker for Banker {
    fn talk(&self, message: &str) {
        println!("Hello sr I'm {}. This is is your message", self.name);
        println!("{}", message);
    }
}

pub struct Girlfriend {
    age: u32,
}

impl Talker for Girlfriend {
    fn talk(&self, message: &str) {
        println!("Hi you. I'm {}. This is your message", self.age);
        println!("{}", message);
    }
}

pub fn init_conversation<T: Talker>(talker: T) {
    talker.talk(&format!("no issues"));
}

pub fn give_turn() {
    let who = rand::random::<bool>();
    let banker = Banker {
        name: "John".to_owned(),
    };
    let girlfriend = Girlfriend { age: 14 };
    match who {
        true => init_conversation(banker),
        false => init_conversation(girlfriend),
    }
}
#[cfg(test)]
mod test_traits {
    use super::*;

    #[test]
    fn test_() {
        give_turn()
    }
}
