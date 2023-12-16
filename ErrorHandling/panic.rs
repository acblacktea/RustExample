fn drink(beverage: &str) {
    if beverage == "lemonade" { panic!("AAAAaaaaa!!!!");}
    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}