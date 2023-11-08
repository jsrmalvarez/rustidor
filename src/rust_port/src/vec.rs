extern crate cty;
use cty::{c_int, c_float, c_void};

#[repr(C)]
pub struct CVec<T> {
    pub x : T,
    pub y : T,
    pub z : T,
}

#[no_mangle]
pub extern "C" fn rstd_iszero(vec: *const c_void) -> bool {
    let vec = unsafe { & *(vec as *const CVec<c_int>) };
    vec.x == 0 && vec.y == 0 && vec.z == 0   
}

#[no_mangle]
pub extern "C" fn rstd_mul(vec: *mut c_void, f:c_float) -> *mut c_void {
    let vec = unsafe { &mut *(vec as *mut CVec<c_float>) };
    vec.x *= f; vec.y *= f; vec.z *= f;    
    vec as *mut CVec<c_float> as *mut c_void
}
