use std::iter::{Enumerate, Peekable};
use std::panic;
use std::slice::Iter;
use crate::parser::tokens::{Span, Token, Tokens};
use line_col::LineColLookup;
use crate::parser::errors::{ErrorKind, ParseErrors};
use colored::Colorize;


struct ErrorClient<'a> {
    original_source: &'a String,
    lookup: LineColLookup<'a>
}


impl <'a>ErrorClient<'a> {
    pub fn new(source: &'a String) -> ErrorClient<'a> {
        ErrorClient {original_source: source, lookup: LineColLookup::new(source.as_str())}
    }
    fn is_inside_bounds(&self, span: &Span) -> bool {
        span.start < self.original_source.len() && span.end <= self.original_source.len()
    }
    pub fn raise_parse_err(&self, kind: ParseErrors, origin: &Span, at: &Span, exit: bool) {
            let err = kind.to_string();
            dbg!(at);
            let (ln, clm) = if self.is_inside_bounds(origin) { self.lookup.get(origin.start) } else {(0, 0)};
            let content = &self.original_source[origin.start..=at.end];
            let highlighted = &self.create_spanned_chars(&at, '^')[origin.start..=at.end];
            println!("> {}", content);
            println!("  {}", highlighted.red());
            eprintln!("Compiler error at line {}:{}\n{}", ln, clm, err);
            if exit {
                std::process::exit(1);
            }
    }

    fn create_spanned_chars(&self, span: &Span, with: char) -> String {
        let mut cnt = self.original_source.to_string().chars().collect::<Vec<char>>();
        for ix in span.start..=span.end {
            cnt.insert(ix, with);
        }
        cnt.iter().map(|chr| if chr != &with {' '} else {*chr}).collect::<String>()
    }
}




fn calc_line_and_col(text: &LineColLookup, span: &Span) -> (usize, usize) {
    text.get(span.start)
}


type TokenIter<'a> = Peekable<Enumerate<Iter<'a, Token>>>;

struct Parser<'a> {
    err_client: ErrorClient<'a>,
    iter: &'a mut TokenIter<'a>,
    source: &'a String
}



impl <'a>Parser<'a> {
    pub fn parse(&mut self) {
        while let Some((ix, token)) = self.iter.next() {
            match token.kind {
                Tokens::Class => self.parse_class(ix, token),
                _ => {} //never happens
            }
        }
    }
    fn parse_class(&mut self, ix: usize, start: &Token) {
        let next = self.iter.peek();
        if next.is_some() {
            let (_, next) = next.unwrap();
            if let Tokens::Ident(ident) = &next.kind {

            } else {
                self.err_client.raise_parse_err(ParseErrors::ExpectedAfterButReceived(Tokens::Ident("".to_string()), start.kind.clone(), next.kind.clone()), &start.span, &next.span, true);
            }
        } else {
            //todo! raise compiler error
        }
    }
}

pub fn parse<'a>(src: String, tokens: Vec<Token>) {
    let err_client = ErrorClient::new(&src);
    let mut iter: TokenIter = tokens.iter().enumerate().peekable();
    let mut parser = Parser {
        err_client,
        iter: &mut iter,
        source: &src
    };
    parser.parse();
}