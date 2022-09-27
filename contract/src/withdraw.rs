use beam_bvm_interface::root::KeyTag_Internal;
use beam_bvm_util::safe::{
    common::{
        get::get_height,
        util::{halt, halt_if},
    },
    contract::{
        assets::{add_sig, funds_unlock},
        var::{del_var, load_var, save_var},
    },
};
use common::{
    params::{CtorParams, WithdrawParams},
    types::{AccountData, Key},
};
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
        KeyTag_Internal,
    ) > 0;
    let empty: bool = !loaded || (height - account_data.height >= ctor_params.backlog_period);

    if params.amount > 0 {
        if empty {
            account_data.height = height;
            account_data.amount = params.amount;
        } else {
            match account_data.amount.checked_add(params.amount) {
                Some(val) => account_data.amount = val,
                None => halt(),
            }
        }

        halt_if(account_data.amount > ctor_params.max_withdraw);
        save_var::<Key, AccountData>(
            &params.key,
            size_of::<Key>() as u32,
            &account_data,
            size_of::<AccountData>() as u32,
            KeyTag_Internal,
        );

        funds_unlock(params.key.asset_id, params.amount);
    } else if loaded && empty {
        del_var::<Key>(&params.key, size_of::<Key>() as u32);
    }

    add_sig(&params.key.account)
}
