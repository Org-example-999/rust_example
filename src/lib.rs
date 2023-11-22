fn main(){
    let string = return_hello_string();
    println!("{}", string);
}

pub fn return_hello_string() -> &'static str {
    return "Hello world!";
}