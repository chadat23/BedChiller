#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    input * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_input(2);
        assert_eq!(result, 4);
    }
}
