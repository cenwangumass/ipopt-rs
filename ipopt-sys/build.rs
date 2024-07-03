use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

const LIB_NAME: &str = "ipopt";
const CNLP_LIB_NAME: &str = "ipopt_cnlp";

fn build_cnlp(ipopt_include_path: impl AsRef<Path>) -> PathBuf {
    cmake::Config::new("cnlp")
        .define(
            "Ipopt_INCLUDE_DIRS:STRING",
            ipopt_include_path.as_ref().to_str().unwrap(),
        )
        .build()
}

fn link(cnlp_path: &PathBuf, ipopt_search_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // Link CNLP
    println!("cargo:rustc-link-lib=static={}", CNLP_LIB_NAME);
    println!(
        "cargo:rustc-link-search=native={}",
        cnlp_path.join("lib").display()
    );

    // Link IPOPT
    println!("cargo:rustc-link-lib=dylib={}", LIB_NAME);
    println!(
        "cargo:rustc-link-search=native={}",
        ipopt_search_path.display()
    );

    // Link C++
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // Generate Rust bindings
    let header = cnlp_path.join("include").join("c_api.h");
    let output = PathBuf::from(&env::var("OUT_DIR").unwrap());
    bindgen::builder()
        .header(header.to_str().unwrap())
        .generate()?
        .write_to_file(output.join("ipopt_cnlp.rs"))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let ipopt_path = env::var("IPOPT");

    match ipopt_path {
        Ok(ipopt_path) => {
            let ipopt_path = PathBuf::from(&ipopt_path);
            let include_path = ipopt_path.join("include");
            let lib_path = ipopt_path.join("lib");
            let cnlp_path = build_cnlp(&include_path);
            link(&cnlp_path, &lib_path)?;
            Ok(())
        }
        Err(_) => Err("Environment variable `IPOPT` not set")?,
    }
}
