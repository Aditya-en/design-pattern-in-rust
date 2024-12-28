#[derive(Debug)]
struct Pizza {
    size: String,
    crust: String,
    toppings: Vec<String>,
    cheeze: bool,
    extra_sauce: bool,
}

#[derive(Debug)]
struct PizzaBuilder {
    size: Option<String>,
    crust: Option<String>,
    toppings: Option<Vec<String>>,
    cheeze: Option<bool>,
    extra_sauce: Option<bool>,
}

impl PizzaBuilder {
    fn new() -> Self {
        Self {
            size : None,
            crust : None,
            toppings : None,
            cheeze : None,
            extra_sauce : None,
        }
    }

    fn size(&mut self, size: String) -> &mut Self {
        self.size = Some(size);
        self
    }
    fn crust(&mut self, crust: String) -> &mut Self {
        self.crust = Some(crust);
        self
    }
    fn toppings(&mut self, toppings: Vec<String>) -> &mut Self {
        self.toppings = Some(toppings);
        self
    }
    fn cheeze(&mut self, cheeze: bool) -> &mut Self {
        self.cheeze = Some(cheeze);
        self
    }
    fn extra_sauce(&mut self, extra_sauce: bool) -> &mut Self {
        self.extra_sauce = Some(extra_sauce);
        self
    }
    fn build(&self) -> Pizza {
        Pizza {
            size: self.size.clone().unwrap_or("medium".to_string()),
            crust: self.crust.clone().unwrap_or("thin".to_string()),
            toppings: self.toppings.clone().unwrap_or(vec!["default".to_string()]),
            cheeze: self.cheeze.clone().unwrap_or(false),
            extra_sauce: self.extra_sauce.clone().unwrap_or(false),
        }
    }
}

fn main() {
    let size = "large".to_string();
    let crust = "thick".to_string();
    let toppings = vec!["pepperoni".to_string(), "mushrooms".to_string()];
    let cheeze = true;
    let extra_sauce = true;

    let pizza_basic = PizzaBuilder::new().build();
    println!("pizza_basic {:?} ", pizza_basic);
    
    let pizza_with_toppings = PizzaBuilder::new()
        .toppings(toppings.clone())
        .build();
    println!("pizza_with_toppings {:?}", pizza_with_toppings);

    let pizza_large_toppings_cheeze = PizzaBuilder::new()
        .toppings(toppings.clone())
        .size("large".to_string())
        .cheeze(cheeze)
        .build();
    println!("pizza_large_toppings_cheeze {:?}", pizza_large_toppings_cheeze);

    let pizza_small_extra_sauce = PizzaBuilder::new()
        .size("small".to_string())
        .extra_sauce(extra_sauce)
        .build();
    println!("pizza_small_extra_sauce {:?}", pizza_small_extra_sauce);

}


