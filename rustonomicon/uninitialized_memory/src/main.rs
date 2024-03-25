use std::mem::{self, MaybeUninit};
use std::ptr;

fn main() {
    check_unitialized_memory();
    drop_flags();
    unchecked_uninitialized_memory();
}

fn check_unitialized_memory(){
    let x: i32;
    if true {
        x = 1;
    } else {
        x = 2;
    }
    println!("{}", x);

    let x: i32;
    if true {
        x = 1;
        println!("{}", x);
    }

    let _x: i32;
    loop {
        // Rust doesn't understand that this branch will be taken unconditionally,
        // because it relies on actual values.
        if true {
            // But it does understand that it will only be taken once because
            // we unconditionally break out of it. Therefore `x` doesn't
            // need to be marked as mutable.
            _x = 0;
            break;
        }
    }

    let x = 0;
    let y = Box::new(0);
    let _z1 = x; // x is still valid because i32 is Copy
    let _z2 = y; // y is now logically uninitialized because Box isn't Copy

    let mut _y = Box::new(0);
    let _z = _y; // y is now logically uninitialized because Box isn't Copy
    _y = Box::new(1); // reinitialize y
}

fn drop_flags(){
    {
        let mut x = Box::new(0); // x was uninit; just overwrite.
        let mut _y = x;           // y was uninit; just overwrite and make x uninit.
        x = Box::new(0);         // x was uninit; just overwrite.
        _y = x;                   // y was init; Drop y, overwrite it, and make x uninit!
                                // y goes out of scope; y was init; Drop y!
                                // x goes out of scope; x was uninit; do nothing.
    }
    {
        let mut _x = Box::new(0);    // x was uninit; just overwrite.
        if 1 < 0 {
            drop(_x)                 // x gets moved out; make x uninit.
        } else {
            println!("{}", _x);
            drop(_x)                 // x gets moved out; make x uninit.
        }
        _x = Box::new(0);            // x was uninit; just overwrite.
                                    // x goes out of scope; x was init; Drop x!
    }
}


struct Demo {
    field: bool,
}

fn unchecked_uninitialized_memory(){
    // Size of the array is hard-coded but easy to change (meaning, changing just
    // the constant is sufficient). This means we can't use [a, b, c] syntax to
    // initialize the array, though, as we would have to keep that in sync
    // with `SIZE`!
    const SIZE: usize = 10;

    let x = {
        // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
        // safe because the type we are claiming to have initialized here is a
        // bunch of `MaybeUninit`s, which do not require initialization.
        let mut x: [MaybeUninit<Box<u32>>; SIZE] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
        // assignment instead of `ptr::write` does not cause the old
        // uninitialized value to be dropped.
        // Exception safety is not a concern because Box can't panic
        for i in 0..SIZE {
            x[i] = MaybeUninit::new(Box::new(i as u32));
        }

        // Everything is initialized. Transmute the array to the initialized type.
        unsafe { mem::transmute::<_, [Box<u32>; SIZE]>(x) }
    };

    dbg!(x);

    let mut uninit = MaybeUninit::<Demo>::uninit();
    // `&uninit.as_mut().field` would create a reference to an uninitialized `bool`,
    // and thus be Undefined Behavior!
    let f1_ptr = unsafe { ptr::addr_of_mut!((*uninit.as_mut_ptr()).field) };
    unsafe { f1_ptr.write(true); }
    
    let _init = unsafe { uninit.assume_init() };
}


