# Bindgen
https://rust-lang.github.io/rust-bindgen/

## Library Usage with build.rs

[Cargo.toml](Cargo.toml)
```toml
[build-dependencies]
bindgen = "*"
```

[include/wrapper.h](include/wrapper.h)
```C
#include <bzlib.h>
```

[build.rs](build.rs) - `build_shared_lib`
- **cargo:rustc-link-search** - absolute path to look for shared libraries
- **cargo:rustc-link-lib** - link to lib: if `X`, then links to `libX.a`
- **cargo:rerun-if-changed** - rerun build if header file changed
```rust
let bindings = bindgen::Builder::default()
    .header("include/wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("...");
let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("...");
```

[src/lib.rs](src/lib.rs)
```rust
unsafe {
    let input = include_str!("XXX.txt").as_bytes();
    let mut compressed_output: Vec<u8> = vec![0; input.len()];
    let mut decompressed_output: Vec<u8> = vec![0; input.len()];

    // mutable C-type object to construct in C - memory is zeroed
    let mut stream: bz_stream = mem::zeroed();
    // instantiate
    let result = BZ2_bzCompressInit(&mut stream as *mut _, ...); 
    // match error codes 
    match result {
        r if r == (BZ_CONFIG_ERROR as _) => panic!("BZ_CONFIG_ERROR"),
        ...
        r if r == (BZ_OK as _) => {},
        r => panic!("Unknown return value = {}", r),
    }

    // populate input ref `as_ptr` + its len, output `as_mut_ptr` + its len
    stream.next_in = input.as_ptr() as *mut _;
    stream.avail_in = input.len() as _;
    stream.next_out = compressed_output.as_mut_ptr() as *mut _;
    stream.avail_out = compressed_output.len() as _;

    // Compress `input` into `compressed_output`.
    let result = BZ2_bzCompress(&mut stream as *mut _, BZ_FINISH as _);
    match result { ...
    }

    // Finish the compression stream.
    let result = BZ2_bzCompressEnd(&mut stream as *mut _);
    ...

    // Construct a decompression stream - shadowed
    let mut stream: bz_stream = mem::zeroed(); 
    let result = BZ2_bzDecompressInit(&mut stream as *mut _, ...); 
    ...

    // Decompress `compressed_output` into `decompressed_output`.
    ...
    let result = BZ2_bzDecompress(&mut stream as *mut _);
    ...

    // Close the decompression stream.
    let result = BZ2_bzDecompressEnd(&mut stream as *mut _);
    ...

    assert_eq!(input, &decompressed_output[..]);
}

```

## Bindings for non-system libs
[build.rs](build.rs) - `build_static_lib`
- `std::process::Command::new("...").arg("...")...output().expect("...").status.success()`:
  - Run `clang` to compile the `hello.c` file into a `hello.o` object file
  - Run `ar` to generate the `libhello.a` file from the `hello.o` file. 
- `.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))`

## generate bindings from the commandline
```bash
cargo install bindgen-cli
ll ~/.cargo/bin
bindgen include/input.h -o src/bindings.rs
```
```C
typedef struct CoolStruct {
    int x;
    int y;
} CoolStruct;
```
-->
```rust
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CoolStruct {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
```
```C
void cool_function(int i, char c, CoolStruct* cs);
```
-->
```rust
extern "C" {
    pub fn cool_function(i: ::std::os::raw::c_int, c: ::std::os::raw::c_char, cs: *mut CoolStruct);
}
```