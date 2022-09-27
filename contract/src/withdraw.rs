/*
BEAM_EXPORT void Method_3(const Faucet::Withdraw& r)
{
    Height h = Env::get_Height();

    Faucet::Params pars;
    Env::LoadVar_T((uint8_t) 0, pars);

    Faucet::AccountData ad;
    bool bLoaded = Env::LoadVar_T(r.m_Key, ad);
    bool bEmpty = !bLoaded || (h - ad.m_h0 >= pars.m_BacklogPeriod);

    if (r.m_Amount)
    {
        if (bEmpty)
        {
            ad.m_h0 = h;
            ad.m_Amount = r.m_Amount;
        }
        else
            Strict::Add(ad.m_Amount, r.m_Amount);

        Env::Halt_if(ad.m_Amount > pars.m_MaxWithdraw);
        Env::SaveVar_T(r.m_Key, ad);

        Env::FundsUnlock(r.m_Key.m_Aid, r.m_Amount); // would fail if not enough funds in the contract
    }
    else
    {
        if (bLoaded && bEmpty)
            Env::DelVar_T(r.m_Key);
    }

    Env::AddSig(r.m_Key.m_Account);
}*/

use beam_bvm_interface::root::KeyTag_Internal;
use beam_bvm_util::safe::{common::get::get_height, contract::var::load_var};
use common::{params::{CtorParams, WithdrawParams}, types::{AccountData, Key}};
use core::mem::size_of;

#[export_name = "Method_3"]
pub fn withdraw(params: &WithdrawParams) {
    let height = get_height();
    let mut ctor_params: CtorParams = Default::default();

    load_var::<u8, CtorParams>(
        0 as *const u8,
        size_of::<u8>() as u32,
        &mut ctor_params,
        size_of::<CtorParams>() as u32,
        KeyTag_Internal,
    );

    let mut account_data: AccountData = Default::default();
    
    let loaded: bool = load_var::<Key, AccountData>(
        &params.key,
        size_of::<Key>() as u32,
        &mut account_data,
        size_of::<AccountData>() as u32,
        KeyTag_Internal
    ) > 0;
    let empty: bool = !loaded || (height - account_data.height >= ctor_params.backlog_period);

    if (params.amount > 0)
    {
        if (empty)
        {
            account_data.height = height;
            account_data.amount = params.amount;
        }
        else
        {
            //Strict::Add(ad.m_Amount, r.m_Amount);
        }
            Strict::Add(ad.m_Amount, r.m_Amount);

        Env::Halt_if(ad.m_Amount > pars.m_MaxWithdraw);
        Env::SaveVar_T(r.m_Key, ad);

        Env::FundsUnlock(r.m_Key.m_Aid, r.m_Amount); // would fail if not enough funds in the contract
    }
    else
    {
        if (bLoaded && bEmpty)
            Env::DelVar_T(r.m_Key);
    }

    Env::AddSig(r.m_Key.m_Account);

}
