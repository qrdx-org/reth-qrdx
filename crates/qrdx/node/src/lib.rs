//! QRDX Node implementation.

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
use reth_chainspec::EthereumHardforks;
use reth_node_api::{EngineValidator, FullNodeComponents, NodeAddOns};
use reth_node_builder::{
    components::{
        ComponentsBuilder, ConsensusBuilder, EngineValidatorBuilder, ExecutorBuilder,
        NetworkBuilder, PayloadServiceBuilder, PoolBuilder,
    },
    node::{FullNodeTypes, NodeTypes, NodeTypesWithEngine},
    BuilderContext, Node, PayloadBuilderConfig,
};
use reth_node_types::NodeTypesWithDBAdapter;
use reth_payload_builder::{PayloadBuilderHandle, PayloadBuilderService};
use reth_qrdx_chainspec::QrdxChainSpec;
use reth_qrdx_consensus::QrdxBeaconConsensus;
use reth_qrdx_evm::QrdxEvmConfig;

/// Type configuration for a QRDX node.
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct QrdxNode;

impl QrdxNode {
    /// Creates a new instance of [`QrdxNode`].
    pub const fn new() -> Self {
        Self
    }
}

impl NodeTypes for QrdxNode {
    type Primitives = ();
    type ChainSpec = QrdxChainSpec;
}

impl NodeTypesWithEngine for QrdxNode {
    type Engine = reth_ethereum_payload_builder::EthEngineTypes;
}

/// Add-ons w.r.t. QRDX.
#[derive(Debug, Clone)]
pub struct QrdxAddOns;

impl<N: FullNodeTypes> NodeAddOns<N> for QrdxAddOns {
    type EthApi = reth_node_api::EthApiTypes;
}

impl<Types, N> Node<N> for QrdxNode
where
    Types: NodeTypesWithDBAdapter<ChainSpec = QrdxChainSpec>,
    N: FullNodeTypes<Types = Types>,
{
    type ComponentsBuilder = ComponentsBuilder<
        Types,
        QrdxPoolBuilder,
        QrdxPayloadBuilder<Types>,
        QrdxNetworkBuilder,
        QrdxExecutorBuilder<Types>,
        QrdxConsensusBuilder,
        QrdxEngineValidatorBuilder,
    >;
    type AddOns = QrdxAddOns;

    fn components_builder(&self) -> Self::ComponentsBuilder {
        ComponentsBuilder::default()
            .node_types::<Types>()
            .pool(QrdxPoolBuilder::default())
            .payload(QrdxPayloadBuilder::default())
            .network(QrdxNetworkBuilder::default())
            .executor(QrdxExecutorBuilder::default())
            .consensus(QrdxConsensusBuilder::default())
            .engine_validator(QrdxEngineValidatorBuilder::default())
    }
}

/// A basic QRDX transaction pool.
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct QrdxPoolBuilder;

impl<Types, Node> PoolBuilder<Node> for QrdxPoolBuilder
where
    Types: NodeTypesWithDBAdapter<ChainSpec = QrdxChainSpec>,
    Node: FullNodeTypes<Types = Types>,
{
    type Pool = reth_node_builder::pool::Pool<Node>;

    async fn build_pool(self, ctx: &BuilderContext<Node>) -> eyre::Result<Self::Pool> {
        reth_node_builder::pool::PoolBuilder::default().build_pool(ctx).await
    }
}

/// A basic QRDX payload service.
#[derive(Debug, Default, Clone)]
pub struct QrdxPayloadBuilder<Types> {
    _marker: core::marker::PhantomData<Types>,
}

