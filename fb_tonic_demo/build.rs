use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    if let Ok(flatc_bin) = std::env::var("FLATC") {
        std::env::set_var(
            "PATH",
            std::env::var("PATH").unwrap() + format!(":{}", flatc_bin.as_str()).as_str(),
        );
    } else {
        println!("FLATC enviroment variable not found!");
    }

    println!("cargo:rerun-if-changed=src/util/fbgreeting.fbs");
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("src/util/fbgreeting.fbs")],
        out_dir: Path::new("target/flatbuffers/"),
        ..Default::default()
    })
    .expect("flatc");

    let greeter_service = tonic_build::manual::Service::builder()
        .name("Greeter")
        .package("fb.helloworld")
        .method(
            tonic_build::manual::Method::builder()
                .name("say_hello")
                .route_name("SayHello")
                .input_type("crate::util::common::FlatBufferBytes")
                .output_type("crate::util::common::FlatBufferBytes")
                .codec_path("crate::util::common::FlatBufferCodec")
                .build(),
        )
        .build();

    tonic_build::manual::Builder::new()
        .out_dir("src/util")
        .compile(&[greeter_service]);
    Ok(())
}
