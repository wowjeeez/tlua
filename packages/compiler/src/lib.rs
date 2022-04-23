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