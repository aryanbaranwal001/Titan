use crate::AppConfig;
use crate::structure::*;

use proto_build::sf::ethereum::r#type::v2::{
    AccessTuple, AccountCreation, BalanceChange, BigInt, BlockHeader, Call,
    Call as TransactionTraceCall, CodeChange, GasChange, Log, NonceChange, SetCodeAuthorization,
    StorageChange, TransactionReceipt, TransactionTrace, Withdrawal,
};

impl AppConfig {
    pub fn extract_header(&self, h: BlockHeader) -> ExtractedBlockHeader {
        let cfg = &self.block.blockheader;

        ExtractedBlockHeader {
            parent_hash: if cfg.parent_hash {
                Some(h.parent_hash)
            } else {
                None
            },
            uncle_hash: if cfg.uncle_hash {
                Some(h.uncle_hash)
            } else {
                None
            },
            coinbase: if cfg.coinbase { Some(h.coinbase) } else { None },
            state_root: if cfg.state_root {
                Some(h.state_root)
            } else {
                None
            },
            transactions_root: if cfg.transactions_root {
                Some(h.transactions_root)
            } else {
                None
            },
            receipt_root: if cfg.receipt_root {
                Some(h.receipt_root)
            } else {
                None
            },
            logs_bloom: if cfg.logs_bloom {
                Some(h.logs_bloom)
            } else {
                None
            },
            difficulty: if cfg.difficulty { h.difficulty } else { None },
            total_difficulty: if cfg.total_difficulty {
                h.total_difficulty
            } else {
                None
            },

            number: if cfg.number { Some(h.number) } else { None },
            gas_limit: if cfg.gas_limit {
                Some(h.gas_limit)
            } else {
                None
            },
            gas_used: if cfg.gas_used { Some(h.gas_used) } else { None },

            timestamp: if cfg.timestamp { h.timestamp } else { None },
            extra_data: if cfg.extra_data {
                Some(h.extra_data)
            } else {
                None
            },
            mix_hash: if cfg.mix_hash { Some(h.mix_hash) } else { None },
            nonce: if cfg.nonce { Some(h.nonce) } else { None },
            hash: if cfg.hash { Some(h.hash) } else { None },
            base_fee_per_gas: if cfg.base_fee_per_gas {
                h.base_fee_per_gas
            } else {
                None
            },

            withdrawals_root: if cfg.withdrawals_root {
                Some(h.withdrawals_root)
            } else {
                None
            },
            tx_dependency: if cfg.tx_dependency {
                h.tx_dependency
            } else {
                None
            },
            blob_gas_used: if cfg.blob_gas_used {
                h.blob_gas_used
            } else {
                None
            },
            excess_blob_gas: if cfg.excess_blob_gas {
                h.excess_blob_gas
            } else {
                None
            },
            parent_beacon_root: if cfg.parent_beacon_root {
                Some(h.parent_beacon_root)
            } else {
                None
            },
            requests_hash: if cfg.requests_hash {
                Some(h.requests_hash)
            } else {
                None
            },
        }
    }

