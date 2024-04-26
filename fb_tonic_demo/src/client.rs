// // main.rs (客户端)
// extern crate flatbuffers;
// use tonic::Request;
// mod util;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // 连接到服务端
//     let mut client = util::MonsterServiceClient::connect("http://[::1]:8000").await?;

//     // 创建请求
//     let request = tonic::Request::new(util::GetMonsterRequest { id: 123 });

//     // 发起请求并等待响应
//     let response = client.get_monster(request).await?;

//     // 获取 MonsterResponse 中的 monster_data 字段
//     let monster_data_bytes = response.get_ref().monster_data.as_slice();

//     // 使用 flatbuffers 的逻辑来解析字节数据
//     // let monster = match flatbuffers::root::<util::Monster>(monster_data_bytes) {
//     //     Ok(monster): {},
//     //     Error(_): {}
//     // };
//     if let Ok(monster) = flatbuffers::root::<util::Monster>(monster_data_bytes) {
//         println!("Received Monster with ID: {}", monster.id());
//         println!("Received Monster with Name: {:?}", monster.name());
//     } else {
//         println!("bad ass.");
//     }

//     // 处理 monster

//     Ok(())
// }

pub mod util;
use util::common::FlatBuffersObject;
use util::fbperson_generated::fbdemo::FBPerson;
use util::fbperson_generated::fbdemo::FBPersonArgs;

pub mod hello_world {
    include!("util/fb.helloworld.Greeter.rs");
}
use hello_world::greeter_client::GreeterClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let name = builder.create_string("from_sender");

    let data: flatbuffers::WIPOffset<FBPerson<'_>> = FBPerson::create(
        &mut builder,
        &FBPersonArgs {
            name: Some(name),
            age: 11,
            pets: None,
        },
    );
    let req = FlatBuffersObject::new(data, builder);

    let request = tonic::Request::new(req);

    let response = client.say_hello(request).await?;

    // println!("RESPONSE={:?}", response);

    Ok(())
}
