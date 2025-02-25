use nutype_enum::{bitwise_enum, nutype_enum};

use crate::ffi::*;

const _: () = {
    assert!(std::mem::size_of::<AVDictionaryFlags>() == std::mem::size_of_val(&AV_DICT_MATCH_CASE));
};

nutype_enum! {
    /// Dictionary flags used in FFmpeg's AVDictionary API.
    ///
    /// See FFmpeg's `AVDictionary` in the official documentation:
    /// <https://ffmpeg.org/doxygen/trunk/group__lavu__dict.html>
    pub enum AVDictionaryFlags(i32) {
        /// Match keys case-sensitively.
        /// Corresponds to `AV_DICT_MATCH_CASE`.
        MatchCase = AV_DICT_MATCH_CASE as _,

        /// Do not differentiate keys with different suffixes.
        /// Corresponds to `AV_DICT_IGNORE_SUFFIX`.
        IgnoreSuffix = AV_DICT_IGNORE_SUFFIX as _,

        /// Do not duplicate the key string.
        /// Corresponds to `AV_DICT_DONT_STRDUP_KEY`.
        DontStrDupKey = AV_DICT_DONT_STRDUP_KEY as _,

        /// Do not duplicate the value string.
        /// Corresponds to `AV_DICT_DONT_STRDUP_VAL`.
        DontStrDupVal = AV_DICT_DONT_STRDUP_VAL as _,

        /// Do not overwrite existing entries.
        /// Corresponds to `AV_DICT_DONT_OVERWRITE`.
        DontOverwrite = AV_DICT_DONT_OVERWRITE as _,

        /// Append the new value to an existing key instead of replacing it.
        /// Corresponds to `AV_DICT_APPEND`.
        Append = AV_DICT_APPEND as _,

        /// Allow multiple entries with the same key.
        /// Corresponds to `AV_DICT_MULTIKEY`.
        MultiKey = AV_DICT_MULTIKEY as _,
    }
}

bitwise_enum!(AVDictionaryFlags);

impl PartialEq<i32> for AVDictionaryFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<u32> for AVDictionaryFlags {
    fn from(value: u32) -> Self {
        AVDictionaryFlags(value as _)
    }
}

impl From<AVDictionaryFlags> for u32 {
    fn from(value: AVDictionaryFlags) -> Self {
        value.0 as u32
    }
}
