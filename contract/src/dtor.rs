use beam_bvm_util::safe::contract::var::del_var;
use common::params::{DtorParams};
use core::mem::size_of;

#[export_name = "Dtor"] //or "Method_1"
pub fn dtor(_params: *const DtorParams) {
    del_var::<u8>(0 as *const u8, size_of::<u8>() as u32);
}
