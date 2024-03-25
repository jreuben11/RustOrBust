# Rustonomicon

## [safe but unsound](safe_but_unsound/src/lib.rs)
- Vec[T] `as_slice()` -> &[T]
- `cargo test -- --show-output` - println! from unit test
```rust
let v: Vec<u8> = vec![1,2,3];
let a: &[u8] = v.as_slice();
let ui = unsafe {
    *a.get_unchecked(0)
};
```
- `std::ptr`
```rust
pub struct NaiveVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}
...
unsafe {
    ptr::write(self.ptr.add(self.len), elem); // push
    ...
}
```
## [data layout](data_layout/src/main.rs)
- `repr(Rust)`
- Dynamically Sized Types (DSTs)
- Zero Sized Types (ZSTs)
- Empty Types
- Alternative representations
```rust
#![allow(dead_code)]
use std::mem::size_of;
...

println!("sizes - A:{}, APadded:{}", size_of::<A>(), size_of::<APadded>());

const DST_SIZE: usize = 8;
let static_sized: MySuperSliceable<[u8; DST_SIZE]> = MySuperSliceable {
    info: 17,
    data: [0; DST_SIZE],
};
let dynamic_sized: &MySuperSliceable<[u8]> = &static_sized;
println!("{} {:?}", dynamic_sized.info, &dynamic_sized.data); // prints: "17 [0, 0, 0, 0, 0, 0, 0, 0]"

assert_eq!(0, size_of::<LotsOfNothing>());

let res: Result<u32, Void> = Ok(0);
// Err doesn't exist anymore, so Ok is actually irrefutable.
let Ok(_num) = res else { todo!() };

assert_eq!(8, size_of::<MyOption<&u16>>());
assert_eq!(16, size_of::<MyReprOption<&u16>>());
```

## [lifetimes](lifetimes/src/main.rs)
```rust
    aliasing::demo();
    lifetimes::demo();
    lifetime_limits::demo();
    unbounded_lifetimes::demo();
    higher_rank_trait_bounds::demo();
    subtyping_and_variance::demo();
    drop_check::demo();
```
### references
2 kinds of reference:
1. Shared reference: `&`
2. Mutable reference: `&mut`
rules:
- A reference cannot outlive its referent
- A mutable reference cannot be aliased
### aliasing
- why aliasing matters
### lifetimes
```rust
    fn bernoulli_sample() -> bool {
        use rand::distributions::{Bernoulli, Distribution};
        let d = Bernoulli::new(0.5).unwrap();
        let v = d.sample(&mut rand::thread_rng());
        v
    }
```
### lifetime limits
- Improperly reduced borrows
- Elision rules:
  - Each elided lifetime in input position becomes a distinct lifetime parameter.
  - If there is exactly one input lifetime position (elided or not), that lifetime is assigned to all elided output lifetimes.
  - If there are multiple input lifetime positions, but one of them is `&self` or `&mut self`, the lifetime of self is assigned to all elided output lifetimes.
### lifetime ellision
```rust
fn print(s: &str);                                      // elided
fn print<'a>(s: &'a str);                               // expanded

fn debug(lvl: usize, s: &str);                          // elided
fn debug<'a>(lvl: usize, s: &'a str);                   // expanded

fn substr(s: &str, until: usize) -> &str;               // elided
fn substr<'a>(s: &'a str, until: usize) -> &'a str;     // expanded

fn get_str() -> &str;                                   // ILLEGAL

fn frob(s: &str, t: &str) -> &str;                      // ILLEGAL

fn get_mut(&mut self) -> &mut T;                        // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T;              // expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command                  // elided
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command // expanded

fn new(buf: &mut [u8]) -> BufWriter;                    // elided
fn new(buf: &mut [u8]) -> BufWriter<'_>;                // elided (with `rust_2018_idioms`)
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>          // expanded
```
### Unbounded Lifetimes
- any output lifetimes that don't derive from inputs are unbounded
- `transmute` / `transmute_copy`
### Higher-Rank Trait Bounds (HRTBs)
- clousures
- `where for<'a>`
```rust
struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
    where for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn do_it(data: &(u8, u16)) -> &u8 { &data.0 }

fn main() {
    let clo = Closure { data: (0, 1), func: do_it };
    println!("{}", clo.call());
}
```
### subtyping and variance
- subtyping of lifetimes: **Sub <: Super** == **'long <: 'short** iff **'long** defines a region of code that completely contains **'short**. eg `'static` <: ``a`
- variance rules:
  - `F` is **covariant** if `F<Sub>` is a subtype of `F<Super>` (the subtype property is **passed through**)
  - `F` is **contravariant** if `F<Super>` is a subtype of `F<Sub>` (the subtype property is "**inverted**")
  - `F` is **invariant** otherwise (**no subtyping relationship exists**) - must be passed exactly
- interior mutability: `std::cell::Cell::{Cell, RefCell}` - `with_borrow` / `with_borrow_mut`
```rust
    thread_local! {
        pub static STATIC_VECS: RefCell<Vec<&'static str>> = RefCell::new(Vec::new());
    }
    /// saves the input given into a thread local `Vec<&'static str>`
    fn store(input: &'static str) {
        STATIC_VECS.with_borrow_mut(|v| v.push(input));
    }
    ...
    STATIC_VECS.with_borrow(|v| println!("{v:?}"));
