fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}
#[allow(dead_code)]
fn noisy_unuesd_function() {}

fn main() {
    used_function()
}