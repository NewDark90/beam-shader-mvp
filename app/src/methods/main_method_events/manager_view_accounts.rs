
use core::ffi::CStr;

use beam_bvm_interface::root::{*};
use beam_bvm_util::{
    app::{safe, simple, util::var_reader::VarReader},
    common::extensions::*,
};
use shared::params::*;

use crate::util::{doc_writer::*, dump_accounts};

const CID_PROP: &CStr = to_c_str("cid\0");

pub fn write_props(obj_wrap: ObjectFuncs) -> () {
    obj_wrap.string_prop(CID_PROP, "ContractID\0".to_c_str());
}

pub fn run() -> () {

    let mut arg_cid: ContractID = Default::default();;
    simple::doc_get_blob(CID_PROP, &mut arg_cid);

    let key_0 = Env::KeyPrefix {
        m_Cid: arg_cid,
        m_Tag: KeyTag_Internal
    };
    let key_1 = Env::KeyPrefix {
        m_Cid: arg_cid,
        m_Tag: KeyTag_Internal + 1
    };

    let reader = VarReader::new(&key_0, &key_1); 
    dump_accounts(&reader);

}


