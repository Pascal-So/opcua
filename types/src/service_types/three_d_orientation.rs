// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{basic_types::*, encoding::*};

#[derive(Debug, Clone, PartialEq)]
pub struct ThreeDOrientation {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl BinaryEncoder<ThreeDOrientation> for ThreeDOrientation {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.a.byte_len();
        size += self.b.byte_len();
        size += self.c.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.a.encode(stream)?;
        size += self.b.encode(stream)?;
        size += self.c.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let a = f64::decode(stream, decoding_options)?;
        let b = f64::decode(stream, decoding_options)?;
        let c = f64::decode(stream, decoding_options)?;
        Ok(ThreeDOrientation { a, b, c })
    }
}
