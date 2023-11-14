fn main(){
    let string = return_hello_string();
    println!("{}", "Other value");
}

pub fn return_hello_string() -> &'static str {
    return "Hello world!";
}