// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock
// This file was autogenerated from Opc.Ua.NodeSet2.Part999.xml by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::{convert::TryFrom, str::FromStr};

#[allow(unused_imports)]
use crate::{
    address_space::{types::*, EventNotifier},
    prelude::{
        service_types::Argument, DataTypeId, ExtensionObject, LocalizedText, NodeId,
        ReferenceTypeId, UAString, Variant, VariantTypeId,
    },
};

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    add_datatype_1(address_space);
    add_variable_2(address_space);
}

fn add_datatype_1(address_space: &mut AddressSpace) {
    // DataType
    let name = "EnumeratedTestType";
    let node_id = NodeId::new(0, 398);
    let mut node = DataType::new(&node_id, name, name, false);
    node.set_description(LocalizedText::from(
        "A simple enumerated type used for testing.",
    ));
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 11886),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 29),
                &ReferenceTypeId::HasSubtype,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_variable_2(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumValues";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 11886);
    let node = Variable::new_data_value(
        &node_id,
        name,
        name,
        NodeId::new(0, 7594),
        Some(1),
        None,
        value,
    );
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 68),
                &ReferenceTypeId::HasTypeDefinition,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 78),
                &ReferenceTypeId::HasModellingRule,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 398),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}
