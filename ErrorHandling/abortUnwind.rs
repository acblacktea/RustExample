fn drink(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic="abort") {println!("This is not your party. Run!!!");}
        else { println!("Spit it out");}
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

#[cfg(panic="unwind")]
fn ah() {println!("Spit it out!!!");}

#[cfg(not(panic="unwind"))]
fn ah() { println!("This is not your party. Run!!!");}

fn drink2(beverage: &str) {
    if beverage == "lemonade" { ah(); }
    else {println!("Some refreshing {} is all I need.", beverage);}
}
fn main() {
    drink2("water");
    drink2("lemonade");
}