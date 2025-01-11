use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_add(left: u64, right: u64) -> u64 {
    arithmetic::add(left, right)
}

#[wasm_bindgen]
pub fn calculate_multiply(left: u64, right: u64) -> u64 {
    arithmetic::multiply(left, right)
}

#[wasm_bindgen]
pub fn calculate_subtract(left: u64, right: u64) -> u64 {
    arithmetic::subtract(left, right)
}

#[wasm_bindgen]
pub fn calculate_divide(left: u64, right: u64) -> u64 {
    arithmetic::divide(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = calculate_add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_multiply() {
        let result = calculate_multiply(3, 4);
        assert_eq!(result, 12);
    }
}
