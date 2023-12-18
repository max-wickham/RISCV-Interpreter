use std::fs::File;
use std::io::{self, Read};

mod abi;
type SizeInt = u32;
type RegisterValue = SizeInt;

const NUM_REGISTERS: usize = 32;
const MEM_SIZE_WORDS: usize = 1024;

fn mask_generate(value: f32) -> u32{
    match value {
        // negative infinity
        x if x.is_infinite() && x.is_sign_negative() => return 0x00_00_00_00,
        // negative normal number
        x if x.is_normal() && x.is_sign_negative() => return 0x80_00_00_00,
        // negative sub normal number
        x if x.is_subnormal() && x.is_sign_negative() => return 0x80_00_00_00,
        // negative zero
        x if (x == 0.0) && x.is_sign_negative() => return 0x80_00_00_00,
        // positive 0
        x if (x == 0.0) && x.is_sign_positive() => return 0x00_00_00_00,
        // positive sub normal number
        x if x.is_subnormal() && x.is_sign_positive() => return 0x00_00_00_00,
        // positive infinity
        x if x.is_infinite() && x.is_sign_positive() => return 0x00_00_00_00,
        // signaling NaN
        x if x.is_nan() && x.is_sign_positive() => return 0x00_00_00_00,
        // quiet NaN
        x if x.is_nan() && x.is_sign_positive() => return 0x00_00_00_00,
        _ => panic!("Unknown value: {}", value)
    }
}

#[derive()]
pub struct CPUState {
    registers: [RegisterValue; NUM_REGISTERS],
    floating_point_registers: [f32; NUM_REGISTERS],
    memory: [u32; MEM_SIZE_WORDS],
    pc : u32
}


impl CPUState {
    pub fn new() -> Self {
        CPUState {
            registers: [0; NUM_REGISTERS],
            floating_point_registers: [0.0; NUM_REGISTERS],
            memory: [0; MEM_SIZE_WORDS],
            pc: 0
        }
    }
}

