use beam_bvm_util::safe::contract::assets::funds_lock;
use common::params::{DepositParams};

#[export_name = "Method_2"]
pub fn deposit(params: &DepositParams) 
{
    funds_lock(params.asset_id, params.amount);
}
