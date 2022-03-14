//! Blockchain interaction API
use ethers::prelude::SignerMiddleware;
use ethers::providers::{JsonRpcClient, Provider};
use ethers::signers::Signer;

/// Represents a single, coherent API for blockchain interaction by wrapping
/// both signers (optionally) and providers.
#[derive(Clone, Debug)]
pub struct Client<S: Signer + Clone, P: JsonRpcClient + Clone> {
    /// Optional signer type (read-only clients set this to `None`)
    signer: Option<S>,
    /// Provider type, with backend `P` (e.g., `Http`, `Ws`, etc.)
    provider: Provider<P>,
}

impl<S: Signer + Clone, P: JsonRpcClient + Clone> Client<S, P> {
    /// Constructs a new instance of this type
    ///
    /// # Parameters #
    ///
    /// - `signer`, optional signer type
    /// - `provider`, provider type
    pub fn new(signer: Option<S>, provider: Provider<P>) -> Self {
        Self { signer, provider }
    }

    /// Hands out an immutable reference to the underlying signer if it exists
    ///
    /// # Returns #
    ///
    /// An `Option<&S>` which is `Some(s)` if the signer is present and `None`
    /// otherwise.
    pub fn signer(&self) -> Option<&S> {
        self.signer.as_ref()
    }

    /// Hands out an immutable reference to the underlying provider
    ///
    /// # Returns #
    ///
    /// An immutable reference to the underlying provider
    pub fn provider(&self) -> &Provider<P> {
        &self.provider
    }

    /// Constructs a new middleware type from the underlying signer and provider
    /// (if the signer is present)
    ///
    /// # Returns #
    ///
    /// An optional `SignerMiddleware` type which is `Some(m)` if a signer is
    /// present and `None` otherwise
    pub fn middleware(&self) -> Option<SignerMiddleware<Provider<P>, S>> {
        self.signer
            .as_ref()
            .map(|s| SignerMiddleware::new(self.provider.clone(), s.clone()))
    }
}
