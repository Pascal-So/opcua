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
    qualified_name::QualifiedName, service_types::impls::MessageInfo,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AliasNameDataType {
    pub alias_name: QualifiedName,
    pub referenced_nodes: Option<Vec<ExpandedNodeId>>,
}

impl MessageInfo for AliasNameDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::AliasNameDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<AliasNameDataType> for AliasNameDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.alias_name.byte_len();
        size += byte_len_array(&self.referenced_nodes);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.alias_name.encode(stream)?;
        size += write_array(stream, &self.referenced_nodes)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let alias_name = QualifiedName::decode(stream, decoding_options)?;
        let referenced_nodes: Option<Vec<ExpandedNodeId>> = read_array(stream, decoding_options)?;
        Ok(AliasNameDataType {
            alias_name,
            referenced_nodes,
        })
    }
}
