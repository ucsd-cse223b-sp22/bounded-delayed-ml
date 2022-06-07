use crate::err::TribResult;
use crate::ml::MLModel;
use crate::rpc::parameter_server_server::ParameterServer;

#[async_trait::async_trait]
pub trait BinStorage: Send + Sync {
    /// Fetch a [Storage] bin based on the given bin name.
    async fn bin(&self, name: &str) -> TribResult<Box<dyn MLModel>>;
}
