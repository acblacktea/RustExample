fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2;
    let last: &i32;
    let multiplyNum: i32;
    {
        let second = 3;

        multiplyNum = multiply(&first, &second);
        last = choose_first(&second, &first);
    }

    println!("The produce is {}", multiplyNum);
    println!("{} is the first", last);
}