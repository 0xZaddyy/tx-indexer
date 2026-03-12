use std::collections::HashSet;

use tx_indexer_primitives::{HasScriptPubkey, HasSequence};

use crate::classify::classify_script_pubkey;
use crate::types::OutputType;

/// Returns true if any input signals RBF.
pub fn tx_signals_rbf(inputs: &[impl HasSequence]) -> bool {
    inputs.iter().any(|input| input.sequence() < 0xfffffffe)
}

/// Returns true if locktime is non-zero (heuristic for anti-fee-sniping).
pub fn anti_fee_snipe(locktime: u32) -> bool {
    locktime != 0
}

/// Returns true if any output scriptPubKey matches any prevout scriptPubKey.
pub fn address_reuse(outputs: &[impl HasScriptPubkey], prevouts: &[impl HasScriptPubkey]) -> bool {
    let input_scripts: HashSet<Vec<u8>> =
        prevouts.iter().map(|p| p.script_pubkey_bytes()).collect();
    let output_scripts: HashSet<Vec<u8>> =
        outputs.iter().map(|o| o.script_pubkey_bytes()).collect();

    !input_scripts.is_disjoint(&output_scripts)
}

/// Returns true if prevouts have more than one distinct scriptPubKey type.
pub fn mixed_input_types(prevouts: &[impl HasScriptPubkey]) -> bool {
    let types: HashSet<OutputType> = prevouts
        .iter()
        .map(|p| classify_script_pubkey(&p.script_pubkey_bytes()))
        .collect();
    types.len() > 1
}