    pub fn extract_uncles(&self, h: BlockHeader) -> ExtractedUncleBlockHeader {
        let cfg = &self.block.blockheader;

        ExtractedUncleBlockHeader {
            parent_hash: if cfg.parent_hash {
                Some(h.parent_hash)
            } else {
                None
            },
            uncle_hash: if cfg.uncle_hash {
                Some(h.uncle_hash)
            } else {
                None
            },
            coinbase: if cfg.coinbase { Some(h.coinbase) } else { None },
            state_root: if cfg.state_root {
                Some(h.state_root)
            } else {
                None
            },
            transactions_root: if cfg.transactions_root {
                Some(h.transactions_root)
            } else {
                None
            },
            receipt_root: if cfg.receipt_root {
                Some(h.receipt_root)
            } else {
                None
            },
            logs_bloom: if cfg.logs_bloom {
                Some(h.logs_bloom)
            } else {
                None
            },
            difficulty: if cfg.difficulty { h.difficulty } else { None },
            total_difficulty: if cfg.total_difficulty {
                h.total_difficulty
            } else {
                None
            },

            number: if cfg.number { Some(h.number) } else { None },
            gas_limit: if cfg.gas_limit {
                Some(h.gas_limit)
            } else {
                None
            },
            gas_used: if cfg.gas_used { Some(h.gas_used) } else { None },

            timestamp: if cfg.timestamp { h.timestamp } else { None },
            extra_data: if cfg.extra_data {
                Some(h.extra_data)
            } else {
                None
            },
            mix_hash: if cfg.mix_hash { Some(h.mix_hash) } else { None },
            nonce: if cfg.nonce { Some(h.nonce) } else { None },
            hash: if cfg.hash { Some(h.hash) } else { None },
            base_fee_per_gas: if cfg.base_fee_per_gas {
                h.base_fee_per_gas
            } else {
                None
            },

            withdrawals_root: if cfg.withdrawals_root {
                Some(h.withdrawals_root)
            } else {
                None
            },
            tx_dependency: if cfg.tx_dependency {
                h.tx_dependency
            } else {
                None
            },
            blob_gas_used: if cfg.blob_gas_used {
                h.blob_gas_used
            } else {
                None
            },
            excess_blob_gas: if cfg.excess_blob_gas {
                h.excess_blob_gas
            } else {
                None
            },
            parent_beacon_root: if cfg.parent_beacon_root {
                Some(h.parent_beacon_root)
            } else {
                None
            },
            requests_hash: if cfg.requests_hash {
                Some(h.requests_hash)
            } else {
                None
            },
        }
    }

    pub fn extract_system_call(&self, c: Call) -> ExtractedSystemCall {
        let cfg = &self.block.system_calls;

        ExtractedSystemCall {
            index: if cfg.index { Some(c.index) } else { None },
            parent_index: if cfg.parent_index {
                Some(c.parent_index)
            } else {
                None
            },
            depth: if cfg.depth { Some(c.depth) } else { None },
            call_type: if cfg.call_type {
                Some(c.call_type)
            } else {
                None
            },
            caller: if cfg.caller { Some(c.caller) } else { None },
            address: if cfg.address { Some(c.address) } else { None },
            address_delegates_to: if cfg.address_delegates_to {
                c.address_delegates_to
            } else {
                None
            },
            value: if cfg.value { c.value } else { None },
            gas_limit: if cfg.gas_limit {
                Some(c.gas_limit)
            } else {
                None
            },
            gas_consumed: if cfg.gas_consumed {
                Some(c.gas_consumed)
            } else {
                None
            },
            return_data: if cfg.return_data {
                Some(c.return_data)
            } else {
                None
            },
            input: if cfg.input { Some(c.input) } else { None },
            executed_code: if cfg.executed_code {
                Some(c.executed_code)
            } else {
                None
            },
            suicide: if cfg.suicide { Some(c.suicide) } else { None },
            keccak_preimages: if cfg.keccak_preimages {
                Some(c.keccak_preimages)
            } else {
                None
            },
            status_failed: if cfg.status_failed {
                Some(c.status_failed)
            } else {
                None
            },
            status_reverted: if cfg.status_reverted {
                Some(c.status_reverted)
            } else {
                None
            },
            failure_reason: if cfg.failure_reason {
                Some(c.failure_reason)
            } else {
                None
            },
            state_reverted: if cfg.state_reverted {
                Some(c.state_reverted)
            } else {
                None
            },
            begin_ordinal: if cfg.begin_ordinal {
                Some(c.begin_ordinal)
            } else {
                None
            },
            end_ordinal: if cfg.end_ordinal {
                Some(c.end_ordinal)
            } else {
                None
            },
            storage_changes: Some(
                c.storage_changes
                    .into_iter()
                    .map(|s| self.extract_storage_change(s))
                    .collect(),
            ),
            balance_changes: Some(
                c.balance_changes
                    .into_iter()
                    .map(|b| self.extract_balance_change(b))
                    .collect(),
            ),
            code_changes: Some(
                c.code_changes
                    .into_iter()
                    .map(|cc| self.extract_code_change(cc))
                    .collect(),
            ),
            nonce_changes: Some(
                c.nonce_changes
                    .into_iter()
                    .map(|n| self.extract_nonce_change(n))
                    .collect(),
            ),
            gas_changes: Some(
                c.gas_changes
                    .into_iter()
                    .map(|g| self.extract_gas_change(g))
                    .collect(),
            ),
            account_creations: Some(
                c.account_creations
                    .into_iter()
                    .map(|a| self.extract_account_creation(a))
                    .collect(),
            ),
        }
    }

