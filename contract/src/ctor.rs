use beam_bvm_interface::root::KeyTag_Internal;
use beam_bvm_util::contract::simple::var::save_var;
use shared::params::CtorParams;

#[export_name = "Ctor"] //or "Method_0"
pub fn ctor(params: &CtorParams) 
{
    save_var::<u8, CtorParams>(
        &0,
        params,
        KeyTag_Internal,
    );
}
