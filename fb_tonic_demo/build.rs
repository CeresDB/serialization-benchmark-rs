use std::io::Result;

fn main() -> Result<()> {
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
