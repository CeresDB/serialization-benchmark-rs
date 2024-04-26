//! This module defines common request/response types as well as the JsonCodec that is used by the
//! json.helloworld.Greeter service which is defined manually (instead of via proto files) by the
//! `build_json_codec_service` function in the `examples/build.rs` file.
extern crate flatbuffers;
use crate::util::fbperson_generated::fbdemo::FBPerson;
use bytes::{Buf, BufMut};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use tonic::{
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
    Status,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloResponse {
    pub message: String,
}

pub trait FlatBufferSerializable {
    fn serialize(&self) -> &[u8];
}

pub struct FlatBuffersObject<'a> {
    data: flatbuffers::WIPOffset<FBPerson<'a>>,
    builder: flatbuffers::FlatBufferBuilder<'a>,
}

impl<'a> FlatBuffersObject<'a> {
    pub fn new(
        data: flatbuffers::WIPOffset<FBPerson<'a>>,
        builder: flatbuffers::FlatBufferBuilder<'a>,
    ) -> Self {
        Self { data, builder }
    }
}

impl FlatBufferSerializable for FlatBuffersObject<'_> {
    fn serialize(&self) -> &[u8] {
        self.builder.finish(self.data, None);
        let buf = self.builder.finished_data();
        buf
    }
}

#[derive(Debug)]
pub struct FlatBufferEncoder<T>(PhantomData<T>);

impl<T: FlatBufferSerializable> Encoder for FlatBufferEncoder<T> {
    type Item = T;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        let out = item.serialize();
        buf.put_slice(out);
        Ok(())
        //serde_json::to_writer(buf.writer(), &item).map_err(|e| Status::internal(e.to_string()))
    }
}

#[derive(Debug)]
pub struct FlatBufferDecoder<U: 'static>(PhantomData<&'static U>);

impl<U: 'static + flatbuffers::Follow<'static> + flatbuffers::Verifiable> Decoder
    for FlatBufferDecoder<U>
{
    type Item = U::Inner;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        if !buf.has_remaining() {
            return Ok(None);
        }

        let item =
            flatbuffers::root::<U>(buf.chunk()).map_err(|e| Status::internal(e.to_string()))?;
        Ok(Some(item))
    }
}

/// A [`Codec`] that implements `application/grpc+json` via the serde library.
#[derive(Debug, Clone)]
pub struct FlatBufferCodec<T, U: 'static>(PhantomData<(T, &'static U)>);

impl<T, U> Default for FlatBufferCodec<T, U> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T, U> Codec for FlatBufferCodec<T, U>
where
    T: FlatBufferSerializable + Send + 'static,
    U: flatbuffers::Follow<'static> + flatbuffers::Verifiable + Sync + Send + 'static,
    U::Inner: Send + 'static,
{
    type Encode = T;
    type Decode = U::Inner;
    type Encoder = FlatBufferEncoder<T>;
    type Decoder = FlatBufferDecoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        FlatBufferEncoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        FlatBufferDecoder(PhantomData)
    }
}
