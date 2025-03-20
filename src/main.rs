mod ast;

use std::time::Instant;

fn main() {
    let now = Instant::now();

    ast::run_lexer(true);
    let elapsed_time = now.elapsed();
    println!("Total Elapsed: {:.2?}", elapsed_time);
}
