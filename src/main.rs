/// This is the main file for the RASM assembler in Rust. It was originally written in Python and
/// eventually ported to Rust after the codegen API was finished
///
/// Began being written by StandingPad on January 1st 2023 while braindead from 5 hours of sleep
use std::{fs, env, fmt};
use logos::Logos;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    input::{Stream, ValueInput},
    prelude::*, combinator::To,
};

#[derive(Logos, Clone, PartialEq)]
enum Token<'a> {
    #[token("section")]
    Section,
    #[regex("constants")]
    Constants,
    #[regex("imports")]
    Imports,
    #[regex("exports")]
    Exports,
    #[regex("aliases")]
    Aliases,
    #[regex("code")]
    Code,

    #[regex("const|global|local")]
    RegLoc(&'a str),
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
    Int(&'a str),
    #[regex("\"[.a-zA-Z][_0-9a-zA-Z]*\"")]
    String(&'a str),

    #[regex("true|false")]
    Bool(&'a str),
    
    #[regex("(?i)alloc|free|frame_alloc|frame_free|jump|call|ext_call|ret|mov|cpy|ref|stack_push|stack_mov|stack_pop|add|sub|mul|div|mod|equal|not_equal|greater|less|greater_equal|less_equal")]
    Instruction(&'a str),

    #[regex("[_a-zA-Z][_0-9a-zA-Z]*")]
    Identifier(&'a str),

    #[regex(r"[ \t\f\n]+", logos::skip)]
    Whitespace,
    #[regex(r"#[^\n]*", logos::skip)]
    Comment,
    
    #[error]
    Error
}


impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Section           => write!(f, "section"),
            Token::Constants         => write!(f, "constants"),
            Token::Imports           => write!(f, "imports"),
            Token::Exports           => write!(f, "exports"),
            Token::Aliases           => write!(f, "aliases"),
            Token::Code              => write!(f, "code"),
            Token::RegLoc(s)         => write!(f, "{}", s),
            Token::Alias             => write!(f, "alias"),
            Token::Arrow             => write!(f, "=>"),
            Token::Minus             => write!(f, "-"),
            Token::LParen            => write!(f, "("),
            Token::RParen            => write!(f, ")"),
            Token::LBrac             => write!(f, "["),
            Token::RBrac             => write!(f, "]"),
            Token::Period            => write!(f, "."),
            Token::Comma             => write!(f, ","),
            Token::Int(s)            => write!(f, "number {}", s),
            Token::String(s)         => write!(f, "string {}", s),
            Token::Bool(s)           => write!(f, "bool {}", s),
            Token::Instruction(s)    => write!(f, "instruction {}", s),
            Token::Identifier(s)     => write!(f, "identifier {}", s),
            Token::Whitespace        => write!(f, "<whitespace>"),
            Token::Comment           => write!(f, "<comment>"),
            Token::Error             => write!(f, "<error>"),
        }
    }
}

enum RegLoc {
    CONST,
    GLOBAL,
    LOCAL,
}

enum Expr {
    Int(i64),
    String(String),
    Bool(bool),
    
    Register((RegLoc, u32)),
    Assignment(String, Box<Self>),


    ConstSection(Vec<Self>),
    AliasesSection(Vec<Self>),
}

fn parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I,Expr, extra::Err<Rich<'a, Token<'a>>>> {
    // Parse identifiers
    let ident = select! { Token::Identifier(s) => s.to_string() };
    
    // Parse u32 integers
    let u32_literal = select! { Token::Int(s) => s.parse().unwrap() };

    // Parse literals in the constant pool
    let const_literal = select!{
        Token::Int(s) => Expr::Int(s.parse().unwrap()),
        Token::String(s) => Expr::String(s.to_string()),
        Token::Bool(s) => Expr::Bool(s.parse().unwrap()),
    };
    
    // Parse register locations
    let reg_loc = select! {
        Token::RegLoc(s) if s == "const" => RegLoc::CONST,
        Token::RegLoc(s) if s == "global" => RegLoc::GLOBAL,
        Token::RegLoc(s) if s == "local" => RegLoc::LOCAL 
    };
    
    // Parse the constants section
    let constant_array = just(Token::Section)
        .then(just(Token::Constants))
        .ignore_then(const_literal
            .then_ignore(just(Token::Comma))
            .repeated()
            .collect()
            .map(Expr::ConstSection)
            .delimited_by(just(Token::LBrac), just(Token::RBrac)
            )
        );
    
    // Parse the aliases section
    let alias_section = just(Token::Section)
        .then(just(Token::Aliases))
        .ignore_then(
            ident
            .then(just(Token::Arrow)
                .ignore_then(reg_loc)
                .then_ignore(just(Token::LBrac))
                .then(u32_literal)
                .then_ignore(just(Token::RBrac))
                .map(Expr::Register)
            )
            .map(|(name, reg_obj)| Expr::Assignment(name, Box::new(reg_obj)))
            .repeated()
            .collect()
            .map(Expr::AliasesSection)
        );
        
    constant_array
        .or(alias_section)
}


fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");
    let src = src.as_str();
    let token_iter = Token::lexer(src)
        .spanned()
        // Map the `Range<usize>` logos gives us into chumsky's `SimpleSpan`, because it's easier to work with
        .map(|(tok, span)| {
            (tok, span.into())
        });

    // Turn the token iterator into a stream that chumsky can use for things like backtracking
    let token_stream = Stream::from_iter(token_iter)
        // Tell chumsky to split the (Token, SimpleSpan) stream into its parts so that it can handle the spans for us
        // This involves giving chumsky an 'end of input' span: we just use a zero-width span at the end of the string
        .spanned((src.len()..src.len()).into());

    // Parse the token stream with our chumsky parser
    match parser().parse(token_stream).into_result() {
        // If parsing was successful, attempt to evaluate the expression
        Ok(_) => return,
        Err(errs) => errs.into_iter().for_each(|e| {
            Report::build(ReportKind::Error, (), e.span().start)
                .with_code(3)
                .with_message(e.to_string())
                .with_label(
                    Label::new(e.span().into_range())
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .eprint(Source::from(src))
                .unwrap()
        }),
    };
}
