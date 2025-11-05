//! QRDX CLI implementation.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use clap::Parser;
use reth_chainspec::ChainSpec;
use reth_cli::chainspec::ChainSpecParser;
use reth_cli_commands::node::NoArgs;
use reth_node_builder::{NodeBuilder, WithLaunchContext};
use reth_qrdx_chainspec::{QrdxChainSpec, QRDX_DEV, QRDX_MAINNET};
use reth_qrdx_node::QrdxNode;
use std::sync::Arc;

pub mod chainspec;

pub use reth_cli::cli::{Cli, Commands};
pub use reth_node_core::args::LogArgs;

/// QRDX chain spec parser.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct QrdxChainSpecParser;

impl ChainSpecParser for QrdxChainSpecParser {
    type ChainSpec = QrdxChainSpec;

    const SUPPORTED_CHAINS: &'static [&'static str] = &["qrdx", "qrdx-dev"];

    fn parse(s: &str) -> eyre::Result<Arc<Self::ChainSpec>> {
        Ok(match s {
            "qrdx" | "qrdx-mainnet" => QRDX_MAINNET.clone(),
            "qrdx-dev" | "dev" => QRDX_DEV.clone(),
            _ => {
                // Try to parse as a JSON file
                let raw_json = std::fs::read_to_string(s)?;
                Arc::new(serde_json::from_str::<QrdxChainSpec>(&raw_json)?)
            }
        })
    }
}

/// Launches the QRDX node.
pub async fn run() -> eyre::Result<()> {
    let _guard = reth_cli_util::sigsegv_handler::install();

    // Enable backtraces unless a RUST_BACKTRACE value has already been explicitly provided.
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1");
        }
    }

    let cli = Cli::<QrdxChainSpecParser, NoArgs>::parse();
    cli.run(|builder, _| async move {
        let handle = builder.node(QrdxNode::default()).launch().await?;
        handle.node_exit_future.await
    })
    .await
}
