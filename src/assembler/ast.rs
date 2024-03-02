use std::collections::HashMap;

// pub trait ASTLine {
//     // Methods for ASTLine
//     // fn generate_byte_string(&self, index: isize, labels: &HashMap<&String, &String>);
// }

// macro_rules! line_struct {
//     // Entry point for the macro
//     ($struct_name:ident) => {
//         // Implement methods for the struct
//     };
// }
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Line {
    ASTLabel(Box<ASTLabel>),
    ASTInstruction(Box<ASTInstruction>),
    ASTWord(Box<ASTWord>),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct LineList{
    lines: Vec<Box<Line>>,
}

impl LineList{

    // pub fn add_line_front(&mut self, line: Box<Line>){
    //     self.lines.insert(0, line);
    // }

    pub fn new(lines:  Vec<Box<Line>>) -> Self {
        LineList {
            lines: lines,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ASTLabel {
    // A labeled AST line
    label: String,
    labelled_line: Box<Line>
    // next_line: Option<&'a dyn ASTLine>,
}
// line_struct!(ASTLabel);

impl ASTLabel {
    pub fn new(label: String, line: Box<Line>) -> Self {
        ASTLabel {
            label: label,
            labelled_line: line,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ASTInstruction {
    // A set of tokens forming an instruction
    tokens: Vec<String>,
    // next_line: Option<&'a dyn ASTLine>,
}
// line_struct!(ASTInstruction);


impl ASTInstruction {
    pub fn new(tokens: Vec<String>) -> Self {
        ASTInstruction {
            tokens: tokens,
        }
    }

    // pub fn add_token_front(&mut self, token: String) {
    //     self.tokens.insert(0, token);
    // }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ASTWord {
    // generated from a .word statement
    token: String,
    // next_line: Option<&'a dyn ASTLine>,
}
// line_struct!(ASTWord);
impl  ASTWord {
    pub fn new(token: String) -> Self {
        ASTWord {
            token: token,
        }
    }
}
