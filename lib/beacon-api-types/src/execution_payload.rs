use unionlabs::primitives::{Bytes, H160, H256, U256};
#[cfg(feature = "ssz")]
use {
    crate::{
        BYTES_PER_LOGS_BLOOM, MAX_BYTES_PER_TRANSACTION, MAX_EXTRA_DATA_BYTES,
        MAX_TRANSACTIONS_PER_PAYLOAD, MAX_WITHDRAWALS_PER_PAYLOAD,
    },
    ssz::types::{List, Vector},
};

use crate::Withdrawal;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExecutionPayload {
    /// Execution block header fields
    pub parent_hash: H256,
    pub fee_recipient: H160,
    pub state_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: Bytes,
    /// 'difficulty' in the yellow paper
    pub prev_randao: H256,
    /// 'number' in the yellow paper
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::string"))]
    pub block_number: u64,
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::string"))]
    pub gas_limit: u64,
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::string"))]
    pub gas_used: u64,
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::string"))]
    pub timestamp: u64,
    pub extra_data: Bytes,
    pub base_fee_per_gas: U256,
    /// Extra payload fields
    /// Hash of execution block
    pub block_hash: H256,
    pub transactions: Vec<Bytes>,
    pub withdrawals: Vec<Withdrawal>,
    // blob_gas_used: uint64  # [New in Deneb:EIP4844]
    #[cfg_attr(feature = "serde", serde(default, with = "::serde_utils::string"))]
    pub blob_gas_used: u64,
    // excess_blob_gas: uint64  # [New in Deneb:EIP4844]
    #[cfg_attr(feature = "serde", serde(default, with = "::serde_utils::string"))]
    pub excess_blob_gas: u64,
}

#[cfg(feature = "ssz")]
#[derive(Debug, Clone, PartialEq, ssz::Ssz)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExecutionPayloadSsz<
    C: BYTES_PER_LOGS_BLOOM
        + MAX_EXTRA_DATA_BYTES
        + MAX_BYTES_PER_TRANSACTION
        + MAX_TRANSACTIONS_PER_PAYLOAD
        + MAX_WITHDRAWALS_PER_PAYLOAD,
> {
    /// Execution block header fields
    pub parent_hash: H256,
    pub fee_recipient: H160,
    pub state_root: H256,
    pub receipts_root: H256,
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::hex_string"))]
    pub logs_bloom: Vector<u8, C::BYTES_PER_LOGS_BLOOM>,
    /// 'difficulty' in the yellow paper
    pub prev_randao: H256,
    /// 'number' in the yellow paper
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::string"))]
    pub block_number: u64,
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::string"))]
    pub gas_limit: u64,
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::string"))]
    pub gas_used: u64,
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::string"))]
    pub timestamp: u64,
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::hex_string"))]
    pub extra_data: List<u8, C::MAX_EXTRA_DATA_BYTES>,
    pub base_fee_per_gas: U256,
    /// Extra payload fields
    /// Hash of execution block
    pub block_hash: H256,
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::hex_string_list"))]
    pub transactions: List<List<u8, C::MAX_BYTES_PER_TRANSACTION>, C::MAX_TRANSACTIONS_PER_PAYLOAD>,
    pub withdrawals: List<Withdrawal, C::MAX_WITHDRAWALS_PER_PAYLOAD>,
    // blob_gas_used: uint64  # [New in Deneb:EIP4844]
    #[cfg_attr(feature = "serde", serde(default, with = "::serde_utils::string"))]
    pub blob_gas_used: u64,
    // excess_blob_gas: uint64  # [New in Deneb:EIP4844]
    #[cfg_attr(feature = "serde", serde(default, with = "::serde_utils::string"))]
    pub excess_blob_gas: u64,
}

// #[cfg(feature = "ssz")]
// impl<C> ExecutionPayload<C>
// where
//     C: BYTES_PER_LOGS_BLOOM
//         + MAX_EXTRA_DATA_BYTES
//         + MAX_BYTES_PER_TRANSACTION
//         + MAX_TRANSACTIONS_PER_PAYLOAD
//         + MAX_WITHDRAWALS_PER_PAYLOAD,
// {
//     #[must_use]
//     pub fn to_header(self) -> ExecutionPayloadHeader<C> {
//         ExecutionPayloadHeader {
//             parent_hash: self.parent_hash,
//             fee_recipient: self.fee_recipient,
//             state_root: self.state_root,
//             receipts_root: self.receipts_root,
//             logs_bloom: self.logs_bloom,
//             prev_randao: self.prev_randao,
//             block_number: self.block_number,
//             gas_limit: self.gas_limit,
//             gas_used: self.gas_used,
//             timestamp: self.timestamp,
//             extra_data: self.extra_data,
//             base_fee_per_gas: self.base_fee_per_gas,
//             block_hash: self.block_hash,
//             transactions_root: self.transactions.tree_hash_root().into(),
//             withdrawals_root: self.withdrawals.tree_hash_root().into(),
//             blob_gas_used: self.blob_gas_used,
//             excess_blob_gas: self.excess_blob_gas,
//         }
//     }
// }
