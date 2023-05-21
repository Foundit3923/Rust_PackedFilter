pub trait PackedContains {
  fn packed_contains(&self, text: &str, query: &str) -> bool;
}
