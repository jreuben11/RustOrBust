#![allow(dead_code)]
use std::mem::size_of;

// repr(Rust)
struct A {
    a: u8,
    b: u32,
    c: u16,
}
struct APadded {
    a: u8,
    _pad1: [u8; 3], // to align `b`
    b: u32,
    c: u16,
    _pad2: [u8; 2], // to make overall size multiple of 4
}

// DST - Dynamically Sized Types
struct MySuperSliceable<T: ?Sized> {
    info: u32,
    data: T,
}

// ZST - Zero Sized Types
struct Nothing; // No fields = no size
// All fields have no size = no size
struct LotsOfNothing {
    foo: Nothing,
    qux: (),      // empty tuple has no size
    baz: [u8; 0], // empty array has no size
}

// Empty Types
enum Void {} // No variants = EMPTY

// Alternative representations
enum MyOption<T> {
    Some(T),
    None,
}
#[repr(u8)]
enum MyReprOption<T> {
    Some(T),
    None,
}

fn main() {
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

}
