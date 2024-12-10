use flate2::{write::DeflateEncoder, Compression};
use std::io::{Read, Write};

pub fn serialize_and_compress<T>(data: &T) -> Vec<u8>
where
    T: serde::Serialize,
{
    let buffer = bincode::serialize(data).expect("Serialization failed");

    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&buffer).expect("Failed to compress");
    encoder.finish().expect("Failed to finish compression")
}

pub fn decompress_and_deserialize<T>(data: &[u8]) -> T
where
    T: serde::de::DeserializeOwned,
{
    let mut decoder = flate2::read::DeflateDecoder::new(data);
    let mut buffer = Vec::new();
    decoder
        .read_to_end(&mut buffer)
        .expect("Failed to decompress");
    bincode::deserialize(&buffer).expect("Deserialization failed")
}
