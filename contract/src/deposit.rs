
use beam_bvm_util::contract::safe::funds_lock;
use shared::params::{DepositParams};

#[export_name = "Method_2"]
pub fn deposit(params: &DepositParams) 
{
    funds_lock(params.asset_id, params.amount);
}
