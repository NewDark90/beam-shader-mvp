use wasm_bindgen::prelude::*;
use beam_bvm_interface::root::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct CtorParams
{
    pub limit: Epoch,
    pub pk_admin: PubKey
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct AmountWithAsset 
{
    pub amount: Amount,
    pub assetId: AssetID
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Epoch
{
    pub height: Height,
    pub amount: Amount
}