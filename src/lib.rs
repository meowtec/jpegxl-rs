/*
This file is part of jpegxl-rs.

jpegxl-rs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

jpegxl-rs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with jpegxl-rs.  If not, see <https://www.gnu.org/licenses/>.
*/

#![warn(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! [![Documentation](https://docs.rs/jpegxl-rs/badge.svg)](https://docs.rs/jpegxl-rs/)
//! [![Crates.io](https://img.shields.io/crates/v/jpegxl-rs.svg)](https://crates.io/crates/jpegxl-rs)
//! [![CI](https://github.com/inflation/jpegxl-rs/workflows/CI/badge.svg)](
//! https://github.com/inflation/jpegxl-rs/actions?query=workflow%3ACI)
//! [![Coverage Status](https://coveralls.io/repos/github/inflation/jpegxl-rs/badge.svg?branch=master)](
//! https://coveralls.io/github/inflation/jpegxl-rs?branch=master)
//! [![License: GPL-3.0-or-later](https://img.shields.io/crates/l/jpegxl-rs)](
//! https://github.com/inflation/jpegxl-rs/blob/master/LICENSE)
//!
//! A safe JPEGXL wrapper over `jpeg-xl` library. Check out the original [library](https://gitlab.com/wg1/jpeg-xl)
//! and the [bindings](https://github.com/inflation/jpegxl-sys).
//!
//! # Building
//!
//! The library build `jpeg-xl` and link to `lib(std)c++` by default. Optionally, you can use `system-jxl` feature to
//! dynamically link to it. If you don't have it in the default include and library paths,
//! set them with `DEP_JXL_INCLUDE` and `DEP_JXL_LIB` respectively.
//!
//! If you don't want to depend on C++ standard library, disable `threads` feature.
//!
//! # Usage
//!
//! ## Decoding
//!
//! ```
//! # use jpegxl_rs::*;
//! # || -> Result<(), Box<dyn std::error::Error>> {
//! let mut decoder = decoder_builder().build()?;
//!
//! // Use multithread
//! use jpegxl_rs::ThreadsRunner;
//! let runner = ThreadsRunner::default();
//! let mut decoder = decoder_builder()
//!                       .parallel_runner(&runner)
//!                       .build()?;
//!
//! // Customize pixel format
//! let mut decoder = decoder_builder()
//!                       .num_channels(3)
//!                       .endian(Endianness::Big)
//!                       .align(8)
//!                       .build()?;
//!
//! // Set custom memory manager
//! use jpegxl_rs::memory::MallocManager;
//! let manager = MallocManager::default();
//! let mut decoder = decoder_builder()
//!                       .memory_manager(&manager)
//!                       .build()?;
//! # Ok(()) };
//! ```
//!
//! ## Encoding
//!
//! ```
//! # use jpegxl_rs::{encoder_builder, encode::EncoderResult};
//! # || -> Result<(), Box<dyn std::error::Error>> {
//! use image::io::Reader as ImageReader;
//! let sample = ImageReader::open("test/sample.png")?.decode()?.to_rgba16();
//! let mut encoder = encoder_builder().build()?;
//! let buffer: EncoderResult<f32> = encoder.encode(&sample, sample.width(), sample.height())?;
//! # Ok(()) };
//! ```
//!
//! Set encoder options
//!
//! ```
//! # || -> Result<(), Box<dyn std::error::Error>> {
//! # use jpegxl_rs::{*, encode::EncoderSpeed};
//! let mut encoder = encoder_builder()
//!                     .lossless(true)
//!                     .speed(EncoderSpeed::Falcon)
//!                     .build()?;
//! // You can change the settings after initialization
//! encoder.set_lossless(false);
//! encoder.set_quality(3.0);
//! # Ok(()) };
//! ```
//!
//! ## [`image`](https://crates.io/crates/image) crate integration
//!
//! The integration is enabled by default. If you don't need it, disable `image-support` feature.
//!
//! ```
//! # || -> Result<(), Box<dyn std::error::Error>> {
//! use jpegxl_rs::image::ToDynamic;
//! use jpegxl_rs::decoder_builder;
//! use image::DynamicImage;
//!
//! let sample = std::fs::read("test/sample.jxl")?;
//! let decoder = decoder_builder().build()?;
//! let img = decoder.decode::<u8>(&sample)?.into_dynamic_image();       
//! # Ok(()) };
//! ```

mod common;
pub mod decode;
pub mod encode;
mod errors;
pub mod memory;
pub mod parallel;

#[cfg(feature = "image-support")]
pub mod image;

pub use common::Endianness;
pub use decode::decoder_builder;
pub use encode::encoder_builder;
pub use errors::{DecodeError, EncodeError};
pub use parallel::ThreadsRunner;
