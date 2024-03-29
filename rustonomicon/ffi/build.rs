extern crate cmake;

use std::env;
use std::fs;

use cmake::Config;

fn main() {
	let src = env::current_dir().unwrap().join("snappy");

	let out = Config::new("snappy")
		.define("CMAKE_VERBOSE_MAKEFILE", "ON")
		.build_target("snappy")
		.build();

	let build = out.join("build");

	// NOTE: the cfg! macro doesn't work when cross-compiling, it would return values for the host
	let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS is set by cargo.");
	let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("CARGO_CFG_TARGET_ENV is set by cargo.");

	fs::copy(src.join("snappy.h"), build.join("snappy.h")).unwrap();
    println!("cargo:rustc-link-lib=dylib=stdc++");
	println!("cargo:rustc-link-search=native={}", build.display());
	println!("cargo:rustc-link-lib=static=snappy");
	println!("cargo:include={}", build.display());
    println!("{}----{}", target_os, target_env);

}