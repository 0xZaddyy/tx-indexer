use tx_indexer_primitives::HasScriptPubkey;

use crate::classify::classify_script_pubkey;
use crate::types::OutputType;

/// Classify an output by its scriptPubKey.
pub fn output_type(output: &(impl HasScriptPubkey + ?Sized)) -> OutputType {
    classify_script_pubkey(&output.script_pubkey_bytes())
}

/// Bundled trait for output-level fingerprints.
pub trait HasOutputFingerprints: HasScriptPubkey {
    fn output_type(&self) -> OutputType {
        output_type(self)
    }
}

impl<T: HasScriptPubkey> HasOutputFingerprints for T {}
