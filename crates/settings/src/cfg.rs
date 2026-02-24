use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BlockCfg {
    pub hash: bool,
    pub number: bool,
    pub size: bool,
    pub detail_level: bool,
    pub ver: bool,

    pub blockheader: BlockHeaderCfg,
    pub system_calls: SystemCallCfg,
    pub uncles: UncleBlockHeaderCfg,
    pub transaction_traces: TransactionTraceCfg,
    pub balance_changes: BalanceChangesCfg,
    pub code_changes: CodeChangesCfg,
    pub withdrawals: WithdrawalCfg,
}

#[derive(Deserialize, Debug)]
pub struct BlockHeaderCfg {
    pub enabled: bool,

    pub parent_hash: bool,
    pub uncle_hash: bool,
    pub coinbase: bool,
    pub state_root: bool,
    pub transactions_root: bool,
    pub receipt_root: bool,
    pub logs_bloom: bool,
    pub difficulty: bool,
    pub total_difficulty: bool,
    pub number: bool,
    pub gas_limit: bool,
    pub gas_used: bool,
    pub timestamp: bool,
    pub extra_data: bool,
    pub mix_hash: bool,
    pub nonce: bool,
    pub hash: bool,
    pub base_fee_per_gas: bool,
    pub withdrawals_root: bool,
    pub tx_dependency: bool,
    pub blob_gas_used: bool,
    pub excess_blob_gas: bool,
    pub parent_beacon_root: bool,
    pub requests_hash: bool,
}

#[derive(Deserialize, Debug)]
pub struct SystemCallCfg {
    pub enabled: bool,

    pub index: bool,
    pub parent_index: bool,
    pub depth: bool,
    pub call_type: bool,
    pub caller: bool,
    pub address: bool,
    pub address_delegates_to: bool,
    pub value: bool,
    pub gas_limit: bool,
    pub gas_consumed: bool,
    pub return_data: bool,
    pub input: bool,
    pub executed_code: bool,
    pub suicide: bool,
    pub keccak_preimages: bool,
    pub status_failed: bool,
    pub status_reverted: bool,
    pub failure_reason: bool,
    pub state_reverted: bool,
    pub begin_ordinal: bool,
    pub end_ordinal: bool,
    pub storage_changes: StorageChangesCfg,
    pub balance_changes: BalanceChangesCfg,
    pub nonce_changes: NonceChangesCfg,
    pub code_changes: CodeChangesCfg,
    pub gas_changes: GasChangesCfg,
    pub account_creations: AccountCreationsCfg,
}

#[derive(Deserialize, Debug)]
pub struct UncleBlockHeaderCfg {
    pub enabled: bool,

    pub parent_hash: bool,
    pub uncle_hash: bool,
    pub coinbase: bool,
    pub state_root: bool,
    pub transactions_root: bool,
    pub receipt_root: bool,
    pub logs_bloom: bool,
    pub difficulty: bool,
    pub total_difficulty: bool,
    pub number: bool,
    pub gas_limit: bool,
    pub gas_used: bool,
    pub timestamp: bool,
    pub extra_data: bool,
    pub mix_hash: bool,
    pub nonce: bool,
    pub hash: bool,
    pub base_fee_per_gas: bool,
    pub withdrawals_root: bool,
    pub tx_dependency: bool,
    pub blob_gas_used: bool,
    pub excess_blob_gas: bool,
    pub parent_beacon_root: bool,
    pub requests_hash: bool,
}

#[derive(Deserialize, Debug)]
pub struct TransactionTraceCfg {
    pub enabled: bool,

    pub to: bool,
    pub nonce: bool,
    pub gas_price: bool,
    pub gas_limit: bool,
    pub value: bool,
    pub input: bool,
    pub v: bool,
    pub r: bool,
    pub s: bool,
    pub gas_used: bool,
    pub r#type: bool,
    pub max_fee_per_gas: bool,
    pub max_priority_fee_per_gas: bool,
    pub index: bool,
    pub hash: bool,
    pub from: bool,
    pub return_data: bool,
    pub public_key: bool,
    pub begin_ordinal: bool,
    pub end_ordinal: bool,
    pub status: bool,
    pub blob_gas: bool,
    pub blob_gas_fee_cap: bool,
    pub blob_hashes: bool,

