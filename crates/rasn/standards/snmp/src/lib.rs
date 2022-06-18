#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

pub mod v1;
pub mod v2;
pub mod v2c;
pub mod v3;

use rasn::{types::Integer, AsnType, Decode, Encode};

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SnmpMessageHeader {
    pub version: Integer,
}
