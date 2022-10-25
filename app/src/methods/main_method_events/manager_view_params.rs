use core::ffi::CStr;

use beam_bvm_interface::root::*;
use beam_bvm_util::{
    app::{safe, simple, util::var_reader::VarReader},
    common::extensions::*,
};
use shared::params::*;

use crate::util::doc_writer::*;

const CID_PROP: &CStr = to_c_str("cid\0");

pub fn write_props(obj_wrap: ObjectFuncs) -> () {
    obj_wrap.string_prop(CID_PROP, "ContractID\0".to_c_str());
}

pub fn run() -> () {

    let mut arg_cid: ContractID = Default::default();
    simple::doc_get_blob(CID_PROP, &mut arg_cid);

    let mut key: Env::Key_T<u8> = Default::default();
    key.m_Prefix.m_Cid = arg_cid;
    key.m_KeyInContract = 0;

    let mut params: CtorParams = Default::default();

    if VarReader::read_single(&key, &mut params) {
        DOC_WRITER.string_prop("error\0".to_c_str(), "failed to read.\0".to_c_str());
        return;
    }

    DOC_WRITER.object_prop("params\0".to_c_str(), |params_obj| {
        params_obj
            .u64_prop("backlogPeriod\0".to_c_str(), params.backlog_period)
            .u64_prop("withdrawLimit\0".to_c_str(), params.max_withdraw);
    });

}