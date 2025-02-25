use nutype_enum::{bitwise_enum, nutype_enum};

use crate::ffi::*;

const _: () = {
    assert!(std::mem::size_of::<AVPktFlags>() == std::mem::size_of_val(&AV_PKT_FLAG_KEY));
};

nutype_enum! {
    /// Packet flags used in FFmpeg's `AVPacket`.
    ///
    /// These flags describe metadata about a packet, such as whether it is a keyframe or corrupt.
    ///
    /// See the official FFmpeg documentation:
    /// <https://ffmpeg.org/doxygen/trunk/avcodec_8h.html>
    pub enum AVPktFlags(i32) {
        /// This packet contains a **keyframe**.
        /// - **Used for**: Identifying keyframes in video streams.
        /// - **Binary representation**: `0b00001`
        /// - **Equivalent to**: `AV_PKT_FLAG_KEY`
        Key = AV_PKT_FLAG_KEY as _,

        /// This packet is **corrupt**.
        /// - **Used for**: Marking damaged or incomplete data.
        /// - **Binary representation**: `0b00010`
        /// - **Equivalent to**: `AV_PKT_FLAG_CORRUPT`
        Corrupt = AV_PKT_FLAG_CORRUPT as _,

        /// This packet should be **discarded**.
        /// - **Used for**: Frames that should be ignored by decoders.
        /// - **Binary representation**: `0b00100`
        /// - **Equivalent to**: `AV_PKT_FLAG_DISCARD`
        Discard = AV_PKT_FLAG_DISCARD as _,

        /// This packet comes from a **trusted source**.
        /// - **Used for**: Security and validation checks.
        /// - **Binary representation**: `0b01000`
        /// - **Equivalent to**: `AV_PKT_FLAG_TRUSTED`
        Trusted = AV_PKT_FLAG_TRUSTED as _,

        /// This packet is **disposable** (e.g., non-reference frames).
        /// - **Used for**: Frames that can be dropped without affecting playback.
        /// - **Binary representation**: `0b10000`
        /// - **Equivalent to**: `AV_PKT_FLAG_DISPOSABLE`
        Disposable = AV_PKT_FLAG_DISPOSABLE as _,
    }
}

bitwise_enum!(AVPktFlags);

impl PartialEq<i32> for AVPktFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<u32> for AVPktFlags {
    fn from(value: u32) -> Self {
        AVPktFlags(value as _)
    }
}

impl From<AVPktFlags> for u32 {
    fn from(value: AVPktFlags) -> Self {
        value.0 as u32
    }
}
