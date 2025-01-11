pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

pub fn subtract(left: u64, right: u64) -> u64 {
    left - right
}

pub fn divide(left: u64, right: u64) -> u64 {
    if right == 0 {
        panic!("Division by zero!");
    }
    left / right
}