    fn extract_storage_change(&self, s: StorageChange) -> ExtractedStorageChange {
        let cfg = &self.block.system_calls.storage_changes;
        ExtractedStorageChange {
            address: if cfg.address { Some(s.address) } else { None },
            key: if cfg.key { Some(s.key) } else { None },
            old_value: if cfg.old_value {
                Some(s.old_value)
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(s.new_value)
            } else {
                None
            },
            ordinal: if cfg.ordinal { Some(s.ordinal) } else { None },
        }
    }

    fn extract_balance_change(&self, b: BalanceChange) -> ExtractedBalanceChange {
        let cfg = &self.block.system_calls.balance_changes;
        ExtractedBalanceChange {
            address: if cfg.address { Some(b.address) } else { None },
            old_value: if cfg.old_value { b.old_value } else { None },
            new_value: if cfg.new_value { b.new_value } else { None },
            reason: if cfg.reason { Some(b.reason) } else { None },
            ordinal: if cfg.ordinal { Some(b.ordinal) } else { None },
        }
    }

    fn extract_nonce_change(&self, n: NonceChange) -> ExtractedNonceChange {
        let cfg = &self.block.system_calls.nonce_changes;
        ExtractedNonceChange {
            address: if cfg.address { Some(n.address) } else { None },
            old_value: if cfg.old_value {
                Some(BigInt {
                    bytes: n.old_value.to_be_bytes().to_vec(),
                })
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(BigInt {
                    bytes: n.new_value.to_be_bytes().to_vec(),
                })
            } else {
                None
            },
            ordinal: if cfg.ordinal { Some(n.ordinal) } else { None },
        }
    }

    fn extract_code_change(&self, cc: CodeChange) -> ExtractedCodeChange {
        let cfg = &self.block.system_calls.code_changes;
        ExtractedCodeChange {
            address: if cfg.address { Some(cc.address) } else { None },
            old_hash: if cfg.old_hash {
                Some(cc.old_hash)
            } else {
                None
            },
            old_code: if cfg.old_code {
                Some(cc.old_code)
            } else {
                None
            },
            new_hash: if cfg.new_hash {
                Some(cc.new_hash)
            } else {
                None
            },
            new_code: if cfg.new_code {
                Some(cc.new_code)
            } else {
                None
            },
            ordinal: cc.ordinal,
        }
    }

    fn extract_gas_change(&self, g: GasChange) -> ExtractedGasChange {
        let cfg = &self.block.system_calls.gas_changes;
        ExtractedGasChange {
            old_value: if cfg.old_value {
                Some(g.old_value)
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(g.new_value)
            } else {
                None
            },
            reason: if cfg.reason { Some(g.reason) } else { None },
            ordinal: if cfg.ordinal { Some(g.ordinal) } else { None },
        }
    }

    fn extract_account_creation(&self, a: AccountCreation) -> ExtractedAccountCreations {
        let cfg = &self.block.system_calls.account_creations;
        ExtractedAccountCreations {
            account: if cfg.account { Some(a.account) } else { None },
            ordinal: if cfg.ordinal { Some(a.ordinal) } else { None },
        }
    }

