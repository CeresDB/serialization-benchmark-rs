// use flatbuffers::FlatBufferBuilder;
// use tonic::{transport::Server, Request, Response, Status};
// mod util;

// #[derive(Debug, Default)]
// struct MyMonsterService;

// #[tonic::async_trait]
// impl util::monster_service_server::MonsterService for MyMonsterService {
//     async fn get_monster(
//         &self,
//         request: Request<util::GetMonsterRequest>,
//     ) -> Result<Response<util::MonsterResponse>, Status> {
//         let mut builder = FlatBufferBuilder::new();
//         let name = builder.create_string("bob");
//         let data: flatbuffers::WIPOffset<util::Monster<'_>> = util::Monster::create(
//             &mut builder,
//             &util::MonsterArgs {
//                 name: Some(name),
//                 id: 11,
//             },
//         );
//         builder.finish(data, None);
//         let monster_data_bytes = builder.finished_data();

//         Ok(Response::new(util::MonsterResponse {
//             monster_data: monster_data_bytes.to_vec(),
//         }))
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let address = "[::1]:8000".parse().unwrap();
//     let voting_service = MyMonsterService::default();

//     Server::builder()
//         .add_service(util::MonsterServiceServer::new(voting_service))
//         .serve(address)
//         .await?;
//     Ok(())
// }

use tonic::{transport::Server, Request, Response, Status};

pub mod util;
use util::common::FlatBuffersObject;
use util::fbperson_generated::fbdemo::FBPerson;
use util::fbperson_generated::fbdemo::FBPersonArgs;

pub mod hello_world {
    include!("util/fb.helloworld.Greeter.rs");
}
use hello_world::greeter_server::{Greeter, GreeterServer};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<FlatBuffersObject<'static>>,
    ) -> Result<Response<FlatBuffersObject<'static>>, Status> {
        // println!("Got a request from {:?}", request.remote_addr());

        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

        let name = builder.create_string("from_receiver");

        let data: flatbuffers::WIPOffset<FBPerson<'_>> = FBPerson::create(
            &mut builder,
            &FBPersonArgs {
                name: Some(name),
                age: 11,
                pets: None,
            },
        );

        let resp: FlatBuffersObject<'_> = FlatBuffersObject::new(data, builder);

        // let reply = HelloResponse {
        //     message: format!("Hello {}!", request.into_inner().name),
        // };
        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
