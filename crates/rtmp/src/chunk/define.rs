use bytes::Bytes;

use crate::messages::MessageTypeID;

// common chunk stream ids

pub const COMMAND_CHUNK_STREAM_ID: u32 = 3;
pub const AUDIO_CHUNK_STREAM_ID: u32 = 4;
pub const VIDEO_CHUNK_STREAM_ID: u32 = 5;

#[derive(Debug, PartialEq, Eq, Clone, Copy, num_derive::FromPrimitive, Hash)]
#[repr(u8)]
/// A chunk type represents the format of the chunk header.
pub enum ChunkType {
    /// Chunk type 0 - 5.3.1.2.1
    Type0 = 0,
    /// Chunk type 1 - 5.3.1.2.2
    Type1 = 1,
    /// Chunk type 2 - 5.3.1.2.3
    Type2 = 2,
    /// Chunk type 3 - 5.3.1.1.4
    Type3 = 3,
}

#[derive(Eq, PartialEq, Debug, Clone)]
/// A chunk basic header.
pub struct ChunkBasicHeader {
    /// Used for decoding the header only.
    pub(super) format: ChunkType, // 2 bits

    pub chunk_stream_id: u32, // 6 bits (if format == 0, 8 bits, if format == 1, 16 bits)
}

#[derive(Eq, PartialEq, Debug, Clone)]
/// A chunk message header.
pub struct ChunkMessageHeader {
    /// The timestamp of the message.
    pub timestamp: u32, /* 3 bytes (when writing the header, if the timestamp is >= 0xFFFFFF,
                         * write 0xFFFFFF) */
    /// The length of the message.
    pub msg_length: u32, // 3 bytes
    /// The type of the message.
    pub msg_type_id: MessageTypeID, // 1 byte
    /// The stream id of the message.
    pub msg_stream_id: u32, // 4 bytes
    /// Whether the timestamp is extended.
    pub(super) was_extended_timestamp: bool, // used for reading the header only
}

impl ChunkMessageHeader {
    /// is_extended_timestamp returns true if the timestamp is >= 0xFFFFFF.
    /// This means that the timestamp is extended and is written in the extended
    /// timestamp field.
    #[inline]
    pub fn is_extended_timestamp(&self) -> bool {
        self.timestamp >= 0xFFFFFF
    }
}

/// A chunk.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Chunk {
    /// The basic header of the chunk.
    pub basic_header: ChunkBasicHeader,
    /// The message header of the chunk.
    pub message_header: ChunkMessageHeader,
    /// The payload of the chunk.
    pub payload: Bytes,
}

impl Chunk {
    /// new creates a new chunk.
    /// Helper function to create a new chunk.
    pub fn new(
        chunk_stream_id: u32,
        timestamp: u32,
        msg_type_id: MessageTypeID,
        msg_stream_id: u32,
        payload: Bytes,
    ) -> Self {
        Self {
            basic_header: ChunkBasicHeader {
                chunk_stream_id,
                format: ChunkType::Type0,
            },
            message_header: ChunkMessageHeader {
                timestamp,
                msg_length: payload.len() as u32,
                msg_type_id,
                msg_stream_id,
                was_extended_timestamp: false,
            },
            payload,
        }
    }
}

/// We bump our chunk size to 4096 bytes.
pub const CHUNK_SIZE: usize = 4096;

/// Not apart of the spec but we have a limit on how big a chunk can be.
/// This is the maximum chunk size we will accept. If the peer requests a chunk
/// size bigger than this, we will close the connection.
pub const MAX_CHUNK_SIZE: usize = 4096 * 16; // 64 KB

/// The default chunk size is 128 bytes.
/// 5.4.1 "The maximum chunk size defaults to 128 bytes ..."
pub const INIT_CHUNK_SIZE: usize = 128;
