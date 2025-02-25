use nutype_enum::nutype_enum;

use crate::ffi::*;

const _: () = {
    assert!(std::mem::size_of::<AVSampleFormat>() == std::mem::size_of_val(&AV_SAMPLE_FMT_NONE));
};


nutype_enum! {
    /// Audio sample formats used in FFmpeg's `AVSampleFormat` enumeration.
    ///
    /// The sample format defines how audio samples are stored in memory, including:
    /// - **Bit depth** (8-bit, 16-bit, 32-bit, 64-bit)
    /// - **Signed vs Unsigned** (U8 is unsigned, others are signed)
    /// - **Floating-point vs Integer**
    /// - **Packed vs Planar** (Planar formats store each channel separately)
    ///
    /// See the official FFmpeg documentation:
    /// <https://ffmpeg.org/doxygen/trunk/samplefmt_8h.html>
    pub enum AVSampleFormat(i32) {
        /// No sample format specified or unknown format.
        /// Corresponds to `AV_SAMPLE_FMT_NONE`.
        None = AV_SAMPLE_FMT_NONE as _,

        /// Unsigned 8-bit PCM format (0 to 255 range).
        /// - **Binary representation**: `0bxxxxxxxx` (8 bits)
        /// - **Range**: `[0, 255]`
        /// - **Stored as**: `u8`
        /// - **Interleaved**
        /// Corresponds to `AV_SAMPLE_FMT_U8`.
        U8 = AV_SAMPLE_FMT_U8 as _,

        /// Signed 16-bit PCM format.
        /// - **Binary representation**: `0bxxxxxxxxxxxxxxxx` (16 bits)
        /// - **Range**: `[-32,768, 32,767]`
        /// - **Stored as**: `i16`
        /// - **Interleaved**
        /// Corresponds to `AV_SAMPLE_FMT_S16`.
        S16 = AV_SAMPLE_FMT_S16 as _,

        /// Signed 32-bit PCM format.
        /// - **Binary representation**: `0bxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx` (32 bits)
        /// - **Range**: `[-2^31, 2^31-1]`
        /// - **Stored as**: `i32`
        /// - **Interleaved**
        /// Corresponds to `AV_SAMPLE_FMT_S32`.
        S32 = AV_SAMPLE_FMT_S32 as _,

        /// 32-bit Floating-point PCM format.
        /// - **Binary representation**: IEEE-754 32-bit float
        /// - **Range**: `[-1.0, 1.0]` (normalized)
        /// - **Stored as**: `f32`
        /// - **Interleaved**
        /// Corresponds to `AV_SAMPLE_FMT_FLT`.
        Flt = AV_SAMPLE_FMT_FLT as _,

        /// 64-bit Floating-point PCM format.
        /// - **Binary representation**: IEEE-754 64-bit float
        /// - **Range**: `[-1.0, 1.0]` (normalized)
        /// - **Stored as**: `f64`
        /// - **Interleaved**
        /// Corresponds to `AV_SAMPLE_FMT_Dbl`.
        Dbl = AV_SAMPLE_FMT_DBL as _,

        /// **Planar** Unsigned 8-bit PCM format.
        /// - **Binary representation**: `0bxxxxxxxx` (8 bits)
        /// - **Range**: `[0, 255]`
        /// - **Stored as**: `u8`
        /// - **Planar (separate channel planes)**
        /// Corresponds to `AV_SAMPLE_FMT_U8P`.
        U8p = AV_SAMPLE_FMT_U8P as _,

        /// **Planar** Signed 16-bit PCM format.
        /// - **Binary representation**: `0bxxxxxxxxxxxxxxxx` (16 bits)
        /// - **Range**: `[-32,768, 32,767]`
        /// - **Stored as**: `i16`
        /// - **Planar (separate channel planes)**
        /// Corresponds to `AV_SAMPLE_FMT_S16P`.
        S16p = AV_SAMPLE_FMT_S16P as _,

        /// **Planar** Signed 32-bit PCM format.
        /// - **Binary representation**: `0bxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx` (32 bits)
        /// - **Range**: `[-2^31, 2^31-1]`
        /// - **Stored as**: `i32`
        /// - **Planar (separate channel planes)**
        /// Corresponds to `AV_SAMPLE_FMT_S32P`.
        S32p = AV_SAMPLE_FMT_S32P as _,

        /// **Planar** 32-bit Floating-point PCM format.
        /// - **Binary representation**: IEEE-754 32-bit float
        /// - **Range**: `[-1.0, 1.0]` (normalized)
        /// - **Stored as**: `f32`
        /// - **Planar (separate channel planes)**
        /// Corresponds to `AV_SAMPLE_FMT_FLTP`.
        Fltp = AV_SAMPLE_FMT_FLTP as _,

        /// **Planar** 64-bit Floating-point PCM format.
        /// - **Binary representation**: IEEE-754 64-bit float
        /// - **Range**: `[-1.0, 1.0]` (normalized)
        /// - **Stored as**: `f64`
        /// - **Planar (separate channel planes)**
        /// Corresponds to `AV_SAMPLE_FMT_DBLP`.
        Dblp = AV_SAMPLE_FMT_DBLP as _,

        /// Signed 64-bit PCM format.
        /// - **Binary representation**: `0bxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`
        /// - **Range**: `[-2^63, 2^63-1]`
        /// - **Stored as**: `i64`
        /// - **Interleaved**
        /// Corresponds to `AV_SAMPLE_FMT_S64`.
        S64 = AV_SAMPLE_FMT_S64 as _,

        /// **Planar** Signed 64-bit PCM format.
        /// - **Binary representation**: `0bxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`
        /// - **Range**: `[-2^63, 2^63-1]`
        /// - **Stored as**: `i64`
        /// - **Planar (separate channel planes)**
        /// Corresponds to `AV_SAMPLE_FMT_S64P`.
        S64p = AV_SAMPLE_FMT_S64P as _,

        /// Number of sample formats available (internal use only).
        /// **DO NOT USE** if linking dynamically, as the number may change.
        /// Corresponds to `AV_SAMPLE_FMT_NB`.
        Nb = AV_SAMPLE_FMT_NB as _,
    }
}

impl PartialEq<i32> for AVSampleFormat {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<u32> for AVSampleFormat {
    fn from(value: u32) -> Self {
        AVSampleFormat(value as _)
    }
}

impl From<AVSampleFormat> for u32 {
    fn from(value: AVSampleFormat) -> Self {
        value.0 as u32
    }
}
