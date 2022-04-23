use crate::parser::tokens::Tokens;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Parse(ParseErrors),
    Type(TypeErrors),
}
#[derive(Clone, Debug)]
pub enum ParseErrors {
    ExpectedAfterButReceived(Tokens, Tokens, Tokens)
}

const VOWELS: &'static str = "aeiou";

fn disp_enum(val: &Tokens, upper_first: bool, add_prefix: bool) -> String {
    let matched = match val {
        Tokens::Ident(_) => "identifier".into(),
        Tokens::String(_) => "string literal".into(),
        Tokens::Class => "class declaration".into(),

        _ => format!("{:?}", val)
    };
    return if add_prefix {
        a_or_an(matched, upper_first)
    } else {
        matched
    }
}

fn str_cap(s: String) -> String {
    format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..])
}

fn a_or_an(str: String, upper_first: bool) -> String {
    let first_char = str.chars().nth(0).unwrap().to_lowercase().to_string();
    let an = if upper_first {"An "} else {"an "};
    let a = if upper_first {"A "} else {"a "};
    return if VOWELS.contains(&first_char) {
        an.to_owned() + &str
    } else {
        a.to_owned() + &str
    }
}

impl ToString for ParseErrors {
    fn to_string(&self) -> String {
        String::from(match self {
            ParseErrors::ExpectedAfterButReceived(exp, after, recv) => format!("{} expected after {}, but received {}.", str_cap(disp_enum(exp, true, false)), disp_enum(after, false, true), disp_enum(recv, false, true))
        })
    }
}

#[derive(Clone, Debug)]
pub enum TypeErrors {

}
