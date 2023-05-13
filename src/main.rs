const LAST_BITS_ON: u64 = 0x101010101010101;
const SIGNIFICANT_BITS_ON: u64 = 0x8080808080808080;

macro_rules! bitcount{
    ($a:expr)=>{
        {
            ($a + $a/255) & 255
        }
    }
}

macro_rules! count{
    ($a:expr, $result:expr)=>{
        {
            $result += bitcount!($a);
            $result
        }
    }
}

macro_rules! reduce{
    ($v:expr)=>{
        {
            (((!$v - LAST_BITS_ON) ^ !$v) & SIGNIFICANT_BITS_ON) >> 7
        }
    }
}

macro_rules! xnor{
    ($t:expr, $q:expr, $c:expr)=>{
        {
            !($t ^ $q * ($c)) & !($t ^ LAST_BITS_ON * ($c))
        }
    }
}

fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod tests {
    use super::*;

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
