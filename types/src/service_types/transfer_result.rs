// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, encoding::*, node_ids::ObjectId, service_types::impls::MessageInfo,
    status_codes::StatusCode,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TransferResult {
    pub status_code: StatusCode,
    pub available_sequence_numbers: Option<Vec<u32>>,
}

impl MessageInfo for TransferResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::TransferResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<TransferResult> for TransferResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += byte_len_array(&self.available_sequence_numbers);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += write_array(stream, &self.available_sequence_numbers)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream, decoding_options)?;
        let available_sequence_numbers: Option<Vec<u32>> = read_array(stream, decoding_options)?;
        Ok(TransferResult {
            status_code,
            available_sequence_numbers,
        })
    }
}
