use rust_example;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn return_hello_srting_test(){
        assert_eq!("Hello world!", rust_example::return_hello_string())
    }
}