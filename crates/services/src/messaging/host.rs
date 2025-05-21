use async_nats::{Client, HeaderMap, Subject};
use wasmtime::component::{Linker, Resource};
use wasmtime_wasi::{ResourceTable, ResourceTableError};

use crate::messaging::generated::wasi::messaging::types::{
    Error, HostMessage, Message, Metadata, Topic,
};
use crate::messaging::generated::wasi::messaging::{producer, types};
use crate::{Ctx, Resources};

pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub struct MsgHost<'a> {
    table: &'a mut ResourceTable,
    resources: &'a Resources,
}

impl MsgHost<'_> {
    const fn new(c: &mut Ctx) -> MsgHost<'_> {
        MsgHost {
            table: &mut c.table,
            resources: &c.resources,
        }
    }
}

/// Add all the `messaging` world's interfaces to a [`Linker`].
pub fn add_to_linker(l: &mut Linker<Ctx>) -> anyhow::Result<()> {
    producer::add_to_linker_get_host(l, MsgHost::new)?;
    types::add_to_linker_get_host(l, MsgHost::new)
}

impl types::Host for MsgHost<'_> {
    fn convert_error(&mut self, err: Error) -> anyhow::Result<Error> {
        Ok(err)
    }
}

impl HostMessage for MsgHost<'_> {
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
    async fn topic(&mut self, res_msg: Resource<Message>) -> anyhow::Result<Option<Topic>> {
        tracing::trace!("HostMessage::topic");
        let msg = self.table.get(&res_msg)?;
        let topic = msg.subject.to_string();
        if topic.is_empty() { Ok(None) } else { Ok(Some(topic)) }
    }

    /// An optional content-type describing the format of the data in the
    /// message. This is sometimes described as the "format" type".
    async fn content_type(&mut self, res_msg: Resource<Message>) -> anyhow::Result<Option<String>> {
        tracing::trace!("HostMessage::content_type");
        let msg = self.table.get(&res_msg)?;
        let content_type = msg.headers.as_ref().and_then(|h| h.get("content-type"));
        if let Some(content_type) = content_type {
            Ok(Some(content_type.to_string()))
        } else {
            let content_type = msg.headers.as_ref().and_then(|h| h.get("Content-Type"));
            if let Some(content_type) = content_type {
                Ok(Some(content_type.to_string()))
            } else {
                Ok(None)
            }
        }
    }

    /// Set the content-type describing the format of the data in the message.
    /// This is sometimes described as the "format" type.
    async fn set_content_type(
        &mut self, res_msg: Resource<Message>, content_type: String,
    ) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::set_content_type");
        let msg = self.table.get_mut(&res_msg)?;
        let mut headers = msg.headers.take().unwrap_or_default();
        headers.insert("content-type".to_string(), content_type);
        msg.headers = Some(headers);
        Ok(())
    }

    /// An opaque blob of data.
    async fn data(&mut self, res_msg: Resource<Message>) -> anyhow::Result<Vec<u8>> {
        tracing::trace!("HostMessage::data");
        let msg = self.table.get(&res_msg)?;
        Ok(msg.payload.clone().into())
    }

    /// Set the opaque blob of data for this message, discarding the old value".
    async fn set_data(&mut self, res_msg: Resource<Message>, data: Vec<u8>) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::set_data");
        let msg = self.table.get_mut(&res_msg)?;
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
    async fn metadata(&mut self, res_msg: Resource<Message>) -> anyhow::Result<Option<Metadata>> {
        tracing::trace!("HostMessage::metadata");
        let msg = self.table.get(&res_msg)?;

        let metadata = msg.headers.as_ref().map(|h| {
            h.iter()
                .map(|(k, v)| {
                    (
                        k.to_string(),
                        v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","),
                    )
                })
                .collect::<Vec<_>>()
        });
        if let Some(metadata) = metadata { Ok(Some(metadata.into())) } else { Ok(None) }
    }

    /// Add a new key-value pair to the metadata, overwriting any existing value
    /// for the same key.
    ///
    /// For NATS the value at a metadata key is a vector of strings. To populate
    /// a multi-valued metadata key, the caller should use a comma-separated list
    /// of values. This function will split the string on commas and store each
    /// value as a separate entry in the vector.
    /// TODO: Test this assumption with real world scenarios.
    async fn add_metadata(
        &mut self, res_msg: Resource<Message>, key: String, value: String,
    ) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::add_metadata");
        let msg = self.table.get_mut(&res_msg)?;
        let mut headers = msg.headers.take().unwrap_or_default();
        let values = value.split(',').map(|x| x.to_string()).collect::<Vec<String>>();
        headers.insert(key.clone(), values[0].clone());
        for v in values.iter().skip(1) {
            headers.append(key.clone(), v.clone());
        }
        Ok(())
    }

    /// Set the metadata as a whole, overwriting any existing metadata.
    ///
    /// For NATS the value at a metadata key is a vector of strings. To populate
    /// a multi-valued metadata key, the caller should use a comma-separated list
    /// of values. This function will split the string on commas and store each
    /// value as a separate entry in the vector.
    /// TODO: Test this assumption with real world scenarios.
    async fn set_metadata(
        &mut self, res_msg: Resource<Message>, meta: Metadata,
    ) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::set_metadata");
        let msg = self.table.get_mut(&res_msg)?;
        let mut headers = HeaderMap::new();
        for (k, v) in meta.iter() {
            let values = v.split(',').map(|x| x.to_string()).collect::<Vec<String>>();
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
        tracing::trace!("HostMessage::remove_metadata");
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
    async fn drop(
        &mut self, res_msg: Resource<Message>,
    ) -> anyhow::Result<()> {
        tracing::trace!("HostMessage::drop");
        self.table.delete(res_msg)?;
        Ok(())
    }
}

