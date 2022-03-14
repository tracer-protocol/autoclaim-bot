use ethers::prelude::SignerMiddleware;
use ethers::providers::{JsonRpcClient, Provider};
use ethers::signers::Signer;

#[derive(Clone, Debug)]
pub struct Client<S: Signer + Clone, P: JsonRpcClient + Clone> {
    signer: Option<S>,
    provider: Provider<P>,
}

impl<S: Signer + Clone, P: JsonRpcClient + Clone> Client<S, P> {
    pub fn new(signer: Option<S>, provider: Provider<P>) -> Self {
        Self { signer, provider }
    }

    pub fn signer(&self) -> Option<&S> {
        self.signer.as_ref()
    }

    pub fn provider(&self) -> &Provider<P> {
        &self.provider
    }

    pub fn middleware(&self) -> Option<SignerMiddleware<Provider<P>, S>> {
        self.signer
            .as_ref()
            .map(|s| SignerMiddleware::new(self.provider.clone(), s.clone()))
    }
}
