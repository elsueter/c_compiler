mod ast;

fn main() {
    let result = ast::process_string("1+2/3".to_string());
    println!("{:?}", result);
}
