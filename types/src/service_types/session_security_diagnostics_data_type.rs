// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, byte_string::ByteString, encoding::*, node_id::NodeId, node_ids::ObjectId,
    service_types::enums::MessageSecurityMode, service_types::impls::MessageInfo, string::UAString,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SessionSecurityDiagnosticsDataType {
    pub session_id: NodeId,
    pub client_user_id_of_session: UAString,
    pub client_user_id_history: Option<Vec<UAString>>,
    pub authentication_mechanism: UAString,
    pub encoding: UAString,
    pub transport_protocol: UAString,
    pub security_mode: MessageSecurityMode,
    pub security_policy_uri: UAString,
    pub client_certificate: ByteString,
}

impl MessageInfo for SessionSecurityDiagnosticsDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::SessionSecurityDiagnosticsDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SessionSecurityDiagnosticsDataType> for SessionSecurityDiagnosticsDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.session_id.byte_len();
        size += self.client_user_id_of_session.byte_len();
        size += byte_len_array(&self.client_user_id_history);
        size += self.authentication_mechanism.byte_len();
        size += self.encoding.byte_len();
        size += self.transport_protocol.byte_len();
        size += self.security_mode.byte_len();
        size += self.security_policy_uri.byte_len();
        size += self.client_certificate.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.session_id.encode(stream)?;
        size += self.client_user_id_of_session.encode(stream)?;
        size += write_array(stream, &self.client_user_id_history)?;
        size += self.authentication_mechanism.encode(stream)?;
        size += self.encoding.encode(stream)?;
        size += self.transport_protocol.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.security_policy_uri.encode(stream)?;
        size += self.client_certificate.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let session_id = NodeId::decode(stream, decoding_options)?;
        let client_user_id_of_session = UAString::decode(stream, decoding_options)?;
        let client_user_id_history: Option<Vec<UAString>> = read_array(stream, decoding_options)?;
        let authentication_mechanism = UAString::decode(stream, decoding_options)?;
        let encoding = UAString::decode(stream, decoding_options)?;
        let transport_protocol = UAString::decode(stream, decoding_options)?;
        let security_mode = MessageSecurityMode::decode(stream, decoding_options)?;
        let security_policy_uri = UAString::decode(stream, decoding_options)?;
        let client_certificate = ByteString::decode(stream, decoding_options)?;
        Ok(SessionSecurityDiagnosticsDataType {
            session_id,
            client_user_id_of_session,
            client_user_id_history,
            authentication_mechanism,
            encoding,
            transport_protocol,
            security_mode,
            security_policy_uri,
            client_certificate,
        })
    }
}
