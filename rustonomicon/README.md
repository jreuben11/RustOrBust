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