impl<Types, Node, Pool> PayloadServiceBuilder<Node, Pool> for QrdxPayloadBuilder<Types>
where
    Types: NodeTypesWithDBAdapter<ChainSpec = QrdxChainSpec>,
    Node: FullNodeTypes<Types = Types>,
    Pool: reth_node_builder::pool::TransactionPool + Unpin + 'static,
{
    async fn spawn_payload_service(
        self,
        ctx: &BuilderContext<Node>,
        pool: Pool,
    ) -> eyre::Result<PayloadBuilderHandle<<Node::Types as NodeTypesWithEngine>::Engine>> {
        let payload_builder = reth_ethereum_payload_builder::EthereumPayloadBuilder::new(
            QrdxEvmConfig::new(ctx.chain_spec()),
        );
        let conf = ctx.payload_builder_config();

        let payload_job_config = reth_basic_payload_builder::BasicPayloadJobGeneratorConfig::default()
            .interval(conf.interval())
            .deadline(conf.deadline())
            .max_payload_tasks(conf.max_payload_tasks());

        let payload_generator = reth_basic_payload_builder::BasicPayloadJobGenerator::with_builder(
            ctx.provider().clone(),
            pool,
            ctx.task_executor().clone(),
            payload_job_config,
            payload_builder,
        );

        let (payload_service, payload_builder) =
            PayloadBuilderService::new(payload_generator, ctx.provider().canonical_state_stream());

        ctx.task_executor().spawn_critical("qrdx payload builder service", Box::pin(payload_service));

        Ok(payload_builder)
    }
}

/// A basic QRDX network builder.
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct QrdxNetworkBuilder;

impl<Node, Pool> NetworkBuilder<Node, Pool> for QrdxNetworkBuilder
where
    Node: FullNodeTypes<Types: NodeTypes<ChainSpec = QrdxChainSpec>>,
    Pool: reth_node_builder::pool::TransactionPool + Unpin + 'static,
{
    async fn build_network(
        self,
        ctx: &BuilderContext<Node>,
        pool: Pool,
    ) -> eyre::Result<reth_network::NetworkHandle> {
        reth_node_builder::network::NetworkBuilder::default().build_network(ctx, pool).await
    }
}

/// A basic QRDX executor builder.
#[derive(Debug, Default, Clone, Copy)]
pub struct QrdxExecutorBuilder<Types> {
    _marker: core::marker::PhantomData<Types>,
}

impl<Types, Node> ExecutorBuilder<Node> for QrdxExecutorBuilder<Types>
where
    Types: NodeTypesWithDBAdapter<ChainSpec = QrdxChainSpec>,
    Node: FullNodeTypes<Types = Types>,
{
    type EVM = QrdxEvmConfig;
    type Executor = reth_ethereum_payload_builder::EthExecutionStrategyFactory<Arc<QrdxChainSpec>>;

    async fn build_evm(
        self,
        ctx: &BuilderContext<Node>,
    ) -> eyre::Result<(Self::EVM, Self::Executor)> {
        let evm_config = QrdxEvmConfig::new(ctx.chain_spec());
        let executor = reth_ethereum_payload_builder::EthExecutionStrategyFactory::new(
            ctx.chain_spec(),
            evm_config.clone(),
        );

        Ok((evm_config, executor))
    }
}

/// A basic QRDX consensus builder.
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct QrdxConsensusBuilder;

impl<Node> ConsensusBuilder<Node> for QrdxConsensusBuilder
where
    Node: FullNodeTypes<Types: NodeTypes<ChainSpec = QrdxChainSpec>>,
{
    type Consensus = Arc<QrdxBeaconConsensus>;

    async fn build_consensus(self, ctx: &BuilderContext<Node>) -> eyre::Result<Self::Consensus> {
        Ok(Arc::new(QrdxBeaconConsensus::new(ctx.chain_spec())))
    }
}

/// Builder for [`reth_node_api::EngineValidator`].
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct QrdxEngineValidatorBuilder;

impl<Node, Types> EngineValidatorBuilder<Node> for QrdxEngineValidatorBuilder
where
    Types: NodeTypesWithDBAdapter<ChainSpec = QrdxChainSpec>,
    Node: FullNodeComponents<Types = Types>,
{
    type Validator = reth_node_api::EngineValidatorWrapper<reth_ethereum_engine_primitives::EthEngineValidator>;

    async fn build(self, ctx: &reth_node_builder::AddOnsContext<'_, Node>) -> eyre::Result<Self::Validator> {
        Ok(reth_node_api::EngineValidatorWrapper::new(
            reth_ethereum_engine_primitives::EthEngineValidator::new(ctx.config.chain.clone()),
        ))
    }
}
