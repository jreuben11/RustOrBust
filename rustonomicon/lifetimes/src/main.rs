#![allow(dead_code)]

mod aliasing {
    fn compute1(input: &u32, output: &mut u32) {
        let mut temp = *output; 
        if *input > 10 {
            temp = 1;
        }
        if *input > 5 {
            temp *= 2;
        }
        *output = temp;
    }
    // aliasing rules lead to compiler optimizations
    fn compute2(input: &u32, output: &mut u32) {
        let cached_input = *input; // keep `*input` in a register
        if cached_input > 10 {
            // eliminate a read-modify-write in the other branch
            *output = 2;
        } else if cached_input > 5 {
            *output *= 2;
        }
    }

    pub fn demo(){
        let v = vec![0,6,11];
        let mut i: u32; 
        let mut o: u32 = 1; 
        for v_i in v {
            i = v_i;
            compute1(&i,&mut o);
            println!("compute1: {}->{}", i, o);
            compute2(&i,&mut o);
            println!("compute2: {}->{}", i, o);
        }
    }
}

mod lifetimes {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct X<'a>(&'a i32);
    impl Drop for X<'_> {
        fn drop(&mut self) {}
    }

    fn bernoulli_sample() -> bool {
        use rand::distributions::{Bernoulli, Distribution};
        let d = Bernoulli::new(0.5).unwrap();
        let v = d.sample(&mut rand::thread_rng());
        v
    }

    pub fn demo(){
        let mut data = vec![1, 2, 3];
        let x = &data[0];
        // scope optimization of lifetimes
        if bernoulli_sample() {
            println!("{}", x); // This is the last use of `x` in this branch
            data.push(4);      // So we can push here
        } else {
            // There's no use of `x` in here, so effectively the last use is the
            // creation of x at the top of the example.
            data.push(5);
        }
    }
}

mod lifetime_limits {
    #[derive(Debug)]
    struct NaiveFoo;
    impl NaiveFoo {
        fn mutate_and_share(&mut self) -> &Self { &*self }
        fn share(&self) {}
    }

    pub fn demo(){
        let mut foo = NaiveFoo;
        let _loan = foo.mutate_and_share(); // --- mutable borrow occurs here
        foo.share(); // --- mutable borrow occurs here
        // println!("{:?}", _loan); // compiler error ---- mutable borrow later used here
    }
}

mod lifetime_ellision {
    // fn get_str() -> &str;                                   // ILLEGAL
    // fn frob(s: &str, t: &str) -> &str;                      // ILLEGAL
}

mod unbounded_lifetimes {
    
    // Unbounded Lifetime
    fn get_str<'a>(s: *const String) -> &'a str {
        unsafe { &*s }
    }

    pub fn demo(){
        let soon_dropped = String::from("hello");
        let dangling = get_str(&soon_dropped);
        drop(soon_dropped);
        println!("Invalid str: {}", dangling); // Invalid str: gӚ_`
    }
}

mod higher_rank_trait_bounds {
    // Higher-Rank Trait Bounds (HRTBs) - for lifetime ellision
    struct Closure<F> {
        data: (u8, u16),
        func: F,
    }
    impl<F> Closure<F>
        where F: Fn(&(u8, u16)) -> &u8,
    {
        fn call(&self) -> &u8 {
            (self.func)(&self.data)
        }
    }
    fn do_it(data: &(u8, u16)) -> &u8 { &data.0 }

    pub fn demo(){
        let clo = Closure { data: (0, 1), func: do_it };
        println!("{}", clo.call());
    }
}

mod subtyping_and_variance {

