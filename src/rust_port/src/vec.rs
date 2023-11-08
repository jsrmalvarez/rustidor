extern crate cty;
use cty::{c_int, c_void};

#[repr(C)]
pub struct CVec {    
    pub x : cty::c_int,
    pub y : cty::c_int,
    pub z : cty::c_int,
}

#[no_mangle]
pub extern "C" fn rstd_iszero(vec: *const cty::c_void) -> bool {
    let vec = unsafe { & *(vec as *const CVec) };
    vec.x == 0 && vec.y == 0 && vec.z == 0   
}
