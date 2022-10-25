
use core::ffi::CStr;

use beam_bvm_interface::root::*;
use beam_bvm_util::{
    app::{safe, simple, util::funds_walker::{FundsWalker, ValueFunds}},
    common::extensions::*,
};
use shared::params::*;

use crate::util::doc_writer::*;

const CID_PROP: &CStr = to_c_str("cid\0");

pub fn write_props(obj_wrap: ObjectFuncs) -> () {
    obj_wrap.string_prop(CID_PROP, "ContractID\0".to_c_str());
}

pub fn run() -> () {

    let mut arg_cid: ContractID = Default::default();;
    simple::doc_get_blob(CID_PROP, &mut arg_cid);

    let mut funds_walker = FundsWalker::new();

    funds_walker.r#enum(&arg_cid, None);

    DOC_WRITER.array_prop("funds\0".to_c_str(), |funds_arr| {
        funds_walker.move_all(|asset: u32, funds: &ValueFunds| {
            funds_arr.object(|funds_obj|{
                funds_obj
                    .u32_prop("Aid\0".to_c_str(), asset)
                    .u64_prop("Amount\0".to_c_str(), funds.low);
            });
        });
    });

}