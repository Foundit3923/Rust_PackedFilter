pub trait PackedContains {
    fn packed_contains(
      self,
      query: &str
    ) -> bool;
  }
  
  #[macro_export]
  macro_rules! create_tests {
    () => {
      #[cfg(test)]
      mod tests {
        use super::*;
  
        #[test]
        fn test_match() {
          // const text:&'static str = "abc";
          // const query:&'static str = "bc";
          assert!("abc".packed_contains("bc"));
        }
      }
    };
  }