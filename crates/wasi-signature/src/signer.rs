use wasmtime::component::Resource;

use crate::bindings::wasi::signature::signature_types::SigningSuite;
use crate::bindings::wasi::signature::signer;
use crate::SignatureView;

#[async_trait::async_trait]
impl<T: SignatureView> signer::Host for T {
    async fn sign(
        &mut self, msg: Vec<u8>,
    ) -> wasmtime::Result<Result<Vec<u8>, Resource<wasmtime::Error>>> {
        tracing::debug!("Host::sign");
        todo!()
        // let bucket = self.table().get_mut(&bucket)?;

        // let res = bucket.get_many(keys).await?;
        // let resp = res.into_iter().map(|(k, v)| Some((k, v))).collect::<Vec<_>>();

        // Ok(Ok(resp))
    }

    async fn suite(&mut self) -> wasmtime::Result<SigningSuite> {
        tracing::debug!("Host::suite");
        todo!()
        // let bucket = self.table().get_mut(&bucket)?;
        // Ok(Ok(bucket.set_many(key_values).await?))
    }
}