    pub fn extract_transaction_traces(&self, t: TransactionTrace) -> ExtractedTransactionTraces {
        let cfg = &self.block.transaction_traces;

        ExtractedTransactionTraces {
            to: if cfg.to { Some(t.to) } else { None },
            nonce: if cfg.nonce { Some(t.nonce) } else { None },
            gas_price: if cfg.gas_price { t.gas_price } else { None },
            gas_limit: if cfg.gas_limit {
                Some(t.gas_limit)
            } else {
                None
            },
            value: if cfg.value { t.value } else { None },
            input: if cfg.input { Some(t.input) } else { None },
            v: if cfg.v { Some(t.v) } else { None },
            r: if cfg.r { Some(t.r) } else { None },
            s: if cfg.s { Some(t.s) } else { None },
            gas_used: if cfg.gas_used { Some(t.gas_used) } else { None },
            r#type: if cfg.r#type { Some(t.r#type) } else { None },
            max_fee_per_gas: if cfg.max_fee_per_gas {
                t.max_fee_per_gas
            } else {
                None
            },
            max_priority_fee_per_gas: if cfg.max_priority_fee_per_gas {
                t.max_priority_fee_per_gas
            } else {
                None
            },
            index: if cfg.index { Some(t.index) } else { None },
            hash: if cfg.hash { Some(t.hash) } else { None },
            from: if cfg.from { Some(t.from) } else { None },
            return_data: if cfg.return_data {
                Some(t.return_data)
            } else {
                None
            },
            public_key: if cfg.public_key {
                Some(t.public_key)
            } else {
                None
            },
            begin_ordinal: if cfg.begin_ordinal {
                Some(t.begin_ordinal)
            } else {
                None
            },
            end_ordinal: if cfg.end_ordinal {
                Some(t.end_ordinal)
            } else {
                None
            },
            status: if cfg.status { Some(t.status) } else { None },
            blob_gas: if cfg.blob_gas { t.blob_gas } else { None },
            blob_gas_fee_cap: if cfg.blob_gas_fee_cap {
                t.blob_gas_fee_cap
            } else {
                None
            },
            blob_hashes: if cfg.blob_hashes {
                Some(t.blob_hashes)
            } else {
                None
            },

            access_list: Some(
                t.access_list
                    .into_iter()
                    .map(|a| self.extract_access_tuple(a))
                    .collect(),
            ),
            receipt: t.receipt.map(|r| self.extract_transaction_receipt(r)),
            calls: Some(
                t.calls
                    .into_iter()
                    .map(|c| self.extract_transaction_trace_call(c))
                    .collect(),
            ),
            set_code_authorizations: Some(
                t.set_code_authorizations
                    .into_iter()
                    .map(|s| self.extract_set_code_authorization(s))
                    .collect(),
            ),
        }
    }

    fn extract_access_tuple(&self, a: AccessTuple) -> ExtractedAccessTuple {
        let cfg = &self.block.transaction_traces.access_list;
        ExtractedAccessTuple {
            address: if cfg.address { Some(a.address) } else { None },
            storage_keys: if cfg.storage_keys {
                Some(a.storage_keys)
            } else {
                None
            },
        }
    }

    fn extract_transaction_receipt(&self, r: TransactionReceipt) -> ExtractedTransactionReceipt {
        let cfg = &self.block.transaction_traces.receipt;
        ExtractedTransactionReceipt {
            state_root: if cfg.state_root {
                Some(r.state_root)
            } else {
                None
            },
            cumulative_gas_used: if cfg.cumulative_gas_used {
                Some(r.cumulative_gas_used)
            } else {
                None
            },
            logs_bloom: if cfg.logs_bloom {
                Some(r.logs_bloom)
            } else {
                None
            },
            logs: Some(r.logs.into_iter().map(|l| self.extract_log(l)).collect()),
            blob_gas_used: if cfg.blob_gas_used {
                r.blob_gas_used
            } else {
                None
            },
            blob_gas_price: if cfg.blob_gas_price {
                r.blob_gas_price
            } else {
                None
            },
        }
    }

    fn extract_log(&self, l: Log) -> ExtractedLog {
        let cfg = &self.block.transaction_traces.receipt.logs;
        ExtractedLog {
            address: if cfg.address { Some(l.address) } else { None },
            topics: if cfg.topics { Some(l.topics) } else { None },
            data: if cfg.data { Some(l.data) } else { None },
            index: if cfg.index { Some(l.index) } else { None },
            block_index: if cfg.block_index {
                Some(l.block_index)
            } else {
                None
            },
            ordinal: if cfg.ordinal { Some(l.ordinal) } else { None },
        }
    }

