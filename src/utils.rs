/// utils.rs implements some commonly used utility functions.

pub fn isalnum(c: &char) -> bool {
    c.is_alphabetic()
}

pub fn is_digit(c: &char) -> bool {
    match c.to_digit(10) {
        Option::Some(val) => val < 10,
        Option::None => false,
    }
}
