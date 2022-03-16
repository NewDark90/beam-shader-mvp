use beam_bvm_interface::root::*;

use crate::types::*;

pub struct State
{
    pub epoch: Epoch,
    pub params: CtorParams,
    pub enabled: u8
}

impl State 
{
    pub fn get_key() -> u8 { return 0; }

    pub fn update_epoch(&self, height: Height) -> bool 
    {
        if (height - self.epoch.height < self.params.limit.height)
        { 
            return false; 
        }

        self.epoch.height  = height;
        self.epoch.amount = self.params.limit.amount;

        return true;
    }

    pub fn load(self)
    { 
        Env::LoadVar_T(self.get_key(), self); 
    }

    pub fn save(self)
    { 
        Env::SaveVar_T(self.get_key(), self); 
    }
}

/*



use wasm_bindgen::prelude::*;
use bvm::root::*;
use bvm::root::Env::*;
use std::mem::size_of;
use std::os::raw::c_void;
    
#[wasm_bindgen]
pub struct Params {
    s_iMethod: u32, //0

    m_BacklogPeriod: Height,
    m_MaxWithdraw: Amount
}

#[wasm_bindgen]
pub fn Ctor(r: Params)
{
    let mut parameters = r;
    let param_ptr: *mut c_void = &mut parameters as *mut _ as *mut c_void;
    unsafe
    {
        SaveVar(
            0 as *const c_void, 
            size_of::<u8>() as u32, 
            param_ptr, 
            size_of::<Params>() as u32, 
            KeyTag_Internal
        );
    }
}

*/