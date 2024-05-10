use std::sync::Arc;

use ethers::{
    middleware::SignerMiddleware, providers::Middleware, signers::Signer, types::Address,
};

use crate::{
    agreement_contract::AgreementContract, ground_cycle_contract::GroundCycleContract,
    ground_cycle_no_crypto_contract::GroundCycleNoCryptoContract,
};

// Client to interact with smart contract on behalf of face of some wallet.
pub(crate) struct Client<M> {
    provider: M,
    pub(crate) agreement_addr: Address,
    pub(crate) ground_cycle_addr: Address,
    pub(crate) ground_cycle_no_crypto_addr: Address,
}

impl<M> Client<M>
where
    M: Middleware + Clone,
{
    pub(crate) fn new(
        provider: M,
        agreement_addr: Address,
        ground_cycle_addr: Address,
        ground_cycle_no_crypto_addr: Address,
    ) -> Self {
        Self {
            provider,
            agreement_addr,
            ground_cycle_addr,
            ground_cycle_no_crypto_addr,
        }
    }

    pub(crate) fn agreement(&self) -> AgreementContract<M> {
        AgreementContract::new(self.agreement_addr, Arc::new(self.provider.clone()))
    }

    pub(crate) fn agreement_signer<S: Signer>(
        &self,
        wallet: S,
    ) -> AgreementContract<SignerMiddleware<M, S>> {
        let client = Arc::new(SignerMiddleware::new(self.provider.clone(), wallet));
        AgreementContract::new(self.agreement_addr, client)
    }

    pub(crate) fn ground_cycle(&self) -> GroundCycleContract<M> {
        GroundCycleContract::new(self.ground_cycle_addr, Arc::new(self.provider.clone()))
    }

    pub(crate) fn ground_cycle_signer<S: Signer>(
        &self,
        wallet: S,
    ) -> GroundCycleContract<SignerMiddleware<M, S>> {
        let client = Arc::new(SignerMiddleware::new(self.provider.clone(), wallet));
        GroundCycleContract::new(self.ground_cycle_addr, client)
    }

    pub(crate) fn ground_cycle_no_crypto(&self) -> GroundCycleNoCryptoContract<M> {
        GroundCycleNoCryptoContract::new(
            self.ground_cycle_no_crypto_addr,
            Arc::new(self.provider.clone()),
        )
    }

    pub(crate) fn ground_cycle_no_crypto_signer<S: Signer>(
        &self,
        wallet: S,
    ) -> GroundCycleNoCryptoContract<SignerMiddleware<M, S>> {
        let client = Arc::new(SignerMiddleware::new(self.provider.clone(), wallet));
        GroundCycleNoCryptoContract::new(self.ground_cycle_no_crypto_addr, client)
    }
}
