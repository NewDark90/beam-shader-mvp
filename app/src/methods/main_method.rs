/*

void OnError(const char *sz)
{
    Env::DocAddText("error", sz);
}

typedef Env::Key_T<Faucet::Key> KeyAccount;

void DumpAccounts(Env::VarReader &r)
{
    Env::DocArray gr("accounts");

    while (true)
    {
        KeyAccount key;
        Faucet::AccountData d;

        if (!r.MoveNext_T(key, d))
            break;

        Env::DocGroup gr("");

        Env::DocAddBlob_T("Account", key.m_KeyInContract.m_Account);
        Env::DocAddNum("AssetID", key.m_KeyInContract.m_Aid);
        Env::DocAddNum("Amount", d.m_Amount);
        Env::DocAddNum("h0", d.m_h0);
    }
}

void DumpAccount(const PubKey &pubKey, const ContractID &cid)
{
    KeyAccount k0, k1;
    k0.m_Prefix.m_Cid = cid;
    k0.m_KeyInContract.m_Account = pubKey;
    k0.m_KeyInContract.m_Aid = 0;

    _POD_(k1) = k0;
    k1.m_KeyInContract.m_Aid = static_cast<AssetID>(-1);

    Env::VarReader r(k0, k1);
    DumpAccounts(r);
}

void On_manager_view(int unused = 0)
{
    EnumAndDumpContracts(Faucet::s_SID);
}

void On_manager_create(const Height &backlogPeriod, const Amount &withdrawLimit, int unused = 0)
{
    if (!backlogPeriod || !withdrawLimit)
        return OnError("backlog and withdraw limit should be nnz");

    Faucet::Params pars;
    pars.m_BacklogPeriod = backlogPeriod;
    pars.m_MaxWithdraw = withdrawLimit;

    Env::GenerateKernel(nullptr, pars.s_iMethod, &pars, sizeof(pars), nullptr, 0, nullptr, 0, "create Faucet contract", 0);
}

void On_manager_destroy(const ContractID &cid, int unused = 0)
{
    Env::GenerateKernel(&cid, 1, nullptr, 0, nullptr, 0, nullptr, 0, "destroy Faucet contract", 0);
}

void On_manager_view_params(const ContractID &cid, int unused = 0)
{
    Env::Key_T<uint8_t> k;
    k.m_Prefix.m_Cid = cid;
    k.m_KeyInContract = 0;

    Faucet::Params pars;
    if (!Env::VarReader::Read_T(k, pars))
        return OnError("failed to read");

    Env::DocGroup gr("params");
    Env::DocAddNum("backlogPeriod", pars.m_BacklogPeriod);
    Env::DocAddNum("withdrawLimit", pars.m_MaxWithdraw);
}

void On_manager_view_funds(const ContractID &cid, int unused = 0)
{
    Env::DocArray gr0("funds");

    WalkerFunds wlk;
    for (wlk.Enum(cid); wlk.MoveNext();)
    {
        Env::DocGroup gr("");

        Env::DocAddNum("Aid", wlk.m_Aid);
        Env::DocAddNum("Amount", wlk.m_Val.m_Lo);
    }
}

void On_manager_view_accounts(const ContractID &cid, int unused = 0)
{
    Env::KeyPrefix k0, k1;
    k0.m_Cid = cid;
    k1.m_Cid = cid;
    k1.m_Tag = KeyTag::Internal + 1;

    Env::VarReader r(k0, k1); // enum all internal contract vars
    DumpAccounts(r);
}

void On_manager_view_account(const ContractID &cid, const PubKey &pubKey, int unused = 0)
{
    DumpAccount(pubKey, cid);
}

void DeriveMyPk(PubKey &pubKey, const ContractID &cid)
{
    Env::DerivePk(pubKey, &cid, sizeof(cid));
}

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

extern "C" void Method_1()
{
    Env::DocGroup root("");

    char szRole[0x10], szAction[0x10];

    if (!Env::DocGetText("role", szRole, sizeof(szRole)))
        return OnError("Role not specified");

    if (!Env::DocGetText("action", szAction, sizeof(szAction)))
        return OnError("Action not specified");

    const char *szErr = nullptr;

    if (!Env::Strcmp(szRole, "manager"))
    {
        if (!Env::Strcmp(szAction, "create"))
        {
            Height arg_backlogPeriod;
            Env::DocGet("backlogPeriod", arg_backlogPeriod);
            Amount arg_withdrawLimit;
            Env::DocGet("withdrawLimit", arg_withdrawLimit);
            On_manager_create(arg_backlogPeriod, arg_withdrawLimit, 0);
            return;
        }
        if (!Env::Strcmp(szAction, "destroy"))
        {
            ContractID arg_cid;
            Env::DocGet("cid", arg_cid);
            On_manager_destroy(arg_cid, 0);
            return;
        }
        if (!Env::Strcmp(szAction, "view"))
        {
            On_manager_view(0);
            return;
        }
        if (!Env::Strcmp(szAction, "view_params"))
        {
            ContractID arg_cid;
            Env::DocGet("cid", arg_cid);
            On_manager_view_params(arg_cid, 0);
            return;
        }
        if (!Env::Strcmp(szAction, "view_funds"))
        {
            ContractID arg_cid;
            Env::DocGet("cid", arg_cid);
            On_manager_view_funds(arg_cid, 0);
            return;
        }
        if (!Env::Strcmp(szAction, "view_accounts"))
        {
            ContractID arg_cid;
            Env::DocGet("cid", arg_cid);
            On_manager_view_accounts(arg_cid, 0);
            return;
        }
        if (!Env::Strcmp(szAction, "view_account"))
        {
            ContractID arg_cid;
            Env::DocGet("cid", arg_cid);
            PubKey arg_pubKey;
            Env::DocGet("pubKey", arg_pubKey);
            On_manager_view_account(arg_cid, arg_pubKey, 0);
            return;
        }
        return OnError("invalid Action");
    }
    if (!Env::Strcmp(szRole, "my_account"))
    {
        if (!Env::Strcmp(szAction, "view"))
        {
            ContractID arg_cid;
            Env::DocGet("cid", arg_cid);
            On_my_account_view(arg_cid, 0);
            return;
        }
        if (!Env::Strcmp(szAction, "deposit"))
        {
            ContractID arg_cid;
            Env::DocGet("cid", arg_cid);
            Amount arg_amount;
            Env::DocGet("amount", arg_amount);
            AssetID arg_aid;
            Env::DocGet("aid", arg_aid);
            On_my_account_deposit(arg_cid, arg_amount, arg_aid, 0);
            return;
        }
        if (!Env::Strcmp(szAction, "withdraw"))
        {
            ContractID arg_cid;
            Env::DocGet("cid", arg_cid);
            Amount arg_amount;
            Env::DocGet("amount", arg_amount);
            AssetID arg_aid;
            Env::DocGet("aid", arg_aid);
            On_my_account_withdraw(arg_cid, arg_amount, arg_aid, 0);
            return;
        }
        return OnError("invalid Action");
    }

    OnError("unknown Role");
}
*/

use core::ffi::CStr;

use alloc::borrow::ToOwned;
//use alloc::{borrow::ToOwned, format};
use beam_bvm_util::{app::{simple::doc_get_text, safe::doc_add_text}, common::{extensions::*, safe::halt}};

use crate::{
    methods::main_method_events::*,
    util::{ShouldRunParams, doc_writer::*},
};

#[export_name = "Method_1"]
pub fn main_method() {

    let should_run_params = ShouldRunParams::new();

    DOC_WRITER.object(|root| {

        if should_run_params.is_error() {
            doc_add_text(to_c_str("error\0"), to_c_str("role and action are required.\0"));
            halt();
            return;
        }

        if should_run_params.is_role(to_c_str("manager\0")) {
            if should_run_params.is_role(to_c_str("create\0")) {
                manager_create::run();
            }
            if should_run_params.is_role(to_c_str("destroy\0")) {
                manager_destroy::run();
            }
        }
        else if should_run_params.is_role(to_c_str("my_account\0")) {

        }

    });
}
