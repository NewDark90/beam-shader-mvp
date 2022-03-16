use wasm_bindgen::prelude::*;
use beam_bvm_interface::root::*;

use crate::types::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct DepositParamsWrapper
{
    value: AmountWithAsset
}

pub(crate) fn Deposit(params: DepositParamsWrapper)
{
    unsafe 
    {
        Env::FundsLock(params.value.assetId, params.value.amount);
    }
}

#[wasm_bindgen]
pub fn Method_2(params: DepositParamsWrapper) { return Deposit(params); }