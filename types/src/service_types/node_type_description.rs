// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, encoding::*, node_id::ExpandedNodeId, node_ids::ObjectId,
    service_types::impls::MessageInfo, service_types::QueryDataDescription,
};

#[derive(Debug, Clone, PartialEq)]
pub struct NodeTypeDescription {
    pub type_definition_node: ExpandedNodeId,
    pub include_sub_types: bool,
    pub data_to_return: Option<Vec<QueryDataDescription>>,
}

impl MessageInfo for NodeTypeDescription {
    fn object_id(&self) -> ObjectId {
        ObjectId::NodeTypeDescription_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<NodeTypeDescription> for NodeTypeDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.type_definition_node.byte_len();
        size += self.include_sub_types.byte_len();
        size += byte_len_array(&self.data_to_return);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.type_definition_node.encode(stream)?;
        size += self.include_sub_types.encode(stream)?;
        size += write_array(stream, &self.data_to_return)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let type_definition_node = ExpandedNodeId::decode(stream, decoding_options)?;
        let include_sub_types = bool::decode(stream, decoding_options)?;
        let data_to_return: Option<Vec<QueryDataDescription>> =
            read_array(stream, decoding_options)?;
        Ok(NodeTypeDescription {
            type_definition_node,
            include_sub_types,
            data_to_return,
        })
    }
}
