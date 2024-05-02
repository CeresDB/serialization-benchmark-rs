//! This module defines common request/response types as well as the JsonCodec that is used by the
//! json.helloworld.Greeter service which is defined manually (instead of via proto files) by the
//! `build_json_codec_service` function in the `examples/build.rs` file.
extern crate flatbuffers;
use bytes::{Buf, BufMut};
use std::io::Read;
use tonic::{
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
    Status,
};

// TODO: Any better solutions to manage the flatbuffer objects?
// As the associated type Encode / Decode of Trait Codec has a 'static lifetime bound which means
// items been encoded or decoded shall not have any non static references.
// However flatbuffer related types always have a 'fbb lifetime bound, I found no way to implement
// something like serde do.
pub struct FlatBufferBytes {
    data: Vec<u8>,
    head: usize,
}

impl FlatBufferBytes {
    pub fn new(data: Vec<u8>, head: usize) -> Self {
        Self { data, head }
    }

    pub fn valid_slice(&self) -> &[u8] {
        &(self.data[self.head..])
    }

    pub fn serialize<'buf, T: flatbuffers::Follow<'buf> + 'buf>(
        mut builder: flatbuffers::FlatBufferBuilder<'buf>,
        root_offset: flatbuffers::WIPOffset<T>,
    ) -> Self {
        builder.finish(root_offset, None);
        let (data, head) = builder.collapse();
        Self { data, head }
    }

    pub fn deserialize<'buf, T: flatbuffers::Follow<'buf> + flatbuffers::Verifiable + 'buf>(
        &'buf self,
    ) -> Result<T::Inner, Box<dyn std::error::Error>> {
        let data = self.valid_slice();
        flatbuffers::root::<T>(data).map_err(|x| Box::new(x) as Box<dyn std::error::Error>)
    }
}

#[derive(Debug)]
pub struct FlatBufferEncoder();

impl Encoder for FlatBufferEncoder {
    type Item = FlatBufferBytes;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        buf.put_slice(item.valid_slice());
        Ok(())
    }
}

#[derive(Debug)]
pub struct FlatBufferDecoder();

impl Decoder for FlatBufferDecoder {
    type Item = FlatBufferBytes;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        if !buf.has_remaining() {
            return Ok(None);
        }
        let mut data: Vec<u8> = Vec::new();
        buf.reader()
            .read_to_end(&mut data)
            .map_err(|e| Status::internal(e.to_string()))?;
        let item = FlatBufferBytes::new(data, 0);
        Ok(Some(item))
    }
}

/// A [`Codec`] that implements `application/grpc+json` via the serde library.
#[derive(Debug, Clone, Default)]
pub struct FlatBufferCodec();

impl Codec for FlatBufferCodec {
    type Encode = FlatBufferBytes;
    type Decode = FlatBufferBytes;
    type Encoder = FlatBufferEncoder;
    type Decoder = FlatBufferDecoder;

    fn encoder(&mut self) -> Self::Encoder {
        FlatBufferEncoder()
    }

    fn decoder(&mut self) -> Self::Decoder {
        FlatBufferDecoder()
    }
}
