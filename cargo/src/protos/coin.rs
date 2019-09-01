// This file is generated by rust-protobuf 2.8.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `coin.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct Coin {
    // message fields
    pub public_key: ::std::vec::Vec<u8>,
    pub private_key: ::std::vec::Vec<u8>,
    pub wif: ::std::string::String,
    pub address: ::std::string::String,
    pub rel: ::std::string::String,
    pub base: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Coin {
    fn default() -> &'a Coin {
        <Coin as ::protobuf::Message>::default_instance()
    }
}

impl Coin {
    pub fn new() -> Coin {
        ::std::default::Default::default()
    }

    // bytes public_key = 1;


    pub fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }
    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public_key
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.public_key, ::std::vec::Vec::new())
    }

    // bytes private_key = 2;


    pub fn get_private_key(&self) -> &[u8] {
        &self.private_key
    }
    pub fn clear_private_key(&mut self) {
        self.private_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_private_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.private_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.private_key
    }

    // Take field
    pub fn take_private_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.private_key, ::std::vec::Vec::new())
    }

    // string wif = 3;


    pub fn get_wif(&self) -> &str {
        &self.wif
    }
    pub fn clear_wif(&mut self) {
        self.wif.clear();
    }

    // Param is passed by value, moved
    pub fn set_wif(&mut self, v: ::std::string::String) {
        self.wif = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wif(&mut self) -> &mut ::std::string::String {
        &mut self.wif
    }

    // Take field
    pub fn take_wif(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.wif, ::std::string::String::new())
    }

    // string address = 4;


    pub fn get_address(&self) -> &str {
        &self.address
    }
    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    // string rel = 5;


    pub fn get_rel(&self) -> &str {
        &self.rel
    }
    pub fn clear_rel(&mut self) {
        self.rel.clear();
    }

    // Param is passed by value, moved
    pub fn set_rel(&mut self, v: ::std::string::String) {
        self.rel = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rel(&mut self) -> &mut ::std::string::String {
        &mut self.rel
    }

    // Take field
    pub fn take_rel(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.rel, ::std::string::String::new())
    }

    // string base = 6;


    pub fn get_base(&self) -> &str {
        &self.base
    }
    pub fn clear_base(&mut self) {
        self.base.clear();
    }

    // Param is passed by value, moved
    pub fn set_base(&mut self, v: ::std::string::String) {
        self.base = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base(&mut self) -> &mut ::std::string::String {
        &mut self.base
    }

    // Take field
    pub fn take_base(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.base, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Coin {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.public_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.private_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.wif)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.rel)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.base)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.public_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.public_key);
        }
        if !self.private_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.private_key);
        }
        if !self.wif.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.wif);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.address);
        }
        if !self.rel.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.rel);
        }
        if !self.base.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.base);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.public_key.is_empty() {
            os.write_bytes(1, &self.public_key)?;
        }
        if !self.private_key.is_empty() {
            os.write_bytes(2, &self.private_key)?;
        }
        if !self.wif.is_empty() {
            os.write_string(3, &self.wif)?;
        }
        if !self.address.is_empty() {
            os.write_string(4, &self.address)?;
        }
        if !self.rel.is_empty() {
            os.write_string(5, &self.rel)?;
        }
        if !self.base.is_empty() {
            os.write_string(6, &self.base)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Coin {
        Coin::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    |m: &Coin| { &m.public_key },
                    |m: &mut Coin| { &mut m.public_key },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "private_key",
                    |m: &Coin| { &m.private_key },
                    |m: &mut Coin| { &mut m.private_key },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "wif",
                    |m: &Coin| { &m.wif },
                    |m: &mut Coin| { &mut m.wif },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    |m: &Coin| { &m.address },
                    |m: &mut Coin| { &mut m.address },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rel",
                    |m: &Coin| { &m.rel },
                    |m: &mut Coin| { &mut m.rel },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "base",
                    |m: &Coin| { &m.base },
                    |m: &mut Coin| { &mut m.base },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Coin>(
                    "Coin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Coin {
        static mut instance: ::protobuf::lazy::Lazy<Coin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Coin,
        };
        unsafe {
            instance.get(Coin::new)
        }
    }
}

