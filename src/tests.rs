#[macro_export]
macro_rules! packed_contains_tests {
  ($($name:ident: $type:ty,)*) => {
  $(
    mod $name {
      use super::*;

      #[test]
      fn test_match() {
          let dummy = <$type>::new();
          assert!(dummy.packed_contains("abc", "bc"));
      }
      #[test]
      fn test_empty_query() {
        let dummy = <$type>::new();
        assert!(dummy.packed_contains("abc", "")==false);
      }
    }
  )*
  }
}

#[cfg(test)]
mod tests {
  use crate::implementation_naive::{Naive};
  use crate::implementation_packed_c::{PackedC};
  use crate::packed_contains::{PackedContains};
  use crate::{bitcount};
  use crate::{count};
  use crate::{reduce};
  use crate::{xnor};
  use crate::bit_hacks::{LAST_BITS_ON};
  use crate::bit_hacks::{SIGNIFICANT_BITS_ON};
  packed_contains_tests! {
    naive: Naive,
    packed_c: PackedC,
  }

  #[test]
  fn test_bitcount() {
      assert_eq!(2, bitcount!(0b100000001));
  }

  #[test]
  fn test_count() {
      let mut results: u64 = 3;
      assert_eq!(5, count!(0b100000001u64, results));
  }

  #[test]
  fn test_reduce() {
      assert_eq!(0b00000001, reduce!(0b11111111u64));
  }

  #[test]
  fn test_xnor() {
      assert_eq!(0xFFFFFFFFFFFFFFFF, xnor!(0x6161616161616161, LAST_BITS_ON, 0x61));
  }

  #[test]
  fn test_reduce_xnor() {
      assert_eq!(0b00000001, reduce!(xnor!(0b01100001u64, 0b00000001u64, 0b01100001u64)));
  }
}


