

#[macro_use] extern crate function_name;

macro_rules! function_path {() => (concat!(
    module_path!(), "::", function_name!()
))}

fn main() {
    naive::swap();
    stack_pinned::swap();
    heap_pinned::swap();
}

mod naive {

    // naive self-referential struct
    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
            }
        }

        fn init(&mut self) {
            let self_ref: *const String = &self.a;
            self.b = self_ref;
        }

        fn a(&self) -> &str {
            &self.a
        }

        fn b(&self) -> &String {
            assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
            unsafe { &*(self.b) }
        }
    }

    #[named]
    pub fn swap() {
        println!("\n{}:", function_path!().to_uppercase());

        let mut test1 = Test::new("test1");
        test1.init();
        let mut test2 = Test::new("test2");
        test2.init();
    
        println!("a: {}, b: {}", test1.a(), test1.b());
        println!("a: {}, b: {}", test2.a(), test2.b());
    
        std::mem::swap(&mut test1, &mut test2);
        test1.a = "I've totally changed now!".to_string();
        println!("a: {}, b: {}", test2.a(), test2.b());
    }
}

mod stack_pinned {
    use std::pin::Pin;
    use std::marker::PhantomPinned;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned, // This makes our type `!Unpin`
            }
        }
    
        fn init(self: Pin<&mut Self>) {
            let self_ptr: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };
            this.b = self_ptr;
        }
    
        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }
    
        fn b(self: Pin<&Self>) -> &String {
            assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
            unsafe { &*(self.b) }
        }
    }

    #[named]
    pub fn swap() {
        println!("\n{}:", function_path!().to_uppercase());

        // test1 is safe to move before we initialize it
        let mut test1 = Test::new("test1");
        // Notice how we shadow `test1` to prevent it from being accessed again
        let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
        Test::init(test1.as_mut());
    
        let mut test2 = Test::new("test2");
        let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
        Test::init(test2.as_mut());
    
        println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
        // the following will give compile error: PhantomPinned` cannot be unpinned
        // std::mem::swap(test1.get_mut(), test2.get_mut());
        println!("a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));
    }
}

mod heap_pinned {
    use std::pin::Pin;
    use std::marker::PhantomPinned;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Pin<Box<Self>> {
            let t = Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned,
            };
            let mut boxed = Box::pin(t);
            let self_ptr: *const String = &boxed.a;
            unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

            boxed
        }

        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }

        fn b(self: Pin<&Self>) -> &String {
            unsafe { &*(self.b) }
        }
    }

    #[named]
    pub fn swap() {
        println!("\n{}:", function_path!().to_uppercase());

        let test1 = Test::new("test1");
        let test2 = Test::new("test2");
    
        println!("a: {}, b: {}",test1.as_ref().a(), test1.as_ref().b());
        println!("a: {}, b: {}",test2.as_ref().a(), test2.as_ref().b());
    }
}
