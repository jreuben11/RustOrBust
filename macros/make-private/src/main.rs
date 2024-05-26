use function_like_compose_macro::compose;
use make_private_macro::private;

fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

private!(
    struct Example {
        string_value: String,
        number_value: i32,
    }
);

fn main() {
    let e = Example::new();
    e.get_string_value();
    e.get_number_value();

    let composed = compose!(add_one . add_one . stringify);
    println!("{:?}", composed(5));
}
