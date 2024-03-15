source: https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust
# 1. refresher
- [sync::mpsc](ch1_refresher/src/main.rs)
# 2. cargo refreher
- [doc tests](ch2_crate_test/src/lib.rs)
- [cargo config](.cargo/config)
- [cargo workspace](Cargo.toml)
# 3. efficient storage refresher
- [mem::size_of](ch3_efficient_storage/src/main.rs)
- `Sized` / `?Sized`
# 4. lists
- [bencher](ch4_lists/benches/example.rs)
- [tests](ch4_lists/src/lib.rs)
- [single linked list](ch4_lists/src/singly_linked_list.rs)
  - `type Link = Option<Rc<RefCell<Node>>>`
  - `String.to_owned`
  - `borrow_mut`
  - `Rc::try_unwrap(x)
                .ok()
                .expect("xxx")
                .into_inner()
                .value`
  - LinkedList
  - `Option.take()` https://doc.rust-lang.org/std/option/enum.Option.html#method.take 
  - `mem::replace()` https://doc.rust-lang.org/stable/std/mem/fn.replace.html
- [double linked list](ch4_lists/src/doubly_linked_list.rs)
  - implementing `iter` https://doc.rust-lang.org/std/iter/index.html#implementing-iterator `Iterator`, `IntoIterator`, `DoubleEndedIterator`
  - borrow
