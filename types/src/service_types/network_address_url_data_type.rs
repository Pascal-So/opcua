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
pub struct NetworkAddressUrlDataType {
    pub network_interface: UAString,
    pub url: UAString,
}

impl BinaryEncoder<NetworkAddressUrlDataType> for NetworkAddressUrlDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.network_interface.byte_len();
        size += self.url.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.network_interface.encode(stream)?;
        size += self.url.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let network_interface = UAString::decode(stream, decoding_options)?;
        let url = UAString::decode(stream, decoding_options)?;
        Ok(NetworkAddressUrlDataType {
            network_interface,
            url,
        })
    }
}
