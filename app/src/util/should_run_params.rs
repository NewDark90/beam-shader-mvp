use core::ffi::CStr;

use alloc::{ffi::CString};
use beam_bvm_util::{common::{self, extensions::*, types::SizedResult}, app::simple::doc_get_text};


const ACTION_PROP: &str = "action\0";
const ROLE_PROP: &str  = "role\0";

pub struct ShouldRunParams {
    is_error: bool,
    role: SizedResult<CString>,
    action: SizedResult<CString>
}


impl ShouldRunParams {
    pub fn new() -> Self {
        let role_result = doc_get_text(&to_c_string(ROLE_PROP));
        let action_result = doc_get_text(&to_c_string(ACTION_PROP));

        let is_error = action_result.is_err() || role_result.is_err(); 

        Self { 
            role: role_result.unwrap_or_default(), 
            action: action_result.unwrap_or_default(),
            is_error
        }
    }

    pub fn role(&self) -> &CStr { &self.role.value() }
    pub fn action(&self) -> &CStr { &self.action.value() }
    pub fn is_error(&self) -> bool { self.is_error }

    pub fn is_role(&self, other: &CStr) -> bool { 
        common::simple::strcmp(&self.role.value(), other) == 0 
    }

    pub fn is_action(&self, other: &CStr) -> bool {
        common::simple::strcmp(&self.action.value(), other) == 0  
    }
}