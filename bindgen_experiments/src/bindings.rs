/* automatically generated by rust-bindgen 0.69.4 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CoolStruct {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CoolStruct() {
    const UNINIT: ::std::mem::MaybeUninit<CoolStruct> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<CoolStruct>(),
        8usize,
        concat!("Size of: ", stringify!(CoolStruct))
    );
    assert_eq!(
        ::std::mem::align_of::<CoolStruct>(),
        4usize,
        concat!("Alignment of ", stringify!(CoolStruct))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CoolStruct),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CoolStruct),
            "::",
            stringify!(y)
        )
    );
}
extern "C" {
    pub fn cool_function(i: ::std::os::raw::c_int, c: ::std::os::raw::c_char, cs: *mut CoolStruct);
}
