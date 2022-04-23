use crate::parser::tokens::{Comment, Span, Token, Tokens};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    cursor: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: String::from(input),
            cursor: 0,
        }
    }

    fn move_cursor(&mut self, n: usize) {
        self.cursor += n;
    }

    fn incr_cursor(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.cursor);
        self.cursor += 1;
        c
    }

    fn peek(&self, n: usize) -> Option<&str> {
        if self.input.len() <= self.cursor + n {
            None
        } else {
            Some(&self.input[self.cursor..self.cursor + n])
        }
    }

    fn peek_is_number(&mut self) -> bool {
        if let Some(c) = self.next_char() {
            return match c {
                '0'..='9' => true,
                _ => false,
            };
        }

        false
    }

    fn next_char_is_number(&self) -> bool {
        match self.next_char() {
            Some('0'..='9') => true,
            _ => false,
        }
    }

    fn cur_char(&self) -> Option<char> {
        self.input.chars().nth(self.cursor)
    }

    fn next_char(&self) -> Option<char> {
        self.input.chars().nth(self.cursor + 1)
    }

    pub fn match_chars(&mut self, other: &str) -> bool {
        if self.input.len() < other.len() {
            return false;
        }
        for (i, c) in self.input.chars().skip(self.cursor).enumerate() {
            if Some(c) != other.chars().nth(i) {
                return false;
            }
        }
        true
    }

    fn multi_line_comment(&mut self) -> Option<Token> {
        let start = self.cursor;

        self.move_cursor(4);

        let mut comment = String::new();

        loop {
            if self.match_chars("]]--") {
                self.move_cursor(4);
                let end = self.cursor;
                break Some(Token {
                    kind: Tokens::Comment(Comment::MultiLine(comment)),
                    span: Span { start, end },
                });
            } else if let Some(c) = self.incr_cursor() {
                comment.push(c);
            } else {
                break None;
            }
        }
    }

    fn single_line_comment(&mut self) -> Option<Token> {
        let start = self.cursor;
        self.move_cursor(2);
        let mut comment = String::new();
        loop {
            if Some('\n') == self.cur_char() {
                break;
            }
            if let Some(c) = self.incr_cursor() {
                comment.push(c);
            } else {
                break;
            }
        }
        let end = self.cursor;
        Some(Token {
            kind: Tokens::Comment(Comment::SingleLine(comment)),
            span: Span { start, end },
        })
    }

    fn single_line_string(&mut self) -> Option<Token> {
        let start = self.cursor;
        let closing = self.incr_cursor();
        let mut s = String::new();
        loop {
            match self.incr_cursor() {
                Some(e) if Some(e) == closing => break,
                Some(sc) if sc == '\n' => return None,
                Some(sc) => s.push(sc),
                None => return None,
            }
        }
        let end = self.cursor;
        Some(Token {
            kind: Tokens::String(s),
            span: Span { start, end },
        })
    }

    fn multi_line_string(&mut self) -> Option<Token> {
        let start = self.cursor;
        self.move_cursor(2);
        let mut s = String::new();
        let end = self.cursor;
        loop {
            if self.match_chars("]]") {
                self.move_cursor(2);
                break Some(Token {
                    kind: Tokens::String(s),
                    span: Span { start, end },
                });
            }
            match self.incr_cursor() {
                Some(sc) => s.push(sc),
                None => break None,
            }
        }
    }

    fn identifier(&mut self) -> Option<Token> {
        let mut s = String::new();
        let start = self.cursor;

        while let Some(n) = self.cur_char() {
            match n {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
                    if let Some(c) = self.incr_cursor() {
                        s.push(c);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        let end = self.cursor;

        let span = Span { start, end };

        let kind = match s.as_str() {
            "false" => Tokens::False,
            "true" => Tokens::True,
            "nil" => Tokens::Nil,
            "not" => Tokens::Not,
            "for" => Tokens::For,
            "do" => Tokens::Do,
            "in" => Tokens::In,
            "function" => Tokens::Function,
            "break" => Tokens::Break,
            "return" => Tokens::Return,
            "while" => Tokens::While,
            "repeat" => Tokens::Repeat,
            "until" => Tokens::Until,
            "or" => Tokens::Or,
            "and" => Tokens::And,
            "goto" => Tokens::Goto,
            "end" => Tokens::End,
            "if" => Tokens::If,
            "then" => Tokens::Then,
            "elseif" => Tokens::ElseIf,
            "else" => Tokens::Else,
            "local" => Tokens::Local,
            "const" => Tokens::Const,
            "class" => Tokens::Class,
            "public" => Tokens::Public,
            "private" => Tokens::Private,
            "type" => Tokens::Type,
            "interface" => Tokens::Interface,
            "extends" => Tokens::Extends,
            "implements" => Tokens::Implements,
            "switch" => Tokens::Switch,
            _ => Tokens::Ident(s),
        };

        Some(Token { kind, span })
    }

    fn number(&mut self) -> Option<Token> {
        let start = self.cursor;
        let mut s = String::new();

        if self.cur_char() == Some('-') {
            if let Some(c) = self.incr_cursor() {
                s.push(c);
            }
        }

        while let Some(n) = self.cur_char() {
            match n {
                '0'..='9' | '.' => {
                    if let Some(c) = self.incr_cursor() {
                        s.push(c);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        let span = Span {
            start,
            end: self.cursor,
        };

        match s.parse() {
            Ok(num) => Some(Token {
                kind: Tokens::Number(num),
                span,
            }),
            _ => Some(Token {
                kind: Tokens::Unknown(s),
                span,
            }),
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if let Some("--[[") = &self.peek(4) {
            self.multi_line_comment()
        } else if let Some(c) = self.cur_char() {
            let start = self.cursor;
            let next = self.next_char();
            match c {
                '\'' | '"' | '`' => self.single_line_string(),
                '[' if next == Some('[') => self.multi_line_string(),
                '=' if next == Some('=') => {
                    self.move_cursor(2);
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::EQ,
                        span: Span { start, end },
                    })
                }
                '=' if next == Some('>') => {
                    self.move_cursor(2);
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Arrow,
                        span: Span { start, end },
                    })
                }
                '=' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Assign,
                        span: Span { start, end },
                    })
                }

                ';' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::SemiColon,
                        span: Span { start, end },
                    })
                }
                '[' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::LBracket,
                        span: Span { start, end },
                    })
                }
                ']' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::RBracket,
                        span: Span { start, end },
                    })
                }
                'A'..='Z' | 'a'..='z' | '_' => self.identifier(),
                '\n' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::NewLine,
                        span: Span { start, end },
                    })
                }
                ' ' => {
                    self.incr_cursor();
                    self.next()
                }
                '.' if self.next_char_is_number() => self.number(),
                '0'..='9' => self.number(),
                '-' if next == Some('-') => self.single_line_comment(),
                '-' => {
                    if self.peek_is_number() {
                        return self.number();
                    }
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Minus,
                        span: Span { start, end },
                    })
                }
                '(' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::LParen,
                        span: Span { start, end },
                    })
                }
                ')' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::RParen,
                        span: Span { start, end },
                    })
                }
                '{' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::LCurly,
                        span: Span { start, end },
                    })
                }
                '}' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::RCurly,
                        span: Span { start, end },
                    })
                }
                ',' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Comma,
                        span: Span { start, end },
                    })
                }
                '.' if next == Some('.') => {
                    self.move_cursor(2);
                    if self.cur_char() == Some('.') {
                        self.incr_cursor();
                        let end = self.cursor;
                        Some(Token {
                            kind: Tokens::Dots,
                            span: Span { start, end },
                        })
                    } else {
                        let end = self.cursor;
                        Some(Token {
                            kind: Tokens::Concat,
                            span: Span { start, end },
                        })
                    }
                }
                '.' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Period,
                        span: Span { start, end },
                    })
                }
                ':' if next == Some(':') => {
                    self.move_cursor(2);
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::DBColon,
                        span: Span { start, end },
                    })
                }
                ':' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Colon,
                        span: Span { start, end },
                    })
                }
                '<' if next == Some('<') => {
                    self.move_cursor(2);
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::SHL,
                        span: Span { start, end },
                    })
                }
                '<' if next == Some('=') => {
                    self.move_cursor(2);
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::LTE,
                        span: Span { start, end },
                    })
                }
                '<' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::LT,
                        span: Span { start, end },
                    })
                }
                '>' if next == Some('>') => {
                    self.move_cursor(2);
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::SHR,
                        span: Span { start, end },
                    })
                }
                '>' if next == Some('=') => {
                    self.move_cursor(2);
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::GTE,
                        span: Span { start, end },
                    })
                }
                '>' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::GT,
                        span: Span { start, end },
                    })
                }
                '+' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Plus,
                        span: Span { start, end },
                    })
                }
                '#' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Hash,
                        span: Span { start, end },
                    })
                }
                '*' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Mul,
                        span: Span { start, end },
                    })
                }
                '/' if next == Some('/') => {
                    self.move_cursor(2);
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::IntDiv,
                        span: Span { start, end },
                    })
                }
                '/' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Div,
                        span: Span { start, end },
                    })
                }
                '%' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Mod,
                        span: Span { start, end },
                    })
                }
                '^' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Pow,
                        span: Span { start, end },
                    })
                }
                '&' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::BitAnd,
                        span: Span { start, end },
                    })
                }
                '|' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::BitOr,
                        span: Span { start, end },
                    })
                }
                '~' if next == Some('=') => {
                    self.move_cursor(2);
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::NEQ,
                        span: Span { start, end },
                    })
                }
                '~' => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::BitXor,
                        span: Span { start, end },
                    })
                }
                unknown => {
                    self.incr_cursor();
                    let end = self.cursor;
                    Some(Token {
                        kind: Tokens::Unknown(unknown.to_string()),
                        span: Span { start, end },
                    })
                }
            }
        } else {
            None
        }
    }
}

#[test]
fn parse() {
    let lex: Vec<_> = Lexer::new(r#"
        local func = () => hello
        class WOW
        ----comment
            public method()
                print("wow")
            end
        end
    "#).collect();
    dbg!(lex);
}