impl ::protobuf::Clear for Coin {
    fn clear(&mut self) {
        self.public_key.clear();
        self.private_key.clear();
        self.wif.clear();
        self.address.clear();
        self.rel.clear();
        self.base.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Coin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Coin {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CoinList {
    // message fields
    pub coin: ::protobuf::RepeatedField<Coin>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CoinList {
    fn default() -> &'a CoinList {
        <CoinList as ::protobuf::Message>::default_instance()
    }
}

impl CoinList {
    pub fn new() -> CoinList {
        ::std::default::Default::default()
    }

    // repeated .Coin coin = 1;


    pub fn get_coin(&self) -> &[Coin] {
        &self.coin
    }
    pub fn clear_coin(&mut self) {
        self.coin.clear();
    }

    // Param is passed by value, moved
    pub fn set_coin(&mut self, v: ::protobuf::RepeatedField<Coin>) {
        self.coin = v;
    }

    // Mutable pointer to the field.
    pub fn mut_coin(&mut self) -> &mut ::protobuf::RepeatedField<Coin> {
        &mut self.coin
    }

    // Take field
    pub fn take_coin(&mut self) -> ::protobuf::RepeatedField<Coin> {
        ::std::mem::replace(&mut self.coin, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for CoinList {
    fn is_initialized(&self) -> bool {
        for v in &self.coin {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.coin)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.coin {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.coin {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CoinList {
        CoinList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Coin>>(
                    "coin",
                    |m: &CoinList| { &m.coin },
                    |m: &mut CoinList| { &mut m.coin },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CoinList>(
                    "CoinList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CoinList {
        static mut instance: ::protobuf::lazy::Lazy<CoinList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CoinList,
        };
        unsafe {
            instance.get(CoinList::new)
        }
    }
}

impl ::protobuf::Clear for CoinList {
    fn clear(&mut self) {
        self.coin.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CoinList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CoinList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TickerList {
    // message fields
    pub strings: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TickerList {
    fn default() -> &'a TickerList {
        <TickerList as ::protobuf::Message>::default_instance()
    }
}

impl TickerList {
    pub fn new() -> TickerList {
        ::std::default::Default::default()
    }

    // repeated string strings = 1;


    pub fn get_strings(&self) -> &[::std::string::String] {
        &self.strings
    }
    pub fn clear_strings(&mut self) {
        self.strings.clear();
    }

    // Param is passed by value, moved
    pub fn set_strings(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.strings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_strings(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.strings
    }

    // Take field
    pub fn take_strings(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.strings, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for TickerList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.strings)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.strings {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.strings {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TickerList {
        TickerList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "strings",
                    |m: &TickerList| { &m.strings },
                    |m: &mut TickerList| { &mut m.strings },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TickerList>(
                    "TickerList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TickerList {
        static mut instance: ::protobuf::lazy::Lazy<TickerList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TickerList,
        };
        unsafe {
            instance.get(TickerList::new)
        }
    }
}

impl ::protobuf::Clear for TickerList {
    fn clear(&mut self) {
        self.strings.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TickerList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TickerList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\ncoin.proto\"\x98\x01\n\x04Coin\x12\x1d\n\npublic_key\x18\x01\x20\x01\
    (\x0cR\tpublicKey\x12\x1f\n\x0bprivate_key\x18\x02\x20\x01(\x0cR\nprivat\
    eKey\x12\x10\n\x03wif\x18\x03\x20\x01(\tR\x03wif\x12\x18\n\x07address\
    \x18\x04\x20\x01(\tR\x07address\x12\x10\n\x03rel\x18\x05\x20\x01(\tR\x03\
    rel\x12\x12\n\x04base\x18\x06\x20\x01(\tR\x04base\"%\n\x08CoinList\x12\
    \x19\n\x04coin\x18\x01\x20\x03(\x0b2\x05.CoinR\x04coin\"&\n\nTickerList\
    \x12\x18\n\x07strings\x18\x01\x20\x03(\tR\x07stringsB-\n\x1acom.example.\
    wallet_flutterB\x06Protos\xa2\x02\x06Protosb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
