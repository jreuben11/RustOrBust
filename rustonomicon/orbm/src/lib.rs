#![allow(dead_code)]
#![allow(internal_features)]
#![feature(ptr_internals, allocator_api)]

struct Foo {
    a: u8,
    b: u32,
    c: bool,
}
enum Bar {
    X(u32),
    Y(bool),
}
struct Unit;

pub fn ctors_demo(){
    let _foo = Foo { a: 0, b: 1, c: false };
    let _bar = Bar::X(0);
    let _empty = Unit;
}

mod dtors{
    use std::alloc::{Allocator, Global, Layout};
    use std::mem;
    use std::ptr::{drop_in_place, NonNull, Unique};
    
    struct Box<T>{ ptr: Unique<T> }
    impl<T> Drop for Box<T> {
        fn drop(&mut self) {
            unsafe {
                drop_in_place(self.ptr.as_ptr());
                let c: NonNull<T> = self.ptr.into();
                Global.deallocate(c.cast(), Layout::new::<T>())
            }
        }
    }

    struct SuperBox<T> { my_box: Option<Box<T>> }
    impl<T> Drop for SuperBox<T> {
        fn drop(&mut self) {
            unsafe {
                // Hyper-optimized: deallocate the box's contents for it
                // without `drop`ing the contents. Need to set the `box`
                // field as `None` to prevent Rust from trying to Drop it.
                let my_box = self.my_box.take().unwrap();
                let c: NonNull<T> = my_box.ptr.into();
                Global.deallocate(c.cast(), Layout::new::<T>());
                mem::forget(my_box);
            }
        }
    }

    struct Boxy<T> {
        data1: Box<T>,
        data2: Box<T>,
        info: u32,
    }
    enum Link {
        Next(Box<Link>),
        None,
    }


}