    pub access_list: AccessTupleCfg,
    pub receipt: TransactionReceiptCfg,
    pub calls: TracsactionTraceCallCfg,
    pub set_code_authorizations: SetCodeAuthorizationCfg,
}

#[derive(Deserialize, Debug)]
pub struct WithdrawalCfg {
    pub enabled: bool,

    pub index: bool,
    pub validator_index: bool,
    pub address: bool,
    pub amount: bool,
}

#[derive(Deserialize, Debug)]
pub struct StorageChangesCfg {
    pub enabled: bool,

    pub address: bool,
    pub key: bool,
    pub old_value: bool,
    pub new_value: bool,
    pub ordinal: bool,
}
#[derive(Deserialize, Debug)]
pub struct BalanceChangesCfg {
    pub enabled: bool,

    pub address: bool,
    pub old_value: bool,
    pub new_value: bool,
    pub reason: bool,
    pub ordinal: bool,
}
#[derive(Deserialize, Debug)]
pub struct NonceChangesCfg {
    pub enabled: bool,

    pub address: bool,
    pub old_value: bool,
    pub new_value: bool,
    pub ordinal: bool,
}

#[derive(Deserialize, Debug)]
pub struct CodeChangesCfg {
    pub enabled: bool,

    pub address: bool,
    pub old_hash: bool,
    pub old_code: bool,
    pub new_hash: bool,
    pub new_code: bool,
    pub ordinal: bool,
}

#[derive(Deserialize, Debug)]
pub struct GasChangesCfg {
    pub enabled: bool,

    pub old_value: bool,
    pub new_value: bool,
    pub reason: bool,
    pub ordinal: bool,
}

#[derive(Deserialize, Debug)]
pub struct AccountCreationsCfg {
    pub enabled: bool,

    pub account: bool,
    pub ordinal: bool,
}

#[derive(Deserialize, Debug)]
pub struct SetCodeAuthorizationCfg {
    pub enabled: bool,

    pub discarded: bool,
    pub chain_id: bool,
    pub address: bool,
    pub nonce: bool,
    pub v: bool,
    pub r: bool,
    pub s: bool,
    pub authority: bool,
}

#[derive(Deserialize, Debug)]
pub struct TracsactionTraceCallCfg {
    pub enabled: bool,

    pub index: bool,
    pub parent_index: bool,
    pub depth: bool,
    pub call_type: bool,
    pub caller: bool,
    pub address: bool,
    pub address_delegates_to: bool,
    pub value: bool,
    pub gas_limit: bool,
    pub gas_consumed: bool,
    pub return_data: bool,
    pub input: bool,
    pub executed_code: bool,
    pub suicide: bool,
    pub keccak_preimages: bool,
    pub status_failed: bool,
    pub status_reverted: bool,
    pub failure_reason: bool,
    pub state_reverted: bool,
    pub begin_ordinal: bool,
    pub end_ordinal: bool,

    pub storage_changes: StorageChangesCfg,
    pub balance_changes: BalanceChangesCfg,
    pub nonce_changes: NonceChangesCfg,
    pub code_changes: CodeChangesCfg,
    pub gas_changes: GasChangesCfg,
    pub account_creations: AccountCreationsCfg,
}

#[derive(Deserialize, Debug)]
pub struct TransactionReceiptCfg {
    pub enabled: bool,

    pub state_root: bool,
    pub cumulative_gas_used: bool,
    pub logs_bloom: bool,
    pub blob_gas_used: bool,
    pub blob_gas_price: bool,

    pub logs: LogCfg,
}

#[derive(Deserialize, Debug)]
pub struct LogCfg {
    pub enabled: bool,

    pub address: bool,
    pub topics: bool,
    pub data: bool,
    pub index: bool,
    pub block_index: bool,
    pub ordinal: bool,
}

#[derive(Deserialize, Debug)]
pub struct AccessTupleCfg {
    pub enabled: bool,

    pub address: bool,
    pub storage_keys: bool,
}

// enabled is not used for many of the inner subfields
// make sure to fix that
