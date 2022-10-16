use beam_bvm_util::contract::simple::var::del_var;
use shared::params::{DtorParams};

#[export_name = "Dtor"] //or "Method_1"
pub fn dtor(_params: *const DtorParams) {
    del_var::<u8>(&0);
}
