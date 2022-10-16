use core::{ptr::null};

use beam_bvm_interface::root::*;
use beam_bvm_util::{common, app};
use shared::params::{CtorParams, ContractParams};
use const_format::formatcp;

use crate::util::{doc_writer::*, ShouldRunParams};

const BACKLOG_PERIOD_PROP: &str = "backlogPeriod\0";
const WITHDRAW_LIMIT_PROP: &str = "withdrawLimit\0";

pub fn should_run(params: ShouldRunParams) -> bool {
    if  common::simple::strcmp(params.role(), "manager\0") == 0 &&
        common::simple::strcmp(params.action(), "create\0") == 0
    { true } 
    else 
    { false }
}

pub fn run() -> () {
    let mut backlog_period: Height = 0;
    let mut withdraw_limit: Amount = 0;

    let backlog_period_result = app::simple::doc_get_blob::<Height>(BACKLOG_PERIOD_PROP, &mut backlog_period);
    let withdraw_limit_result = app::simple::doc_get_blob::<Amount>(WITHDRAW_LIMIT_PROP, &mut withdraw_limit);

    if backlog_period_result.is_err() || withdraw_limit_result.is_err()
    {
        DOC_WRITER.string_prop("error\0", formatcp!("{0} and {1} are required.\0", "backlogPeriod", "withdrawLimit"));
        return;
    }

    let ctor = CtorParams {
        backlog_period: backlog_period,
        max_withdraw: withdraw_limit
    };
    unsafe {
        app::simple::generate_kernel::<CtorParams>(
            &ContractID::default(), 
            ctor.method_id(), 
            &ctor, 
            None,  
            None, 
            Some("create Faucet contract\0"), 
            0
        );
    }
}