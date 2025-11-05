//! QRDX consensus implementation.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

extern crate alloc;

use alloc::fmt::Debug;
use alloy_primitives::U256;
use reth_chainspec::EthereumHardforks;
use reth_consensus::{Consensus, ConsensusError, HeaderValidator, PostExecutionInput};
use reth_primitives_traits::{BlockBody, SealedBlock, SealedHeader};

/// QRDX consensus implementation.
///
/// Uses standard Ethereum consensus rules.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct QrdxBeaconConsensus {
    /// Configuration
    chain_spec: alloc::sync::Arc<reth_qrdx_chainspec::QrdxChainSpec>,
}

impl QrdxBeaconConsensus {
    /// Create a new instance of [`QrdxBeaconConsensus`]
    pub fn new(chain_spec: alloc::sync::Arc<reth_qrdx_chainspec::QrdxChainSpec>) -> Self {
        Self { chain_spec }
    }
}

impl<H, B> Consensus<H, B> for QrdxBeaconConsensus
where
    H: reth_primitives_traits::BlockHeader,
    B: BlockBody,
{
    fn validate_header_against_parent(
        &self,
        header: &SealedHeader<H>,
        parent: &SealedHeader<H>,
    ) -> Result<(), ConsensusError> {
        reth_consensus::validate_header_base_fee(header, parent, &self.chain_spec)?;
        reth_consensus::validate_header_extradata(header)?;
        reth_consensus::validate_against_parent_timestamp(header, parent)?;

        if self.chain_spec.is_paris_active_at_timestamp(header.timestamp()) {
            reth_consensus::validate_header_unset_difficulty(header.header())?;
        }

        if self.chain_spec.is_shanghai_active_at_timestamp(header.timestamp()) {
            reth_consensus::validate_against_parent_withdrawals_root(header, parent)?;
        }

        if self.chain_spec.is_cancun_active_at_timestamp(header.timestamp()) {
            reth_consensus::validate_against_parent_eip4844(header, parent)?;
            reth_consensus::validate_against_parent_blob_gas_used(header.header(), parent.header())?;
            reth_consensus::validate_against_parent_excess_blob_gas(
                header.header(),
                parent.header(),
            )?;
        }

        Ok(())
    }

    fn validate_header_with_total_difficulty(
        &self,
        header: &H,
        _total_difficulty: U256,
    ) -> Result<(), ConsensusError> {
        reth_consensus::validate_header_extradata(header)?;

        if self.chain_spec.is_paris_active_at_timestamp(header.timestamp()) {
            reth_consensus::validate_header_unset_difficulty(header)?;
        }

        Ok(())
    }

    fn validate_block_post_execution(
        &self,
        block: &SealedBlock<H, B>,
        input: PostExecutionInput<'_>,
    ) -> Result<(), ConsensusError> {
        reth_consensus::validate_block_post_execution(block, &self.chain_spec, input)
    }
}

impl<H> HeaderValidator<H> for QrdxBeaconConsensus
where
    H: reth_primitives_traits::BlockHeader,
{
    fn validate_header(&self, header: &SealedHeader<H>) -> Result<(), ConsensusError> {
        reth_consensus::validate_header_standalone(header, &self.chain_spec)
    }

    fn validate_header_against_parent(
        &self,
        header: &SealedHeader<H>,
        parent: &SealedHeader<H>,
    ) -> Result<(), ConsensusError> {
        self.validate_header_against_parent(header, parent)
    }

    fn validate_header_with_total_difficulty(
        &self,
        header: &H,
        total_difficulty: U256,
    ) -> Result<(), ConsensusError> {
        self.validate_header_with_total_difficulty(header, total_difficulty)
    }
}
