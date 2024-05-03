use wasmtime::component::Resource;

use crate::bindings::wasi::signature::verifier;
use crate::SignatureView;

#[async_trait::async_trait]
impl<T: SignatureView> verifier::Host for T {
    async fn verify(
        &mut self, msg: Vec<u8>, signature: Vec<u8>,
    ) -> wasmtime::Result<Result<(), Resource<wasmtime::Error>>> {
        tracing::debug!("Host::sign");
        todo!()
        // let bucket = self.table().get_mut(&bucket)?;

        // let res = bucket.get_many(keys).await?;
        // let resp = res.into_iter().map(|(k, v)| Some((k, v))).collect::<Vec<_>>();

        // Ok(Ok(resp))
    }
}
