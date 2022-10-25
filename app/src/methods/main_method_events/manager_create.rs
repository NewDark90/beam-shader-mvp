use core::{ffi::CStr};

use beam_bvm_interface::root::*;
use beam_bvm_util::{common::{extensions::*}, app::{self, safe::doc_add_text}};
use shared::params::{CtorParams, ContractParams};

use crate::util::{doc_writer::*};


const BACKLOG_PERIOD_PROP: &CStr = to_c_str("backlogPeriod\0");
const WITHDRAW_LIMIT_PROP: &CStr = to_c_str("withdrawLimit\0");

pub fn write_props(obj_wrap: ObjectFuncs) -> () {
    obj_wrap
        .string_prop(&BACKLOG_PERIOD_PROP, to_c_str("Height\0"))
        .string_prop(&WITHDRAW_LIMIT_PROP, to_c_str("Amount\0"));
}

pub fn run() -> () {
    let mut backlog_period: Height = 0;
    let mut withdraw_limit: Amount = 0;

    let backlog_period_result = app::simple::doc_get_blob::<Height>(&BACKLOG_PERIOD_PROP, &mut backlog_period);
    let withdraw_limit_result = app::simple::doc_get_blob::<Amount>(&WITHDRAW_LIMIT_PROP, &mut withdraw_limit);

    if backlog_period_result.is_err() || withdraw_limit_result.is_err()
    {
        doc_add_text("error\0".to_c_str(), "backlogPeriod and withdrawLimit are required.\0".to_c_str());
        return;
    }

    let ctor = CtorParams {
        backlog_period: backlog_period,
        max_withdraw: withdraw_limit
    };
    
    app::simple::generate_kernel::<CtorParams>(
        &ContractID::default(), 
        ctor.as_method_id(), 
        &ctor, 
        None,  
        None, 
        "create Faucet contract\0".to_c_str(), 
        0
    );
}