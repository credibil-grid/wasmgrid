use std::time::Duration;

use anyhow::anyhow;
use async_nats::{Client, HeaderMap, Subject};
use wasmtime::component::Resource;
use wasmtime_wasi::ResourceTableError;

use super::generated::wasi::messaging::request_reply::RequestOptions;
use super::generated::wasi::messaging::types::{Error, HostMessage, Message, Metadata, Topic};
use super::generated::wasi::messaging::{producer, request_reply, types};
use crate::Host;

pub type Result<T, E = Error> = anyhow::Result<T, E>;

impl types::Host for Host<'_> {
    fn convert_error(&mut self, err: Error) -> anyhow::Result<Error> {
        Ok(err)
    }
}

impl HostMessage for Host<'_> {
    /// Create a new message with the given payload.
    async fn new(&mut self, data: Vec<u8>) -> anyhow::Result<Resource<Message>> {
        tracing::trace!("HostMessage::new with {} bytes", data.len());
        let msg = Message {
            subject: Subject::from_static(""),
            reply: None,
            payload: data.clone().into(),
            headers: None,
            status: None,
            description: None,
            length: data.len(),
        };
        Ok(self.table.push(msg)?)
    }

    /// The topic/subject/channel this message was received on, if any.
    async fn topic(&mut self, this: Resource<Message>) -> anyhow::Result<Option<Topic>> {
        tracing::trace!("HostMessage::topic");
        let msg = self.table.get(&this)?;
        let topic = msg.subject.to_string();
        if topic.is_empty() { Ok(None) } else { Ok(Some(topic)) }
    }

    /// An optional content-type describing the format of the data in the
    /// message. This is sometimes described as the "format" type".
    async fn content_type(&mut self, this: Resource<Message>) -> anyhow::Result<Option<String>> {
        tracing::trace!("HostMessage::content_type");
        let msg = self.table.get(&this)?;
        let content_type = msg.headers.as_ref().and_then(|h| h.get("content-type"));
        content_type.map_or_else(
            || {
                let content_type = msg.headers.as_ref().and_then(|h| h.get("Content-Type"));
                content_type.map_or_else(|| Ok(None), |ct| Ok(Some(ct.to_string())))
            },
            |ct| Ok(Some(ct.to_string())),
        )
    }

    /// Set the content-type describing the format of the data in the message.
    /// This is sometimes described as the "format" type.
    async fn set_content_type(
        &mut self, this: Resource<Message>, content_type: String,
    ) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::set_content_type {content_type}");
        let msg = self.table.get_mut(&this)?;
        let mut headers = msg.headers.take().unwrap_or_default();
        headers.insert("content-type".to_string(), content_type);
        msg.headers = Some(headers);
        Ok(())
    }

    /// An opaque blob of data.
    async fn data(&mut self, this: Resource<Message>) -> anyhow::Result<Vec<u8>> {
        tracing::trace!("HostMessage::data");
        let msg = self.table.get(&this)?;
        Ok(msg.payload.clone().into())
    }

    /// Set the opaque blob of data for this message, discarding the old value".
    async fn set_data(&mut self, this: Resource<Message>, data: Vec<u8>) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::set_data");
        let msg = self.table.get_mut(&this)?;
        msg.payload = data.clone().into();
        msg.length = data.len();
        Ok(())
    }

    /// Optional metadata (also called headers or attributes in some systems)
    /// attached to the message. This metadata is simply decoration and should
    /// not be interpreted by a host to ensure portability across different
    /// implementors (e.g., Kafka -> NATS, etc.).
    ///
    /// For NATS the value at a metadata key is a vector of strings. This
    /// function will return those values concatenated into a single string
    /// with a comma separating each value.
    /// TODO: Test this assumption with real world scenarios.
    async fn metadata(&mut self, this: Resource<Message>) -> anyhow::Result<Option<Metadata>> {
        tracing::trace!("HostMessage::metadata");
        let msg = self.table.get(&this)?;

        let metadata = msg.headers.as_ref().map(|h| {
            h.iter()
                .map(|(k, v)| {
                    (
                        k.to_string(),
                        v.iter()
                            .map(std::string::ToString::to_string)
                            .collect::<Vec<String>>()
                            .join(","),
                    )
                })
                .collect::<Vec<_>>()
        });
        metadata.map_or_else(|| Ok(None), |m| Ok(Some(m)))
    }

    /// Add a new key-value pair to the metadata, overwriting any existing value
    /// for the same key.
    ///
    /// For NATS the value at a metadata key is a vector of strings. To populate
    /// a multi-valued metadata key, the caller should use a comma-separated
    /// list of values. This function will split the string on commas and store
    /// each value as a separate entry in the vector.
    /// TODO: Test this assumption with real world scenarios.
    async fn add_metadata(
        &mut self, this: Resource<Message>, key: String, value: String,
    ) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::add_metadata {key}={value}");
        let msg = self.table.get_mut(&this)?;
        let mut headers = msg.headers.take().unwrap_or_default();
        let values =
            value.split(',').map(std::string::ToString::to_string).collect::<Vec<String>>();
        headers.insert(key.clone(), values[0].clone());
        for v in values.iter().skip(1) {
            headers.append(key.clone(), v.clone());
        }
        Ok(())
    }

    /// Set the metadata as a whole, overwriting any existing metadata.
    ///
    /// For NATS the value at a metadata key is a vector of strings. To populate
    /// a multi-valued metadata key, the caller should use a comma-separated
    /// list of values. This function will split the string on commas and store
    /// each value as a separate entry in the vector.
    /// TODO: Test this assumption with real world scenarios.
    async fn set_metadata(
        &mut self, this: Resource<Message>, meta: Metadata,
    ) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::set_metadata");
        let msg = self.table.get_mut(&this)?;
        let mut headers = HeaderMap::new();
        for (k, v) in &meta {
            let values =
                v.split(',').map(std::string::ToString::to_string).collect::<Vec<String>>();
            headers.insert(k.clone(), values[0].clone());
            for v in values.iter().skip(1) {
                headers.append(k.clone(), v.clone());
            }
        }
        msg.headers = Some(headers);
        Ok(())
    }

    /// Remove a key-value pair from the metadata.
    ///
    /// The NATS header API does not support removing a single key from the
    /// set of headers. So this function will copy the existing headers,
    /// skipping the key to be removed.
    async fn remove_metadata(
        &mut self, msg_res: Resource<Message>, key: String,
    ) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::remove_metadata {key}");
        let msg = self.table.get_mut(&msg_res)?;
        let existing_headers = msg.headers.take().unwrap_or_default();
        let mut new_headers = HeaderMap::new();
        for (k, v) in existing_headers.iter() {
            if k.to_string() != key {
                new_headers.insert(k.clone(), v[0].clone());
                for v in v.iter().skip(1) {
                    new_headers.append(k.clone(), v.clone());
                }
            }
        }
        msg.headers = Some(new_headers);
        Ok(())
    }

    /// Remove a message from the resource table.
    async fn drop(&mut self, this: Resource<Message>) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::drop");
        self.table.delete(this)?;
        Ok(())
    }
}

