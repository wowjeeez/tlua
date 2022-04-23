

mod parser;
pub struct Compiler {
    files: Vec<String>,
    strict_mode: bool,
    mangle_names: bool,
    bundle: bool
}

impl Compiler {
    pub fn new(files: Vec<String>, strict_mode: bool, mangle_names: bool, bundle: bool) -> Compiler {
            Compiler {
                files,
                strict_mode,
                mangle_names,
                bundle
            }
    }
}




#[test]
fn parse() {
    use crate::parser::{Lexer, parse};
    let inp = r#"
        local func = () => hello
        type myType<T> = T extends boolean ? "yes" : "no"
        class WOW
        ----comment
            public method()
                print("wow")
            end
        end
    "#;
    println!("Input length: {}", inp.len());
    let lex: Vec<_> = Lexer::new(inp).collect();
    parse(inp.to_string(), lex);
}