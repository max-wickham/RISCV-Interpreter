mod asm;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Binary;
use std::hash::Hash;
use std::num::ParseIntError;

use crate::assembler::ast::{ASTInstruction, ASTLabel, ASTWord, Line, LineList};
use crate::riscv_spec::{
    BRACKET_INSTRUCTIONS, B_TYPE_INSTRUCTIONS, FUNCT_3_BITS, FUNCT_7_BITS, I_TYPE_INSTRUCTIONS,
    J_TYPE_INSTRUCTIONS, OPCODE_BITS, REGISTER_BITS, R_TYPE_INSTRUCTIONS, S_TYPE_INSTRUCTIONS,
    U_TYPE_INSTRUCTIONS,
};

fn int<'a>(string: &'a str, base: u32) -> i64 {
    // print!("{}", string);
    i64::from_str_radix(string, base).unwrap()
}

fn int_to_4_byte_vec(integer: i64) -> Vec<u8> {
    return vec![
        (integer >> 24 & 0xFF) as u8,
        (integer >> 16 & 0xFF) as u8,
        (integer >> 8 & 0xFF) as u8,
        (integer & 0xFF) as u8,
    ];
}

fn generate_bytes_r_type(tokens: &Vec<String>) -> Vec<u8> {
    let instruction = tokens[0].to_lowercase();
    let rd = &tokens[1];
    let rs1 = &tokens[2];
    let rs2 = &tokens[3];

    let binary = (int(FUNCT_7_BITS[&instruction], 2) & 0b1111111) << 25
        | (int(REGISTER_BITS[rs2], 2) & 0b11111) << 20
        | (int(REGISTER_BITS[rs1], 2) & 0b11111) << 15
        | (int(FUNCT_3_BITS[&instruction], 2) & 0b111) << 12
        | (int(REGISTER_BITS[rd], 2) & 0b11111) << 7
        | (int(OPCODE_BITS[&instruction], 2) & 0b1111111);

    int_to_4_byte_vec(binary)
}

fn generate_bytes_i_type(tokens: &Vec<String>) -> Vec<u8> {
    let instruction = tokens[0].to_lowercase();
    let opcode = int(OPCODE_BITS[&instruction], 2);
    let funct_3 = int(FUNCT_3_BITS[&instruction], 2);
    let rd = int(REGISTER_BITS[&tokens[1]], 2);
    let imm;
    let rs1;

    if BRACKET_INSTRUCTIONS.contains(&instruction.as_str()) {
        let rs1_string = REGISTER_BITS[tokens[2].split("(").nth(1).unwrap()];
        rs1 = int(&rs1_string[..rs1_string.len() - 1], 2) & 0xFFFFFFFF;
        // TODO handle none base 10
        imm = int(tokens[2].split("(").nth(0).unwrap(), 10);
    } else {
        // println!("{:?}",tokens);
        rs1 = int(REGISTER_BITS[&tokens[2]], 2);
        // TODO handle none base 10
        imm = int(&tokens[3], 10) & 0xFFFFFFFF;
    }

    let binary = (imm & 0xFFF) << 20
        | ((rs1 & 0b11111) << 15)
        | ((funct_3 & 0b111) << 12)
        | ((rd & 0b11111) << 7)
        | (opcode & 0b1111111);

    int_to_4_byte_vec(binary)
}

fn generate_bytes_s_type(tokens: &Vec<String>) -> Vec<u8> {
    let instruction = tokens[0].to_lowercase();
    let opcode = int(OPCODE_BITS[&instruction], 2);
    let funct_3 = int(FUNCT_3_BITS[&instruction], 2);
    let rs2 = int(REGISTER_BITS[&tokens[1]], 2);
    // TODO deal with none base 10 cases
    let imm = int(&tokens[2], 10);
    let rs1 = int(REGISTER_BITS[&tokens[3]], 2);

    let binary = (((imm >> 5) & 0xFF) << 25)
        | ((rs2 & 0b11111) << 12)
        | ((rs1 & 0b11111) << 15)
        | ((funct_3 & 0b111) << 12)
        | (((imm) & 0x1F) << 7)
        | (opcode & 0b1111111);

    int_to_4_byte_vec(binary)
}

