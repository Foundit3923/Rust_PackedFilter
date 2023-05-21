use crate::packed_contains::{PackedContains};

#[derive(Default)]
pub struct Naive {}

impl Naive {
  pub fn new() -> Self{
      Naive {  }
  }
}

impl PackedContains for Naive {
  fn packed_contains(self: &Naive, text: &str, query: &str) -> bool {
    if query == "" {
      return false;
    }
    else{
    text.contains(query)
    }
  }
}