    fn extract_transaction_trace_call(
        &self,
        c: TransactionTraceCall,
    ) -> ExtractedTracsactionTraceCall {
        let cfg = &self.block.transaction_traces.calls;

        ExtractedTracsactionTraceCall {
            index: if cfg.index { Some(c.index) } else { None },
            parent_index: if cfg.parent_index {
                Some(c.parent_index)
            } else {
                None
            },
            depth: if cfg.depth { Some(c.depth) } else { None },
            call_type: if cfg.call_type {
                Some(c.call_type)
            } else {
                None
            },
            caller: if cfg.caller { Some(c.caller) } else { None },
            address: if cfg.address { Some(c.address) } else { None },
            address_delegates_to: if cfg.address_delegates_to {
                c.address_delegates_to
            } else {
                None
            },
            value: if cfg.value { c.value } else { None },
            gas_limit: if cfg.gas_limit {
                Some(c.gas_limit)
            } else {
                None
            },
            gas_consumed: if cfg.gas_consumed {
                Some(c.gas_consumed)
            } else {
                None
            },
            return_data: if cfg.return_data {
                Some(c.return_data)
            } else {
                None
            },
            input: if cfg.input { Some(c.input) } else { None },
            executed_code: if cfg.executed_code {
                Some(c.executed_code)
            } else {
                None
            },
            suicide: if cfg.suicide { Some(c.suicide) } else { None },
            keccak_preimages: if cfg.keccak_preimages {
                Some(c.keccak_preimages)
            } else {
                None
            },
            status_failed: if cfg.status_failed {
                Some(c.status_failed)
            } else {
                None
            },
            status_reverted: if cfg.status_reverted {
                Some(c.status_reverted)
            } else {
                None
            },
            failure_reason: if cfg.failure_reason {
                Some(c.failure_reason)
            } else {
                None
            },
            state_reverted: if cfg.state_reverted {
                Some(c.state_reverted)
            } else {
                None
            },
            begin_ordinal: if cfg.begin_ordinal {
                Some(c.begin_ordinal)
            } else {
                None
            },
            end_ordinal: if cfg.end_ordinal {
                Some(c.end_ordinal)
            } else {
                None
            },
            storage_changes: Some(
                c.storage_changes
                    .into_iter()
                    .map(|s| self.extract_storage_change_for_transaction_trace(s))
                    .collect(),
            ),
            balance_changes: Some(
                c.balance_changes
                    .into_iter()
                    .map(|b| self.extract_balance_change_for_transaction_trace(b))
                    .collect(),
            ),
            code_changes: Some(
                c.code_changes
                    .into_iter()
                    .map(|cc| self.extract_code_change_for_transaction_trace(cc))
                    .collect(),
            ),
            nonce_changes: Some(
                c.nonce_changes
                    .into_iter()
                    .map(|n| self.extract_nonce_change_for_transaction_trace(n))
                    .collect(),
            ),
            gas_changes: Some(
                c.gas_changes
                    .into_iter()
                    .map(|g| self.extract_gas_change_for_transaction_trace(g))
                    .collect(),
            ),
            account_creations: Some(
                c.account_creations
                    .into_iter()
                    .map(|a| self.extract_account_creation_for_transaction_trace(a))
                    .collect(),
            ),
        }
    }

    fn extract_storage_change_for_transaction_trace(
        &self,
        s: StorageChange,
    ) -> ExtractedStorageChange {
        let cfg = &self.block.transaction_traces.calls.storage_changes;
        ExtractedStorageChange {
            address: if cfg.address { Some(s.address) } else { None },
            key: if cfg.key { Some(s.key) } else { None },
            old_value: if cfg.old_value {
                Some(s.old_value)
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(s.new_value)
            } else {
                None
            },
            ordinal: if cfg.ordinal { Some(s.ordinal) } else { None },
        }
    }

    fn extract_balance_change_for_transaction_trace(
        &self,
        b: BalanceChange,
    ) -> ExtractedBalanceChange {
        let cfg = &self.block.transaction_traces.calls.balance_changes;
        ExtractedBalanceChange {
            address: if cfg.address { Some(b.address) } else { None },
            old_value: if cfg.old_value { b.old_value } else { None },
            new_value: if cfg.new_value { b.new_value } else { None },
            reason: if cfg.reason { Some(b.reason) } else { None },
            ordinal: if cfg.ordinal { Some(b.ordinal) } else { None },
        }
    }

    fn extract_nonce_change_for_transaction_trace(&self, n: NonceChange) -> ExtractedNonceChange {
        let cfg = &self.block.transaction_traces.calls.nonce_changes;
        ExtractedNonceChange {
            address: if cfg.address { Some(n.address) } else { None },
            old_value: if cfg.old_value {
                Some(BigInt {
                    bytes: n.old_value.to_be_bytes().to_vec(),
                })
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(BigInt {
                    bytes: n.new_value.to_be_bytes().to_vec(),
                })
            } else {
                None
            },
            ordinal: if cfg.ordinal { Some(n.ordinal) } else { None },
        }
    }

