use anyhow::{Context, Result};
use std::env;
use std::fs;
use wit_parser::{LexSpan as Span, LexToken as Token, LexTokenizer as Tokenizer};

fn main() -> Result<()> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        anyhow::bail!("expected at least one WIT path");
    }
    for path in args.drain(..) {
        dump_file(&path)?;
    }
    Ok(())
}

fn dump_file(path: &str) -> Result<()> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {path}"))?;
    let mut tokenizer =
        Tokenizer::new(&contents, 0, None).with_context(|| format!("failed to lex {path}"))?;
    println!("FILE\t{path}");
    while let Some((span, token)) = tokenizer
        .next()
        .with_context(|| format!("failed to lex {path}"))?
    {
        println!("TOKEN\t{}\t{}\t{}", token_name(token), span.start, span.end);
    }
    Ok(())
}

fn token_name(token: Token) -> &'static str {
    match token {
        Token::Whitespace => "Whitespace",
        Token::Comment => "Comment",
        Token::Equals => "Equals",
        Token::Comma => "Comma",
        Token::Colon => "Colon",
        Token::Period => "Period",
        Token::Semicolon => "Semicolon",
        Token::LeftParen => "LeftParen",
        Token::RightParen => "RightParen",
        Token::LeftBrace => "LeftBrace",
        Token::RightBrace => "RightBrace",
        Token::LessThan => "LessThan",
        Token::GreaterThan => "GreaterThan",
        Token::RArrow => "RArrow",
        Token::Star => "Star",
        Token::At => "At",
        Token::Slash => "Slash",
        Token::Plus => "Plus",
        Token::Minus => "Minus",
        Token::Use => "Use",
        Token::Type => "Type",
        Token::Func => "Func",
        Token::U8 => "U8",
        Token::U16 => "U16",
        Token::U32 => "U32",
        Token::U64 => "U64",
        Token::S8 => "S8",
        Token::S16 => "S16",
        Token::S32 => "S32",
        Token::S64 => "S64",
        Token::F32 => "F32",
        Token::F64 => "F64",
        Token::Char => "Char",
        Token::Record => "Record",
        Token::Resource => "Resource",
        Token::Own => "Own",
        Token::Borrow => "Borrow",
        Token::Flags => "Flags",
        Token::Variant => "Variant",
        Token::Enum => "Enum",
        Token::Bool => "Bool",
        Token::String_ => "String_",
        Token::Option_ => "Option_",
        Token::Result_ => "Result_",
        Token::Future => "Future",
        Token::Stream => "Stream",
        Token::ErrorContext => "ErrorContext",
        Token::List => "List",
        Token::Underscore => "Underscore",
        Token::As => "As",
        Token::From_ => "From",
        Token::Static => "Static",
        Token::Interface => "Interface",
        Token::Tuple => "Tuple",
        Token::Import => "Import",
        Token::Export => "Export",
        Token::World => "World",
        Token::Package => "Package",
        Token::Constructor => "Constructor",
        Token::Async => "Async",
        Token::Id => "Id",
        Token::ExplicitId => "ExplicitId",
        Token::Integer => "Integer",
        Token::Include => "Include",
        Token::With => "With",
    }
}
