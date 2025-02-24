// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ir::cfg;
use num_derive::FromPrimitive;

/// Data stream encoding/decoding version
pub const DEFAULT_VERSION: u8 = 0;

/// Parameter type list and the representation of type code
/// in the input schema.
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive)]
pub enum ParamType {
    U8 = 0,
    I8 = 1,
    U16 = 2,
    I16 = 3,
    U32 = 4,
    I32 = 5,
    U64 = 6,
    I64 = 7,
    U128 = 8,
    I128 = 9,
    Bool = 10,
    Str = 11,
    Parampack = 12,

    // Array params
    U8Array = 32,
    I8Array = 33,
    U16Array = 34,
    I16Array = 35,
    U32Array = 36,
    I32Array = 37,
    U64Array = 38,
    I64Array = 39,
    U128Array = 40,
    I128Array = 41,
    BoolArray = 42,
    StrArray = 43,

    // Map params
    StrU8Map = 64,
    StrI8Map = 65,
    StrU16Map = 66,
    StrI16Map = 67,
    StrU32Map = 68,
    StrI32Map = 69,
    StrU64Map = 70,
    StrI64Map = 71,
    StrU128Map = 72,
    StrI128Map = 73,
    StrBoolMap = 74,
    StrStrMap = 75,
}

impl ParamType {
    pub fn value(&self) -> String {
        format!("{self:?}")
    }

    pub fn get_encode_func_name(&self) -> String {
        format!("data_stream_encode_{}", self.value()).to_lowercase()
    }

    pub fn get_decode_func_name(&self) -> String {
        format!("data_stream_decode_{}", self.value()).to_lowercase()
    }
}

/// ValidParamType defines the valid contract method parameter types.
pub trait ValidParamType {
    fn is_valid_param_type(&self) -> bool;
}

impl From<ParamType> for u8 {
    fn from(ty: ParamType) -> Self {
        ty as u8
    }
}

impl TryInto<ParamType> for cfg::Type {
    type Error = String;

    fn try_into(self) -> Result<ParamType, Self::Error> {
        let err = Err(format!("invalid type to param type: {self:?}"));
        match self {
            cfg::Type::Primitive(prim_ty) => match prim_ty {
                cfg::PrimitiveType::Str => Ok(ParamType::Str),
                cfg::PrimitiveType::Bool => Ok(ParamType::Bool),
                cfg::PrimitiveType::Void => err,
                cfg::PrimitiveType::Int(int_ty) => match int_ty {
                    cfg::IntType::I8 => Ok(ParamType::I8),
                    cfg::IntType::I16 => Ok(ParamType::I16),
                    cfg::IntType::I32 => Ok(ParamType::I32),
                    cfg::IntType::I64 => Ok(ParamType::I64),
                    cfg::IntType::I128 => Ok(ParamType::I128),
                    cfg::IntType::U8 => Ok(ParamType::U8),
                    cfg::IntType::U16 => Ok(ParamType::U16),
                    cfg::IntType::U32 => Ok(ParamType::U32),
                    cfg::IntType::U64 => Ok(ParamType::U64),
                    cfg::IntType::U128 => Ok(ParamType::U128),
                },
            },
            cfg::Type::Map { key, value } => {
                if key.is_string() {
                    match value.as_ref() {
                        cfg::Type::Primitive(prim_ty) => match prim_ty {
                            cfg::PrimitiveType::Str => Ok(ParamType::StrStrMap),
                            cfg::PrimitiveType::Bool => Ok(ParamType::StrBoolMap),
                            cfg::PrimitiveType::Void => err,
                            cfg::PrimitiveType::Int(int_ty) => match int_ty {
                                cfg::IntType::I8 => Ok(ParamType::StrI8Map),
                                cfg::IntType::I16 => Ok(ParamType::StrI16Map),
                                cfg::IntType::I32 => Ok(ParamType::StrI32Map),
                                cfg::IntType::I64 => Ok(ParamType::StrI64Map),
                                cfg::IntType::I128 => Ok(ParamType::StrI128Map),
                                cfg::IntType::U8 => Ok(ParamType::StrU8Map),
                                cfg::IntType::U16 => Ok(ParamType::StrU16Map),
                                cfg::IntType::U32 => Ok(ParamType::StrU32Map),
                                cfg::IntType::U64 => Ok(ParamType::StrU64Map),
                                cfg::IntType::U128 => Ok(ParamType::StrU128Map),
                            },
                        },
                        _ => err,
                    }
                } else {
                    err
                }
            }
            cfg::Type::Array { elem, len: _ } => match elem.as_ref() {
                cfg::Type::Primitive(prim_ty) => match prim_ty {
                    cfg::PrimitiveType::Str => Ok(ParamType::StrArray),
                    cfg::PrimitiveType::Bool => Ok(ParamType::BoolArray),
                    cfg::PrimitiveType::Void => err,
                    cfg::PrimitiveType::Int(int_ty) => match int_ty {
                        cfg::IntType::I8 => Ok(ParamType::I8Array),
                        cfg::IntType::I16 => Ok(ParamType::I16Array),
                        cfg::IntType::I32 => Ok(ParamType::I32Array),
                        cfg::IntType::I64 => Ok(ParamType::I64Array),
                        cfg::IntType::I128 => Ok(ParamType::I128Array),
                        cfg::IntType::U8 => Ok(ParamType::U8Array),
                        cfg::IntType::U16 => Ok(ParamType::U16Array),
                        cfg::IntType::U32 => Ok(ParamType::U32Array),
                        cfg::IntType::U64 => Ok(ParamType::U64Array),
                        cfg::IntType::U128 => Ok(ParamType::U128Array),
                    },
                },
                _ => err,
            },
            cfg::Type::Builtin(cfg::BuiltinType::Parampack) => Ok(ParamType::Parampack),
            _ => err,
        }
    }
}
