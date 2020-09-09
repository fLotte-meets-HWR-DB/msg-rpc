use serde::Serialize;
use rmp_serde::Serializer;
use byteorder::{BigEndian, ByteOrder};
use crc::crc32;
use std::convert::TryInto;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum DeserializeError {
    LengthError,
    ChecksumError,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Message {
    length: u32,
    method: [u8; 4],
    data: Vec<u8>,
    pub crc: Option<u32>,
}

impl Message {
    /// Creates a new message
    pub fn new(method: [u8; 4], data: Vec<u8>) -> Self {
        Self {
            length: (12 + data.len()) as u32,
            method,
            data,
            crc: None,
        }
    }

    /// Creates a new message with data being a serializable type
    pub fn new_with_serialize(method: [u8; 4], data: impl Serialize) -> Self {
        let mut buf = Vec::new();
        data.serialize(&mut Serializer::new(&mut buf)).unwrap();
        Self {
            method,
            length: (12 + buf.len()) as u32,
            data: buf,
            crc: None,
        }
    }

    /// Deserializes a vector of bytes into the message
    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Self, DeserializeError> {

        if bytes.len() < 4 {
            return Err(DeserializeError::LengthError)
        }

        let length = BigEndian::read_u32(&bytes[0..4]);

        if bytes.len() != length as usize {
            return Err(DeserializeError::LengthError)
        }

        let crc = BigEndian::read_u32(&bytes[(length as usize - 4)..(length as usize)]);
        let calc_crc = crc32::checksum_ieee(&bytes[0..(length as usize - 4)]);

        if calc_crc != crc {
            return Err(DeserializeError::ChecksumError)
        }

        let method: [u8; 4] = bytes[4..8].try_into().unwrap();
        let data = bytes[8..(length as usize - 4)].to_vec();

        Ok(Self {
            length,
            method,
            data,
            crc: Some(crc)
        })
    }

    /// Returns the serialized bytes version
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut length_raw = [0u8; 4];
        BigEndian::write_u32(&mut length_raw, self.length);
        data.append(&mut length_raw.to_vec());
        data.append(&mut self.method.clone().to_vec());
        data.append(&mut self.data.clone());
        let crc_sum = crc32::checksum_ieee(&data);
        let mut checksum_raw = [0u8; 4];
        BigEndian::write_u32(&mut checksum_raw, crc_sum);
        data.append(&mut checksum_raw.to_vec());

        data
    }
}