    fn debug<'a>(a: &'a str, b: &'a str) { // immutable -> covariant arg to param (can pass longer lived)
        println!("a = {a:?} b = {b:?}");
    }
    fn assign<T>(input: &mut T, val: T) {  // mutable -> invariant arg to param (must be exactly the same)
        *input = val;
    }

    use std::cell::RefCell;
    thread_local! {
        pub static STATIC_VECS: RefCell<Vec<&'static str>> = RefCell::new(Vec::new());
    }
    /// saves the input given into a thread local `Vec<&'static str>`
    fn store(input: &'static str) {
        STATIC_VECS.with_borrow_mut(|v| v.push(input));
    }
    /// Calls the function with it's input (must have the same lifetime!)
    fn call_closure<'a>(input: &'a str, f: fn(&'a str)) {
        f(input);
    }

    use std::cell::Cell;

    struct MyType<'a, 'b, A: 'a, B: 'b, C, D, E, F, G, H, In, Out, Mixed> {
        a: &'a A,     // covariant over 'a and A
        b: &'b mut B, // covariant over 'b and invariant over B

        c: *const C,  // covariant over C
        d: *mut D,    // invariant over D

        e: E,         // covariant over E
        f: Vec<F>,    // covariant over F
        g: Cell<G>,   // invariant over G

        h1: H,        // would also be covariant over H except...
        h2: Cell<H>,  // invariant over H, because invariance wins all conflicts

        i: fn(In) -> Out,       // contravariant over In, covariant over Out

        k1: fn(Mixed) -> usize, // would be contravariant over Mixed except..
        k2: Mixed,              // invariant over Mixed, because invariance wins all conflicts
    }

    pub fn demo(){

        let hello: &'static str = "hello";
        {
            let world = String::from("world");
            let world = &world; // 'world has a shorter lifetime than 'static
            debug(hello, world); // hello silently downgrades from SUBTYPE `&'static str` into SUPERTYPE `&'world str`
        } 
        let mut hello: &'static str = "hello";
        let mut bob = String::from("bob");
        {
            let _world = String::from("world");
            // assign(&mut hello, &_world); // compiler error --- world borrowed value does not live long enough
        }
        // assign(&mut hello, &bob); // compiler error --- invariant lifetimes: bob borrowed value does not live long enough compared to static
        assign(&mut bob, (&mut hello).to_string()); // ok: string literal from static has same lifetime
        println!("{bob}");  
    
        let hello: Box<&'static str> = Box::new("hello");
        let mut _world: Box<&str>;
        _world = hello; // move ownership of hello, destroying the static ref string
        
        call_closure("hello", store); // "hello" is 'static. Can call `store` fine
        let _smuggle = String::from("smuggle");
        // call_closure(&_smuggle, store); // compiler error: contravariance --- `fn(&'static str)` cannot be a subtype of `fn(&'a str)`
        STATIC_VECS.with_borrow(|v| println!("{v:?}"));
    }
}

mod drop_check {
    #![feature(dropck_eyepatch)]
    #![allow(unused_attributes)]
    struct Inspector<'a>(&'a u8, &'static str);
    
    // NIGhTLY ONLY: #[may_dangle]
    // unsafe impl<#[may_dangle] 'a> Drop for Inspector<'a> {
    //     fn drop(&mut self) {
    //         println!("Inspector(_, {}) knows when *not* to inspect.", self.1);
    //     }
    // }
    
    struct World<'a> {
        days: Box<u8>,
        inspector: Option<Inspector<'a>>,
    }
    
    pub fn demo() {
        let mut world = World {
            inspector: None,
            days: Box::new(1),
        };
        world.inspector = Some(Inspector(&world.days, "gadget"));
    }

}

mod phantom_data {
    use std::marker;

    struct Iter<'a, T: 'a> {
        ptr: *const T,
        end: *const T,
        _marker: marker::PhantomData<&'a T>,
    }

    struct MyVec<T> {
        data: *const T, // `*const` for variance 
        len: usize,
        cap: usize,
    }
    
    impl<T> Drop for MyVec<T> { 
        fn drop(&mut self) {}
    }
}


fn main() {
    aliasing::demo();
    lifetimes::demo();
    lifetime_limits::demo();
    unbounded_lifetimes::demo();
    higher_rank_trait_bounds::demo();
    subtyping_and_variance::demo();
    drop_check::demo();
}
