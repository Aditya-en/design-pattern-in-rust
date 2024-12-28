trait toppings {
    fn description(&self) -> String;
}

struct Pepperoni {
    desc: String,
}
impl toppings for Pepperoni{
    fn description(&self) -> String {
        self.desc.clone() + &"Pepperoni toppings.".to_string()
    }
}
struct Olive {
    desc: String,
}
impl toppings for Olive {
    fn description(&self) -> String {
        self.desc.clone() + &"Olive toppings.".to_string()
    }
}
struct Mushrooms {
    desc: String,
}
impl toppings for Mushrooms {
    fn description(&self) -> String {
        self.desc.clone() + &"Mushrooms toppings.".to_string()
    }
}

enum toppingsType {
    Pepperoni,
    Olive,
    Mushrooms,
}

fn factory(topping_type: toppingsType, desc: String) -> Box<dyn toppings>{
    match topping_type {
        toppingsType::Pepperoni => {
            Box::new(Pepperoni{desc:desc})
        }
        toppingsType::Olive => {
            Box::new(Olive{desc:desc})
        }
        toppingsType::Mushrooms => {
            Box::new(Mushrooms{desc:desc})
        }
    }

}
fn main() {
    let pepp = factory(toppingsType::Pepperoni, "this is Pepperoni".to_string());
    let olive = factory(toppingsType::Olive, "this is Olive".to_string());
    let mush = factory(toppingsType::Mushrooms, "this is Mushrooms".to_string());
    println!("desc of pepp {}", pepp.description());
    println!("desc of olive {}", olive.description());
    println!("desc of mush {}", mush.description());

}
