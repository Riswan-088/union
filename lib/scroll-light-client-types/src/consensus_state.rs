use unionlabs::primitives::H256;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConsensusState {
    pub state_root: H256,
    pub timestamp: u64,
    pub ibc_storage_root: H256,
}
