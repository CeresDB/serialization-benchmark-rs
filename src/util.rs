extern crate flatbuffers;
mod fbperson_generated;
mod pbperson;
use fbperson_generated::fbdemo::{FBPerson, FBPersonArgs, FBPet, FBPetArgs};
use fury::{from_buffer, to_buffer, Fury};
use pbperson::{PbPerson, PbPet};
use prost::Message;
use std::time::{Duration, Instant};

pub type EnDecodeDuration = (Duration, Duration);

pub trait Serializable {
    fn serialize_and_deserialize(&mut self, duration_trace: &mut EnDecodeDuration);
}

pub struct RawPet {
    pub name: String,
}

pub struct RawPerson {
    pub name: String,
    pub age: i32,
    pub pets: Vec<RawPet>,
}

#[derive(Fury, Debug, PartialEq)]
#[tag("example.foo")]
struct FuryPet {
    name: String,
}

#[derive(Fury, Debug, PartialEq)]
#[tag("example.bar")]
struct FuryPerson {
    name: String,
    age: i32,
    pets: Vec<FuryPet>,
}

pub struct FuryObject {
    data: FuryPerson,
}

impl FuryObject {
    pub fn new(raw: &RawPerson) -> Self {
        FuryObject {
            data: FuryPerson {
                name: raw.name.clone(),
                age: raw.age,
                pets: raw
                    .pets
                    .iter()
                    .map(|x| FuryPet {
                        name: x.name.clone(),
                    })
                    .collect(),
            },
        }
    }
}

impl Serializable for FuryObject {
    fn serialize_and_deserialize(&mut self, duration_trace: &mut EnDecodeDuration) {
        // serialize
        let start_time = Instant::now();
        let bin = to_buffer(&self.data);
        duration_trace.0 += start_time.elapsed();

        // deserialize
        let start_time = Instant::now();
        let _: FuryPerson = from_buffer(&bin).expect("should success");
        duration_trace.1 += start_time.elapsed();
    }
}

pub struct FlatBuffersObject<'a> {
    data: flatbuffers::WIPOffset<FBPerson<'a>>,
    builder: flatbuffers::FlatBufferBuilder<'a>,
}

impl FlatBuffersObject<'_> {
    pub fn new(raw: &RawPerson) -> Self {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let pets: Vec<_> = raw
            .pets
            .iter()
            .map(|x| {
                let pet_name = builder.create_string(x.name.as_str());
                FBPet::create(
                    &mut builder,
                    &FBPetArgs {
                        name: Some(pet_name),
                    },
                )
            })
            .collect();

        let name = builder.create_string(raw.name.as_str());
        let pets = builder.create_vector(&pets);

        let data: flatbuffers::WIPOffset<FBPerson<'_>> = FBPerson::create(
            &mut builder,
            &FBPersonArgs {
                name: Some(name),
                age: raw.age,
                pets: Some(pets),
            },
        );
        FlatBuffersObject { data, builder }
    }
}

impl Serializable for FlatBuffersObject<'_> {
    fn serialize_and_deserialize(&mut self, duration_trace: &mut EnDecodeDuration) {
        // serialize
        let start_time = Instant::now();
        self.builder.finish(self.data, None);
        let buf = self.builder.finished_data();
        duration_trace.0 += start_time.elapsed();

        // deserialize
        let start_time = Instant::now();
        let _ = flatbuffers::root::<FBPerson>(buf).unwrap();
        duration_trace.1 += start_time.elapsed();
    }
}

pub struct ProtobufObject {
    data: PbPerson,
}

impl ProtobufObject {
    pub fn new(raw_person: &RawPerson) -> Self {
        let data = PbPerson {
            name: raw_person.name.clone(),
            age: raw_person.age,
            pets: raw_person
                .pets
                .iter()
                .map(|x| PbPet {
                    name: x.name.clone(),
                })
                .collect(),
        };
        ProtobufObject { data }
    }
}

impl Serializable for ProtobufObject {
    fn serialize_and_deserialize(&mut self, duration_trace: &mut EnDecodeDuration) {
        // serialize
        let start_time = Instant::now();
        let mut buf = Vec::new();
        self.data
            .encode(&mut buf)
            .expect("Failed to encode message");
        duration_trace.0 += start_time.elapsed();

        // deserialize
        let start_time = Instant::now();

        let _ = PbPerson::decode(&*buf).expect("Message decode failed");
        duration_trace.1 += start_time.elapsed();
    }
}
