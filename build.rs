use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .out_dir("src/util")
        .compile(&["src/util/pbperson.proto"], &["src/util"])?;
    Ok(())
}
