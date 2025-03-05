use std::io;

use byteorder::{BigEndian, WriteBytesExt};

use super::define::{self, EventMessageStreamBegin};
use super::errors::EventMessagesError;
use crate::chunk::{Chunk, ChunkEncoder};
use crate::messages::MessageType;

impl EventMessageStreamBegin {
    pub fn write(&self, encoder: &ChunkEncoder, writer: &mut impl io::Write) -> Result<(), EventMessagesError> {
        let mut data = Vec::new();

        data.write_u16::<BigEndian>(define::EventType::StreamBegin as u16)
            .expect("write u16");
        data.write_u32::<BigEndian>(self.stream_id).expect("write u32");

        encoder.write_chunk(writer, Chunk::new(0x02, 0, MessageType::UserControlEvent, 0, data.into()))?;

        Ok(())
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use bytes::{BufMut, Bytes, BytesMut};

    use super::*;
    use crate::chunk::ChunkDecoder;
    use crate::user_control_messages::define::EventMessageStreamBegin;

    #[test]
    fn test_write_stream_begin() {
        let mut buf = BytesMut::new();
        let encoder = ChunkEncoder::default();

        EventMessageStreamBegin { stream_id: 1 }
            .write(&encoder, &mut (&mut buf).writer())
            .unwrap();

        let mut decoder = ChunkDecoder::default();

        let chunk = decoder.read_chunk(&mut buf).expect("read chunk").expect("chunk");
        assert_eq!(chunk.basic_header.chunk_stream_id, 0x02);
        assert_eq!(chunk.message_header.msg_type_id as u8, 0x04);
        assert_eq!(chunk.message_header.msg_stream_id, 0);
        assert_eq!(chunk.payload, Bytes::from(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x01]));
    }
}
