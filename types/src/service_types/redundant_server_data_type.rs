// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, encoding::*, node_ids::ObjectId, service_types::enums::ServerState,
    service_types::impls::MessageInfo, string::UAString,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RedundantServerDataType {
    pub server_id: UAString,
    pub service_level: u8,
    pub server_state: ServerState,
}

impl MessageInfo for RedundantServerDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::RedundantServerDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RedundantServerDataType> for RedundantServerDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.server_id.byte_len();
        size += self.service_level.byte_len();
        size += self.server_state.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.server_id.encode(stream)?;
        size += self.service_level.encode(stream)?;
        size += self.server_state.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let server_id = UAString::decode(stream, decoding_options)?;
        let service_level = u8::decode(stream, decoding_options)?;
        let server_state = ServerState::decode(stream, decoding_options)?;
        Ok(RedundantServerDataType {
            server_id,
            service_level,
            server_state,
        })
    }
}
