use crate::packed_contains::PackedContains;

use libc::c_char;
use std::ffi::CString;
use std::str;
 
extern "C" {
  fn search_test(
    query_array: *const c_char,
    query_len: usize,
    text: *const c_char,
    text_len: usize,
  ) -> u64;
}

#[derive(Default)]
pub struct PackedC {}

impl PackedC {
    pub fn new() -> Self{
        PackedC {  }
    }
}

impl PackedContains for PackedC {
  fn packed_contains(self: &PackedC, text: &str, query: &str) -> bool {
    let c_text = CString::new(text).unwrap();
    let c_query = CString::new(query).unwrap();
    let result = unsafe{ search_test(
      c_query.as_ptr(), query.len(),
      c_text.as_ptr(), text.len()
    )};
    result != 0
  }
}