fn generate_bytes_u_type(tokens: &Vec<String>) -> Vec<u8> {
    let instruction = tokens[0].to_lowercase();
    let opcode = int(OPCODE_BITS[&instruction], 2);
    let rd = int(REGISTER_BITS[&tokens[1]], 2);
    // TODO deal with none base 10 cases
    let imm = int(&tokens[2], 10);

    let binary = ((imm & 0xFFFFF) << 12) | ((rd & 0b11111) << 7) | (opcode & 0b1111111);

    int_to_4_byte_vec(binary)
}

fn generate_bytes_b_type(tokens: &Vec<String>) -> Vec<u8> {
    let instruction = tokens[0].to_lowercase();
    let opcode = int(OPCODE_BITS[&instruction], 2);
    let funct_3 = int(FUNCT_3_BITS[&instruction], 2);
    let rs1 = int(REGISTER_BITS[&tokens[1].to_lowercase()], 2);
    let rs2 = int(REGISTER_BITS[&tokens[2].to_lowercase()], 2);
    // TODO deal with none base 10 cases
    let imm = int(&tokens[3], 10);

    let mut binary = (opcode & 0b1111111)
        | (rs1 & 0b11111) << 15
        | (rs2 & 0b11111) << 20
        | (funct_3 & 0b111) << 12;

    binary |= ((imm >> 12) & 0b1) << 31;
    binary |= ((imm >> 5) & 0b111111) << 25;
    binary |= ((imm >> 11) & 0b1) << 7;
    binary |= ((imm >> 1) & 0b1111) << 8;
    int_to_4_byte_vec(binary)
}

fn generate_bytes_j_type(tokens: &Vec<String>) -> Vec<u8> {
    let instruction = tokens[0].to_lowercase();
    let opcode = int(OPCODE_BITS[&instruction], 2);
    let rd = int(REGISTER_BITS[&tokens[1]], 2);
    // TODO deal with none base 10 cases
    let imm = int(&tokens[2], 10);

    let mut binary = ((rd & 0b11111) << 7) | (opcode & 0b1111111);

    binary |= (((imm >> 12) & 0b11111111) << 12)
        | (((imm >> 11) & 0b1) << 20)
        | (((imm >> 1) & 0b1111111111) << 21)
        | (((imm >> 20) & 0b1) << 31);

    int_to_4_byte_vec(binary)
}

fn remove_labels_from_tokens(tokens: &mut Vec<String>, labels: &HashMap<String,i64>) {
    for item in tokens.iter_mut() {
        // println!("{}",item);
        if let Some(replacement) = labels.get(item) {
            // println!("{}",item);
            *item = (*replacement).to_string();
        }
    }
}

lazy_static! {
    static ref INSTRUCTION_METHOD_MAP: HashMap<String, fn(&Vec<String>) -> Vec<u8>> = {
        let mut map: HashMap<String, fn(&Vec<String>) -> Vec<u8>> = HashMap::new();
        let pairs: [(&Vec<&str>, fn (&Vec<String>) -> Vec<u8>); 6] = [
            (&Vec::from(R_TYPE_INSTRUCTIONS), generate_bytes_r_type),
            (&Vec::from(I_TYPE_INSTRUCTIONS), generate_bytes_i_type),
            (&Vec::from(S_TYPE_INSTRUCTIONS), generate_bytes_s_type),
            (&Vec::from(U_TYPE_INSTRUCTIONS), generate_bytes_u_type),
            (&Vec::from(B_TYPE_INSTRUCTIONS), generate_bytes_b_type),
            (&Vec::from(J_TYPE_INSTRUCTIONS), generate_bytes_j_type),
        ];
        for (instruction_list, method) in pairs {
            for opcode in instruction_list {
                let opcode = *opcode;
                map.insert((*opcode).to_string(), method);
            }
        }
        map
    };
}

fn generate_bytes_line(line: Line, labels: &HashMap<String,i64>) -> Vec<u8> {
    // TODO
    // Create map of opcode to function
    // Create function for each type of instruction
    //
    match line {
        Line::ASTInstruction(mut ast_instruction) => {
            println!("{:?}",(*ast_instruction).tokens);
            let instruction: String = (*ast_instruction).tokens[0].to_string();
            if instruction.to_lowercase() == "ecall" {
                return int_to_4_byte_vec(int("00000073",16));
            }
            let method = INSTRUCTION_METHOD_MAP[&instruction];
            remove_labels_from_tokens(&mut (*ast_instruction).tokens, labels);
            return method(&(*ast_instruction).tokens);
        }

        Line::ASTWord(word) => {
            // TODO
        }

        Line::ASTLabel(_) => {}
    }
    vec![]
}

