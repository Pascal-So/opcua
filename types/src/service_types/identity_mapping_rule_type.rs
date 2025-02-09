// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, encoding::*, node_ids::ObjectId, service_types::enums::IdentityCriteriaType,
    service_types::impls::MessageInfo, string::UAString,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IdentityMappingRuleType {
    pub criteria_type: IdentityCriteriaType,
    pub criteria: UAString,
}

impl MessageInfo for IdentityMappingRuleType {
    fn object_id(&self) -> ObjectId {
        ObjectId::IdentityMappingRuleType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<IdentityMappingRuleType> for IdentityMappingRuleType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.criteria_type.byte_len();
        size += self.criteria.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.criteria_type.encode(stream)?;
        size += self.criteria.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let criteria_type = IdentityCriteriaType::decode(stream, decoding_options)?;
        let criteria = UAString::decode(stream, decoding_options)?;
        Ok(IdentityMappingRuleType {
            criteria_type,
            criteria,
        })
    }
}
