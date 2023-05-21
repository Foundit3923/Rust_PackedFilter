pub const LAST_BITS_ON: u64 = 0x101010101010101;
pub const SIGNIFICANT_BITS_ON: u64 = 0x8080808080808080;

#[macro_export]
macro_rules! bitcount{
    ($a:expr)=>{
        {
            ($a + $a/255) & 255
        }
    }
}

#[macro_export]
macro_rules! count{
    ($a:expr, $result:expr)=>{
        {
            $result += bitcount!($a);
            $result
        }
    }
}

#[macro_export]
macro_rules! reduce{
    ($v:expr)=>{
        {
            (((!$v - LAST_BITS_ON) ^ !$v) & SIGNIFICANT_BITS_ON) >> 7
        }
    }
}

#[macro_export]
macro_rules! xnor{
    ($t:expr, $q:expr, $c:expr)=>{
        {
            !($t ^ $q * ($c)) & !($t ^ LAST_BITS_ON * ($c))
        }
    }
}