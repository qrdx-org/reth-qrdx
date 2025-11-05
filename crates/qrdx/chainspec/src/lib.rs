//! QRDX chain specs.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod dev;
mod qrdx_mainnet;

pub use dev::QRDX_DEV;
pub use qrdx_mainnet::QRDX_MAINNET;

/// Re-export for convenience
pub use reth_qrdx_forks::*;

use alloc::{boxed::Box, vec::Vec};
use alloy_chains::Chain;
use alloy_consensus::{BlockHeader, Header};
use alloy_eips::eip7840::BlobParams;
use alloy_genesis::Genesis;
use alloy_primitives::{B256, U256};
use derive_more::{Constructor, Deref, From, Into};
use reth_chainspec::{
    BaseFeeParams, ChainSpec, ChainSpecBuilder, DepositContract, EthChainSpec, EthereumHardforks,
    ForkFilter, ForkId, Hardforks, Head,
};
use reth_ethereum_forks::ChainHardforks;
use reth_network_peers::NodeRecord;
use reth_primitives_traits::{sync::LazyLock, SealedHeader};

/// Chain spec builder for QRDX chain.
#[derive(Debug, Default, From)]
pub struct QrdxChainSpecBuilder {
    /// [`ChainSpecBuilder`]
    inner: ChainSpecBuilder,
}

impl QrdxChainSpecBuilder {
    /// Construct a new builder from the qrdx mainnet chain spec.
    pub fn qrdx_mainnet() -> Self {
        let mut inner = ChainSpecBuilder::default()
            .chain(QRDX_MAINNET.chain)
            .genesis(QRDX_MAINNET.genesis.clone());
        let forks = QRDX_MAINNET.hardforks.clone();
        inner = inner.with_forks(forks);

        Self { inner }
    }

    /// Set the chain ID
    pub fn chain(mut self, chain: Chain) -> Self {
        self.inner = self.inner.chain(chain);
        self
    }

    /// Set the genesis block.
    pub fn genesis(mut self, genesis: Genesis) -> Self {
        self.inner = self.inner.genesis(genesis);
        self
    }

    /// Build the resulting [`QrdxChainSpec`].
    ///
    /// # Panics
    ///
    /// This function panics if the chain ID and genesis is not set
    pub fn build(self) -> QrdxChainSpec {
        QrdxChainSpec { inner: self.inner.build() }
    }
}

/// QRDX chain spec type.
#[derive(Debug, Clone, Deref, Into, Constructor, PartialEq, Eq)]
pub struct QrdxChainSpec {
    /// [`ChainSpec`].
    pub inner: ChainSpec,
}

impl QrdxChainSpec {
    /// Converts the given [`Genesis`] into a [`QrdxChainSpec`].
    pub fn from_genesis(genesis: Genesis) -> Self {
        genesis.into()
    }
}

impl EthChainSpec for QrdxChainSpec {
    type Header = Header;

    fn chain(&self) -> Chain {
        self.inner.chain()
    }

    fn base_fee_params_at_timestamp(&self, timestamp: u64) -> BaseFeeParams {
        self.inner.base_fee_params_at_timestamp(timestamp)
    }

    fn blob_params_at_timestamp(&self, timestamp: u64) -> Option<BlobParams> {
        self.inner.blob_params_at_timestamp(timestamp)
    }

    fn deposit_contract(&self) -> Option<&DepositContract> {
        self.inner.deposit_contract()
    }

    fn genesis_hash(&self) -> B256 {
        self.inner.genesis_hash()
    }

    fn prune_delete_limit(&self) -> usize {
        self.inner.prune_delete_limit()
    }

    fn display_hardforks(&self) -> Box<dyn core::fmt::Display> {
        self.inner.display_hardforks()
    }

    fn genesis_header(&self) -> &Self::Header {
        self.inner.genesis_header()
    }

    fn genesis(&self) -> &Genesis {
        self.inner.genesis()
    }

    fn bootnodes(&self) -> Option<Vec<NodeRecord>> {
        self.inner.bootnodes()
    }

    fn is_optimism(&self) -> bool {
        false
    }

    fn final_paris_total_difficulty(&self) -> Option<U256> {
        self.inner.final_paris_total_difficulty()
    }
}

impl Hardforks for QrdxChainSpec {
    fn fork<H: alloy_hardforks::Hardfork>(&self, fork: H) -> reth_ethereum_forks::ForkCondition {
        self.inner.fork(fork)
    }

