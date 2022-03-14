use ethers::providers::{JsonRpcClient, Provider};
use ethers::signers::Signer;

#[derive(Clone, Debug)]
pub struct Client<S: Signer, P: JsonRpcClient> {
    signer: Option<S>,
    provider: Provider<P>,
}

impl<S: Signer, P: JsonRpcClient> Client<S, P> {
    pub fn new(signer: Option<S>, provider: Provider<P>) -> Self {
        Self { signer, provider }
    }

    pub fn signer(&self) -> Option<&S> {
        self.signer.as_ref()
    }

    pub fn provider(&self) -> &Provider<P> {
        &self.provider
    }
}
