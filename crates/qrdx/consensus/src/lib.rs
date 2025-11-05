//! QRDX consensus implementation.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use reth_qrdx_chainspec::QrdxChainSpec;

/// QRDX consensus implementation.
///
/// Uses standard Ethereum Beacon consensus rules with QRDX chain spec.
pub type QrdxBeaconConsensus = reth_ethereum_consensus::EthBeaconConsensus<QrdxChainSpec>;
