use std::io;

use serde::ser::Impossible;
use serde::Serialize;

use crate::error::{self, Result};
use crate::{error::Error, CompactFormatter, Formatter, PrettyFormatter};
use crate::{format_escaped_str, format_key};

#[allow(dead_code)]
pub struct Serializer<W, F = CompactFormatter> {
    pub(crate) writer: W,
    pub(crate) formatter: F,
}

impl<'a, W> Serializer<W, PrettyFormatter<'a>>
where
    W: io::Write,
{
    pub fn pretty(writer: W) -> Self {
        Serializer::with_formatter(writer, PrettyFormatter::new())
    }
}

impl<W, F> Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    pub fn with_formatter(writer: W, formatter: F) -> Self {
        Serializer { writer, formatter }
    }
}

#[allow(dead_code, unused_variables)]
impl<'a, W, F> serde::ser::Serializer for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compount<'a, W, F>;
    type SerializeTuple = Compount<'a, W, F>;
    type SerializeTupleStruct = Compount<'a, W, F>;
    type SerializeTupleVariant = Compount<'a, W, F>;
    type SerializeMap = Compount<'a, W, F>;
    type SerializeStruct = Compount<'a, W, F>;
    type SerializeStructVariant = Compount<'a, W, F>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.formatter
            .write_bool(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.formatter
            .write_i8(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.formatter
            .write_i16(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.formatter
            .write_i32(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.formatter
            .write_i64(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.formatter
            .write_u8(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.formatter
            .write_u16(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.formatter
            .write_u32(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.formatter
            .write_u64(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.formatter
            .write_f32(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.formatter
            .write_f64(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.formatter
            .write_char(&mut self.writer, v)
            .map_err(Error::io)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        format_escaped_str(&mut self.writer, &mut self.formatter, v).map_err(Error::io)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        todo!()
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: Serialize,
        T: ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.formatter
            .write_null(&mut self.writer)
            .map_err(Error::io)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        self.formatter
            .write_null(&mut self.writer)
            .map_err(Error::io)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.formatter
            .write_string(&mut self.writer, variant)
            .map_err(Error::io)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
        T: ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
        T: ?Sized,
    {
        self.formatter
            .begin_object(&mut self.writer)
            .map_err(Error::io)?;
        self.formatter
            .begin_object_key(&mut self.writer, true)
            .map_err(Error::io)?;
        self.serialize_str(variant)?;
        self.formatter
            .end_object_key(&mut self.writer)
            .map_err(Error::io)?;
        self.formatter
            .begin_object_value(&mut self.writer)
            .map_err(Error::io)?;
        value.serialize(&mut *self)?;
        self.formatter
            .end_object_value(&mut self.writer)
            .map_err(Error::io)?;
        self.formatter
            .end_object(&mut self.writer)
            .map_err(Error::io)
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> std::prelude::v1::Result<Self::SerializeSeq, Self::Error> {
        self.formatter
            .begin_array(&mut self.writer)
            .map_err(Error::io)?;

        if len == Some(0) {
            self.formatter
                .end_array(&mut self.writer)
                .map_err(Error::io)?;

            Ok(Compount::Map {
                ser: self,
                state: State::Empty,
            })
        } else {
            Ok(Compount::Map {
                ser: self,
                state: State::First,
            })
        }
    }

    fn serialize_tuple(
        self,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleVariant, Self::Error> {
        self.formatter
            .begin_object(&mut self.writer)
            .map_err(Error::io)?;
        self.formatter
            .begin_object_key(&mut self.writer, true)
            .map_err(Error::io)?;
        self.serialize_str(variant)?;
        self.formatter
            .end_object_key(&mut self.writer)
            .map_err(Error::io)?;
        self.formatter
            .begin_object_value(&mut self.writer)
            .map_err(Error::io)?;
        self.serialize_seq(Some(len))
    }

    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> std::prelude::v1::Result<Self::SerializeMap, Self::Error> {
        self.formatter
            .begin_object(&mut self.writer)
            .map_err(Error::io)?;
        if len == Some(0) {
            self.formatter
                .end_object(&mut self.writer)
                .map_err(Error::io)?;
            Ok(Compount::Map {
                ser: self,
                state: State::Empty,
            })
        } else {
            Ok(Compount::Map {
                ser: self,
                state: State::First,
            })
        }
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStructVariant, Self::Error> {
        self.formatter
            .begin_object(&mut self.writer)
            .map_err(Error::io)?;
        self.formatter
            .begin_object_key(&mut self.writer, true)
            .map_err(Error::io)?;
        self.serialize_str(variant)?;
        self.formatter
            .end_object_key(&mut self.writer)
            .map_err(Error::io)?;
        self.formatter
            .begin_object_value(&mut self.writer)
            .map_err(Error::io)?;
        self.serialize_map(Some(len))
    }
}

struct MapKeySerializer<'a, W: 'a, F: 'a> {
    ser: &'a mut Serializer<W, F>,
}

impl<'a, W, F> serde::ser::Serializer for MapKeySerializer<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        todo!()
    }

    fn serialize_i8(self, _v: i8) -> Result<()> {
        todo!()
    }

    fn serialize_i16(self, _v: i16) -> Result<()> {
        todo!()
    }

    fn serialize_i32(self, _v: i32) -> Result<()> {
        todo!()
    }

    fn serialize_i64(self, _v: i64) -> Result<()> {
        todo!()
    }

    fn serialize_u8(self, _v: u8) -> Result<()> {
        todo!()
    }

    fn serialize_u16(self, _v: u16) -> Result<()> {
        todo!()
    }

    fn serialize_u32(self, _v: u32) -> Result<()> {
        todo!()
    }

    fn serialize_u64(self, _v: u64) -> Result<()> {
        todo!()
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        todo!()
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        todo!()
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        format_key(&mut self.ser.writer, &mut self.ser.formatter, v).map_err(Error::io)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        todo!()
    }

    fn serialize_none(self) -> Result<()> {
        todo!()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: Serialize,
        T: ?Sized,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<()> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: Serialize,
        T: ?Sized,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: Serialize,
        T: ?Sized,
    {
        todo!()
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> std::prelude::v1::Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::prelude::v1::Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

#[derive(Eq, PartialEq)]
pub enum State {
    Empty,
    First,
    Rest,
}

pub enum Compount<'a, W: 'a, F: 'a> {
    Map {
        ser: &'a mut Serializer<W, F>,
        state: State,
    },
}

impl<'a, W, F> serde::ser::SerializeSeq for Compount<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = error::Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: Serialize,
        T: ?Sized,
    {
        match self {
            Compount::Map { ser, state } => {
                ser.formatter
                    .begin_array_value(&mut ser.writer, *state == State::First)
                    .map_err(Error::io)?;

                *state = State::Rest;

                value.serialize(&mut **ser)?;

                ser.formatter
                    .end_array_value(&mut ser.writer)
                    .map_err(Error::io)
            }
        }
    }

    fn end(self) -> Result<()> {
        match self {
            Compount::Map { ser, state } => match state {
                State::Empty => Ok(()),
                _ => ser.formatter.end_array(&mut ser.writer).map_err(Error::io),
            },
        }
    }
}

impl<'a, W, F> serde::ser::SerializeTuple for Compount<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();

    type Error = error::Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: Serialize,
        T: ?Sized,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        serde::ser::SerializeSeq::end(self)
    }
}
impl<'a, W, F> serde::ser::SerializeTupleStruct for Compount<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();

    type Error = error::Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: Serialize,
        T: ?Sized,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        serde::ser::SerializeSeq::end(self)
    }
}
impl<'a, W, F> serde::ser::SerializeTupleVariant for Compount<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();

    type Error = error::Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: Serialize,
        T: ?Sized,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        match self {
            Compount::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => ser
                        .formatter
                        .end_array(&mut ser.writer)
                        .map_err(Error::io)?,
                }

                ser.formatter
                    .end_object_value(&mut ser.writer)
                    .map_err(Error::io)?;
                ser.formatter.end_object(&mut ser.writer).map_err(Error::io)
            }
        }
    }
}

impl<'a, W, F> serde::ser::SerializeMap for Compount<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();

    type Error = error::Error;

    fn serialize_key<T>(&mut self, key: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: Serialize,
        T: ?Sized,
    {
        match self {
            Compount::Map { ser, state } => {
                ser.formatter
                    .begin_object_key(&mut ser.writer, *state == State::First)
                    .map_err(Error::io)?;
                *state = State::Rest;

                key.serialize(MapKeySerializer { ser: *ser })?;
                ser.formatter
                    .end_object_key(&mut ser.writer)
                    .map_err(Error::io)
            }
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: Serialize,
        T: ?Sized,
    {
        match self {
            Compount::Map { ser, .. } => {
                ser.formatter
                    .begin_object_value(&mut ser.writer)
                    .map_err(Error::io)?;
                value.serialize(&mut **ser)?;
                ser.formatter
                    .end_object_value(&mut ser.writer)
                    .map_err(Error::io)
            }
        }
    }

    fn end(self) -> Result<()> {
        match self {
            Compount::Map { ser, state } => match state {
                State::Empty => Ok(()),
                _ => ser.formatter.end_object(&mut ser.writer).map_err(Error::io),
            },
        }
    }
}

impl<'a, W, F> serde::ser::SerializeStruct for Compount<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();

    type Error = error::Error;

    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: Serialize,
        T: ?Sized,
    {
        match self {
            Compount::Map { .. } => serde::ser::SerializeMap::serialize_entry(self, key, value),
        }
    }

    fn end(self) -> Result<()> {
        match self {
            Compount::Map { .. } => serde::ser::SerializeMap::end(self),
        }
    }
}

impl<'a, W, F> serde::ser::SerializeStructVariant for Compount<'a, W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();

    type Error = error::Error;

    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: Serialize,
        T: ?Sized,
    {
        match self {
            Compount::Map { .. } => serde::ser::SerializeStruct::serialize_field(self, key, value),
        }
    }

    fn end(self) -> Result<()> {
        match self {
            Compount::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => ser
                        .formatter
                        .end_object(&mut ser.writer)
                        .map_err(Error::io)?,
                }
                ser.formatter
                    .end_object_value(&mut ser.writer)
                    .map_err(Error::io)?;
                ser.formatter.end_object(&mut ser.writer).map_err(Error::io)
            }
        }
    }
}
