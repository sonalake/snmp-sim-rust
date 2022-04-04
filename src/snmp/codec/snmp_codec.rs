use crate::snmp::codec::snmp_codec_error::CodecError;
use arrayref::array_ref;
use bytes::Buf;
use bytes::BytesMut;
use num_traits::ToPrimitive;
use rasn::ber::de::DecoderOptions;
use rasn::Decode;
use tokio_util::codec::{Decoder, Encoder};

use rasn_snmp::v3::Message as SnmpV3Message;
use rasn_snmp::{v1::Message as SnmpV1Message, v1::Pdus as SnmpV1Pdus};
use rasn_snmp::{v2::Pdus as SnmpV2Pdus, v2c::Message as SnmpV2CMessage};

#[derive(Debug)]
pub enum GenericSnmpMessage {
    V1Message(SnmpV1Message<SnmpV1Pdus>),
    V2Message(SnmpV2CMessage<SnmpV2Pdus>),
    V3Message(SnmpV3Message),
}

#[derive(Default)]
pub struct SnmpCodec {}

mod header {
    use rasn::{types::Integer, AsnType, Decode};

    #[derive(AsnType, Debug, Clone, Decode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct HeaderVersion {
        pub version: Integer,
    }
}

impl SnmpCodec {
    pub const SNMP_VERSION1: u32 = 0;
    pub const SNMP_VERSION2: u32 = 1;
    pub const SNMP_VERSION3: u32 = 3;

    pub fn new() -> Self {
        SnmpCodec {}
    }
}

fn pop(header: &[u8]) -> &[u8; 3] {
    array_ref!(header, 2, 3)
}

pub fn decode<T: Decode>(decoder: &mut rasn::ber::de::Decoder) -> Result<T, rasn::ber::de::Error> {
    T::decode(decoder)
}

impl Decoder for SnmpCodec {
    type Item = GenericSnmpMessage;
    type Error = CodecError;

    #[tracing::instrument(name = "SnmpCodec::decode", level = "info", skip(self, data))]
    fn decode(&mut self, data: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if data.is_empty() {
            // more bytes needs to be read
            return Ok(None);
        }

        tracing::debug!("Received data: {:02X?}", data.as_ref());
        let version: rasn::types::Integer = rasn::ber::decode(pop(&data)).map_err(|e| CodecError::Decoder(e))?;

        let mut decoder = rasn::ber::de::Decoder::new(data, DecoderOptions::der());

        let result = match version.to_u32().unwrap_or(std::u32::MIN) {
            SnmpCodec::SNMP_VERSION1 => decode(&mut decoder)
                .map(|de| Some(GenericSnmpMessage::V1Message(de)))
                .map_err(|e| CodecError::Decoder(e)),

            SnmpCodec::SNMP_VERSION2 => decode(&mut decoder)
                .map(|de| Some(GenericSnmpMessage::V2Message(de)))
                .map_err(|e| CodecError::Decoder(e)),

            SnmpCodec::SNMP_VERSION3 => decode(&mut decoder)
                .map(|de| Some(GenericSnmpMessage::V3Message(de)))
                .map_err(|e| CodecError::Decoder(e)),

            ver_u32 => Err(CodecError::InvalidVersion(ver_u32)),
        };

        // todo: rasn-snmp implementation issue
        // the data needs to be consumed by the decoder => Decoder::new has to be
        // changed to accept the data as immutable then the input data needs to
        // be shorten by the decoced data length => the snmp-parser crate
        // implements that feature correctly
        #[allow(mutable_borrow_reservation_conflict)]
        data.advance(decoder.decoded_len());

        result
    }
}

impl Encoder<GenericSnmpMessage> for SnmpCodec {
    type Error = CodecError;

    #[tracing::instrument(name = "SnmpCodec::encode", level = "info", skip(self, message))]
    fn encode(&mut self, message: GenericSnmpMessage, result: &mut BytesMut) -> Result<(), Self::Error> {
        match message {
            GenericSnmpMessage::V1Message(content) => rasn::ber::encode(&content)
                .map(|data| {
                    // Reserve space in the buffer
                    result.reserve(data.len());

                    // Write the encoded message into the buffer
                    result.extend_from_slice(&data);
                })
                .map_err(|e| CodecError::Encoder(e)),

            GenericSnmpMessage::V2Message(content) => rasn::ber::encode(&content)
                .map(|data| {
                    // Reserve space in the buffer
                    result.reserve(data.len());

                    // Write the encoded message into the buffer
                    result.extend_from_slice(&data);
                })
                .map_err(|e| CodecError::Encoder(e)),

            GenericSnmpMessage::V3Message(content) => rasn::ber::encode(&content)
                .map(|data| {
                    // Reserve space in the buffer
                    result.reserve(data.len());

                    // Write the encoded message into the buffer
                    result.extend_from_slice(&data);
                })
                .map_err(|e| CodecError::Encoder(e)),
        }
    }
}
