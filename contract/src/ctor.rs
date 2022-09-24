use crate::util::*;
use common::params::CtorParams;

#[export_name="Ctor"] //or "Method_0"
pub fn ctor(_params: *const CtorParams) {
    shared()
}