    fn extract_code_change_for_transaction_trace(&self, cc: CodeChange) -> ExtractedCodeChange {
        let cfg = &self.block.transaction_traces.calls.code_changes;
        ExtractedCodeChange {
            address: if cfg.address { Some(cc.address) } else { None },
            old_hash: if cfg.old_hash {
                Some(cc.old_hash)
            } else {
                None
            },
            old_code: if cfg.old_code {
                Some(cc.old_code)
            } else {
                None
            },
            new_hash: if cfg.new_hash {
                Some(cc.new_hash)
            } else {
                None
            },
            new_code: if cfg.new_code {
                Some(cc.new_code)
            } else {
                None
            },
            ordinal: cc.ordinal,
        }
    }

    fn extract_gas_change_for_transaction_trace(&self, g: GasChange) -> ExtractedGasChange {
        let cfg = &self.block.transaction_traces.calls.gas_changes;
        ExtractedGasChange {
            old_value: if cfg.old_value {
                Some(g.old_value)
            } else {
                None
            },
            new_value: if cfg.new_value {
                Some(g.new_value)
            } else {
                None
            },
            reason: if cfg.reason { Some(g.reason) } else { None },
            ordinal: if cfg.ordinal { Some(g.ordinal) } else { None },
        }
    }

    fn extract_account_creation_for_transaction_trace(
        &self,
        a: AccountCreation,
    ) -> ExtractedAccountCreations {
        let cfg = &self.block.transaction_traces.calls.account_creations;
        ExtractedAccountCreations {
            account: if cfg.account { Some(a.account) } else { None },
            ordinal: if cfg.ordinal { Some(a.ordinal) } else { None },
        }
    }

    fn extract_set_code_authorization(
        &self,
        s: SetCodeAuthorization,
    ) -> ExtractedSetCodeAuthorization {
        let cfg = &self.block.transaction_traces.set_code_authorizations;
        ExtractedSetCodeAuthorization {
            discarded: if cfg.discarded {
                Some(s.discarded)
            } else {
                None
            },
            chain_id: if cfg.chain_id { Some(s.chain_id) } else { None },
            address: if cfg.address { Some(s.address) } else { None },
            nonce: if cfg.nonce { Some(s.nonce) } else { None },
            v: if cfg.v { Some(s.v) } else { None },
            r: if cfg.r { Some(s.r) } else { None },
            s: if cfg.s { Some(s.s) } else { None },
            authority: if cfg.authority { s.authority } else { None },
        }
    }

    pub fn extract_block_balance_change(&self, b: BalanceChange) -> ExtractedBalanceChange {
        let cfg = &self.block.balance_changes;
        ExtractedBalanceChange {
            address: if cfg.address { Some(b.address) } else { None },
            old_value: if cfg.old_value { b.old_value } else { None },
            new_value: if cfg.new_value { b.new_value } else { None },
            reason: if cfg.reason { Some(b.reason) } else { None },
            ordinal: if cfg.ordinal { Some(b.ordinal) } else { None },
        }
    }

    pub fn extract_block_code_change(&self, cc: CodeChange) -> ExtractedCodeChange {
        let cfg = &self.block.code_changes;
        ExtractedCodeChange {
            address: if cfg.address { Some(cc.address) } else { None },
            old_hash: if cfg.old_hash {
                Some(cc.old_hash)
            } else {
                None
            },
            old_code: if cfg.old_code {
                Some(cc.old_code)
            } else {
                None
            },
            new_hash: if cfg.new_hash {
                Some(cc.new_hash)
            } else {
                None
            },
            new_code: if cfg.new_code {
                Some(cc.new_code)
            } else {
                None
            },
            ordinal: cc.ordinal,
        }
    }

    pub fn extract_withdrawal(&self, w: Withdrawal) -> ExtractedWithdrawal {
        let cfg = &self.block.withdrawals;
        ExtractedWithdrawal {
            index: if cfg.index { Some(w.index) } else { None },
            validator_index: if cfg.validator_index {
                Some(w.validator_index)
            } else {
                None
            },
            address: if cfg.address { Some(w.address) } else { None },
            amount: if cfg.amount { Some(w.amount) } else { None },
        }
    }
}

// my todos
// 1. enabled is not implemented for all structs, implement those
