#![allow(unused)]
use tonic::{transport::Server, Request, Response, Status};

pub mod util;
use util::common::FlatBufferBytes;
use util::fbgreeting_generated::fbdemo::Greetings;
use util::fbgreeting_generated::fbdemo::GreetingsArgs;

pub mod hello_world {
    include!("util/fb.helloworld.Greeter.rs");
}
use hello_world::greeter_server::{Greeter, GreeterServer};

pub mod pb_hello_world {
    tonic::include_proto!("helloworld");
}
use pb_hello_world::greeter_server::{Greeter as PBGreeter, GreeterServer as PBGreeterServer};
use pb_hello_world::{HelloReply, HelloRequest};

#[derive(Default)]
pub struct PBMyGreeter {}

#[tonic::async_trait]
impl PBGreeter for PBMyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = pb_hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<FlatBufferBytes>,
    ) -> Result<Response<FlatBufferBytes>, Status> {
        let req = request.into_inner();
        if let Ok(greetings) = req.deserialize::<Greetings>() {
            println!(
                "Greetings from {:?}: {:?}",
                greetings.name().unwrap(),
                greetings.words().unwrap()
            );
        }

        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let name = builder.create_string("bob");
        let words = builder.create_string("world~~");

        let root_offset: flatbuffers::WIPOffset<Greetings<'_>> = Greetings::create(
            &mut builder,
            &GreetingsArgs {
                name: Some(name),
                words: Some(words),
            },
        );
        let resp = FlatBufferBytes::serialize(builder, root_offset);
        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();
    let pb_greeter = PBMyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(PBGreeterServer::new(pb_greeter))
        .serve(addr)
        .await?;

    Ok(())
}
