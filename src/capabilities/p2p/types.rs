use bytes::{Buf, BufMut, Bytes, BytesMut};
use wasmtime::component::Resource;
use wasmtime_wasi::{
    HostInputStream, HostOutputStream, InputStream, OutputStream, StreamResult, Subscribe, WasiView,
};

use super::bindings::wasi::blobstore::types::{
    Error, Host, HostIncomingValue, HostOutgoingValue, IncomingValueAsyncBody,
    IncomingValueSyncBody,
};
use super::bindings::wasi::p2p::types;
use crate::runtime::State;

impl types::Host for State {}

/// A BlobValue is a wrapper for a Blob that implements the traits necessary for incoming and
/// outgoing values.
#[derive(Clone)]
pub struct BlobValue {
    pub blob: Blob,
}

impl BlobValue {
    pub fn new(blob: Blob) -> Self {
        Self { blob }
    }
}

/// A Blob is modelled as a reference to contiguous memory. It uses a `BytesMut` to more easily
/// support conversion to slices of bytes.
#[derive(Clone)]
pub struct Blob {
    data: BytesMut,
}

impl Blob {
    pub fn new() -> Self {
        Self { data: BytesMut::new() }
    }
}

impl From<BytesMut> for Blob {
    fn from(data: BytesMut) -> Self {
        Self { data }
    }
}

impl From<Bytes> for Blob {
    fn from(bytes: Bytes) -> Self {
        let mut data = BytesMut::new();
        data.put(bytes);
        Self { data  }
    }
}

impl Into<Bytes> for Blob {
    fn into(self) -> Bytes {
        Bytes::from(self.data)
    }
}

#[async_trait::async_trait]
impl Subscribe for Blob {
    async fn ready(&mut self) {}
}

impl HostOutputStream for Blob {
    fn write(&mut self, bytes: Bytes) -> StreamResult<()> {
        self.data.put(bytes);
        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    // TODO: Configure
    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}

impl HostInputStream for Blob {
    fn read(&mut self, size: usize) -> StreamResult<Bytes> {
        Ok(self.data.copy_to_bytes(size))
    }

    fn skip(&mut self, size: usize) -> StreamResult<usize> {
        self.data.advance(size);
        Ok(self.data.remaining())
    }
}

impl Host for State {}

#[async_trait::async_trait]
impl HostOutgoingValue for State {
    async fn new_outgoing_value(&mut self) -> wasmtime::Result<Resource<BlobValue>> {
        tracing::debug!("HostOutgoingValue::new_outgoing_value");
        Ok(self.table().push(BlobValue::new(Blob::new()))?)
    }

    async fn outgoing_value_write_body(
        &mut self, value: Resource<BlobValue>,
    ) -> wasmtime::Result<Result<Resource<OutputStream>, ()>> {
        tracing::debug!("HostOutgoingValue::outgoing_value_write_body");
        let value = self.table().get_mut(&value)?;
        let os = Box::new(value.blob.clone()) as Box<dyn HostOutputStream>;
        Ok(Ok(self.table().push(os)?))
    }

    fn drop(&mut self, value: Resource<BlobValue>) -> wasmtime::Result<()> {
        tracing::debug!("HostOutgoingValue::drop");
        self.table().delete(value)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl HostIncomingValue for State {
    async fn incoming_value_consume_sync(
        &mut self, value: Resource<BlobValue>,
    ) -> wasmtime::Result<Result<IncomingValueSyncBody, Error>> {
        tracing::debug!("HostIncomingValue::incoming_value_consume_sync");
        let value = self.table().get_mut(&value)?;
        let data = value.blob.data.to_vec();
        Ok(Ok(data))
    }

    async fn incoming_value_consume_async(
        &mut self, value: Resource<BlobValue>,
    ) -> wasmtime::Result<Result<Resource<IncomingValueAsyncBody>, Error>> {
        tracing::debug!("HostIncomingValue::incoming_value_consume_async");
        let value = self.table().get_mut(&value)?;
        let s = Box::new(value.blob.clone()) as Box<dyn HostInputStream>;
        let t = InputStream::Host(s);
        Ok(Ok(self.table().push(t)?))
    }

    async fn size(&mut self, value: Resource<BlobValue>) -> wasmtime::Result<u64> {
        tracing::debug!("HostIncomingValue::size");
        let value = self.table().get_mut(&value)?;
        Ok(value.blob.data.len() as u64)
    }

    fn drop(&mut self, value: Resource<BlobValue>) -> wasmtime::Result<()> {
        tracing::debug!("HostIncomingValue::drop");
        self.table().delete(value)?;
        Ok(())
    }
}
