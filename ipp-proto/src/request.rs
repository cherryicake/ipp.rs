//!
//! IPP request
//!
use std::{
    io::{self, Cursor, Read, Write},
    sync::Mutex,
};

use crate::{
    attribute::*,
    ipp::{DelimiterTag, IppVersion, Operation},
    parser::{IppParser, ParseError},
    value::*,
    IppHeader, IppWriter,
};

use bytes::Bytes;
use futures::{Async, Poll, Stream};
use log::debug;

const CHUNK_SIZE: usize = 32768;

/// IPP request/response struct
pub struct IppRequestResponse {
    /// IPP header
    header: IppHeader,
    /// IPP attributes
    attributes: IppAttributes,
    /// Optional payload after IPP-encoded stream (for example binary data for Print-Job operation)
    payload: Option<Box<Read>>,
}
unsafe impl Send for IppRequestResponse {}

struct IppReadAdapter {
    inner: Mutex<Box<Read>>,
}
unsafe impl Send for IppReadAdapter {}

impl Read for IppReadAdapter {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.lock().unwrap().read(buf)
    }
}

impl Stream for IppReadAdapter {
    type Item = Bytes;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut buf = vec![0; CHUNK_SIZE];
        match self.inner.lock().unwrap().read(&mut buf) {
            Ok(size) => {
                if size > 0 {
                    Ok(Async::Ready(Some(buf[0..size].into())))
                } else {
                    Ok(Async::Ready(None))
                }
            }
            Err(err) => Err(err),
        }
    }
}

pub trait IppRequestTrait {
    fn header(&self) -> &IppHeader;
}

impl IppRequestTrait for IppRequestResponse {
    /// Get header
    fn header(&self) -> &IppHeader {
        &self.header
    }
}

impl IppRequestResponse {
    /// Create new IPP request for the operation and uri
    pub fn new(operation: Operation, uri: &str) -> IppRequestResponse {
        let hdr = IppHeader::new(IppVersion::Ipp11, operation as u16, 1);
        let mut retval = IppRequestResponse {
            header: hdr,
            attributes: IppAttributes::new(),
            payload: None,
        };

        retval.set_attribute(
            DelimiterTag::OperationAttributes,
            IppAttribute::new(ATTRIBUTES_CHARSET, IppValue::Charset("utf-8".to_string())),
        );
        retval.set_attribute(
            DelimiterTag::OperationAttributes,
            IppAttribute::new(ATTRIBUTES_NATURAL_LANGUAGE, IppValue::NaturalLanguage("en".to_string())),
        );

        retval.set_attribute(
            DelimiterTag::OperationAttributes,
            IppAttribute::new(PRINTER_URI, IppValue::Uri(uri.replace("http", "ipp").to_string())),
        );

        retval
    }

    /// Create response from status and id
    pub fn new_response(status: u16, id: u32) -> IppRequestResponse {
        let hdr = IppHeader::new(IppVersion::Ipp11, status, id);
        let mut retval = IppRequestResponse {
            header: hdr,
            attributes: IppAttributes::new(),
            payload: None,
        };

        retval.set_attribute(
            DelimiterTag::OperationAttributes,
            IppAttribute::new(ATTRIBUTES_CHARSET, IppValue::Charset("utf-8".to_string())),
        );
        retval.set_attribute(
            DelimiterTag::OperationAttributes,
            IppAttribute::new(ATTRIBUTES_NATURAL_LANGUAGE, IppValue::NaturalLanguage("en".to_string())),
        );

        retval
    }

    /// Create IppRequestResponse from the parser
    pub fn from_parser(parser: IppParser) -> Result<IppRequestResponse, ParseError> {
        let res = parser.parse()?;

        Ok(IppRequestResponse {
            header: res.header,
            attributes: res.attributes,
            payload: None,
        })
    }

    /// Get IPP header
    pub fn header(&self) -> &IppHeader {
        &self.header
    }

    /// Get mutable IPP header
    pub fn header_mut(&mut self) -> &mut IppHeader {
        &mut self.header
    }

    /// Get attributes
    pub fn attributes(&self) -> &IppAttributes {
        &self.attributes
    }

    /// Get payload
    pub fn payload(&self) -> &Option<Box<Read>> {
        &self.payload
    }

    /// Set payload
    pub fn set_payload(&mut self, payload: Box<Read>) {
        self.payload = Some(payload)
    }

    /// Set attribute
    pub fn set_attribute(&mut self, group: DelimiterTag, attribute: IppAttribute) {
        self.attributes.add(group, attribute);
    }

    /// Serialize request into the binary stream (TCP)
    pub fn write(&mut self, writer: &mut Write) -> io::Result<usize> {
        let mut retval = self.header.write(writer)?;

        retval += self.attributes.write(writer)?;

        debug!("Wrote {} bytes IPP stream", retval);

        if let Some(ref mut payload) = self.payload {
            let size = io::copy(payload, writer)? as usize;
            debug!("Wrote {} bytes payload", size);
            retval += size;
        }

        Ok(retval)
    }

    fn into_ipp_adapter(self) -> IppReadAdapter {
        // NOTE: workaround for embedded firmware bug, don't refactor into chain calls!
        // When Transfer-Encoding: chunked is used the IPP metadata must be serialized in one chunk.
        // Otherwise many printers will fail with error 400.
        let mut cursor = Cursor::new(Vec::with_capacity(1024));
        self.header
            .write(&mut cursor)
            .and_then(|_| self.attributes.write(&mut cursor))
            .unwrap();
        cursor.set_position(0);

        IppReadAdapter {
            inner: Mutex::new(Box::new(
                cursor.chain(self.payload.unwrap_or_else(|| Box::new(io::empty()))),
            )),
        }
    }

    /// Convert request into reader
    pub fn into_reader(self) -> impl Read {
        self.into_ipp_adapter()
    }

    pub fn into_stream(self) -> Box<dyn Stream<Item = Bytes, Error = io::Error> + Send + 'static> {
        Box::new(self.into_ipp_adapter())
    }
}