fn pseudo_parse_line(line: Line) -> Vec<Box<Line>> {
    // println!("{:?}",line);
    match line {
        Line::ASTInstruction(ast_instruction) => match ast_instruction.tokens[0].as_str() {
            "li" => {
                let rd = ast_instruction.tokens[1].clone();
                let imm = ast_instruction.tokens[2].parse::<i32>().unwrap();
                if (imm < 2048) && (imm >= -2048) {
                    return vec![Box::new(Line::ASTInstruction(Box::new(
                        ASTInstruction::new(vec![
                            "addi".to_string(),
                            rd,
                            "x0".to_string(),
                            imm.to_string(),
                        ]),
                    )))];
                } else {
                    let imm_hi = imm >> 12;
                    let imm_lo = imm & 0xFF;
                    return vec![
                        Box::new(Line::ASTInstruction(Box::new(ASTInstruction::new(vec![
                            "lui".to_string(),
                            rd.clone(),
                            imm_hi.to_string(),
                        ])))),
                        Box::new(Line::ASTInstruction(Box::new(ASTInstruction::new(vec![
                            "addi".to_string(),
                            rd.clone(),
                            rd.clone(),
                            imm_lo.to_string(),
                        ])))),
                    ];
                }
                vec![]
            }

            _ => vec![Box::new(Line::ASTInstruction(ast_instruction))],
        },

        Line::ASTLabel(ast_label) => {
            let mut ast_label = ast_label;
            let mut sub_lines = pseudo_parse_line(*ast_label.labelled_line);
            // println!("{:?}", sub_lines);
            ast_label.labelled_line = Box::new((*sub_lines[0]).clone());
            sub_lines.remove(0);
            sub_lines.insert(0, Box::new(Line::ASTLabel(ast_label)));
            sub_lines
        }

        Line::ASTWord(ast_word) => {
            vec![Box::new(Line::ASTWord(ast_word))]
        }
    }
}

fn pseudo_parse_lines(lines: LineList) -> LineList {
    let mut new_lines: Vec<Box<Line>> = vec![];

    for line in lines.lines {
        new_lines.append(&mut pseudo_parse_line(*line))
    }

    LineList::new(new_lines)
}

fn pre_process(program_text: &str) -> String {
    let lines: Vec<&str> = program_text
        .lines()
        .map(|line| line.split('#').next().unwrap_or(""))
        .collect();

    // Reconstruct the string
    let program_text: String = lines.join("\n");

    let mut text = program_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n");
    text = text + "\n";
    text = text.replace("\n", ";");
    text = text.replace(",", " ");
    let text = String::from("{") + &text + "}";
    text
}

pub fn assemble(program_text: &String) -> Vec<u8> {
    // TODO better passing of li instructions to support labels

    let processed_program_text = pre_process(&program_text);

    let program_ast: LineList = asm::ProgramParser::new()
        .parse(&processed_program_text)
        .unwrap();

    let pre_processed_ast = pseudo_parse_lines(program_ast);

    let mut labels: HashMap<String, i64> = HashMap::new();
    let mut cleaned_lines: Vec<Box<Line>> = vec![];
    for (index, line) in pre_processed_ast.lines.iter().enumerate() {
        let mut line = *(*line).clone();
        while let Line::ASTLabel(label) = line {
            labels.insert((*label.label).to_string(), (index * 4) as i64);
            line = *label.labelled_line;
        }
        cleaned_lines.push(Box::new(line));
    }
    let cleaned_lines = LineList::new(cleaned_lines);
    // println!("{:?}", &cleaned_lines);
    // println!("{:?}", &labels);
    let mut binary: Vec<u8> = vec![];
    for line in cleaned_lines.lines{
        binary.append(&mut generate_bytes_line(*line, &labels));
    }
    print!("{:?}", &binary);
    binary
}
