#![allow(dead_code)]
#![feature(negative_impls)]

use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn race(){
    let data = vec![1, 2, 3, 4];
    // Arc so that the memory the AtomicUsize is stored in still exists for
    // the other thread to increment, even if we completely finish executing
    // before it. Rust won't compile the program without it, because of the
    // lifetime requirements of thread::spawn!
    let idx = Arc::new(AtomicUsize::new(0));
    let other_idx = idx.clone();

    // `move` captures other_idx by-value, moving it into this thread
    thread::spawn(move || {
        // It's ok to mutate idx because this value
        // is an atomic, so it can't cause a Data Race.
        other_idx.fetch_add(10, Ordering::SeqCst);
    });
    // Index with the value loaded from the atomic. This is safe because we
    // read the atomic memory only once, and then pass a copy of that value
    // to the Vec's indexing implementation. This indexing will be correctly
    // bounds checked, and there's no chance of the value getting changed
    // in the middle. However our program may panic if the thread we spawned
    // managed to increment before this ran. A race condition because correct
    // program execution (panicking is rarely correct) depends on order of
    // thread execution.
    println!("{}", data[idx.load(Ordering::SeqCst)]);
    if idx.load(Ordering::SeqCst) < data.len() {
        unsafe {
            // Incorrectly loading the idx after we did the bounds check.
            // It could have changed. This is a race condition, *and dangerous*
            // because we decided to do `get_unchecked`, which is `unsafe`.
            println!("{}", data.get_unchecked(idx.load(Ordering::SeqCst)));
        }
    }
}

mod send_and_sync {
    struct MyBox(*mut u8);
    unsafe impl Send for MyBox {}
    unsafe impl Sync for MyBox {}

    // I have some magic semantics for some synchronization primitive!
    struct SpecialThreadToken(u8);
    impl !Send for SpecialThreadToken {}
    impl !Sync for SpecialThreadToken {}

    use std::{
        mem::{align_of, size_of},
        ptr,
    };
    use std::ops::{Deref, DerefMut};
    
    pub struct Carton<T>(ptr::NonNull<T>);
    
    impl<T> Carton<T> {
        pub fn new(value: T) -> Self {
            // Allocate enough memory on the heap to store one T.
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
    
            // NonNull is just a wrapper that enforces that the pointer isn't null.
            let ptr = {
                // Safety: memptr is dereferenceable because we created it from a
                // reference and have exclusive access.
                ptr::NonNull::new(memptr)
                    .expect("Guaranteed non-null if posix_memalign returns 0")
            };
    
            // Move value from the stack to the location we allocated on the heap.
            unsafe {
                // Safety: If non-null, posix_memalign gives us a ptr that is valid
                // for writes and properly aligned.
                ptr.as_ptr().write(value);
            } 
            Self(ptr)
        }
    }

    impl<T> Deref for Carton<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            unsafe {
                // Safety: The pointer is aligned, initialized, and dereferenceable
                //   by the logic in [`Self::new`]. We require readers to borrow the
                //   Carton, and the lifetime of the return value is elided to the
                //   lifetime of the input. This means the borrow checker will
                //   enforce that no one can mutate the contents of the Carton until
                //   the reference returned is dropped.
                self.0.as_ref()
            }
        }
    }
    
    impl<T> DerefMut for Carton<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                // Safety: The pointer is aligned, initialized, and dereferenceable
                //   by the logic in [`Self::new`]. We require writers to mutably
                //   borrow the Carton, and the lifetime of the return value is
                //   elided to the lifetime of the input. This means the borrow
                //   checker will enforce that no one else can access the contents
                //   of the Carton until the mutable reference returned is dropped.
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

    // Safety: No one besides us has the raw pointer, so we can safely transfer the
    // Carton to another thread if T can be safely transferred.
    unsafe impl<T> Send for Carton<T> where T: Send {}

    // Safety: Since there exists a public way to go from a `&Carton<T>` to a `&T`
    // in an unsynchronized fashion (such as `Deref`), then `Carton<T>` can't be
    // `Sync` if `T` isn't.
    // Conversely, `Carton` itself does not use any interior mutability whatsoever:
    // all the mutations are performed through an exclusive reference (`&mut`). This
    // means it suffices that `T` be `Sync` for `Carton<T>` to be `Sync`:
    unsafe impl<T> Sync for Carton<T> where T: Sync  {}
}

mod atomics {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    // use std::thread;

    pub fn spin_locking() {
        let lock = Arc::new(AtomicBool::new(false)); // value answers "am I locked?"
        // ... distribute lock to threads somehow ...
        // Try to acquire the lock by setting it to true
        while !lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire).is_err(){}
        // broke out of the loop, so we successfully acquired the lock!
        // ... scary data accesses ...
        // ok we're done, release the lock
        lock.store(false, Ordering::Release);
    }
}

fn main() {
    race();
    // let _c = send_and_sync::Carton::new(1); // -> assertion `left == right` failed: Failed to allocate or invalid alignment
    // let _c = send_and_sync::Carton::new(String::from("xxx")); // -> 992747 segmentation fault (core dumped)  cargo run
    atomics::spin_locking();
}
