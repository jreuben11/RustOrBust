// #[cfg(some_condition)]       // will not compile without `rustc --cfg some_condition` !!!
#[allow(dead_code)]
fn conditional_function() {
    println!("condition met!");
}

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    //conditional_function();
    are_you_on_linux();
}