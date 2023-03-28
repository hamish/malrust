fn READ(input:String) -> String{
    input
}
fn EVAL(input: String) -> String{
    input
}
fn PRINT(input: String) -> String{
    input
}

fn main() {
    println!("{}", READ(EVAL(PRINT("Hello, world!".to_string()))));
}
