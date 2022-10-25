use beam_bvm_interface::root::*;
use beam_bvm_util::{
    app::{safe, simple, util::enum_and_dump_contracts::*},
    common::extensions::*,
};
use shared::{params::*, contract_sid::SID};

use crate::util::doc_writer::*;

pub fn write_props(obj_wrap: ObjectFuncs) -> () {
    //Intentionally empty
}

pub fn run() -> () {
    enum_and_dump_contracts(&SID)
}