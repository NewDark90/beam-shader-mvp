use core::mem::size_of_val;

use beam_bvm_util::{app::util::var_reader::*, common::extensions::*};
use shared::types::*;

use super::doc_writer::*;

pub fn dump_accounts(reader: &VarReader) -> () {
    DOC_WRITER.array_prop("accounts\0".to_c_str(), |accounts| {
        reader.move_all_t(|key: &KeyAccount, val: &AccountData| {
            accounts.object(|account| {
                account.blob_prop("Account\0".to_c_str(), &key.m_KeyInContract.account, size_of_val(&key.m_KeyInContract.account) as u32)
                    .u32_prop("AssetID\0".to_c_str(), key.m_KeyInContract.asset_id)
                    .u64_prop("Amount\0".to_c_str(), val.amount)
                    .u64_prop("h0\0".to_c_str(), val.height);
            });
        });
    });
}