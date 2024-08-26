#![allow(unused)]
use std::error::Error;
use std::fmt::Debug;

fn type_system_data_structures() {
    // widening / narrowing
    let x = 42i32; // Integer literal with type suffix
    let y: i64 = x.into();
    let z: i16 = x.try_into().unwrap();

    struct TextMatch(usize, String);
    let m = TextMatch(12, "needle".to_owned());
    assert_eq!(m.0, 12);

    enum HttpResultCode {
        Ok = 200,
        NotFound = 404,
        Teapot = 418,
    }
    let code = HttpResultCode::NotFound;
    assert_eq!(code as i32, 404);
}

fn type_system_behaviors() {
    enum Shape {
        Rectangle { width: f64, height: f64 },
        Circle { radius: f64 },
    }
    impl Shape {
        pub fn area(&self) -> f64 {
            match self {
                Shape::Rectangle { width, height } => width * height,
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            }
        }
    }

    // function pointers
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    // Explicit coercion to `fn` type is required...
    let op: fn(i32, i32) -> i32 = sum;
    // `fn` types implement `Copy`
    let op1 = op;
    let op2 = op;
    // `fn` types implement `Eq`
    assert!(op1 == op2);
    // `fn` implements `std::fmt::Pointer`, used by the {:p} format specifier.
    println!("op = {:p}", op);

    // closures
    // In real code, an `Iterator` method would be more appropriate.
    pub fn modify_all(data: &mut [u32], mutator: fn(u32) -> u32) {
        for value in data {
            *value = mutator(*value);
        }
    }
    // simple mutation of a slice:
    fn add2(v: u32) -> u32 {
        v + 2
    }
    let mut data = vec![1, 2, 3];
    modify_all(&mut data, add2);
    assert_eq!(data, vec![3, 4, 5,]);
    let amount_to_add = 3;
    let add_n = |y| {
        // a closure capturing `amount_to_add`
        y + amount_to_add
    };
    let z = add_n(5);
    assert_eq!(z, 8);

    // traits
    pub trait Sort {
        /// Re-arrange contents into sorted order.
        fn sort(&mut self);
    }
    /// Marker trait to indicate that a [`Sortable`] sorts stably.
    pub trait StableSort: Sort {}
    // trait bounds:
    pub fn dump_sorted<T>(mut collection: T)
    where
        T: Sort + IntoIterator,
        T::Item: Debug,
    {
        // Next line requires `T: Sort` trait bound.
        collection.sort();
        // Next line requires `T: IntoIterator` trait bound.
        for item in collection {
            // Next line requires `T::Item : Debug` trait bound
            println!("{:?}", item);
        }
    }
}

// 1.
// 2.
// 4.
fn idiomatic_errors() {
    // minimal error
    #[derive(Debug)]
    pub struct MyError1(String);
    impl std::fmt::Display for MyError1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl std::error::Error for MyError1 {}
    impl std::convert::From<String> for MyError1 {
        fn from(msg: String) -> Self {
            Self(msg)
        }
    }

    // nested enum error
    #[derive(Debug)]
    pub enum MyError2 {
        Io(std::io::Error),
        Utf8(std::string::FromUtf8Error),
        General(String),
    }
    impl std::fmt::Display for MyError2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MyError2::Io(e) => write!(f, "IO error: {}", e),
                MyError2::Utf8(e) => write!(f, "UTF-8 error: {}", e),
                MyError2::General(s) => write!(f, "General error: {}", s),
            }
        }
    }

    impl Error for MyError2 {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                MyError2::Io(e) => Some(e),
                MyError2::Utf8(e) => Some(e),
                MyError2::General(_) => None,
            }
        }
    }
    use std::io::BufRead;
    /// Return the first line of the given file.
    pub fn first_line(filename: &str) -> Result<String, MyError2> {
        let file = std::fs::File::open(filename).map_err(MyError2::Io)?;
        let mut reader = std::io::BufReader::new(file);

        // (A real implementation could just use `reader.read_line()`)
        let mut buf = vec![];
        let len = reader.read_until(b'\n', &mut buf).map_err(MyError2::Io)?;
        let result = String::from_utf8(buf).map_err(MyError2::Utf8)?;
        if result.len() > 100 {
            return Err(MyError2::General(format!("Line too long: {}", len)));
        }
        Ok(result)
    }
}

fn main() {
    type_system_data_structures();
    type_system_behaviors();
    idiomatic_errors();
}