```
```rust
    fn debug<'a>(a: &'a str, b: &'a str) { // immutable -> covariant arg to param (can pass longer lived)
        println!("a = {a:?} b = {b:?}");
    }
    fn assign<T>(input: &mut T, val: T) {  // mutable -> invariant arg to param (must be exactly the same)
        *input = val;
    }

```
|   	            |   `'a`	    |   `T`	            |   `U`	    |   	
|---	            |---	        |---	            |---	    |
| `&'a T` 	        | covariant  	| covariant  	    |   	    |
| `&'a mut T` 	    | covariant 	| invariant  	    |   	    |
| `Box<T>`          |   	        | covariant  	    |   	    |
| `Vec<T>`	        |   	        | covariant  	    |   	    |
| `UnsafeCell<T>`	|   	        | invariant  	    |   	    |
| `Cell<T>`	        |   	        | invariant  	    |   	    |
| `fn(T) -> U`	    |   	        | contravariant  	| covariant |
| `*const T`        |   	        | covariant  	    |   	    |
|  `*mut T`	        |   	        | covariant  	    |   	    |
|   	            |   	        |     	            |   	    |
- A struct "inherits" the variance of its fields
### Drop Check
```rust
#![feature(dropck_eyepatch)]
#![allow(unused_attributes)]

struct Inspector<'a>(&'a u8, &'static str);
    
// NIGhTLY ONLY: #[may_dangle] -escape hatch
unsafe impl<#[may_dangle] 'a> Drop for Inspector<'a> {
    fn drop(&mut self) {
        println!("Inspector(_, {}) knows when *not* to inspect.", self.1);
    }
}
```
- Sound generic drop is enforced by the drop checker
- For a generic type to soundly implement drop, its generics arguments must strictly outlive it.
### phantom data
- market type for bounding lifetimes for purpose of static analysis
```rust
    use std::marker;

    struct Iter<'a, T: 'a> {
        ptr: *const T,
        end: *const T,
        _marker: marker::PhantomData<&'a T>,
    }
```
- Generic parameters and drop-checking
  
| Phantom type	                | variance of `'a`  | variance of `T`	    | `Send`/`Sync` (or lack thereof)	| dangling `'a` or `T` in drop glue (e.g., `#[may_dangle] Drop`)  |
|---                            |---                |---                    |---                                |---                                                              |
| `PhantomData<T>`              |	-	            | covariant	            | inherited	                        | disallowed ("owns T")                                           |
| `PhantomData<&'a T>`          |	covariant	    | covariant	            | Send + Sync requires T : Sync	    | allowed               |
| `PhantomData<&'a mut T>`      |	covariant	    | invariant	            | inherited	                        | allowed               |
| `PhantomData<*const T>`       |	-	            | covariant	            | !Send + !Sync	                    | allowed               |
| `PhantomData<*mut T>`         |	-	            | invariant	            | !Send + !Sync	                    | allowed               |
| `PhantomData<fn(T)>`          |	-	            | contravariant	        | Send + Sync	                    | allowed               |
| `PhantomData<fn() -> T>`      |	-	            | covariant	            | Send + Sync	                    | allowed               |
| `PhantomData<fn(T) -> T>`     |	-	            | invariant	            | Send + Sync	                    | allowed               |
| `PhantomData<Cell<&'a ()>>`   |	invariant	    | -	                    | Send + !Sync	                    | allowed               |
- `Unique<T>` :
    - wraps a `*const T` for variance
    - includes a `PhantomData<T>`
    - auto-derives `Send`/`Sync` as if `T` was contained
    -  marks the pointer as `NonZero` for the null-pointer optimization
### Splitting Borrows
- mutable slices expose a `split_at_mut` function that consumes the slice and returns two mutable slices.
- linked list, binary tree
