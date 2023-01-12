//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.
//#![feature(cfg_target_abi)]
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const ROOT_FOLDER : &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let _root_folder_path = PathBuf::from(ROOT_FOLDER);
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var_os("TARGET").unwrap();
    /*
    C library compilation
     */



    let mut build_conf = cc::Build::new();
    build_conf
        .define("CYT2B75CAE", None)
        .define("NDEBUG", None)
        .define("tviibe2m", None)
        .cpp(false)
        .flag("-ffunction-sections")
        .flag("-nostdlib")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-function")
        .flag("-Wno-expansion-to-defined");

    /*
    To print the flags used by compiler.
     */
    // let tool = build_conf.get_compiler();
    // let command_string = tool.path();
    // println!("cargo:warning={:?}",command_string);
   
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let mut file_mem = File::create(out.join("memory.x")).unwrap();
    if target == "thumbv7em-none-eabihf" {
        file_mem.write_all(include_bytes!("memory_cm4.x")).unwrap();
        println!("cargo:rerun-if-changed=memory_cm4.x");
        //println!("cargo:warning=memory_cm4.x");
		
    } else if target == "thumbv6m-none-eabi" {
        file_mem.write_all(include_bytes!("memory_cm0.x")).unwrap();
        println!("cargo:rerun-if-changed=memory_cm0.x");
        //println!("cargo:warning=memory_cm0.x");
		
        /* This library is required because when external C library is compiled for CM0 and -Os an optimizing functions is used
        by arm-gcc compiler.
        linker error message:  
        ./c_lib/tviibe2m/src/drivers/sysclk/cy_sysclk.c:2175: undefined reference to `__gnu_thumb1_case_uqi'
        */
        println!("cargo:rustc-link-lib=gcc");
    } 
	if target == "thumbv6m-none-eabi" {
        println!("cargo:rustc-cfg=cm0");
        
    } 
	else if target == "thumbv7em-none-eabihf" {
        println!("cargo:rustc-cfg=cm4");
	}
	else {
        panic!("Unexpected target");
    }
   
    println!("cargo:rustc-link-search={}", out.display());
    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
	
	
	
}