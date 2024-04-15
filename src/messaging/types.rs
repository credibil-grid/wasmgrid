use bytes::Bytes;

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self
    }

    pub async fn connect(&self) -> anyhow::Result<Client> {
        let nats_client = async_nats::connect("demo.nats.io").await?;
        let client = Client { inner: nats_client };
        Ok(client)
    }
}

#[derive(Clone)]
pub struct Client {
    pub inner: async_nats::Client,
}

impl Client {
    pub async fn subscribe(&self, ch: String) -> anyhow::Result<async_nats::Subscriber> {
        Ok(self.inner.subscribe(ch).await?)
    }

    pub async fn publish(&self, ch: String, data: Bytes) -> anyhow::Result<()> {
        Ok(self.inner.publish(ch, data).await?)
    }
}
