#![allow(dead_code)]

mod call_c{
    use libc::{c_int, size_t};

    #[link(name = "snappy")]
    extern {
        fn snappy_compress(input: *const u8,
                        input_length: size_t,
                        compressed: *mut u8,
                        compressed_length: *mut size_t) -> c_int;
        fn snappy_uncompress(compressed: *const u8,
                            compressed_length: size_t,
                            uncompressed: *mut u8,
                            uncompressed_length: *mut size_t) -> c_int;
        pub fn snappy_max_compressed_length(source_length: size_t) -> size_t;
        fn snappy_uncompressed_length(compressed: *const u8,
                                    compressed_length: size_t,
                                    result: *mut size_t) -> c_int;
        fn snappy_validate_compressed_buffer(compressed: *const u8,
                                            compressed_length: size_t) -> c_int;
    }

    pub fn validate_compressed_buffer(src: &[u8]) -> bool {
        unsafe {
            snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0
        }
    }

    pub fn compress(src: &[u8]) -> Vec<u8> {
        unsafe {
            let srclen = src.len() as size_t;
            let psrc = src.as_ptr();

            let mut dstlen = snappy_max_compressed_length(srclen);
            let mut dst = Vec::with_capacity(dstlen as usize);
            let pdst = dst.as_mut_ptr(); // mut pointer to Vec data
            // snappy_compress(
            // input: *const u8,
            // input_length: size_t, 
            // compressed: *mut u8, 
            // compressed_length: *mut size_t
            // ) -> c_int;
            snappy_compress(psrc, srclen, pdst, &mut dstlen);
            dst.set_len(dstlen as usize);
            dst
        }
    }

    pub fn uncompress(src: &[u8]) -> Option<Vec<u8>> {
        unsafe {
            let srclen = src.len() as size_t;
            let psrc = src.as_ptr();

            let mut dstlen: size_t = 0;
            snappy_uncompressed_length(psrc, srclen, &mut dstlen);

            let mut dst = Vec::with_capacity(dstlen as usize);
            let pdst = dst.as_mut_ptr();

            if snappy_uncompress(psrc, srclen, pdst, &mut dstlen) == 0 {
                dst.set_len(dstlen as usize);
                Some(dst)
            } else {
                None // SNAPPY_INVALID_INPUT
            }
        }
    }
}



#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}
// gcc call_rust.c -o call_rust -lffi -L./target/debug
// LD_LIBRARY_PATH=./target/debug ./call_rust


use std::ffi::CString;
use std::ptr;
#[link(name = "readline")]
extern {
    static rl_readline_version: libc::c_int;
    static mut rl_prompt: *const libc::c_char;
}



// ffi@0.1.0: Compiler version doesn't include clang or GCC: "cc" "--version"
// ffi@0.1.0: Compiler version doesn't include clang or GCC: "c++" "--version"
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_snappy() {
        let x = unsafe { call_c::snappy_max_compressed_length(100) };
        assert_eq!(x, 148); // max compressed length of a 100 byte buffer
        
    }

    // TODO: these tests fail due to linker errors: "undefined reference to `operator delete(void*)'""
    // #[test]
    // fn valid() {
    //     let d = vec![0xde, 0xad, 0xd0, 0x0d];
    //     let c: &[u8] = &compress(&d);
    //     assert!(validate_compressed_buffer(c));
    //     assert!(uncompress(c) == Some(d));
    // }

    // #[test]
    // fn invalid() {
    //     let d = vec![0, 0, 0, 0];
    //     assert!(!validate_compressed_buffer(&d));
    //     assert!(uncompress(&d).is_none());
    // }

    // #[test]
    // fn empty() {
    //     let d = vec![];
    //     assert!(!validate_compressed_buffer(&d));
    //     assert!(uncompress(&d).is_none());
    //     let c = compress(&d);
    //     assert!(validate_compressed_buffer(&c));
    //     assert!(uncompress(&c) == Some(d));
    // }

    #[test]
    fn call_readline() {
        let version = unsafe { rl_readline_version as i32 };
        assert!(version > 0);

        let prompt = CString::new("[my-awesome-shell] $").unwrap();
        unsafe {
            rl_prompt = prompt.as_ptr();
            println!("{:?}", rl_prompt);
            rl_prompt = ptr::null();
        }

    }
}
