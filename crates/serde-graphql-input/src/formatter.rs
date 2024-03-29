use std::io;

mod compact {
    use crate::Formatter;

    #[derive(Clone, Debug)]
    pub struct CompactFormatter;
    impl Formatter for CompactFormatter {}
}

mod pretty {
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
}

pub use compact::*;
pub use pretty::*;

pub(crate) fn format_key<W, F>(writer: &mut W, formatter: &mut F, value: &str) -> io::Result<()>
where
    W: io::Write,
    F: Formatter,
{
    formatter.write_string(writer, value)
}

pub(crate) fn format_escaped_str<W, F>(
    writer: &mut W,
    formatter: &mut F,
    value: &str,
) -> io::Result<()>
where
    W: io::Write,
    F: Formatter,
{
    formatter.begin_string(writer)?;
    format_escaped_str_contents(writer, formatter, value)?;
    formatter.end_string(writer)
}

fn format_escaped_str_contents<W, F>(
    writer: &mut W,
    formatter: &mut F,
    value: &str,
) -> io::Result<()>
where
    W: io::Write,
    F: Formatter,
{
    formatter.write_string(writer, value)
}

pub trait Formatter {
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let output = if value { b"true" as &[u8] } else { b"false" };

        writer.write_all(output)
    }

    fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"null")
    }

    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"{")
    }

    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"}")
    }

    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"[")
    }

    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"]")
    }

    fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")
    }

    fn write_string<W>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(value.as_bytes())
    }

    fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")
    }

    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if !first {
            writer.write_all(b",")
        } else {
            Ok(())
        }
    }

    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b":")
    }

    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            Ok(())
        } else {
            writer.write_all(b",")
        }
    }

    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }
}
