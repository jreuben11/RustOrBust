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

## [type_conversions](type_conversions/src/main.rs)
- coercions
- dot operator
```rust
#[derive(Clone)]
struct ContainerA<T>(Arc<T>);
#[allow(noop_method_call)]
fn clone_containers_a<T>(foo: &ContainerA<i32>, bar: &ContainerA<T>) {
    let _foo_cloned: ContainerA<i32> = foo.clone();
    let _bar_cloned: &ContainerA<T> = bar.clone(); // autoref
}
struct ContainerB<T>(Arc<T>);
impl<T> Clone for ContainerB<T> /* where T: Clone */ {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
fn clone_containers_b<T>(foo: &ContainerB<i32>, bar: &ContainerB<T>) {
    let _foo_cloned: ContainerB<i32> = foo.clone();
    let _bar_cloned: ContainerB<T> = bar.clone();
}
```
- casts
- transmutes
  - `mem::transmute<T, U>` / `mem::transmute_copy<T, U>`

## [unitialized memory](uninitialized_memory/src/main.rs)
```rust
use std::mem::{self, MaybeUninit};
use std::ptr;

fn main() {
    check_unitialized_memory();
    drop_flags();
    unchecked_uninitialized_memory();
}
```
- `MaybeUninit`
- `ptr::write(ptr, val)` - takes a val and moves it into the address pointed to by ptr.
- ptr::copy(src, dest, count) - copies the bits that count T items would occupy from src to dest. (equivalent to C's `memmove`)
- `ptr::copy_nonoverlapping(src, dest, count)` does what copy does, but a little faster on the assumption that the two ranges of memory don't overlap. (equivalent to C's `memcpy`)
```rust
    let x = {
        let mut x: [MaybeUninit<Box<u32>>; SIZE] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for i in 0..SIZE {
            x[i] = MaybeUninit::new(Box::new(i as u32));
        }
        unsafe { mem::transmute::<_, [Box<u32>; SIZE]>(x) }
    };
    dbg!(x);

    let mut uninit = MaybeUninit::<Demo>::uninit();
    let f1_ptr = unsafe { ptr::addr_of_mut!((*uninit.as_mut_ptr()).field) };
    unsafe { f1_ptr.write(true); }
    let _init = unsafe { uninit.assume_init() };
```

## [OBRM](orbm/src/lib.rs) Ownership Based Resource Management 
- switch rust versions for feature flags
```bash
rustup default nightly
rustup update
rustup default stable
```
- ctors
- dtors
```rust
#![allow(dead_code)]
#![allow(internal_features)]
#![feature(ptr_internals, allocator_api)]

use std::alloc::{Allocator, Global, Layout};
use std::mem;
use std::ptr::{drop_in_place, NonNull, Unique};
...
    unsafe {
        let my_box = self.my_box.take().unwrap();
        let c: NonNull<T> = my_box.ptr.into();
        Global.deallocate(c.cast(), Layout::new::<T>());
        mem::forget(my_box);
    }
```
- how to instantiate a `Unique<T>` ???
- Leaking (psuedocode)
  - `vec::Drain` ?
## unwinding (no code)
- Exception Safety
  - `Vec::push_all`
  - `BinaryHeap::sift_up`
  - `struct Hole<'a, T: 'a>`
- Poisoning

## [concurrency and parallelism](concurrency_and_parallelism/src/main.rs)
### data race
```rust
    #![feature(negative_impls)]

    struct X(u8);
    impl !Send for X {}
    impl !Sync for X {}
```
### send and sync
  - `cargo add libc` - posix_memalign, free
```rust
    use std::{
        mem::{align_of, size_of},
        ptr,
    };
    use std::ops::{Deref, DerefMut};
    
    pub struct Carton<T>(ptr::NonNull<T>);
    impl<T> Carton<T> {
        pub fn new(value: T) -> Self {
            assert_ne!(size_of::<T>(), 0, "Zero-sized types are out of the scope of this example");
            let mut memptr: *mut T = ptr::null_mut();
            unsafe {
                let ret = libc::posix_memalign(
                    (&mut memptr).cast(),
                    align_of::<T>(),
                    size_of::<T>()
                );
                assert_eq!(ret, 0, "Failed to allocate or invalid alignment");
            };
            let ptr = {
                ptr::NonNull::new(memptr)
                    .expect("Guaranteed non-null if posix_memalign returns 0")
            };
            unsafe {
                ptr.as_ptr().write(value);
            } 
            Self(ptr)
        }
    }

    impl<T> Deref for Carton<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            unsafe {
                self.0.as_ref()
            }
        }
    }
    
    impl<T> DerefMut for Carton<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                self.0.as_mut()
            }
        }
    }

    impl<T> Drop for Carton<T> {
        fn drop(&mut self) {
            unsafe {
                libc::free(self.0.as_ptr().cast());
            }
        }
    }

    unsafe impl<T> Send for Carton<T> where T: Send {}
    unsafe impl<T> Sync for Carton<T> where T: Sync {}
```
### atomic spin_locking
```rust
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    // use std::thread;
    
        let lock = Arc::new(AtomicBool::new(false)); // value answers "am I locked?"
        // ... distribute lock to threads somehow ...
        // Try to acquire the lock by setting it to true
        while !lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire).is_err(){}
        // broke out of the loop, so we successfully acquired the lock!
        // ... scary data accesses ...
        // ok we're done, release the lock
        lock.store(false, Ordering::Release);
```

