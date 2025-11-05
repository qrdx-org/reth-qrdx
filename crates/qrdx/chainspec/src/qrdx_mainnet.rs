//! Chain specification for the QRDX Mainnet network.

use crate::{LazyLock, QrdxChainSpec};
use alloc::{sync::Arc, vec};
use alloy_chains::Chain;
use alloy_primitives::{b256, U256};
use reth_chainspec::{once_cell, BaseFeeParams, BaseFeeParamsKind, ChainSpec};
use reth_qrdx_forks::QRDX_MAINNET_HARDFORKS;
use reth_primitives_traits::SealedHeader;

/// The QRDX Mainnet spec
pub static QRDX_MAINNET: LazyLock<Arc<QrdxChainSpec>> = LazyLock::new(|| {
    let genesis = serde_json::from_str(include_str!("../res/genesis/qrdx_mainnet.json"))
        .expect("Can't deserialize QRDX Mainnet genesis json");
    let hardforks = QRDX_MAINNET_HARDFORKS.clone();
    let genesis_header =
        SealedHeader::seal_slow(reth_chainspec::make_genesis_header(&genesis, &hardforks));
    QrdxChainSpec {
        inner: ChainSpec {
            chain: Chain::from_id(10000),
            genesis_header,
            genesis,
            paris_block_and_final_difficulty: Some((0, U256::from(0))),
            hardforks,
            base_fee_params: BaseFeeParamsKind::Constant(BaseFeeParams::ethereum()),
            prune_delete_limit: 10000,
            ..Default::default()
        },
    }
    .into()
});
