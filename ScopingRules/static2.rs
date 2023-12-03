use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in: {:?}", input);
}

fn main() {
    let i = 5;
    print_it(i);

    print_it(&i);
}