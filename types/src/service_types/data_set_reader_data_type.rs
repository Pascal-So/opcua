// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, encoding::*, extension_object::ExtensionObject, node_ids::ObjectId,
    service_types::enums::DataSetFieldContentMask, service_types::enums::MessageSecurityMode,
    service_types::impls::MessageInfo, service_types::DataSetMetaDataType,
    service_types::EndpointDescription, service_types::KeyValuePair, string::UAString,
    variant::Variant,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DataSetReaderDataType {
    pub name: UAString,
    pub enabled: bool,
    pub publisher_id: Variant,
    pub writer_group_id: u16,
    pub data_set_writer_id: u16,
    pub data_set_meta_data: DataSetMetaDataType,
    pub data_set_field_content_mask: DataSetFieldContentMask,
    pub message_receive_timeout: f64,
    pub key_frame_count: u32,
    pub header_layout_uri: UAString,
    pub security_mode: MessageSecurityMode,
    pub security_group_id: UAString,
    pub security_key_services: Option<Vec<EndpointDescription>>,
    pub data_set_reader_properties: Option<Vec<KeyValuePair>>,
    pub transport_settings: ExtensionObject,
    pub message_settings: ExtensionObject,
    pub subscribed_data_set: ExtensionObject,
}

impl MessageInfo for DataSetReaderDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::DataSetReaderDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DataSetReaderDataType> for DataSetReaderDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.name.byte_len();
        size += self.enabled.byte_len();
        size += self.publisher_id.byte_len();
        size += self.writer_group_id.byte_len();
        size += self.data_set_writer_id.byte_len();
        size += self.data_set_meta_data.byte_len();
        size += self.data_set_field_content_mask.byte_len();
        size += self.message_receive_timeout.byte_len();
        size += self.key_frame_count.byte_len();
        size += self.header_layout_uri.byte_len();
        size += self.security_mode.byte_len();
        size += self.security_group_id.byte_len();
        size += byte_len_array(&self.security_key_services);
        size += byte_len_array(&self.data_set_reader_properties);
        size += self.transport_settings.byte_len();
        size += self.message_settings.byte_len();
        size += self.subscribed_data_set.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.name.encode(stream)?;
        size += self.enabled.encode(stream)?;
        size += self.publisher_id.encode(stream)?;
        size += self.writer_group_id.encode(stream)?;
        size += self.data_set_writer_id.encode(stream)?;
        size += self.data_set_meta_data.encode(stream)?;
        size += self.data_set_field_content_mask.encode(stream)?;
        size += self.message_receive_timeout.encode(stream)?;
        size += self.key_frame_count.encode(stream)?;
        size += self.header_layout_uri.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.security_group_id.encode(stream)?;
        size += write_array(stream, &self.security_key_services)?;
        size += write_array(stream, &self.data_set_reader_properties)?;
        size += self.transport_settings.encode(stream)?;
        size += self.message_settings.encode(stream)?;
        size += self.subscribed_data_set.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let name = UAString::decode(stream, decoding_options)?;
        let enabled = bool::decode(stream, decoding_options)?;
        let publisher_id = Variant::decode(stream, decoding_options)?;
        let writer_group_id = u16::decode(stream, decoding_options)?;
        let data_set_writer_id = u16::decode(stream, decoding_options)?;
        let data_set_meta_data = DataSetMetaDataType::decode(stream, decoding_options)?;
        let data_set_field_content_mask =
            DataSetFieldContentMask::decode(stream, decoding_options)?;
        let message_receive_timeout = f64::decode(stream, decoding_options)?;
        let key_frame_count = u32::decode(stream, decoding_options)?;
        let header_layout_uri = UAString::decode(stream, decoding_options)?;
        let security_mode = MessageSecurityMode::decode(stream, decoding_options)?;
        let security_group_id = UAString::decode(stream, decoding_options)?;
        let security_key_services: Option<Vec<EndpointDescription>> =
            read_array(stream, decoding_options)?;
        let data_set_reader_properties: Option<Vec<KeyValuePair>> =
            read_array(stream, decoding_options)?;
        let transport_settings = ExtensionObject::decode(stream, decoding_options)?;
        let message_settings = ExtensionObject::decode(stream, decoding_options)?;
        let subscribed_data_set = ExtensionObject::decode(stream, decoding_options)?;
        Ok(DataSetReaderDataType {
            name,
            enabled,
            publisher_id,
            writer_group_id,
            data_set_writer_id,
            data_set_meta_data,
            data_set_field_content_mask,
            message_receive_timeout,
            key_frame_count,
            header_layout_uri,
            security_mode,
            security_group_id,
            security_key_services,
            data_set_reader_properties,
            transport_settings,
            message_settings,
            subscribed_data_set,
        })
    }
}
