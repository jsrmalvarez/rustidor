use std::os::raw::c_char;
use std::ffi::CStr;

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn rstd_getweapon(ptr: *const c_char) -> i32 {
  let c_str = CStr::from_ptr(ptr);
  let rust_str = c_str.to_str().expect("Bad encoding");
  match get_weapon(rust_str) {
    Some(weapon_index) => i32::try_from(weapon_index).unwrap_or(-1),
    _ => -1
  }
}

fn get_weapon(name: &str) -> Option<usize>{
  if let Ok(res) = name.parse() {
    Some(res)
  }
  else {
    const ABBREVS:[&str;7] = ["FI", "SG", "CG", "RL", "RI", "GL", "PI"];
    ABBREVS.iter().position(|&x| { x == name.to_uppercase() })
  }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn numeric_input() {
        assert_eq!(get_weapon("2"), Some(2));
        assert_eq!(get_weapon("42"), Some(42));
    }

    #[test]
    fn name_input() {
        assert_eq!(get_weapon("FI"), Some(0));
        assert_eq!(get_weapon("sG"), Some(1));
        assert_eq!(get_weapon("cg"), Some(2));
        assert_eq!(get_weapon("RL"), Some(3));
        assert_eq!(get_weapon("RI"), Some(4));
        assert_eq!(get_weapon("Gl"), Some(5));
        assert_eq!(get_weapon("pi"), Some(6));
    }
    #[test]
    fn erroneous_input() {
      assert_eq!(get_weapon(""), None);
      assert_eq!(get_weapon("a"), None);
      assert_eq!(get_weapon("-8"), None);
      assert_eq!(get_weapon("KK"), None);
    }
}

#[cfg(test)]
mod c_integration_tests {

    use super::*;
    use std::ffi::CString;

    #[test]
    fn number_input(){
      let mut c_string:CString;
      c_string = CString::new("0").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, 0);
      c_string = CString::new("42").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, 42);
    }

    #[test]
    fn name_input(){
      let mut c_string:CString;
      c_string = CString::new("FI").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, 0);
      c_string = CString::new("sG").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, 1);
      c_string = CString::new("cg").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, 2);
      c_string = CString::new("RL").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, 3);
      c_string = CString::new("RI").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, 4);
      c_string = CString::new("Gl").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, 5);
      c_string = CString::new("pi").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, 6);
    }

    #[test]
    fn erroneous_input() {
      let mut c_string:CString;
      c_string = CString::new("").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, -1);
      c_string = CString::new("a").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, -1);
      c_string = CString::new("-8").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, -1);
      c_string = CString::new("KK").unwrap();
      assert_eq!(unsafe{ rstd_getweapon(c_string.as_ptr()) }, -1);
    }

}
