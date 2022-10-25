use core::ffi::CStr;

use beam_bvm_interface::root::{*, Env::KeyPrefix};
use beam_bvm_util::{
    app::{safe, simple, util::var_reader::VarReader},
    common::extensions::*,
};
use shared::{params::*, types::{Key, KeyAccount}};

use crate::util::{doc_writer::*, dump_accounts};

const CONTRACT_ID_PROP: &CStr = to_c_str("cid\0");

pub fn write_props(obj_wrap: ObjectFuncs) -> () {
    obj_wrap.string_prop(CONTRACT_ID_PROP, "ContractID\0".to_c_str());
}

pub fn run() -> () {

    let mut arg_contract_id: ContractID = Default::default();
    simple::doc_get_blob(CONTRACT_ID_PROP, &mut arg_contract_id);

    let mut my_pk: PubKey = Default::default();
    simple::derive_pk(&mut my_pk, &arg_contract_id);


    let key_account_0 = KeyAccount {
        m_Prefix: KeyPrefix {
            m_Cid: arg_contract_id,
            m_Tag: KeyTag_Internal
        },
        m_KeyInContract: Key {
            asset_id: 0,
            account: my_pk
        },
        _phantom_0: Default::default()
    };
    let key_account_1 = KeyAccount {
        m_Prefix: KeyPrefix {
            m_Cid: arg_contract_id,
            m_Tag: KeyTag_Internal
        },
        m_KeyInContract: Key {
            asset_id: u32::MAX,
            account: my_pk
        },
        _phantom_0: Default::default()
    };

    let reader = VarReader::new(&key_account_0, &key_account_1);
    dump_accounts(&reader);
    
}