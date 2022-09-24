#![cfg_attr(not(feature = "std"), no_std)]
#![no_main]

/* 
All methods exposed to the BVM have an exact naming scheme.
Method_n, 0-7.
"Method_0" is the constructor function, and can be named "Ctor"
"Method_1" is the destructor function, and can be named "Dtor"

Either use an attribute to force the function name:
#[export_name="Method_n"]
pub fun my_func() { }

Or you can tell the compiler not to mangle the name with these attributes
#[no_mangle]
#[allow(non_snake_case)]
pub fn Method_0() { }
*/

extern crate common;

pub mod util;
pub mod do_thing;
pub mod do_another;