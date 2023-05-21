use crate::packed_contains::PackedContains;
use crate::create_tests;

impl PackedContains for &str {
  fn packed_contains(
    self,
    query: &str
  ) -> bool {
    self.contains(query)
  }
}

create_tests!();