pub fn decode_instruction(instruction: u32, cpu_state: &mut CPUState) -> bool {
    let mut regs = cpu_state.registers;
    let mut fp_regs = cpu_state.floating_point_registers;
    let pc = &mut cpu_state.pc;
    let mut memory = cpu_state.memory;


    let opcode = instruction & 0b00000000000000000000000001111111;
    let rd = (instruction >> 7) & 0b00000000000000000000000000011111;
    let rs1 = (instruction >> 15) & 0b00000000000000000000000000011111;
    let rs2 = (instruction >> 20) & 0b00000000000000000000000000011111;

    let funct7 = (instruction >> 25) & 0b00000000000000000000000001111111;
    let funct3 = (instruction >> 12) & 0b00000000000000000000000000001111;

    let imm_i = (instruction as u32) >> 20;
    let imm_l = (instruction as u32) >> 20;
    let imm_u = (instruction as i32) >> 12;
    let imm_s = ((instruction >> 25) << 11) | ((instruction >> 7) & 0b11111);
    let imm_b = ((instruction >> 31) << 12) | ((instruction >> 25) & 0b01111110)
        | ((instruction >> 8) & 0b00000001) | ((instruction >> 7) & 0b00001110);

    let imm_uj = ((instruction >> 31) << 20) | ((instruction >> 20) & 0b01111111)
        | ((instruction >> 21) & 0b00000001) | ((instruction >> 12) & 0b00001111);

    match opcode {
        // Arithmetic Int Instructions
        0b0110011 =>  {
            let funct7 = (instruction >> 25) & 0b00000000000000000000000001111111;
            let funct3 = (instruction >> 12) & 0b00000000000000000000000000001111;
            match funct7 {
                0b0000000 => match funct3 {
                    0b111 => regs[rd as usize] = regs[rs1 as usize] & regs[rs2 as usize], // AND
                    0b110 => regs[rd as usize] = regs[rs1 as usize] | regs[rs2 as usize], // OR
                    0b100 => regs[rd as usize] = regs[rs1 as usize] ^ regs[rs2 as usize], // XOR
                    0b011 => regs[rd as usize] = if regs[rs1 as usize] < regs[rs2 as usize] {1} else {0}, // SLTU
                    0b010 => regs[rd as usize] = {
                        if (regs[rs1 as usize] as i32) < (regs[rs2 as usize] as i32) {1} else {0} // SLT
                    },
                    0b001 => regs[rd as usize] = regs[rs1 as usize] << regs[rs2 as usize], // SLL
                    0b101 => regs[rd as usize] = regs[rs1 as usize] >> regs[rs2 as usize], // SRL
                    0b000 => regs[rd as usize] = regs[rs1 as usize] + regs[rs2 as usize], // ADD
                    _ => panic!("Unknown funct3: {}", funct3)
                },
                0b0100000 => match funct3 {
                    0b000 => regs[rd as usize] = regs[rs1 as usize] - regs[rs2 as usize], // SUB
                    0b101 => regs[rd as usize] = regs[rs1 as usize] >> regs[rs2 as usize], // SRA
                    _ => panic!("Unknown funct3: {}", funct3)
                },
                0b0000001 => match funct3 {
                    0b000 => regs[rd as usize] = regs[rs1 as usize] * regs[rs2 as usize], // MUL
                    0b001 => {
                        let result = (regs[rs1 as usize] as i32 as i64) * (regs[rs2 as usize] as i32 as i64);
                        let upper_result = (result >> 32) as u32;
                        regs[rd as usize] = upper_result;
                    }, // MULH
                    0b010 => {
                        let result = (regs[rs1 as usize] as i32 as i64) * (regs[rs2 as usize] as u32 as i64);
                        let upper_result = (result >> 32) as u32;
                        regs[rd as usize] = upper_result;
                    }, // MULHSU
                    0b011 => {
                        let result = (regs[rs1 as usize] as u32 as u64) * (regs[rs2 as usize] as u32 as u64);
                        let upper_result = (result >> 32) as u32;
                        regs[rd as usize] = upper_result;
                    }, // MULHU
                    0b100 => regs[rd as usize] = ((regs[rs1 as usize] as i32) / (regs[rs2 as usize] as i32)) as u32, // DIV
                    0b101 => regs[rd as usize] = regs[rs1 as usize] / regs[rs2 as usize], // DIVU
                    0b110 => regs[rd as usize] = ((regs[rs1 as usize] as i32) % (regs[rs2 as usize] as i32)) as u32, // REM
                    0b111 => regs[rd as usize] = regs[rs1 as usize] % regs[rs2 as usize], // REMU
                    _ => panic!("Unknown funct3: {}", funct3)
                }
                _ => panic!("Unknown funct7: {}", funct7)
            }
        },
        0b0010011 => {

            let funct3: u32 = (instruction >> 12) & 0b00000000000000000000000000001111;
            match funct3 {
                001 => regs[rd as usize] = regs[rs1 as usize] << rs2, // SLLI
                101 => match funct7 {
                    0b0000000 => regs[rd as usize] = regs[rs1 as usize] >> rs2, // SRLI
                    0b0100000 => regs[rd as usize] = ((regs[rs1 as usize] as i32) >> rs2) as u32, // SRAI
                    _ => panic!("Unknown funct3: {}", funct3)
                },
                0b111 => regs[rd as usize] = regs[rs1 as usize] & imm_i, // ANDI
                0b110 => regs[rd as usize] = regs[rs1 as usize] | imm_i, // ORI
                0b100 => regs[rd as usize] = regs[rs1 as usize] ^ imm_i, // XORI
                0b011 => regs[rd as usize] = if regs[rs1 as usize] < imm_i {1} else {0}, // SLTIU
                0b010 => regs[rd as usize] = {
                    if (regs[rs1 as usize] as i32) < (imm_i as i32) {1} else {0} // SLTI
                },
                0b000 => regs[rd as usize] = regs[rs1 as usize] + imm_i, // ADDI
                _ => panic!("Unknown funct3: {}", funct3)
            }

        },
        // Load instructions
        0b0100011 => {
            match funct3 {
                // TODO test sign extension of register value
                0b010 =>  memory[((regs[rs1 as usize] as i32) + (imm_s as i32)) as usize] = regs[rd as usize], // SW
                0b001 =>  {
                    memory[((regs[rs1 as usize] as i32) + (imm_s as i32)) as usize] =
                        (regs[rd as usize]) as u32 & 0x0000FFFF +
                        memory[((regs[rs1 as usize] as i32) + (imm_s as i32)) as usize] & 0xFFFF0000;
                }, // SH
                0b000 =>  {
                    memory[((regs[rs1 as usize] as i32) + (imm_s as i32)) as usize] =
                        (regs[rd as usize]) as u32 & 0x000000FF +
                        memory[((regs[rs1 as usize] as i32) + (imm_s as i32)) as usize] & 0xFFFFFF00;
                }, // SB
                _ => panic!("Unknown funct3: {}", funct3)
            }
        },
        // Store Instructions
        0b0000011 => {
            match funct3 {
                0b010 => regs[rd as usize] = memory[((regs[rs1 as usize] as i32) + (imm_l as i32)) as usize], // LW
                0b001 => {
                    let val = memory[((regs[rs1 as usize] as i32) + (imm_s as i32)) as usize] & 0x0000FFFF;
                    regs[rd as usize] = val + if (val & 0x00008000) == 0x00008000 {0xFFFF0000} else {0x00000000};
                }, // LH
                0b000 => {
                    let val = memory[((regs[rs1 as usize] as i32) + (imm_s as i32)) as usize] & 0x000000FF;
                    regs[rd as usize] = val + if (val & 0x00000080) == 0x00000080 {0xFFFFFF00} else {0x00000000};
                }, // LB
                0b101 => regs[rd as usize] = memory[((regs[rs1 as usize] as i32) + (imm_l as i32)) as usize] & 0x0000FFFF, // LHU
                0b100 => regs[rd as usize] = memory[((regs[rs1 as usize] as i32) + (imm_l as i32)) as usize] & 0x000000FF, // LBU
                _ => panic!("Unknown funct3: {}", funct3)
            }
        },
        // Branch Instructions
        0b1100011 => {
            match funct3 {
                0b000 => {
                    if regs[rs1 as usize] == regs[rs2 as usize] {
                        *pc = (*pc as i32 + ((imm_b as i32) << 1)) as u32;
                    }
                }, // BEQ
                0b001 => {
                    if regs[rs1 as usize] != regs[rs2 as usize] {
                        *pc = (*pc as i32 + ((imm_b as i32) << 1)) as u32;
                    }
                }, // BNE
                0b100 => {
                    if (regs[rs1 as usize] as i32) < (regs[rs2 as usize] as i32){
                        *pc = (*pc as i32 + ((imm_b as i32) << 1)) as u32;
                    }
                }, // BLT
                0b101 => {
                    if (regs[rs1 as usize] as i32) >= (regs[rs2 as usize] as i32){
                        *pc = (*pc as i32 + ((imm_b as i32) << 1)) as u32;
                    }
                }, // BGE
                0b110 => {
                    if regs[rs1 as usize] < regs[rs2 as usize] {
                        *pc = (*pc as i32 + ((imm_b as i32) << 1)) as u32;
                    }
                }, // BLTU
                0b111 => {
                    if regs[rs1 as usize] >= regs[rs2 as usize] {
                        *pc = (*pc as i32 + ((imm_b as i32) << 1)) as u32;
                    }
                }, // BGEU
                _ => panic!("Unknown funct3: {}", funct3)
            }
        },
        // Jump and Link Instructions
        0b1100111 => {
            let jump = ((regs[rs1 as usize] as i32 + imm_i as i32) as u32) & 0xFFFFFFFE;
            regs[rd as usize] = (*pc + 4) as u32;
            *pc = jump as u32;
        }, // JALR
        0b1101111 => {
            regs[rd as usize] = (*pc + 4) as u32;
            *pc = (*pc as i32 + (imm_uj as i32) << 1) as u32;
        }, // JAL

        // Load or Add Immediate Instructions
        0b0110111 => regs[rd as usize] = imm_u as u32, // LUI
        0b0010111 => regs[rd as usize] = (imm_u << 12) as u32 + (*pc as u32), // AUIPC

        // Fence Instructions
        0b0001111 => {}, // FENCE

        // ECall Instructions
        0b1110011 => {
            match funct3 {
                0b000 => {
                    match imm_i {
                        0 => {
                            return true;
                        }, // ECALL
                        1 => {}, // EBREAK
                        _ => panic!("Unknown imm_i: {}", imm_i)
                    }
                },
                0b001 => {}, // CSRRW
                0b010 => {}, // CSRRS
                0b011 => {}, // CSRRC
                0b101 => {}, // CSRRWI
                0b110 => {}, // CSRRSI
                0b111 => {}, // CSRRCI
                _ => panic!("Unknown funct3: {}", funct3)
            }
        },

        // FLoating Point Instructions
        0b0000111 =>  {
            match funct3 {
                0b010 => fp_regs[rd as usize] = (memory[((regs[rs1 as usize] as i32) + (imm_l as i32)) as usize]) as f32, // FLW
                0b011 => fp_regs[rd as usize] = (memory[((regs[rs1 as usize] as i32) + (imm_l as i32)) as usize]) as f32, // FLW
                _ => panic!("Unknown funct3: {}", funct3)
            }
            fp_regs[rd as usize] = (memory[((regs[rs1 as usize] as i32) + (imm_l as i32)) as usize]) as f32
        }, // FLW
        0b0100111 =>  memory[((regs[rs1 as usize] as i32) + (imm_s as i32)) as usize] = fp_regs[rs2 as usize] as u32, // FSW
        0b1010011 => {
            match funct7 {
                    0b0000000 => fp_regs[rd as usize] = fp_regs[rs1 as usize] + fp_regs[rs2 as usize], // FADD.S
                    0b0000100 => fp_regs[rd as usize] = fp_regs[rs1 as usize] - fp_regs[rs2 as usize], // FSUB.S
                    0b0001000 => fp_regs[rd as usize] = fp_regs[rs1 as usize] * fp_regs[rs2 as usize], // FMUL.S
                    0b0001100 => fp_regs[rd as usize] = fp_regs[rs1 as usize] / fp_regs[rs2 as usize], // FDIV.S
                    0b0101100 => fp_regs[rd as usize] = (fp_regs[rs1 as usize]).sqrt(), // FSQRT.S
                    0b0010000 => match funct3 {
                        0b000 => fp_regs[rd as usize] = fp_regs[rs1 as usize].abs() * fp_regs[rs2 as usize].signum(), // FSGNJ.S
                        0b001 => fp_regs[rd as usize] = fp_regs[rs1 as usize].abs() * fp_regs[rs2 as usize].signum() * -1.0, // FSGNJN.S
                        0b010 => fp_regs[rd as usize] = fp_regs[rs1 as usize] * fp_regs[rs2 as usize].signum(), // FSGNJX.S
                        _ => panic!("Unknown funct3: {}", funct3)
                    },
                    0b0010100 => match funct3 {
                        0b000 => fp_regs[rd as usize] = if fp_regs[rs1 as usize] < fp_regs[rs2 as usize] {fp_regs[rs1 as usize]} else {fp_regs[rs2 as usize]}, // FMIN.S
                        0b001 => fp_regs[rd as usize] = if fp_regs[rs1 as usize] > fp_regs[rs2 as usize] {fp_regs[rs1 as usize]} else {fp_regs[rs2 as usize]}, // FMAX.S
                        _ => panic!("Unknown funct3: {}", funct3)
                    }
                    0b1100000 => match rs2 as u32 {
                        0b00000 => regs[rd as usize] = fp_regs[rs1 as usize] as i32 as u32, // FCVT.W.S
                        0b00001 => regs[rd as usize] = fp_regs[rs1 as usize] as u32, // FCVT.WU.S
                        _ => panic!("Unknown rs2: {}", rs2)
                    },
                    0b1110000 => match funct3 {
                        0b000 => regs[rd as usize] = fp_regs[rs1 as usize] as u32, // FMV.X.W
                        0b001 => regs[rd as usize] = mask_generate(fp_regs[rs1 as usize]), // FCLASS.S
                        _ => panic!("Unknown funct3: {}", funct3)
                    },
                    0b1010000 => match funct3 {
                        0b000 => regs[rd as usize] = if fp_regs[rs1 as usize] <=  fp_regs[rs2 as usize] {1} else {0}, // FLE.S
                        0b001 => regs[rd as usize] = if fp_regs[rs1 as usize] <  fp_regs[rs2 as usize] {1} else {0}, // FLT.S
                        0b010 => regs[rd as usize] = if fp_regs[rs1 as usize] ==  fp_regs[rs2 as usize] {1} else {0}, // FEQ.S
                        _ => panic!("Unknown funct3: {}", funct3)
                    },
                    0b1101000 => match rs2 as u32 {
                        0b00000 => regs[rd as usize] = fp_regs[rs1 as usize] as i32 as u32, // FCVT.S.W
                        0b00001 => regs[rd as usize] = fp_regs[rs1 as usize] as u32, // FCVT.S.WU
                        _ => panic!("Unknown rs2: {}", rs2)
                    },
                    0b1111000 => match funct3 {
                        0b000 => fp_regs[rd as usize] = regs[rs1 as usize] as f32, // FMV.W.X
                        _ => panic!("Unknown funct3: {}", funct3)
                    },

                    _ => panic!("Unknown funct7: {}", funct7)
            }
        },
        _ => {
            panic!("Unknown opcode: {}", opcode);
        }
    }
    return false;
}

