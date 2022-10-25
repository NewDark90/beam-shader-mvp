use core::{ffi::CStr};

use beam_bvm_interface::root::*;
use beam_bvm_util::{common::{extensions::*}, app::{simple::{doc_get_blob, generate_kernel}}};
use shared::params::*;

use crate::util::{doc_writer::*};


const CID_PROP: &CStr= to_c_str("cid\0");

pub fn write_props(obj_wrap: ObjectFuncs) -> () {
    obj_wrap.string_prop(&CID_PROP, to_c_str("ContractID\0"));
}

pub fn run() -> () {
    let mut arg_cid: ContractID = ContractID::default();
    doc_get_blob::<ContractID>(&CID_PROP, &mut arg_cid);

    let dtor_params = DtorParams{};
    generate_kernel::<DtorParams>(
        &arg_cid, 
        dtor_params.as_method_id(),
        &dtor_params,
        None,
        None, 
        &"destroy Faucet contract".to_c_string(),
        0
    );
}
