pub fn hello_from_crate_c() -> String {
    "Hello from crate-c!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_from_crate_c() {
        let result = hello_from_crate_c();
        assert_eq!(result, "Hello from crate-c!");
    }
}
