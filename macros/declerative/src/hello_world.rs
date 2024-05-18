macro_rules! hello_world {
    ($something:ty) => {
        impl $something {
            fn hello_world(&self) {
                println!("Hello world!")
            }
        }
    };
}
