#![allow(unused)]
pub mod util;
use util::common::FlatBufferBytes;
use util::fbgreeting_generated::fbdemo::Greetings;
use util::fbgreeting_generated::fbdemo::GreetingsArgs;

pub mod hello_world {
    include!("util/fb.helloworld.Greeter.rs");
}
use hello_world::greeter_client::GreeterClient;

pub mod pb_hello_world {
    tonic::include_proto!("helloworld");
}

use pb_hello_world::greeter_client::GreeterClient as PBGreeterClient;
use pb_hello_world::HelloRequest;

async fn fb_hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let mut builder: flatbuffers::FlatBufferBuilder<'_> =
        flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let name = builder.create_string("Alice");
    let words = builder.create_string("Hello~~");

    let root_offset: flatbuffers::WIPOffset<Greetings<'_>> = Greetings::create(
        &mut builder,
        &GreetingsArgs {
            name: Some(name),
            words: Some(words),
        },
    );
    let request = tonic::Request::new(FlatBufferBytes::serialize(builder, root_offset));
    let response = client.say_hello(request).await?;

    let r = response.into_inner();
    if let Ok(greetings) = r.deserialize::<Greetings>() {
        println!(
            "Greetings from {:?}: {:?}",
            greetings.name().unwrap(),
            greetings.words().unwrap()
        );
    }
    Ok(())
}

async fn pb_hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PBGreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fb_hello().await?;
    pb_hello().await?;
    Ok(())
}
