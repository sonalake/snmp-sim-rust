use crate::snmp::codec::generic_snmp_message::GenericSnmpMessage;
use crate::snmp::codec::snmp_codec_error::CodecError;
use bytes::Buf;
use bytes::BytesMut;
use num_traits::ToPrimitive;
use rasn::ber::de::DecoderOptions;
use rasn::Decode;
use tokio_util::codec::{Decoder, Encoder};

#[derive(Default, Debug)]
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

pub fn decode<T: Decode>(decoder: &mut rasn::ber::de::Decoder) -> Result<T, rasn::ber::de::Error> {
    T::decode(decoder)
}

impl Decoder for SnmpCodec {
    type Item = GenericSnmpMessage;
    type Error = CodecError;

    #[tracing::instrument(level = "info", name = "SnmpCodec::decode", skip(self, data))]
    fn decode(&mut self, data: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if data.is_empty() {
            if data.capacity() < 128 {
                data.reserve(128);
            }
            // more bytes needs to be read
            return Ok(None);
        }

        tracing::debug!("Received data: {:02X?}", data.as_ref());

        let mut decoder = rasn::ber::de::Decoder::new(data, DecoderOptions::ber());
        let version_header: Result<rasn_snmp::SnmpMessageHeader, CodecError> =
            decode(&mut decoder).map_err(CodecError::Decoder);
        let version = version_header
            .unwrap()
            .version
            .to_u32()
            .unwrap_or(std::u32::MIN);
        tracing::debug!("Version: {:02X?}", version);

        let mut decoder = rasn::ber::de::Decoder::new(data, DecoderOptions::ber());

        let result = match version {
            SnmpCodec::SNMP_VERSION1 => decode(&mut decoder)
                .map(|de| Some(GenericSnmpMessage::V1Message(de)))
                .map_err(CodecError::Decoder),

            SnmpCodec::SNMP_VERSION2 => decode(&mut decoder)
                .map(|de| Some(GenericSnmpMessage::V2Message(de)))
                .map_err(CodecError::Decoder),

            SnmpCodec::SNMP_VERSION3 => decode(&mut decoder)
                .map(|de| Some(GenericSnmpMessage::V3Message(Box::new(de))))
                .map_err(CodecError::Decoder),

            ver_u32 => Err(CodecError::InvalidVersion(ver_u32)),
        };

        // todo: rasn-snmp implementation issue
        // the data needs to be consumed by the decoder => Decoder::new has to be
        // changed to accept the data as immutable then the input data needs to
        // be shorten by the decoced data length => the snmp-parser crate
        // implements that feature correctly
        data.advance(decoder.decoded_len());

        result
    }
}

impl Encoder<GenericSnmpMessage> for SnmpCodec {
    type Error = CodecError;

    #[tracing::instrument(level = "info", name = "SnmpCodec::encode", skip(self, message))]
    fn encode(&mut self, message: GenericSnmpMessage, result: &mut BytesMut) -> Result<(), Self::Error> {
        let response = match message {
            GenericSnmpMessage::V1Message(content) => rasn::ber::encode(&content)
                .map(|data| {
                    // Reserve space in the buffer
                    result.reserve(data.len());

                    // Write the encoded message into the buffer
                    result.extend_from_slice(&data);
                })
                .map_err(CodecError::Encoder),

            GenericSnmpMessage::V2Message(content) => rasn::ber::encode(&content)
                .map(|data| {
                    // Reserve space in the buffer
                    result.reserve(data.len());

                    // Write the encoded message into the buffer
                    result.extend_from_slice(&data);
                })
                .map_err(CodecError::Encoder),

            GenericSnmpMessage::V3Message(content) => rasn::ber::encode(&content)
                .map(|data| {
                    // Reserve space in the buffer
                    result.reserve(data.len());

                    // Write the encoded message into the buffer
                    result.extend_from_slice(&data);
                })
                .map_err(CodecError::Encoder),
        };
        tracing::debug!("Sending data: {:02X?}", result.as_ref());
        response
    }
}
