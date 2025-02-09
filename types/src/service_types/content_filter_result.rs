// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, diagnostic_info::DiagnosticInfo, encoding::*, node_ids::ObjectId,
    service_types::impls::MessageInfo, service_types::ContentFilterElementResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ContentFilterResult {
    pub element_results: Option<Vec<ContentFilterElementResult>>,
    pub element_diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for ContentFilterResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::ContentFilterResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ContentFilterResult> for ContentFilterResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.element_results);
        size += byte_len_array(&self.element_diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.element_results)?;
        size += write_array(stream, &self.element_diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let element_results: Option<Vec<ContentFilterElementResult>> =
            read_array(stream, decoding_options)?;
        let element_diagnostic_infos: Option<Vec<DiagnosticInfo>> =
            read_array(stream, decoding_options)?;
        Ok(ContentFilterResult {
            element_results,
            element_diagnostic_infos,
        })
    }
}