## [My Vec](my_vec/src/lib.rs)
- structure:
```rust
// RawVec
struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}
impl<T> RawVec<T> {
    fn new() -> Self {...}
    fn grow(&mut self) {...}
}

// Vec
pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}
impl<T> Drop for Vec<T> {
    fn drop(&mut self) {...}
}
impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {...}
}
impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {...}
}
impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {...}
}

// RawValIter
struct RawValIter<T> {
    start: *const T,
    end: *const T,
}
impl<T> RawValIter<T> {
    unsafe fn new(slice: &[T]) -> Self {...}
}
impl<T> Iterator for RawValIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {...}
    fn size_hint(&self) -> (usize, Option<usize>) {...}
}
impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<T> {...}
}

// IntoIter
pub struct IntoIter<T> {
    _buf: RawVec<T>, // we don't actually care about this. Just need it to live.
    iter: RawValIter<T>,
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {...}
    fn size_hint(&self) -> (usize, Option<usize>) {...}
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {...}
}
impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {...}
}

// Drain
pub struct Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}
impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {...}
    fn size_hint(&self) -> (usize, Option<usize>) {...}
}
impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> {...}
}
impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {..}
}

```
- imports:
```rust
use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};
```
- mechanisms:
- `mem::size_of::<T>()` - `!0` is `usize::MAX`.
- `NonNull::dangling()` doubles as "unallocated" and "zero-sized allocation"
- `Layout::array::<T>`
- `PhantomData<&'a mut Vec<T>>`
- `mem::forget(self);` - used in `IntoIterator`
```rust
let mut cap: usize = if mem::size_of::<T>() == 0 { !0 } else { 0 };
let mut ptr: NonNull<T> = NonNull::dangling();
let val: Option<T> = Some(ptr::read(ptr.as_ptr()));
```
```rust
let new_ptr = if self.cap == 0 {
    unsafe { alloc::alloc(new_layout) }
} else {
    let old_layout = Layout::array::<T>(self.cap).unwrap();
    let old_ptr = self.ptr.as_ptr() as *mut u8;
    unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
};

// If allocation fails, `new_ptr` will be null, in which case we abort.
self.ptr = match NonNull::new(new_ptr as *mut T) {
    Some(p) => p,
    None => alloc::handle_alloc_error(new_layout),
};
```
```rust
unsafe {
    alloc::dealloc(
        self.ptr.as_ptr() as *mut u8,
        Layout::array::<T>(self.cap).unwrap(),
    );
}
```
```rust
let (iter, buf) = unsafe {
    (RawValIter::new(&self), ptr::read(&self.buf))
};
mem::forget(self);
```

## [My Arc](my_arc/src/lib.rs)
- `std::sync::Arc`
- structure:
```rust
pub struct Arc<T> {
    ptr: NonNull<ArcInner<T>>,
    phantom: PhantomData<ArcInner<T>>,
}
pub struct ArcInner<T> {
    rc: AtomicUsize,
    data: T,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {...}
}

unsafe impl<T: Sync + Send> Send for Arc<T> {}
unsafe impl<T: Sync + Send> Sync for Arc<T> {}

impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &T {...}
}
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Arc<T> {...}
}
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {...}
}
```
- imports:
```rust
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{self, AtomicUsize, Ordering};
```
- `let rc: atomic::AtomicUsize = AtomicUsize::new(1)` - the reference count
  - `rc.fetch_add(1, Ordering::Relaxed)`
  - `rc.fetch_sub(1, Ordering::Release)`
- `ptr: NonNull<ArcInner<T>>`
  - `unsafe { ptr.as_ref() }`
- `let phantom: PhantomData<ArcInner<T>> = NonNull::new(Box::into_raw(boxed)).unwrap()`
- `T: Sync + Send`
- `std::process::abort()`
- `atomic::fence(Ordering::Acquire)`