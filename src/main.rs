mod ast;

fn main() {
    let result = ast::to_postfix("123+2/3".to_string().into_bytes());
    for t in result {
        println!("{:?}", t.val);
    }
}
