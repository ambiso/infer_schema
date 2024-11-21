#![allow(warnings)]
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Display;
use std::{collections::HashMap, error::Error};

use serde::ser::Impossible;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Bool,
    ConcreteBool(bool),
    U8,
    ConcreteU8(u8),
    U16,
    ConcreteU16(u16),
    U32,
    ConcreteU32(u32),
    U64,
    ConcreteU64(u64),
    U128,
    ConcreteU128(u128),
    I8,
    ConcreteI8(i8),
    I16,
    ConcreteI16(i16),
    I32,
    ConcreteI32(i32),
    I64,
    ConcreteI64(i64),
    I128,
    ConcreteI128(i128),
    F32,
    // ConcreteF32(f32),
    F64,
    // ConcreteF64(f64),
    String,
    ConcreteString(String),
    Char,
    ConcreteChar(char),
    Bytes,
    ConcreteBytes(Vec<u8>),
    None,
    Unit,
    Seq(Box<Type>),
    Map { fields: BTreeMap<Type, Type> },
    Union { types: BTreeSet<Type> },
}

#[derive(Debug)]
pub struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError")
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl serde::ser::Error for MyError {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        MyError
    }
}

pub struct SerializeSeq {
    ty: Option<Type>,
}

impl Type {
    fn union(mut self, mut other: Type) -> Type {
        let escape_hatch = |a, b| Type::Union {
            types: {
                let mut s = BTreeSet::new();
                s.insert(a);
                s.insert(b);
                s
            },
        };
        if let Type::Union { types } = &mut self {
            types.insert(other);
            self
        } else if let Type::Union { types } = &mut other {
            types.insert(self);
            other
        } else {
            match &mut self {
                Type::Bool => match other {
                    Type::Bool | Type::ConcreteBool(_) => Type::Bool,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteBool(a) => match other {
                    Type::Bool => Type::Bool,
                    Type::ConcreteBool(b) if *a == b => Type::ConcreteBool(*a),
                    Type::ConcreteBool(_) => Type::Bool,
                    _ => escape_hatch(self, other),
                },
                Type::U8 => match other {
                    Type::U8 | Type::ConcreteU8(_) => Type::U8,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteU8(a) => match other {
                    Type::U8 => Type::U8,
                    Type::ConcreteU8(b) if *a == b => Type::ConcreteU8(*a),
                    _ => escape_hatch(self, other),
                },
                Type::U16 => match other {
                    Type::U16 | Type::ConcreteU16(_) => Type::U16,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteU16(a) => match other {
                    Type::U16 => Type::U16,
                    Type::ConcreteU16(b) if *a == b => Type::ConcreteU16(*a),
                    _ => escape_hatch(self, other),
                },
                Type::U32 => match other {
                    Type::U32 | Type::ConcreteU32(_) => Type::U32,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteU32(a) => match other {
                    Type::U32 => Type::U32,
                    Type::ConcreteU32(b) if *a == b => Type::ConcreteU32(*a),
                    _ => escape_hatch(self, other),
                },
                Type::U64 => match other {
                    Type::U64 | Type::ConcreteU64(_) => Type::U64,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteU64(a) => match other {
                    Type::U64 => Type::U64,
                    Type::ConcreteU64(b) if *a == b => Type::ConcreteU64(*a),
                    _ => escape_hatch(self, other),
                },
                Type::U128 => match other {
                    Type::U128 | Type::ConcreteU128(_) => Type::U128,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteU128(a) => match other {
                    Type::U128 => Type::U128,
                    Type::ConcreteU128(b) if *a == b => Type::ConcreteU128(*a),
                    _ => escape_hatch(self, other),
                },
                Type::I8 => match other {
                    Type::I8 | Type::ConcreteI8(_) => Type::I8,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteI8(a) => match other {
                    Type::I8 => Type::I8,
                    Type::ConcreteI8(b) if *a == b => Type::ConcreteI8(*a),
                    _ => escape_hatch(self, other),
                },
                Type::I16 => match other {
                    Type::I16 | Type::ConcreteI16(_) => Type::I16,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteI16(a) => match other {
                    Type::I16 => Type::I16,
                    Type::ConcreteI16(b) if *a == b => Type::ConcreteI16(*a),
                    _ => escape_hatch(self, other),
                },
                Type::I32 => match other {
                    Type::I32 | Type::ConcreteI32(_) => Type::I32,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteI32(a) => match other {
                    Type::I32 => Type::I32,
                    Type::ConcreteI32(b) if *a == b => Type::ConcreteI32(*a),
                    _ => escape_hatch(self, other),
                },
                Type::I64 => match other {
                    Type::I64 | Type::ConcreteI64(_) => Type::I64,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteI64(a) => match other {
                    Type::I64 => Type::I64,
                    Type::ConcreteI64(b) if *a == b => Type::ConcreteI64(*a),
                    _ => escape_hatch(self, other),
                },
                Type::I128 => match other {
                    Type::I128 | Type::ConcreteI128(_) => Type::I128,
                    _ => escape_hatch(self, other),
                },
                Type::ConcreteI128(a) => match other {
                    Type::I128 => Type::I128,
                    Type::ConcreteI128(b) if *a == b => Type::ConcreteI128(*a),
                    _ => escape_hatch(self, other),
                },
                Type::F32 => match other {
                    Type::F32 => Type::F32,
                    _ => escape_hatch(self, other),
                },
                Type::F64 => todo!(),
                Type::String => todo!(),
                Type::ConcreteString(_) => todo!(),
                Type::Char => todo!(),
                Type::ConcreteChar(_) => todo!(),
                Type::Bytes => todo!(),
                Type::ConcreteBytes(vec) => todo!(),
                Type::None => todo!(),
                Type::Unit => todo!(),
                Type::Map { fields: f1 } => match other {
                    Type::Map { fields: f2 } => {
                        for (k, v) in f1.iter_mut() {
                            if let Some(v2) = f2.get(k) {
                                *v = v.clone().union(v2.clone());
                            }
                        }
                        for (k, v) in f2.into_iter() {
                            f1.entry(k).or_insert(v);
                        }
                        self
                    }
                    _ => escape_hatch(self, other),
                },
                Type::Union { types } => todo!(),
                Type::Seq(_) => todo!(),
            }
        }
    }
}

impl serde::ser::SerializeSeq for SerializeSeq {
    type Ok = Type;

    type Error = MyError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let ty = value.serialize(Serializer)?;
        if let Some(existing) = self.ty.take() {
            self.ty = Some(existing.union(ty));
        } else {
            self.ty = Some(ty);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Type::Seq(Box::new(self.ty.unwrap_or(Type::None)))) // TODO: check
    }
}

pub struct SerializeTuple;

impl serde::ser::SerializeTuple for SerializeTuple {
    type Ok = Type;

    type Error = MyError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

pub struct SerializeTupleStruct;

impl serde::ser::SerializeTupleStruct for SerializeTupleStruct {
    type Ok = Type;

    type Error = MyError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

pub struct SerializeTupleVariant;

impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Type;

    type Error = MyError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

struct MapKeySerializer;

impl serde::ser::Serializer for MapKeySerializer {
    type Ok = Type;

    type Error = MyError;

    type SerializeSeq = Impossible<Type, MyError>;

    type SerializeTuple = Impossible<Type, MyError>;

    type SerializeTupleStruct = Impossible<Type, MyError>;

    type SerializeTupleVariant = Impossible<Type, MyError>;

    type SerializeMap = Impossible<Type, MyError>;

    type SerializeStruct = Impossible<Type, MyError>;

    type SerializeStructVariant = Impossible<Type, MyError>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Type::ConcreteString(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

pub struct SerializeMap {
    fields: BTreeMap<Type, Type>,
    key: Option<Type>,
}

impl serde::ser::SerializeMap for SerializeMap {
    type Ok = Type;

    type Error = MyError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.key = Some(key.serialize(MapKeySerializer).unwrap());
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.fields.insert(
            self.key.take().unwrap(),
            value.serialize(Serializer).unwrap(),
        );
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Type::Map {
            fields: self.fields,
        })
    }
}

pub struct SerializeStruct;
impl serde::ser::SerializeStruct for SerializeStruct {
    type Ok = Type;

    type Error = MyError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

pub struct SerializeStructVariant;

impl serde::ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Type;

    type Error = MyError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

pub struct Serializer;
impl serde::ser::Serializer for Serializer {
    type Ok = Type;

    type Error = MyError;

    type SerializeSeq = SerializeSeq;

    type SerializeTuple = SerializeTuple;

    type SerializeTupleStruct = SerializeTupleStruct;

    type SerializeTupleVariant = SerializeTupleVariant;

    type SerializeMap = SerializeMap;

    type SerializeStruct = SerializeStruct;

    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(if v <= 1 {
            Type::Bool
        } else if v <= u8::MAX as u64 {
            Type::ConcreteU8(v as u8)
        } else if v <= u16::MAX as u64 {
            Type::ConcreteU16(v as u16)
        } else if v <= u32::MAX as u64 {
            Type::ConcreteU32(v as u32)
        } else {
            Type::ConcreteU64(v)
        })
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        dbg!(v);
        Ok(Type::ConcreteString(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeSeq { ty: None })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap {
            fields: BTreeMap::new(),
            key: None,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        dbg!(name, len);
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
