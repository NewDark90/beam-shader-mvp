use beam_bvm_interface::root::KeyTag_Internal;
use beam_bvm_util::safe::contract::var::save_var;
use common::params::CtorParams;
use core::mem::size_of;

#[export_name = "Ctor"] //or "Method_0"
pub fn ctor(params: &CtorParams) 
{
    save_var::<u8, CtorParams>(
        0 as *const u8,
        size_of::<u8>() as u32,
        params,
        size_of::<CtorParams>() as u32,
        KeyTag_Internal,
    );
}
