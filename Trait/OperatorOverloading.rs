use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar(i32);

#[derive(Debug)]
struct BarFoo(i32);

impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar(233)
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo(322)
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}