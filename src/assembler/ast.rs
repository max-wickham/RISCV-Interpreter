use std::{collections::HashMap, fmt};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Line {
    ASTLabel(Box<ASTLabel>),
    ASTInstruction(Box<ASTInstruction>),
    ASTWord(Box<ASTWord>),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct LineList{
    pub lines: Vec<Box<Line>>,
}

impl LineList{

    pub fn new(lines:  Vec<Box<Line>>) -> Self {
        LineList {
            lines: lines,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct ASTLabel {
    pub label: String,
    pub labelled_line: Box<Line>
}

impl ASTLabel {
    pub fn new(label: String, line: Box<Line>) -> Self {
        ASTLabel {
            label: label,
            labelled_line: line,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct ASTInstruction {
    pub tokens: Vec<String>,
}

impl fmt::Display for ASTInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define the custom formatting here
        write!(f, "{:?}", self.tokens)
    }
}


impl ASTInstruction {
    pub fn new(tokens: Vec<String>) -> Self {
        ASTInstruction {
            tokens: tokens,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct ASTWord {
    pub tokens: Vec<String>,
}

impl  ASTWord {
    pub fn new(tokens: Vec<String>) -> Self {
        ASTWord {
            tokens: tokens,
        }
    }
}
