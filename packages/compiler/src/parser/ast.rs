use crate::parser::tokens::{Span, Tokens};
use crate::parser::tokens::Comment;
pub enum AstNodes {
    Class()
}

pub struct SpannedCommend {
    comment: Comment,
    span: Span
}

pub struct AstNode {
    kind: AstNodes,
    children: Vec<AstNodes>,
    span: Span,
    comments: Vec<SpannedCommend>
}


pub struct Type {
    name: String
}

pub struct NamedTokenWithTypeInfo {
    name: String,
    typ: Type
}

pub struct FunctionDecl {
    name: String,
    args: Vec<NamedTokenWithTypeInfo>,
    is_class: bool,
}

pub struct ClassNode {
    name: String,
    private_methods: Vec<FunctionDecl>,
    public_methods: Vec<FunctionDecl>,
    private_fields: Vec<NamedTokenWithTypeInfo>,
    public_fields: Vec<NamedTokenWithTypeInfo>
}