use core::{ffi::CStr, mem::size_of};

use beam_bvm_interface::root::*;
use beam_bvm_util::{
    app::{safe, simple::{self, derive_pk}},
    common::extensions::*,
};
use shared::{params::*, types::Key};

use crate::util::doc_writer::*;

const CONTRACT_ID_PROP: &CStr = to_c_str("cid\0");
const AMOUNT_PROP: &CStr =  to_c_str("amount\0");
const ASSET_ID_PROP: &CStr = to_c_str("aid\0");

pub fn write_props(obj_wrap: ObjectFuncs) -> () {
    obj_wrap
        .string_prop(CONTRACT_ID_PROP, "ContractID\0".to_c_str())
        .string_prop(AMOUNT_PROP, "Amount\0".to_c_str())
        .string_prop(ASSET_ID_PROP, "AssetID\0".to_c_str());
}

pub fn run() -> () {

    let mut arg_contract_id: ContractID = Default::default();
    let mut arg_amount: Amount = Default::default();
    let mut arg_asset_id: AssetID = Default::default();
    simple::doc_get_blob(CONTRACT_ID_PROP, &mut arg_contract_id);
    safe::doc_get_num64(AMOUNT_PROP, &mut arg_amount);
    safe::doc_get_num32(ASSET_ID_PROP, &mut arg_asset_id);
    
    if arg_amount == 0 {
        DOC_WRITER.string_prop("error\0".to_c_str(), "amount required.\0".to_c_str());
        return
    }

    let funds_change = FundsChange {
        m_Aid: arg_asset_id,
        m_Amount: arg_amount,
        m_Consume: 0 // 0 withdraw, else deposit
    };

    let mut my_pk: PubKey = Default::default();
    derive_pk(&mut my_pk, &arg_contract_id);

    let deposit_params = WithdrawParams {
        key: Key {
            account: my_pk,
            asset_id: funds_change.m_Aid,
        },
        amount: funds_change.m_Amount,
    };

    let sig_req = SigRequest {
        m_pID: arg_contract_id.as_ptr() as *const c_void,
        m_nID: size_of::<ContractID>() as u32
    };

    simple::generate_kernel(
        &arg_contract_id, 
        deposit_params.as_method_id(), 
        &deposit_params, 
        Some(&funds_change), 
        Some(&sig_req), 
        "withdraw from Faucet\0".to_c_str(), 
        0
    );
}