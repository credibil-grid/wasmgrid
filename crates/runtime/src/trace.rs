//! # Trace
//!
//! This module implements the `wasmtime_wasi::StdoutStream` and
//! `wasmtime_wasi::StderrStream` traits to capture the output of the guest's
//! stdout and stderr streams. This is useful for debugging purposes, as it
//! allows us to see the output of the guest's code in the host's console.

// use opentelemetry::Context;
// use opentelemetry::trace::TraceContextExt;

use anyhow::anyhow;
use bytes::Bytes;
use wasmtime_wasi::p2::{OutputStream, Pollable, StdoutStream, StreamError, StreamResult};

// Capture wasm guest stdout.
pub struct Stdout;

impl StdoutStream for Stdout {
    fn stream(&self) -> Box<dyn OutputStream> {
        Box::new(OutStream {})
    }

    fn isatty(&self) -> bool {
        false
    }
}

struct OutStream;

#[async_trait::async_trait]
impl Pollable for OutStream {
    async fn ready(&mut self) {}
}

// use tracing::callsite::{Callsite, DefaultCallsite, Identifier};
// use tracing::field::{FieldSet, ValueSet};
// use tracing::metadata::Kind;
// use tracing::{Event, Level, Metadata, Value, callsite};

impl OutputStream for OutStream {
    fn write(&mut self, bytes: Bytes) -> StreamResult<()> {
        let out = String::from_utf8_lossy(&bytes);
        print!("{out}");

        // let (level, target, message) =
        //     parser::parse(&out).map_err(StreamError::LastOperationFailed)?;

        // let span = tracing::Span::current();
        // span

        // // let identifier = Callsite::new();
        // let md = span.metadata().unwrap();

        // static CALLSITE: DefaultCallsite = {
        //     // The values of the metadata are unimportant
        //     static META: Metadata<'static> = Metadata::new(
        //         "event ",
        //         target,
        //         Level::INFO,
        //         None,
        //         None,
        //         None,
        //         FieldSet::new(&["message"], Identifier(&CALLSITE)),
        //         Kind::EVENT,
        //     );
        //     DefaultCallsite::new(&META)
        // };
        // let _interest = CALLSITE.interest();

        // let meta = CALLSITE.metadata();
        // let field = meta.fields().field("message").unwrap();
        // // let message = format!("event-from-{idx}", idx = idx);
        // let values = [(&field, Some(&message as &dyn Value))];
        // let value_set = CALLSITE.metadata().fields().value_set(&values);
        // Event::dispatch(meta, &value_set);

        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}

// Implement error tracing for Guests by capturing stderr.
pub struct Errout;

impl StdoutStream for Errout {
    fn stream(&self) -> Box<dyn OutputStream> {
        Box::new(ErroutStream {})
    }

    fn isatty(&self) -> bool {
        false
    }
}

struct ErroutStream;

#[async_trait::async_trait]
impl Pollable for ErroutStream {
    async fn ready(&mut self) {}
}

impl OutputStream for ErroutStream {
    fn write(&mut self, bytes: Bytes) -> StreamResult<()> {
        let out = String::from_utf8(bytes.to_vec())
            .map_err(|e| StreamError::LastOperationFailed(anyhow!(e)))?;
        print!("{out}");
        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}

// mod parser {
//     use anyhow::{Result, anyhow};
//     use nom::bytes::complete::{is_not, take_until};
//     use nom::character::complete::char;
//     use nom::sequence::delimited;
//     use nom::{IResult, Parser};

//     pub fn parse(event: &str) -> Result<(&str, &str, &str)> {
//         let timeless = &event[27..];
//         let (_, (level, target, message)) = (level, target, message)
//             .parse(timeless)
//             .map_err(|e| anyhow!("issue parsing event: {e}"))?;
//         Ok((level, target, message))
//     }

//     fn level(input: &str) -> IResult<&str, &str> {
//         section(input)
//     }

//     fn target(input: &str) -> IResult<&str, &str> {
//         section(input)
//     }

//     fn message(input: &str) -> IResult<&str, &str> {
//         strip_formatting(input).map(|(remainder, _)| ("", remainder))
//     }

//     fn section(input: &str) -> IResult<&str, &str> {
//         let (remainder, _) = strip_formatting(input).unwrap();
//         take_until("\u{1b}")(remainder)
//     }

//     fn strip_formatting(input: &str) -> IResult<&str, &str> {
//         if !input.starts_with('\u{1b}') {
//             return Ok((input, ""));
//         }
//         let (remainder, _) = delimited(char('\u{1b}'), is_not("m"), char('m')).parse(input)?;
//         strip_formatting(remainder.trim_matches([' ', ':']))
//     }

//     mod tests {

//         #[test]
//         fn test_level() {
//             let entry = "2025-08-11T04:48:59.374477Z\u{1b}[0m \u{1b}[32m INFO\u{1b}[0m \u{1b}[2mhttp\u{1b}[0m\u{1b}[2m:\u{1b}[0m received request";

//             let timeless = &entry[27..];
//             let (_, (level, target, message)) = (level, target, message).parse(timeless).unwrap();

//             assert_eq!(level, "INFO");
//             assert_eq!(target, "http");
//             assert_eq!(message, "received request");
//         }
//     }
// }
