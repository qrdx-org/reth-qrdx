//! Chain specification in dev mode for QRDX.

use alloc::sync::Arc;

use alloy_chains::Chain;
use alloy_primitives::U256;
use reth_chainspec::{BaseFeeParams, BaseFeeParamsKind, ChainSpec};
use reth_qrdx_forks::QRDX_DEV_HARDFORKS;
use reth_primitives_traits::SealedHeader;

use crate::{LazyLock, QrdxChainSpec};

/// QRDX dev testnet specification
///
/// Includes 20 prefunded accounts with `10_000` ETH each derived from mnemonic "test test test test
/// test test test test test test test junk".
pub static QRDX_DEV: LazyLock<Arc<QrdxChainSpec>> = LazyLock::new(|| {
    let genesis = serde_json::from_str(include_str!("../res/genesis/qrdx_dev.json"))
        .expect("Can't deserialize QRDX Dev testnet genesis json");
    let hardforks = QRDX_DEV_HARDFORKS.clone();
    let genesis_header =
        SealedHeader::seal_slow(reth_chainspec::make_genesis_header(&genesis, &hardforks));
    QrdxChainSpec {
        inner: ChainSpec {
            chain: Chain::dev(),
            genesis_header,
            genesis,
            paris_block_and_final_difficulty: Some((0, U256::from(0))),
            hardforks,
            base_fee_params: BaseFeeParamsKind::Constant(BaseFeeParams::ethereum()),
            ..Default::default()
        },
    }
    .into()
});
