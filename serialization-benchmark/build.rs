use std::io::Result;
const ENABLE_VENDOR_ENV: &str = "ENABLE_VENDORED";

fn main() -> Result<()> {
    println!("cargo:rerun-if-env-changed={}", ENABLE_VENDOR_ENV);

    let enable_vendor = std::env::var(ENABLE_VENDOR_ENV).unwrap_or("true".to_string());
    if "true" == enable_vendor {
        let protoc_path = protoc_bin_vendored::protoc_bin_path().unwrap();
        std::env::set_var("PROTOC", protoc_path.as_os_str());
    }

    tonic_build::configure()
        .out_dir("src/util")
        .compile(&["src/util/pbperson.proto"], &["src/util"])?;
    Ok(())
}
