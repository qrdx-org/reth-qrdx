#![allow(missing_docs, rustdoc::missing_crate_level_docs)]

#[global_allocator]
static ALLOC: reth_cli_util::allocator::Allocator = reth_cli_util::allocator::new_allocator();

#[tokio::main]
async fn main() {
    if let Err(err) = reth_qrdx_cli::run().await {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
