mod lexer;
mod parser;

use parser::Parser as SyntaxParser;
use parser::enums::{Statement, Expression};
use clap::Parser;
use lexer::{Lexer, Token};
use std::fs;

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    author = "Shell1010 <amin.dev03@gmail.com>",
    about = "Lumina Compiler"
)]
struct Args {
    #[arg(default_value = "main.lum")]
    input: String,

    #[arg(short, long, default_value = "output.out")]
    output: String,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("Lumina Compiler v0.1.0");
        println!("Input file: {}", args.input);
        println!("Output file: {}", args.output);
    }

    let source_code = match fs::read_to_string(&args.input) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read file {}: {}", args.input, err);
            std::process::exit(1);
        }
    };

    if args.verbose {
        println!("Successfully read the input file.");
    }

    if args.verbose {
        println!("Starting lexical analysis...");
    }
    let tokens = lex(&source_code);
    if args.verbose {
        println!("Lexical analysis completed. Tokens: {:?}", tokens);
    }

    if args.verbose {
        println!("Starting syntax analysis...");
    }
    let ast = parse(tokens);
    if args.verbose {
        println!("Syntax analysis completed. AST: {:?}", ast);
    }

    if args.verbose {
        println!("Generating code...");
    }
    let machine_code = generate_code(ast);
    if args.verbose {
        println!("Code generation completed.");
    }

    if args.verbose {
        println!("Writing to output file: {}", args.output);
    }
    match fs::write(&args.output, machine_code) {
        Ok(_) => {
            if args.verbose {
                println!("Compilation successful. Output written to {}", args.output);
            }
        }
        Err(err) => {
            eprintln!("Failed to write output file {}: {}", args.output, err);
            std::process::exit(1);
        }
    }
}

fn lex(source: &str) -> Vec<Token> {
    println!("Lexing source code...");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    println!("{:?}", tokens);
    tokens.to_vec()
}

fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    println!("Parsing tokens...");
    let mut parser = SyntaxParser::new(tokens);
    let ast = parser.parse().unwrap();
    println!("{ast:#?}");
    ast

}

fn generate_code(ast: Vec<Statement>) -> Vec<u8> {
    println!("Generating code from AST...");
    todo!("Implement code generator");
}
