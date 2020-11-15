//! `bpack` is a lossless compression library that is meant
//! to reduce size of data (up to 65535 bytes) containing
//! 95 ascii characters ranging from 32 to 126 (inclusive).
//! In another words it is meant to work with byte slices of the
//! `String` and/or `str` types.
//! It compresses 65535 bytes in about 12ms and decompresses back
//! in about 57ms on Intel(R) Core(TM) i5-4590 CPU @ 3.30GHz.
//! Makes sense to use this library if data size is in range
//! from ~2000 to 65535 bytes.

mod pack;
mod unpack;
mod utils;

pub(self) use utils::{get_bit, set_bit, set_bit_16};

pub use pack::pack;

pub use unpack::unpack;

