source: https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust
# VSCode toolage
- plugin https://github.com/rust-lang/rls-vscode
- Rust Language Server (RLS) https://github.com/rust-lang/rls-vscode - installed via rustup https://rustup.rs/
- Debugging support - LLDB frontend plugin https://github.com/vadimcn/vscode-lldb
# 1. refresher
- [trait mixin + test refresher](ch1_refresher/src/door.rs)
  - `struct`, `impl`
  - tests:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "xxx")]
    fn do_something() {
        assert!(...);
    }
}
```
- `Option<T>`, `Result<T, E>` - `if let Some`, `match .. Some .. None`, `match .. Ok .. Err`
- `vec![]` macro:
```rust
#[macro_export]
macro_rules!
 vec {
     ( $( $x:expr ),* ) => {
         {             
	     let mut temp_vec = Vec::new();              
            $( temp_vec.push($x); )*      
            temp_vec
         }     
    }; 
}
```
- `#[derive(Clone, Debug)]` - `"{:?}"`
- `Box`, `Rc`
- `&x`, `mut`, `clone`
```rust
fn f<'a>(mut pass_through: MyStruct<'a>, x: &'a Vec<u32>) -> MyStruct<'a> {
```
- multiple owners:
```rust
use std::rc::Rc;

#[derive(Debug)]
struct FileName {
    name: Rc<String>,
    ext: Rc<String> 
}

fn ref_counter() {
    let name = Rc::new(String::from("main"));
    let ext = Rc::new(String::from("rs")));

    for _ in 0..3 {
        println!("{;?}", FileName {
                    name: name.clone(), 
                    ext: ext.clone() 
        });
    }
}
```
- `RefCell` interior mutability - maintains single ownership of a value but allows mutable borrowing checked at runtime. Instead of compiler errors, violating rules of borrowing will lead to a runtime `panic!`, crashing the program. used in combination with `Rc` to provide a value to multiple owners with mutability. `borrow_mut()` - mutable reference only lives as long as the assignment
```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}


type Link = Option<Rc<RefCell<Node>>>;

pub fn append(&mut self, value: String) {
    let new = Rc::new(RefCell::new(Node::new(value)));
    match self.tail.take() {
        Some(old) => {
            old.borrow_mut().next = Some(new.clone());
            new.borrow_mut().prev = Some(old);
        }
        None => self.head = Some(new.clone()),
    };
}
```
- [multithreading refresher](ch1_refresher/src/main.rs) 
```rust
use std::thread; 
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
...
fn main() {
    threading();
    shared_state();
    channels();
    ref_counter();
}
```
- marker traits:
  - `Send`: type is safe to send (`move`) from one thread to the other
  - `Sync`: type can be shared across threads without `Mutex`
# 2. cargo refreher
- [doc tests](ch2_crate_test/src/lib.rs)
- [cargo config](.cargo/config)
- [cargo workspace](Cargo.toml)
- cargo flags:
  - `--version` 
  - `--list`              List installed commands 
  - `--explain <CODE>`    Run `rustc --explain CODE` 
  - `--verbose`          (`-vv` very verbose `build.rs` output)
  - `--quiet` 
  - `--color <WHEN>`      
  - `--frozen`            Require `Cargo.lock` and cache are up to date 
  - `--locked`            Require `Cargo.lock` is up to date 
  - `-Z <FLAG>...`        Unstable (nightly-only) flags to Cargo
- cargo commands:
  -  `build`       Compile
  -  `check`       Analyze, but don't build object files 
  -  `clean`       Remove `target` directory 
  -  `doc`         Build project + its dependencies' docs
  -  `new`         Create a new cargo project 
  -  `init`        Create a new cargo project in an existing directory 
  -  `run`         Build and execute `src/main.rs` 
  -  `test`        Run tests 
  -  `bench`       Run benchmarks - `#[bench]` is nightly unstable in RUst 2021
  -  `update`      Update dependencies listed in `Cargo.lock` 
  -  `search`      Search registry for crates 
  -  `publish`     Package as ***.crate** file and upload project to https://crates.io 
  -  `yank`        prohibit updates to a particular published version
  -  `install`     Install a Rust binary 
  -  `uninstall`   Uninstall a Rust binary
