// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{basic_types::*, encoding::*, string::UAString};

#[derive(Debug, Clone, PartialEq)]
pub struct MdnsDiscoveryConfiguration {
    pub mdns_server_name: UAString,
    pub server_capabilities: Option<Vec<UAString>>,
}

impl BinaryEncoder<MdnsDiscoveryConfiguration> for MdnsDiscoveryConfiguration {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.mdns_server_name.byte_len();
        size += byte_len_array(&self.server_capabilities);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.mdns_server_name.encode(stream)?;
        size += write_array(stream, &self.server_capabilities)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let mdns_server_name = UAString::decode(stream, decoding_options)?;
        let server_capabilities: Option<Vec<UAString>> = read_array(stream, decoding_options)?;
        Ok(MdnsDiscoveryConfiguration {
            mdns_server_name,
            server_capabilities,
        })
    }
}
