/// This is the main file for the RASM assembler in Rust. It was originally written in Python and
/// eventually ported to Rust after the codegen API was finished
///
/// Began being written by StandingPad on January 1st 2023 while braindead from 5 hours of sleep
use std::{env, fs::File, io::{self, BufRead}};
use logos::Logos;
use resurgence::{CodeHolder, codegen};

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"#[^\n]*", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]

    #[token("section")]
    Section,
    #[regex("constants|imports|exports|aliases|code")]
    SectionLoc,
    
    #[regex("const|global|local")]
    RegLoc,
    #[token("alias")]
    Alias,
    #[token("=>")]
    Arrow,
    #[token("-")]
    Minus,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBrac,
    #[token("]")]
    RBrac,
    #[token(".")]
    Period,
    #[token(",")]
    Comma,

    #[regex("[0-9][_0-9]*")]
    Int,
    #[regex("\"[.a-zA-Z][_0-9a-zA-Z]*\"")]
    String,

    #[regex("true|false")]
    Bool,
    
    #[regex("(?i)alloc|free|frame_alloc|frame_free|jump|call|ext_call|ret|mov|cpy|ref|stack_push|stack_mov|stack_pop|add|sub|mul|div|mod|equal|not_equal|greater|less|greater_equal|less_equal")]
    Instruction,

    #[regex("[_a-zA-Z][_0-9a-zA-Z]*")]
    Identifier,

    #[error]
    Error,
}

enum CurrentSection {
    Constants,
    Aliases,
    Imports,
    Exports,
    Code,
}

/// Creates tokens from the file itself
fn lex(file: &File) -> Vec<(Token, String)> {
    let mut tokens: Vec<(Token, String)> = Vec::new();
    let file_buffer = io::BufReader::new(file).lines();
    
    // Iterate over each line in the buffer
    for file_line in file_buffer {
        // If the line is valid
        if let Ok(line) = file_line {
            let mut line_tok = Token::lexer(&line);
            loop {
                let token = line_tok.next(); let value = line_tok.slice();
                if token.is_some() {
                    tokens.push((token.unwrap(), value.to_string()));
                } else {
                    break;
                }
            }
        }
    }
    tokens
}

/*
Code related to parsing and generating code
*/
macro_rules! next_set {
    ($itr:ident, $next_elem:ident, $pair:ident) => {
        $next_elem = $itr.next();
        $pair = $next_elem.unwrap();
    };
}
fn parse(tokens: Vec<(Token, String)>) -> CodeHolder {
    let mut code_holder = CodeHolder::new();
    let mut itr = tokens.iter();
    let mut current_section = CurrentSection::Constants;
    loop {
        let mut next_elem = itr.next();
        // We have to break out eventually
        if let Option::None = next_elem {
            break;
        }
        let mut pair = next_elem.unwrap();
        match pair.0 {
            Token::Section => {
                next_set!(itr, next_elem, pair);
                if let Token::SectionLoc = pair.0 {
                    current_section = match pair.1.as_str() {
                        "constants" => CurrentSection::Constants,
                        "alisases" => CurrentSection::Aliases,
                        "imports" => CurrentSection::Imports,
                        "exports" => CurrentSection::Exports,
                        "code" => CurrentSection::Code,
                        _ => panic!("Section name \"{}\" is invalid!", pair.1),
                    };
                    // Let's compile the Constants section to make our life
                    // easier
                    if let CurrentSection::Constants = current_section {
                        loop {
                            next_set!(itr, next_elem, pair);
                            if let Token::Int = pair.0 {
                                next_set!(itr, next_elem, pair);
                                if let Token::Arrow = pair.0 {
                                    next_set!(itr, next_elem, pair);
                                    match pair.0 {
                                        _ => panic!("Expected constant, got {:?}", pair.0),
                                    }
                                }
                            } else {
                                panic!("Expected Int, got {:?}", pair.0);
                            }
                        }
                    }
                } else {
                    panic!("Expected Section Location, got {:?}", pair.0);
                }
            }
            _ => panic!("Invalid Syntax!"),
        }
    }
    code_holder
}

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("File failed to open! {}", e),
        };

        let tokens = lex(&file);

        for tok in tokens {
            println!("{} | {:?}", tok.1, tok.0);
        }

    } else {
        panic!("Must provide a file to compile!");
    }
}
