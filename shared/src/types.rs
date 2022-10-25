
use beam_bvm_interface::root::*;


#[derive(Default)]
pub struct AccountData {
    pub height: Height,
    pub amount: Amount,
}

pub struct Key {
    pub account: PubKey,
    pub asset_id: AssetID,
}


pub type KeyAccount = Env::Key_T<Key> ;