impl types::HostClient for MsgHost<'_> {
    async fn connect(&mut self, name: String) -> Result<Resource<Client>> {
        tracing::trace!("HostClient::connect {name}");
        let client = self.resources.nats()?;
        let resource = self.table.push(client.clone())?;
        Ok(resource)
    }

    async fn disconnect(&mut self, _rep: Resource<Client>) -> Result<()> {
        tracing::trace!("HostClient::disconnect");
        // Do nothing. There is no explicit disconnect in async-nats.
        Ok(())
    }

    async fn drop(&mut self, rep: Resource<Client>) -> anyhow::Result<()> {
        tracing::trace!("HostClient::drop");
        self.table.delete(rep)?;
        Ok(())
    }
}

// Host produces messages.
impl producer::Host for MsgHost<'_> {
    async fn send(
        &mut self, client: Resource<Client>, ch: String, messages: Vec<Message>,
    ) -> Result<Result<(), Resource<Error>>> {
        tracing::trace!("producer::Host::send: {:?}", ch);

        let client = self.table.get(&client)?;
        for m in messages {
            let data = m.data.clone().into();
            client.publish(ch.clone(), data).await?;
        }

        Ok(Ok(()))
    }
}

// Host consumes messages.
// impl consumer::Host for MsgHost<'_> {
//     async fn subscribe_try_receive(
//         &mut self, rep: Resource<Client>, ch: String, t_milliseconds: u32,
//     ) -> Result<Result<Option<Vec<Message>>, Resource<Error>>> {
//         tracing::debug!("consumer::Host::subscribe_try_receive {ch}, {t_milliseconds}");

//         // subscribe to channel
//         let client = self.table.get(&rep)?;
//         let mut subscriber = client.subscribe(ch).await?;

//         // create stream that times out after `t_milliseconds`
//         let stream =
//             subscriber.by_ref().take_until(sleep(Duration::from_millis(u64::from(t_milliseconds))));
//         let messages = stream.map(|m| server::msg_conv(&m)).collect().await;
//         subscriber.unsubscribe().await?;

//         Ok(Ok(Some(messages)))
//     }

//     async fn subscribe_receive(
//         &mut self, rep: Resource<Client>, ch: String,
//     ) -> Result<Result<Vec<Message>, Resource<Error>>> {
//         tracing::trace!("consumer::Host::subscribe_receive {ch}");

//         let client = self.table.get(&rep)?;
//         let mut subscriber = client.subscribe(ch).await?;
//         let messages = subscriber.by_ref().take(1).map(|m| server::msg_conv(&m)).collect().await;
//         subscriber.unsubscribe().await?;

//         Ok(Ok(messages))
//     }

//     // TODO: implement `complete_message` using JetStream
//     async fn complete_message(&mut self, msg: Message) -> Result<Result<(), Resource<Error>>> {
//         tracing::warn!("TODO: consumer::Host::complete_message: {:?}", msg.metadata);
//         Ok(Ok(()))
//     }

//     // TODO: implement `abandon_message` using JetStream
//     async fn abandon_message(&mut self, msg: Message) -> Result<Result<(), Resource<Error>>> {
//         tracing::warn!("TODO: consumer::Host::abandon_message: {:?}", msg.metadata);
//         Ok(Ok(()))
//     }

//     async fn update_guest_configuration(
//         &mut self, gc: GuestConfiguration,
//     ) -> Result<Result<(), Resource<Error>>> {
//         tracing::trace!("consumer::Host::update_guest_configuration");
//         server::subscribe(gc.channels, self.resources, self.instance_pre).await?;
//         Ok(Ok(()))
//     }
// }

// impl messaging_types::HostError for MsgHost<'_> {
//     async fn trace(&mut self) -> Result<String> {
//         tracing::trace!("HostError::trace");
//         Ok("error".to_string())
//     }

//     async fn drop(&mut self, rep: Resource<Error>) -> Result<()> {
//         tracing::trace!("HostError::drop");
//         self.table.delete(rep)?;
//         Ok(())
//     }
// }

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
