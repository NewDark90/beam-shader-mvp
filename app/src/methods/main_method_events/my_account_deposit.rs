use core::ffi::CStr;

use beam_bvm_interface::root::*;
use beam_bvm_util::{
    app::{safe, simple::{self, generate_kernel}},
    common::extensions::*,
};
use shared::params::*;

use crate::util::doc_writer::*;

const CONTRACT_ID_PROP: &CStr = to_c_str("cid\0");
const AMOUNT_PROP: &CStr = to_c_str("amount\0");
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
    let contract_id_result = simple::doc_get_blob(CONTRACT_ID_PROP, &mut arg_contract_id);
    safe::doc_get_num64(AMOUNT_PROP, &mut arg_amount);
    safe::doc_get_num32(ASSET_ID_PROP, &mut arg_asset_id);

    if (arg_amount == 0) {
        DOC_WRITER.string_prop("error\0".to_c_str(), "amount required.\0".to_c_str());
        return
    }

    if contract_id_result.is_err() {
        DOC_WRITER.string_prop("error\0".to_c_str(), "could not get contract param.\0".to_c_str());
        return
    }

    let funds_change = FundsChange {
        m_Aid: arg_asset_id,
        m_Amount: arg_amount,
        m_Consume: 1 // 0 withdraw, else deposit
    };

    let deposit_params = DepositParams {
        asset_id: funds_change.m_Aid,
        amount: funds_change.m_Amount,
    };

    generate_kernel(
        &arg_contract_id, 
        deposit_params.as_method_id(), 
        &deposit_params, 
        Some(&funds_change), 
        None, 
        "deposit to Faucet\0".to_c_str(), 
        0
    );

}

/*

void On_my_account_move(const uint8_t &isDeposit, const ContractID &cid, const Amount &amount, const AssetID &aid, int unused = 0)
{
    if (!amount)
        return OnError("amount should be nnz");

    FundsChange fc;
    fc.m_Amount = amount;
    fc.m_Aid = aid;
    fc.m_Consume = isDeposit;

    if (isDeposit)
    {
        Faucet::Deposit arg;
        arg.m_Aid = fc.m_Aid;
        arg.m_Amount = fc.m_Amount;

        Env::GenerateKernel(&cid, Faucet::Deposit::s_iMethod, &arg, sizeof(arg), &fc, 1, nullptr, 0, "deposit to Faucet", 0);
    }
    else
    {
        Faucet::Withdraw arg;
        arg.m_Amount = fc.m_Amount;
        arg.m_Key.m_Aid = fc.m_Aid;
        DeriveMyPk(arg.m_Key.m_Account, cid);

        SigRequest sig;
        sig.m_pID = &cid;
        sig.m_nID = sizeof(cid);

        Env::GenerateKernel(&cid, Faucet::Withdraw::s_iMethod, &arg, sizeof(arg), &fc, 1, &sig, 1, "withdraw from Faucet", 0);
    }
}

void On_my_account_deposit(const ContractID &cid, const Amount &amount, const AssetID &aid, int unused = 0)
{
    On_my_account_move(1, cid, amount, aid);
}

void On_my_account_withdraw(const ContractID &cid, const Amount &amount, const AssetID &aid, int unused = 0)
{
    On_my_account_move(0, cid, amount, aid);
}

void On_my_account_view(const ContractID &cid, int unused = 0)
{
    PubKey pubKey;
    DeriveMyPk(pubKey, cid);
    DumpAccount(pubKey, cid);
}


*/