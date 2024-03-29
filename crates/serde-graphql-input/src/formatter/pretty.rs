use serde::Serialize;

use crate::error::Result;
use crate::{Formatter, Serializer};

#[allow(dead_code)]
pub struct PrettyFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
}

impl<'a> PrettyFormatter<'a> {
    pub fn new() -> Self {
        PrettyFormatter::with_indent(b" ")
    }

    pub fn with_indent(indent: &'a [u8]) -> Self {
        PrettyFormatter {
            current_indent: 0,
            has_value: false,
            indent,
        }
    }
}

impl<'a> Default for PrettyFormatter<'a> {
    fn default() -> Self {
        PrettyFormatter::new()
    }
}

impl<'a> Formatter for PrettyFormatter<'a> {}

pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);

    let mut ser = Serializer::pretty(&mut writer);
    value.serialize(&mut ser)?;

    let string = unsafe { String::from_utf8_unchecked(writer) };

    Ok(string)
}
