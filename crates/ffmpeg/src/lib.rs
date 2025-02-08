//! A crate designed to provide a simple interface to the native ffmpeg c-bindings.
//!
//! ## Why do we need this?
//!
//! This crate aims to provide a simple-safe interface to the native ffmpeg c-bindings.
//!
//! Currently this crate only supports the latest versions of ffmpeg (7.x.x)
//!
//! ## How is this different from other ffmpeg crates?
//!
//! The other main ffmpeg crate is [ffmpeg-next](https://github.com/zmwangx/rust-ffmpeg).
//!
//! This crate adds a few features and has a safer API. Notably it adds the ability to provide an in-memory decode / encode buffer.
//!
//! ## Examples
//!
//! ### Decoding a audio/video file
//!
//! ```rust
//! # use std::path::PathBuf;
//! # use scuffle_ffmpeg::ffi::AVMediaType;
//! # fn test_fn() -> Result<(), Box<dyn std::error::Error>> {
//! # let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets").join("avc_aac.mp4");
//! // this can be any seekable io stream (std::io::Read + std::io::Seek)
//! // if you don't have seek, you can just use Input::new(std::io::Read) (no seeking support)
//! let input = scuffle_ffmpeg::io::Input::seekable(std::fs::File::open(path)?)?;
//! let streams = input.streams();
//! // implments an iterator over the streams
//! // for stream in streams {
//! //     dbg!(&stream);
//! // }
//!
//! dbg!(&streams);
//!
//! let best_video_stream = streams.best(AVMediaType::AVMEDIA_TYPE_VIDEO).expect("no video stream found");
//! let best_audio_stream = streams.best(AVMediaType::AVMEDIA_TYPE_AUDIO).expect("no audio stream found");
//!
//! dbg!(&best_video_stream);
//! dbg!(&best_audio_stream);
//!
//! let video_decoder = scuffle_ffmpeg::decoder::Decoder::new(&best_video_stream)?.video().expect("not an video decoder");
//! let audio_decoder = scuffle_ffmpeg::decoder::Decoder::new(&best_audio_stream)?.audio().expect("not an audio decoder");
//!
//! dbg!(&video_decoder);
//! dbg!(&audio_decoder);
//! # Ok(())
//! # }
//! # test_fn().expect("failed to run test");
//! ```
//!
//! ### Reencode a audio/video file
//!
//! ```rust
//! # use std::path::PathBuf;
//! # use scuffle_ffmpeg::ffi::{AVMediaType, AVCodecID};
//! # use scuffle_ffmpeg::encoder::{AudioEncoderSettings, VideoEncoderSettings};
//! # use scuffle_ffmpeg::io::OutputOptions;
//! # use scuffle_ffmpeg::frame::AudioChannelLayout;
//! #
//! # fn test_fn() -> Result<(), Box<dyn std::error::Error>> {
//! # let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets").join("avc_aac.mp4");
//!
//! let input = scuffle_ffmpeg::io::Input::seekable(std::fs::File::open(path)?)?;
//! let streams = input.streams();
//!
//! let best_video_stream = streams.best(AVMediaType::AVMEDIA_TYPE_VIDEO).expect("no video stream found");
//! let best_audio_stream = streams.best(AVMediaType::AVMEDIA_TYPE_AUDIO).expect("no audio stream found");
//!
//! let mut video_decoder = scuffle_ffmpeg::decoder::Decoder::new(&best_video_stream)?.video().expect("not an video decoder");
//! let mut audio_decoder = scuffle_ffmpeg::decoder::Decoder::new(&best_audio_stream)?.audio().expect("not an audio decoder");
//!
//! let mut output = scuffle_ffmpeg::io::Output::seekable(std::io::Cursor::new(Vec::new()), OutputOptions::builder().format_name("mp4")?.build())?;
//!
//! let x264 = scuffle_ffmpeg::codec::EncoderCodec::new(AVCodecID::AV_CODEC_ID_H264).expect("no h264 encoder found");
//! let aac = scuffle_ffmpeg::codec::EncoderCodec::new(AVCodecID::AV_CODEC_ID_AAC).expect("no aac encoder found");
//!
//! let video_settings = VideoEncoderSettings::builder()
//!     .width(video_decoder.width())
//!     .height(video_decoder.height())
//!     .frame_rate(video_decoder.frame_rate())
//!     .pixel_format(video_decoder.pixel_format())
//!     .build();
//! let audio_settings = AudioEncoderSettings::builder()
//!     .sample_rate(audio_decoder.sample_rate())
//!     .ch_layout(AudioChannelLayout::new(audio_decoder.channels()).expect("invalid channel layout"))
//!     .sample_fmt(audio_decoder.sample_format())
//!     .build();
//!
//! let mut video_encoder = scuffle_ffmpeg::encoder::Encoder::new(x264, &mut output, best_video_stream.time_base(), best_video_stream.time_base(), video_settings).expect("not an video encoder");
//! let mut audio_encoder = scuffle_ffmpeg::encoder::Encoder::new(aac, &mut output, best_audio_stream.time_base(), best_audio_stream.time_base(), audio_settings).expect("not an audio encoder");
//!
//! output.write_header()?;
//!
//! loop {
//!     let mut audio_done = false;
//!     let mut video_done = false;
//!     if let Some(frame) = audio_decoder.receive_frame()? {
//!         audio_encoder.send_frame(&frame)?;
//!         while let Some(packet) = audio_encoder.receive_packet()? {
//!             output.write_packet(&packet)?;
//!         }
//!     } else {
//!         audio_done = true;
//!     }
//!
//!     if let Some(frame) = video_decoder.receive_frame()? {
//!         video_encoder.send_frame(&frame)?;
//!         while let Some(packet) = video_encoder.receive_packet()? {
//!             output.write_packet(&packet)?;
//!         }
//!     } else {
//!         video_done = true;
//!     }
//!
//!     if audio_done && video_done {
//!         break;
//!     }
//! }
//!
//! video_decoder.send_eof()?;
//! audio_decoder.send_eof()?;
//!
//! while let Some(packet) = video_encoder.receive_packet()? {
//!     output.write_packet(&packet)?;
//! }
//!
//! while let Some(packet) = audio_encoder.receive_packet()? {
//!     output.write_packet(&packet)?;
//! }
//!
//! output.write_trailer()?;
//!
//! let output_data = output.into_inner();
//! // do something with the output data (write to disk, upload to s3, etc)
//! # drop(output_data);
//! # Ok(())
//! # }
//! # test_fn().expect("failed to run test");
//! ```
//!
//! ## Status
//!
//! This crate is currently under development and is not yet stable.
//!
//! Unit tests are not yet fully implemented. Use at your own risk.
//!
//! ## License
//!
//! This project is licensed under the [MIT](./LICENSE.MIT) or [Apache-2.0](./LICENSE.Apache-2.0) license.
//! You can choose between one of them if you use this work.
//!
//! `SPDX-License-Identifier: MIT OR Apache-2.0`
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]
#![deny(missing_docs)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::multiple_unsafe_ops_per_block)]
#![deny(clippy::missing_const_for_fn)]

/// Codec specific functionality.
pub mod codec;
/// Constants.
pub mod consts;
/// Decoder specific functionality.
pub mod decoder;
/// Dictionary specific functionality.
pub mod dict;
/// Encoder specific functionality.
pub mod encoder;
/// Error handling.
pub mod error;
/// Filter graph specific functionality.
pub mod filter_graph;
/// Frame specific functionality.
pub mod frame;
/// Input/Output specific functionality.
pub mod io;
/// Limiter specific functionality.
pub mod limiter;
/// Logging specific functionality.
pub mod log;
/// Packet specific functionality.
pub mod packet;
/// Scalar specific functionality.
pub mod scalar;
/// Stream specific functionality.
pub mod stream;
/// Utility functionality.
pub mod utils;

/// The ffi module.
pub use ffmpeg_sys_next as ffi;

mod smart_object;
