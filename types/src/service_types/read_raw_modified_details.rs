// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{basic_types::*, date_time::DateTime, encoding::*};

#[derive(Debug, Clone, PartialEq)]
pub struct ReadRawModifiedDetails {
    pub is_read_modified: bool,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub num_values_per_node: u32,
    pub return_bounds: bool,
}

impl BinaryEncoder<ReadRawModifiedDetails> for ReadRawModifiedDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.is_read_modified.byte_len();
        size += self.start_time.byte_len();
        size += self.end_time.byte_len();
        size += self.num_values_per_node.byte_len();
        size += self.return_bounds.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.is_read_modified.encode(stream)?;
        size += self.start_time.encode(stream)?;
        size += self.end_time.encode(stream)?;
        size += self.num_values_per_node.encode(stream)?;
        size += self.return_bounds.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let is_read_modified = bool::decode(stream, decoding_options)?;
        let start_time = DateTime::decode(stream, decoding_options)?;
        let end_time = DateTime::decode(stream, decoding_options)?;
        let num_values_per_node = u32::decode(stream, decoding_options)?;
        let return_bounds = bool::decode(stream, decoding_options)?;
        Ok(ReadRawModifiedDetails {
            is_read_modified,
            start_time,
            end_time,
            num_values_per_node,
            return_bounds,
        })
    }
}
