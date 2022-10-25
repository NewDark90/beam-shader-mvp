use core::{mem::size_of_val, ffi::CStr};

use beam_bvm_interface::root::{*, Env::KeyPrefix};
use beam_bvm_util::{
    app::{safe, simple, util::var_reader::VarReader},
    common::extensions::*,
};
use shared::{params::*, types::*};

use crate::util::{doc_writer::*, dump_accounts};

const CID_PROP: &CStr = to_c_str("cid\0");
const PUB_KEY_PROP: &CStr = to_c_str("pubKey\0");

pub fn write_props(obj_wrap: ObjectFuncs) -> () {
    obj_wrap
        .string_prop(CID_PROP, "ContractID\0".to_c_str())
        .string_prop(PUB_KEY_PROP, "PubKey\0".to_c_str());
}

pub fn run() -> () {

    let mut arg_cid: ContractID = Default::default();
    let mut arg_pub_key: PubKey = Default::default();

    simple::doc_get_blob(CID_PROP, &mut arg_cid);
    simple::doc_get_blob(PUB_KEY_PROP, &mut arg_pub_key);

    let key_account_0: KeyAccount = KeyAccount {
        m_Prefix: KeyPrefix {
            m_Cid: arg_cid,
            m_Tag: KeyTag_Internal
        },
        m_KeyInContract: Key {
            asset_id: 0,
            account: arg_pub_key
        },
        _phantom_0: Default::default()
    };
    let key_account_1: KeyAccount = KeyAccount {
        m_Prefix: KeyPrefix {
            m_Cid: arg_cid,
            m_Tag: KeyTag_Internal
        },
        m_KeyInContract: Key {
            asset_id: u32::MAX,
            account: arg_pub_key
        },
        _phantom_0: Default::default()
    };

    let reader = VarReader::new(&key_account_0, &key_account_1);
    dump_accounts(&reader);

}
