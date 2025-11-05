# QRDX Network - Ethereum Hardfork

QRDX is a hardfork of Ethereum designed to run on the qrdx.org domain. This implementation is built on top of Reth, the modular Ethereum execution client.

## Network Configuration

### Chain IDs
- **Mainnet**: 10000
- **Dev Network**: 9999

### Network Details
- **Domain**: qrdx.org
- **Consensus**: Proof of Stake (Beacon Chain compatible)
- **Hardforks**: All Ethereum hardforks up to Prague are enabled

## Architecture

The QRDX implementation consists of several modular crates:

### Core Crates

1. **reth-qrdx-hardforks** (`crates/qrdx/hardforks`)
   - Defines QRDX mainnet and dev hardfork schedules
   - Based on Ethereum hardforks (Frontier through Prague)

2. **reth-qrdx-chainspec** (`crates/qrdx/chainspec`)
   - Chain specifications for QRDX networks
   - Genesis configurations for mainnet and dev
   - Network parameters and base fee configuration

3. **reth-qrdx-primitives** (`crates/qrdx/primitives`)
   - Primitive types and traits for QRDX
   - Re-exports from reth-primitives-traits

4. **reth-qrdx-consensus** (`crates/qrdx/consensus`)
   - Consensus implementation using Ethereum Beacon consensus rules
   - Type alias to `EthBeaconConsensus<QrdxChainSpec>`

5. **reth-qrdx-evm** (`crates/qrdx/evm`)
   - EVM configuration for QRDX
   - Type alias to `EthEvmConfig<QrdxChainSpec>`
   - Uses standard Ethereum EVM rules

6. **reth-qrdx-node** (`crates/qrdx/node`)
   - Node builder and components
   - Pool, payload, network, executor, and consensus builders
   - Engine validator configuration

7. **reth-qrdx-cli** (`crates/qrdx/cli`)
   - Command-line interface
   - Chain spec parser supporting "qrdx" and "qrdx-dev"
   - Node launcher

8. **qrdx-reth** (`crates/qrdx/bin`)
   - Main binary executable
   - Entry point for the QRDX node

## Genesis Configuration

### Dev Network (Chain ID: 9999)

The dev network includes 20 pre-funded accounts with 10,000 ETH each, derived from the mnemonic:
"test test test test test test test test test test test junk"

**Genesis Parameters:**
- Gas Limit: 30,000,000
- Base Fee: 1 gwei
- Timestamp: 1687277010
- Extra Data: "QRDX" (hex encoded)

### Mainnet (Chain ID: 10000)

**Genesis Parameters:**
- Gas Limit: 30,000,000
- Base Fee: 1 gwei
- Timestamp: 1694304256
- Extra Data: "QRDX-MAINNET" (hex encoded)
- Empty allocation (to be distributed according to token economics)

## Hardfork Schedule

All networks follow the Ethereum hardfork schedule with immediate activation:

- Frontier (Block 0)
- Homestead (Block 0)
- DAO Fork (Block 0)
- Tangerine Whistle (Block 0)
- Spurious Dragon (Block 0)
- Byzantium (Block 0)
- Constantinople (Block 0)
- Petersburg (Block 0)
- Istanbul (Block 0)
- Muir Glacier (Block 0)
- Berlin (Block 0)
- London (Block 0)
- Arrow Glacier (Block 0)
- Gray Glacier (Block 0)
- Paris/Merge (Block 0, TTD: 0)
- Shanghai (Timestamp 0)
- Cancun (Timestamp 0)
- Prague (Timestamp 0)

## Building and Running

### Prerequisites
- Rust toolchain (1.88+)
- Cargo

### Building

```bash
cargo build --release -p qrdx-reth
```

### Running the Node

#### Dev Network
```bash
./target/release/qrdx-reth node --chain qrdx-dev
```

#### Mainnet
```bash
./target/release/qrdx-reth node --chain qrdx
```

#### Custom Genesis
```bash
./target/release/qrdx-reth node --chain /path/to/genesis.json
```

## Node Configuration

The QRDX node supports all standard Reth configuration options:

- `--datadir`: Data directory for the node
- `--http`: Enable HTTP RPC server
- `--http.addr`: HTTP RPC server address
- `--http.port`: HTTP RPC server port
- `--ws`: Enable WebSocket RPC server
- `--authrpc.addr`: Engine API authentication RPC address
- `--authrpc.port`: Engine API authentication RPC port
- `--authrpc.jwtsecret`: Path to JWT secret file for Engine API

### Example Configuration

```bash
qrdx-reth node \
  --chain qrdx \
  --datadir /var/lib/qrdx \
  --http \
  --http.addr 0.0.0.0 \
  --http.port 8545 \
  --ws \
  --authrpc.addr 127.0.0.1 \
  --authrpc.port 8551 \
  --authrpc.jwtsecret /path/to/jwt.hex
```

## Network Endpoints

### RPC Endpoints
- HTTP RPC: `http://rpc.qrdx.org:8545`
- WebSocket: `ws://rpc.qrdx.org:8546`

### Consensus Layer
QRDX is compatible with any Ethereum consensus client (Lighthouse, Prysm, Teku, Nimbus, Lodestar) configured for the QRDX network.

## Development

### Testing

```bash
cargo test -p reth-qrdx-chainspec
cargo test -p reth-qrdx-hardforks
cargo test -p reth-qrdx-consensus
cargo test -p reth-qrdx-evm
```

### Linting

```bash
cargo +nightly fmt --all
cargo clippy --workspace --all-features
```

## Architecture Decisions

1. **Ethereum Compatibility**: QRDX uses standard Ethereum consensus and EVM rules to ensure maximum compatibility with existing tools and infrastructure.

2. **Modular Design**: Following Reth's architecture, QRDX is split into focused crates that can be independently developed and tested.

3. **Type Aliases**: Where possible, QRDX uses type aliases to Ethereum implementations (e.g., `EthBeaconConsensus`, `EthEvmConfig`) to minimize code duplication and maintenance.

4. **Chain Spec Flexibility**: The chain spec can be loaded from JSON files, allowing for easy configuration of different networks and test scenarios.

## Future Enhancements

- Custom precompiles for QRDX-specific functionality
- Network-specific optimizations
- Enhanced monitoring and metrics
- Additional RPC methods for QRDX-specific features

## Resources

- [Reth Documentation](https://reth.rs)
- [Reth GitHub](https://github.com/paradigmxyz/reth)
- [Ethereum Specification](https://ethereum.github.io/execution-specs/)
- [QRDX Website](https://qrdx.org)

## License

Licensed under either of:
- Apache License, Version 2.0
- MIT License

at your option.