    fn forks_iter(
        &self,
    ) -> impl Iterator<Item = (&dyn alloy_hardforks::Hardfork, reth_ethereum_forks::ForkCondition)>
    {
        self.inner.forks_iter()
    }

    fn fork_id(&self, head: &Head) -> ForkId {
        self.inner.fork_id(head)
    }

    fn latest_fork_id(&self) -> ForkId {
        self.inner.latest_fork_id()
    }

    fn fork_filter(&self, head: Head) -> ForkFilter {
        self.inner.fork_filter(head)
    }
}

impl EthereumHardforks for QrdxChainSpec {
    fn ethereum_fork_activation(
        &self,
        fork: reth_ethereum_forks::EthereumHardfork,
    ) -> reth_ethereum_forks::ForkCondition {
        self.fork(fork)
    }
}

impl From<Genesis> for QrdxChainSpec {
    fn from(genesis: Genesis) -> Self {
        use reth_ethereum_forks::EthereumHardfork;

        // Block-based hardforks
        let hardfork_opts = [
            (EthereumHardfork::Frontier.boxed(), Some(0)),
            (EthereumHardfork::Homestead.boxed(), genesis.config.homestead_block),
            (EthereumHardfork::Dao.boxed(), genesis.config.dao_fork_block),
            (EthereumHardfork::Tangerine.boxed(), genesis.config.eip150_block),
            (EthereumHardfork::SpuriousDragon.boxed(), genesis.config.eip155_block),
            (EthereumHardfork::Byzantium.boxed(), genesis.config.byzantium_block),
            (EthereumHardfork::Constantinople.boxed(), genesis.config.constantinople_block),
            (EthereumHardfork::Petersburg.boxed(), genesis.config.petersburg_block),
            (EthereumHardfork::Istanbul.boxed(), genesis.config.istanbul_block),
            (EthereumHardfork::MuirGlacier.boxed(), genesis.config.muir_glacier_block),
            (EthereumHardfork::Berlin.boxed(), genesis.config.berlin_block),
            (EthereumHardfork::London.boxed(), genesis.config.london_block),
            (EthereumHardfork::ArrowGlacier.boxed(), genesis.config.arrow_glacier_block),
            (EthereumHardfork::GrayGlacier.boxed(), genesis.config.gray_glacier_block),
        ];

        let mut hardforks = hardfork_opts
            .into_iter()
            .filter_map(|(hardfork, opt)| {
                opt.map(|block| {
                    (
                        hardfork,
                        reth_ethereum_forks::ForkCondition::Block(block),
                    )
                })
            })
            .collect::<Vec<_>>();

        // Paris (Merge) hardfork
        hardforks.push((
            EthereumHardfork::Paris.boxed(),
            reth_ethereum_forks::ForkCondition::TTD {
                activation_block_number: 0,
                total_difficulty: U256::ZERO,
                fork_block: genesis.config.merge_netsplit_block,
            },
        ));

        // Time-based hardforks
        let time_hardfork_opts = [
            (EthereumHardfork::Shanghai.boxed(), genesis.config.shanghai_time),
            (EthereumHardfork::Cancun.boxed(), genesis.config.cancun_time),
            (EthereumHardfork::Prague.boxed(), genesis.config.prague_time),
        ];

        let mut time_hardforks = time_hardfork_opts
            .into_iter()
            .filter_map(|(hardfork, opt)| {
                opt.map(|time| {
                    (
                        hardfork,
                        reth_ethereum_forks::ForkCondition::Timestamp(time),
                    )
                })
            })
            .collect::<Vec<_>>();

        hardforks.append(&mut time_hardforks);

        let chain_hardforks = ChainHardforks::new(hardforks);
        let genesis_header = SealedHeader::seal_slow(reth_chainspec::make_genesis_header(
            &genesis,
            &chain_hardforks,
        ));

        Self {
            inner: ChainSpec {
                chain: genesis.config.chain_id.into(),
                genesis_header,
                genesis,
                hardforks: chain_hardforks,
                paris_block_and_final_difficulty: Some((0, U256::ZERO)),
                ..Default::default()
            },
        }
    }
}

impl From<ChainSpec> for QrdxChainSpec {
    fn from(value: ChainSpec) -> Self {
        Self { inner: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qrdx_dev_genesis() {
        let genesis = QRDX_DEV.genesis_header();
        assert!(genesis.hash_slow() != B256::ZERO);
    }

    #[test]
    fn qrdx_mainnet_genesis() {
        let genesis = QRDX_MAINNET.genesis_header();
        assert!(genesis.hash_slow() != B256::ZERO);
    }
}
