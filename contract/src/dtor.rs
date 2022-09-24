use crate::util::*;
use common::params::DtorParams;

#[export_name="Dtor"] //or "Method_1"
pub fn dtor(_params: *const DtorParams) {
    shared()
}
