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
```