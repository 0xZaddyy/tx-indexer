use std::collections::HashMap;

use tx_indexer_fingerprints::input::HasInputFingerprints;
use tx_indexer_pipeline::{
    engine::EvalContext,
    expr::Expr,
    node::{Node, NodeId},
    value::{Mask, TxSet},
};
use tx_indexer_primitives::unified::AnyTxId;

/// Node that detects transactions signaling opt-in RBF.
///
/// A transaction signals RBF if any of its inputs has sequence < 0xfffffffe (BIP 125).
/// TODO: this is scaffolding. In the pipeline we should just be able to use map/reduce semantics for any fingerprint. Not build custom nodes for each fingerprint.
pub struct SignalsRbfNode {
    input: Expr<TxSet>,
}

impl SignalsRbfNode {
    pub fn new(input: Expr<TxSet>) -> Self {
        Self { input }
    }
}

impl Node for SignalsRbfNode {
    type OutputValue = Mask<AnyTxId>;

    fn dependencies(&self) -> Vec<NodeId> {
        vec![self.input.id()]
    }

    fn evaluate(&self, ctx: &EvalContext) -> HashMap<AnyTxId, bool> {
        let tx_ids = ctx.get(&self.input);
        tx_ids
            .iter()
            .map(|tx_id| {
                let tx = tx_id.with(ctx.unified_storage());
                let any_rbf = tx.inputs().any(|input| input.signals_rbf());
                (*tx_id, any_rbf)
            })
            .collect()
    }

    fn name(&self) -> &'static str {
        "SignalsRbf"
    }
}

pub struct SignalsRbf;

impl SignalsRbf {
    pub fn new(input: Expr<TxSet>) -> Expr<Mask<AnyTxId>> {
        let ctx = input.context().clone();
        ctx.register(SignalsRbfNode::new(input))
    }
}