-  **.cargo/config** file: https://doc.rust-lang.org/cargo/reference/config.html
   -  `paths` - local repos
   -  `[cargo-new]` - commandline args
   -  `[registry]` - remote repos
   -  `[http]` - proxy address + port, `cainfo`, `timeout`
   -  `[build]` - number of `jobs`, binaries (eg **rustc**, **rustdoc**), `incremental` compilation 
- **Cargo.lock** https://doc.rust-lang.org/cargo/faq.html provide deterministic builds of dependency versions. architecture specific. frozen unless you run `cargo update`
- **Cargo.toml** manifest
  - `package.metadata`
  -  `[target.$triple]` 
      -  eg `[target.wasm32-unknown-unknown]` for Wasm target via LLVM backend - has to be installed: `rustup target add wasm32-unknown-unknown`. **wasm-bindgen** https://github.com/rustwasm/wasm-bindgen
      -  specify `linker`, `ar` (archiver), `runner`, compiler `rustflags`
      -  compiled to **target/<profile>/<target-triple>/**
      -  profiles:
        - `[profile.dev]` -> `cargo build`
        -  `[profile.release]` ->  `cargo build --release`
        -  `[profile.test]` -> , `cargo test`
        -  `[profile.bench]` -> `cargo bench` 
        - defaults:
```toml
[profile.release] 
 opt-level = 3 
 debug = false 
 rpath = false 
 lto = false 
 debug-assertions = false
 codegen-units = 16
 panic = 'unwind'
 incremental = false
 overflow-checks = false
```
  - `[dependencies]`
```toml
[dependencies]
# import in Rust with "use x::*"
x = "*"
# (~): Only semver patch increases allowed
# (^): only semver minor increases allowed
# (*): semver major increases allowed. Note: isn't possible to cargo publish a crate with wildcard dependencies
y = { version = "0.5", optional = true } 
v = { git = "https://github.com/vvv", branch = "0.4" }
# import in Rust with "use z::*"
[dependencies.z]
version = "0.5"
features = ["www"] # only install subcomponents

[dev-dependencies]

[build-dependencies]

```
  - `links` -  native libs to be linked dynamically
- `build` - path or name to a build script https://doc.rust-lang.org/cargo/reference/build-scripts.html any output for cargo is passed via `cargo:key=value` format
- place each subproject in a subdirectory and create a `workspace` via **Cargo.toml** :
```toml
[workspace] 
members = [ "core", "web", "data"]
[build-dependencies] # no other dependencies allowed

```
- `rustup component add clippy-preview` https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands
- linking https://doc.rust-lang.org/reference/linkage.html 
  - `--crate-format=` `rlib`, `dylib`, `staticlib`, `library`, or `bin`
  - Static: Via `rlib` format (default native package format). for static libs, all dependencies have to be of rlib type
  - Dynamic: Via shared libraries (`.so` or `.dll`) `rustc -C prefer-dynamic`
- FFI
```rust
extern "C" {
    fn imported_function() -> i32; # must be bound via the linker. call must be wrapped in an `unsafe` section
}

#[no_mangle]
pub extern "C" fn exported_function() -> i32 {      # no_mangle -> can be found using its name
    42
}
```
# 3. efficient storage refresher
- stack - allows for zero overhead structures
- heap - Types that don't have predictable sizes. objects wrapped into `Rc`, `Cell`, `RefCell`, or `Box` instances
- store fixed-size reference `&str` to `String` heap-allocated value, along with metadata for length in bytes. Similar to pointers, a fixed-size view into a previously-unsized value. 
- [mem::size_of](ch3_efficient_storage/src/main.rs)
- generics - equivelent:
```rust
fn f<T: MyTrait>(t: T) {}
fn f <T>(t: T) where T: MyTrait {}
fn f(t: impl MyTrait) {} 
```
  - `fn f(x: T) where T: Clone + Debug + MyTrait {}`
  - Generic type parameters are `Sized` by default `fn f <T: Sized>(t: T) {}` - marker trait for mostly stack-allocated values is the default, can be relaxed to `?Sized`
- memory leaks are still possible in Rust via `Rc`
- `Box<dyn TheTrait>` put a reference to a trait inside of a struct. dynamic dispatch: location is only known at runtime - vtable lookup is slower. use concrete types instead of traits to avoid multiple dereference operations
- `Copy` via assignment for sized stack values is an implicit, bitwise copy. implement or derive `#derive[Clone]` for unsized heap refs `clone()`
- `std::borrow::Cow`  lazily clones a contained reference whenever mutation or ownership is requested https://doc.rust-lang.org/std/borrow/enum.Cow.html
- Rust Persistent Data Structures (RPDS) https://crates.io/crates/rpds utilize copy-on-write with versioning to capture state changes. Persistent data structures: keep original data immutable and store versioned change sets.
# 4. lists
- no `unsafe {}` -> code gets riddled with `RefCells` and `borrow()` to create data structures that the borrow checker passes 
- Rust `LinkedList` uses much unsafe code internally. `PhantomData<T>` zero-size type - informs compiler about generic types (drop behavior, sizes)
- recursive nested `next` pointers can exceed the stack -> stack overflows with autogenerated `Drop`, `Debug` -> implement `Drop` trait `drop()` and dispose of list elements iteratively. 
- [bencher](ch4_lists/benches/example.rs)
- [tests](ch4_lists/src/lib.rs)
- [single linked list](ch4_lists/src/singly_linked_list.rs)
  - `type Link = Option<Rc<RefCell<Node>>>`
  - `String.to_owned`
  - `borrow` / `borrow_mut`
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
- [skip list](ch4_lists/src/skip_list.rs) `O(log n)` probabilistic search on a linked list
```rust
#[derive(Clone)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub value: String,
}

#[derive(Clone)]
pub struct MySkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

fn get_level(&self) -> usize {
    let mut n = 0;
    // bool = p(true) = 0.5
    while rand::random::<bool>() && n < self.max_level {
        n += 1;
    }
    n
}
```
- [dynamic array](ch4_lists/src/dynamic_array.rs)
  - `Box<[Node]>`, `vec!` `into_boxed_slice`, `clone_from_slice`
# 5. Trees
- [tests](ch5_trees/src/lib.rs)
  - **binary_search_tree_walk_in_order**
  - **binary_search_tree_find**
  - **red_black_tree_add**
  - **red_black_tree_walk_in_order**
  - **red_black_tree_find**
  - **binary_heap_add**
  - **binary_heap_pop**
  - **trie_add**
  - **trie_walk_in_order**
  - **trie_find**
  - 
- [binary search tree](ch5_trees/src/binary_search_tree.rs)
  - `mem::replace`
  - pass callback and  build a vector by walking tree:
```rust
 walk(&self, callback: impl Fn(&T) -> ()) {   
    self.walk_in_order(&self.root, &callback);
}

fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&T) -> ()) {
    if let Some(n) = node {
        self.walk_in_order(&n.left, callback);
        callback(&n.dev);
        self.walk_in_order(&n.right, callback);
    }
}

let items: RefCell<Vec<T>> = RefCell::new(vec![]);
tree.walk(|n|items.borrow_mut().push(n.clone()));
```
  - test with rand
```rust
    use rand::thread_rng;
    use rand::seq::SliceRandom;
    use rand::Rng;
    ...
        let mut items: Vec<T> = (0..len).map(new_item_with_id).collect();

        // let mut rng = thread_rng();
        // rng.shuffle(&mut items);
        items.shuffle(&mut thread_rng());

        for item in items.iter() {
            tree.add(item.clone());
        }

        assert_eq!(tree.length, len);
        let v: RefCell<Vec<T>> = RefCell::new(vec![]);
        tree.walk(|n| v.borrow_mut().push(n.clone()));
        let mut items = items;
        // sort in descending order:
        items.sort_by(|a, b| b.numerical_id.cmp(&a.numerical_id));
        assert_eq!(v.into_inner(), items)
```
- [red-black tree](ch5_trees/src/red_black_tree.rs)
- [heap](ch5_trees/src/heap.rs)
  - `Vec<T>.swap_remove()` - remove 1st element of by replacing it with last element
- [trie](ch5_trees/src/trie.rs)
