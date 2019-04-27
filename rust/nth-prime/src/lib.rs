pub fn is_prime(n: &u32) -> bool {
    match n <= &2 {
        true => false,
        false => !(2..*n).any(|factor| n % factor == 0),
    }
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(is_prime).take(n as usize).last().unwrap()
}
