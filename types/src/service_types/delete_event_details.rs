// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{basic_types::*, byte_string::ByteString, encoding::*, node_id::NodeId};

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteEventDetails {
    pub node_id: NodeId,
    pub event_ids: Option<Vec<ByteString>>,
}

impl BinaryEncoder<DeleteEventDetails> for DeleteEventDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += byte_len_array(&self.event_ids);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += write_array(stream, &self.event_ids)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream, decoding_options)?;
        let event_ids: Option<Vec<ByteString>> = read_array(stream, decoding_options)?;
        Ok(DeleteEventDetails { node_id, event_ids })
    }
}
