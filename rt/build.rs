use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put `link.x` in the build directory
    File::create(out_dir.join("link.x"))?.write_all(include_bytes!("link.x"))?;

    cc::Build::new()
        .no_default_flags(true)
        .compiler("arm-none-eabi-gcc")
        .file("src/crt0.s")
        .compile("crt0");
    println!("cargo:rerun-if-changed=src/crt0.s");
    Ok(())
}
