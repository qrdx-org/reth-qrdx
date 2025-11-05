//! QRDX EVM configuration and executor.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

extern crate alloc;

use alloc::sync::Arc;
use alloy_primitives::U256;
use reth_chainspec::EthereumHardforks;
use reth_evm_ethereum::{EthEvmConfig, EthExecutionStrategyFactory};
use reth_evm::{ConfigureEvm, ConfigureEvmEnv};
use reth_primitives_traits::Block;
use reth_qrdx_chainspec::QrdxChainSpec;
use revm_primitives::{AnalysisKind, BlobExcessGasAndPrice, BlockEnv, CfgEnv, CfgEnvWithHandlerCfg, Env, HandlerCfg, SpecId, TxEnv};

/// QRDX EVM configuration.
#[derive(Debug, Clone)]
pub struct QrdxEvmConfig {
    inner: EthEvmConfig<Arc<QrdxChainSpec>>,
}

impl QrdxEvmConfig {
    /// Creates a new QRDX EVM configuration with the given chain spec.
    pub const fn new(chain_spec: Arc<QrdxChainSpec>) -> Self {
        Self { inner: EthEvmConfig::new(chain_spec) }
    }

    /// Returns the chain spec.
    pub fn chain_spec(&self) -> &Arc<QrdxChainSpec> {
        &self.inner.chain_spec
    }
}

impl ConfigureEvmEnv for QrdxEvmConfig {
    type Header = alloy_consensus::Header;
    type Error = <EthEvmConfig<Arc<QrdxChainSpec>> as ConfigureEvmEnv>::Error;

    fn fill_tx_env(&self, tx_env: &mut TxEnv, transaction: &reth_primitives_traits::TransactionSigned, sender: alloy_primitives::Address) {
        self.inner.fill_tx_env(tx_env, transaction, sender)
    }

    fn fill_cfg_env(
        &self,
        cfg_env: &mut CfgEnvWithHandlerCfg,
        header: &Self::Header,
        total_difficulty: U256,
    ) {
        self.inner.fill_cfg_env(cfg_env, header, total_difficulty)
    }

    fn next_cfg_and_block_env(&self, parent: &Self::Header, attributes: reth_evm::NextBlockEnvAttributes) -> Result<(CfgEnvWithHandlerCfg, BlockEnv), Self::Error> {
        self.inner.next_cfg_and_block_env(parent, attributes)
    }
}

impl ConfigureEvm for QrdxEvmConfig {
    type DefaultExternalContext<'a> = ();

    fn evm<DB: revm::Database>(&self, db: DB) -> revm::Evm<'_, Self::DefaultExternalContext<'_>, DB> {
        self.inner.evm(db)
    }

    fn evm_with_inspector<DB, I>(&self, db: DB, inspector: I) -> revm::Evm<'_, I, DB>
    where
        DB: revm::Database,
        I: revm::GetInspector<DB>,
    {
        self.inner.evm_with_inspector(db, inspector)
    }

    fn default_external_context<'a>(&self) -> Self::DefaultExternalContext<'a> {
        ()
    }
}

impl reth_evm::EvmFactory for QrdxEvmConfig {
    type ConfigType = Arc<QrdxChainSpec>;
    type Executor = EthExecutionStrategyFactory<Arc<QrdxChainSpec>>;

    fn strategy(&self, args: reth_evm::EvmFactoryArgs<'_>) -> Self::Executor {
        EthExecutionStrategyFactory::new(self.chain_spec().clone(), args.evm_config)
    }
}