impl types::HostClient for Host<'_> {
    async fn connect(&mut self, name: String) -> Result<Resource<Client>> {
        tracing::trace!("HostClient::connect {name}");
        let client = crate::nats()?;
        let resource = self.table.push(client.clone())?;
        Ok(resource)
    }

    async fn disconnect(&mut self, _rep: Resource<Client>) -> Result<()> {
        tracing::trace!("HostClient::disconnect");
        // Do nothing. There is no explicit disconnect in async-nats.
        Ok(())
    }

    async fn drop(&mut self, this: Resource<Client>) -> anyhow::Result<()> {
        tracing::trace!("HostClient::drop");
        self.table.delete(this)?;
        Ok(())
    }
}

/// The producer interface is used to send messages to a channel/topic.
impl producer::Host for Host<'_> {
    /// Sends the message using the given client.
    async fn send(
        &mut self, res_client: Resource<Client>, topic: Topic, this: Resource<Message>,
    ) -> Result<()> {
        tracing::trace!("producer::Host::send: topic {:?}", topic);

        let client = self.table.get(&res_client)?;
        let msg = self.table.get(&this)?;
        let Some(headers) = msg.headers.clone() else {
            client
                .publish(topic.clone(), msg.payload.clone())
                .await
                .map_err(|e| anyhow!("failed to publish: {e}"))?;
            return Ok(());
        };
        client
            .publish_with_headers(topic.clone(), headers, msg.payload.clone())
            .await
            .map_err(|e| anyhow!("failed to publish: {e}"))?;
        Ok(())
    }
}

