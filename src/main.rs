mod lexer;
use lexer::Lexer;


fn main() {
    let code = r#"
        x = 10
        if x > 5 {
            y = x * 2
            print(y)
        }
        /* This should be a comment */
        x >> 5
        x << 5
    "#;
    
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    println!("{:?}", tokens);
}