fn interpret(file_name : &str) -> io::Result<CPUState> {
    let mut cpu_state = CPUState::new();
    let mut file = File::open(file_name).expect("File not found");
    let file_size = file.metadata()?.len() as usize;
    let mut buffer = Vec::with_capacity(file_size);
    file.read_to_end(&mut buffer)?;
    let num_u32_elements = file_size / std::mem::size_of::<u32>();
    let mut u32_array = vec![0; num_u32_elements];
    // Copy the binary data into the u32 array
    let u32_buffer = u32_array.as_mut_slice();
    u32_buffer.copy_from_slice(bytemuck::cast_slice(&buffer));

    loop {
        let instruction = cpu_state.memory[cpu_state.pc as usize];
        let ecall = decode_instruction(instruction, &mut cpu_state);
        cpu_state.pc += 4;
        if ecall {
            if cpu_state.registers[abi::a7] == 10 {
                break;
            }
        }
    }

    return Ok(cpu_state);
}


fn main() {
    let nan: f64 = f64::NAN;
    let signaling_nan: f64 = f64::from_bits(0x7FF0000000000001); // Signaling NaN bit pattern

    if nan.is_nan() {
        println!("nan is a NaN");
    }

    if signaling_nan.is_nan() {
        println!("signaling_nan is a NaN");
    }
}