/// The request-reply interface is used to send a request and receive a reply.
impl request_reply::Host for Host<'_> {
    /// Performs a request-reply operation using the given client and options
    /// (if any).
    async fn request(
        &mut self, res_client: Resource<Client>, topic: Topic, this: Resource<Message>,
        res_opts: Option<Resource<RequestOptions>>,
    ) -> Result<Vec<Resource<Message>>> {
        tracing::trace!("request_reply::Host::request: topic {:?}", topic);

        let client = self.table.get(&res_client)?;
        let msg = self.table.get(&this)?;

        let payload = msg.payload.clone();
        let headers = msg.headers.clone().unwrap_or_default();
        let timeout = match res_opts {
            Some(opts) => {
                let options = self.table.get(&opts)?;
                options.timeout
            }
            None => None,
        };
        let request = async_nats::Request::new().payload(payload).headers(headers).timeout(timeout);

        // Send and get reply.
        let msg = client
            .send_request(topic.clone(), request)
            .await
            .map_err(|e| anyhow!("failed to send request: {e}"))?;

        Ok(vec![self.table.push(msg)?])
    }

    /// Replies to the given message with the given response message.
    async fn reply(
        &mut self, reply_to: Resource<Message>, response: Resource<Message>,
    ) -> Result<()> {
        tracing::trace!("request_reply::Host::reply");

        let reply_to_msg = self.table.get(&reply_to)?;

        if let Some(reply_subject) = &reply_to_msg.reply {
            let response_msg = self.table.get(&response)?;
            let client = crate::nats()?;
            client
                .publish_with_headers(
                    reply_subject.clone(),
                    response_msg.headers.clone().unwrap_or_default(),
                    response_msg.payload.clone(),
                )
                .await
                .map_err(|e| anyhow!("failed to reply: {e}"))?;
        }

        Ok(())
    }
}

impl request_reply::HostRequestOptions for Host<'_> {
    /// Creates a new request options resource with no options set.
    async fn new(&mut self) -> anyhow::Result<Resource<RequestOptions>> {
        tracing::trace!("request_reply::HostRequestOptions::new");
        let options = RequestOptions::default();
        Ok(self.table.push(options)?)
    }

    /// The maximum amount of time to wait for a response. If the timeout value
    /// is not set, then the request/reply operation will block until a message
    /// is received in response.
    async fn set_timeout_ms(
        &mut self, opt_res: Resource<RequestOptions>, timeout_ms: u32,
    ) -> anyhow::Result<()> {
        tracing::trace!("request_reply::HostRequestOptions::set_timeout_ms {timeout_ms}");
        let options = self.table.get_mut(&opt_res)?;
        options.timeout = Some(Duration::from_millis(u64::from(timeout_ms)));
        Ok(())
    }

    /// The maximum number of replies to expect before returning.
    ///
    /// For NATS, this is not configurable so this function does nothing.
    async fn set_expected_replies(
        &mut self, _opt_res: Resource<RequestOptions>, _expected_replies: u32,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Removes the resource from the resource table.
    async fn drop(&mut self, opt_res: Resource<RequestOptions>) -> anyhow::Result<()> {
        tracing::trace!("request_reply::HostRequestOptions::drop");
        self.table.delete(opt_res).map(|_| Ok(()))?
    }
}

impl From<ResourceTableError> for Error {
    fn from(err: ResourceTableError) -> Self {
        Self::Other(err.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::Other(err.to_string())
    }
}
