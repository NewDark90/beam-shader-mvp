use beam_bvm_interface::root::KeyTag_Internal;
use beam_bvm_util::common;
use beam_bvm_util::contract;
use shared::types::{Key};
use shared::types::AccountData;
use shared::params::{CtorParams, WithdrawParams};

#[export_name = "Method_3"]
pub fn withdraw(params: &WithdrawParams) {
    let height = common::safe::get_height();
    let mut ctor_params: CtorParams = Default::default();

    contract::simple::load_var::<u8, CtorParams>(
        &0,
        &mut ctor_params,
        KeyTag_Internal,
    );

    let mut account_data: AccountData = Default::default();

    let loaded: bool = contract::simple::load_var::<Key, AccountData>(
        &params.key,
        &mut account_data,
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
                None => common::safe::halt(),
            }
        }

        if account_data.amount > ctor_params.max_withdraw {
            common::safe::halt();
        }

        contract::simple::save_var::<Key, AccountData>(
            &params.key,
            &account_data,
            KeyTag_Internal,
        );

        contract::safe::funds_unlock(params.key.asset_id, params.amount);
    } else if loaded && empty {
        contract::simple::del_var::<Key>(&params.key);
    }

    contract::safe::add_sig(&params.key.account)
}
