use std::fmt;

use arcstr::ArcStr;
use logos::Logos;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    InvalidToken,
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\n\f]+", error = LexicalError)]
pub enum Token {
    Error,

    #[token("native")]
    Native,
    #[token("def")]
    Def,
    #[token("type")]
    Type,
    #[token("let")]
    Let,
    #[token("recv")]
    Recv,
    #[token("either")]
    Either,
    #[token("case")]
    Case,
    #[token("choice")]
    Choice,
    #[token("chan")]
    Chan,
    #[token("dual")]
    Dual,
    #[token("recursive")]
    Recursive,
    #[token("iterative")]
    Iterative,
    #[token("box")]
    Box,
    #[token("self")]
    Self_,
    #[token("begin")]
    Begin,
    #[token("loop")]
    Loop,
    #[token("in")]
    In,
    #[token("try")]
    Try,
    #[token("catch")]
    Catch,
    #[token("throw")]
    Throw,
    #[token("do")]
    Do,
    #[token("unfounded")]
    Unfounded,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,

    #[token("!")]
    Unit,
    #[token("?")]
    Question,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
    #[token("<>")]
    Link,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    Le,
    #[token(">=")]
    Ge,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("/")]
    Slash,
    #[token("=")]
    Eq,
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token("=>")]
    DoubleArrow,
    #[token("->")]
    Arrow,
    #[token("<-")]
    ReverseArrow,
    #[token("*")]
    Star,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("%")]
    Percent,
    #[token("@")]
    Amp,
    #[token("::")]
    DoubleColon,
    #[token("'")]
    SingleQuote,

    #[regex(r#""(\\\\|\\"|[^"\\])*""#, |lex| ArcStr::from(apply_string_escapes(lex.slice().to_string())))]
    Str(ArcStr),
    #[regex(r"[-]?[0-9]+", |lex| lex.slice().parse().ok())]
    I32(i32),
    #[regex(r"[0-9]+u64", |lex| lex.slice().parse().ok())]
    U64(u64),

    #[regex(r"[A-Z][A-Za-z0-9_]*", |lex| ArcStr::from(lex.slice()))]
    GlobalIdent(ArcStr),
    #[regex(r"[a-z_][A-Za-z0-9_]*", |lex| ArcStr::from(lex.slice()))]
    LocalIdent(ArcStr),

    #[regex(r#"//[^\n\r]*(\n|\r|\r\n)"#, |lex| ArcStr::from(lex.slice()))]
    Comment(ArcStr),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn apply_string_escapes(input: String) -> ArcStr {
    input[1..input.len() - 1]
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
        .into()
}
