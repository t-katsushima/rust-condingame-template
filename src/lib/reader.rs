use super::common::*;

// Reader
#[macro_export]
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

// sample
pub fn readUsize() -> usize {
    let input_line = get_input_line();
    parse_input!(input_line, usize)
}
