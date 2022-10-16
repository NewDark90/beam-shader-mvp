use beam_bvm_interface::root::*;
use crate::types::*;


pub trait ContractParams {
    fn method_id(&self) -> u32;
}

#[derive(Default)]
pub struct CtorParams {
    pub backlog_period: Height,
    pub max_withdraw: Amount,
}

impl ContractParams for CtorParams {
    fn method_id(&self) -> u32 { 0 }
}


pub struct DtorParams {
    
}

impl ContractParams for DtorParams {
    fn method_id(&self) -> u32 { 1 }
}

pub struct DepositParams {
    pub asset_id: AssetID,
    pub amount: Amount,
}

impl ContractParams for DepositParams {
    fn method_id(&self) -> u32 { 2 }
}

pub struct WithdrawParams {
    pub key: Key,
    pub amount: Amount,
}

impl ContractParams for WithdrawParams {
    fn method_id(&self) -> u32 { 3 }
}