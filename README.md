# BPack

A Rust library for compressing byte slices of String and str types.

`bpack` is a lossless compression library that is meant to reduce size of data (up to 65535 bytes) containing 95 ascii characters ranging from 32 to 126 (inclusive). In another words it is meant to work with byte slices of the `String` and/or `str` types. It compresses 65535 bytes in about 12ms and decompresses back in about 57ms on Intel(R) Core(TM) i5-4590 CPU @ 3.30GHz. Makes sense to use this library if data size is in range from ~2000 to 65535 bytes.

## Usage
Cargo.toml
```
[dependencies]
bpack = "0.1.0"
```



```rust
use bpack::{pack, unpack};

fn main() {
    let data = "some very long string".as_bytes();

    if let Some(packed) = pack(data) {
        let unpacked = unpack(packed);
        assert_eq!(data, unpacked.as_slice());
    }
}

```

