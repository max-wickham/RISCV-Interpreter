use std::fs::File;
use std::io::{self, Read};
use std::ops::{Index, IndexMut};

use hardware::GPIOState;

mod abi;
mod hardware;
pub mod assembler;
mod riscv_spec;
type SizeInt = u32;
type RegisterValue = SizeInt;

const NUM_REGISTERS: usize = 32;
const MEM_SIZE_WORDS: usize = 1024;
const NUM_GPIOS: usize = 31;

fn mask_generate(value: f32) -> u32 {
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
        _ => panic!("Unknown value: {}", value),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CPUMem {
    e: [u32; MEM_SIZE_WORDS],
}

impl Index<usize> for CPUMem {
    type Output = u32;
    fn index<'a>(&'a self, i: usize) -> &'a u32 {
        if i % 4 == 0 {
            &self.e[i / 4 as usize]
        } else {
            // TODO fix
            &self.e[i / 4 as usize]
        }
    }
}

impl IndexMut<usize> for CPUMem {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut u32 {
        &mut self.e[i / 4]
    }
}

impl CPUMem {
    fn new() -> Self {
        CPUMem {
            e: [0; MEM_SIZE_WORDS],
        }
    }
}

#[derive(Clone)]
pub struct CPUState {
    pub registers: [RegisterValue; NUM_REGISTERS],
    pub floating_point_registers: [f32; NUM_REGISTERS],
    // pub memory: CPUMem,
    pub memory_bytes: [u8; MEM_SIZE_WORDS * 4],
    pub pc: u32,
    pub gpio_states: [GPIOState; NUM_GPIOS],
}

impl CPUState {
    pub fn new() -> Self {
        CPUState {
            registers: [0; NUM_REGISTERS],
            floating_point_registers: [0.0; NUM_REGISTERS],
            // memory: CPUMem::new(),
            memory_bytes: [0; MEM_SIZE_WORDS * 4],
            pc: 0,
            gpio_states: [GPIOState::new(); NUM_GPIOS],
        }
    }

    pub fn set_mem(&mut self, address: usize, value: u32) {
        self.memory_bytes[address as usize] = (value & 0xFF) as u8;
        self.memory_bytes[(address + 1) as usize] = ((value >> 8) & 0xFF) as u8;
        self.memory_bytes[(address + 2) as usize] = ((value >> 16) & 0xFF) as u8;
        self.memory_bytes[(address + 3) as usize] = ((value >> 24) & 0xFF) as u8;
    }

    pub fn read_mem(&mut self, address: usize) -> u32 {
        let mut value = 0;
        value += (self.memory_bytes[address as usize] as u32) << 0;
        value += (self.memory_bytes[(address + 1) as usize] as u32) << 8;
        value += (self.memory_bytes[(address + 2) as usize] as u32) << 16;
        value += (self.memory_bytes[(address + 3) as usize] as u32) << 24;
        return value;
    }
}

pub fn decode_instruction(cpu_state: &mut CPUState) -> bool {
    let instruction = cpu_state.read_mem(cpu_state.pc as usize);
    println!("instruction: {:#34b}", instruction);
    // let mut cpu_state.registers = cpu_state.registers;
    // let mut cpu_state.floating_point_registers = cpu_state.floating_point_registers;
    let pc = &mut cpu_state.pc;
    // let mut memory = cpu_state.memory;

    let opcode = instruction & 0b00000000000000000000000001111111;
    let rd = (instruction >> 7) & 0b00000000000000000000000000011111;
    let rs1 = (instruction >> 15) & 0b00000000000000000000000000011111;
    let rs2 = (instruction >> 20) & 0b00000000000000000000000000011111;

    let funct7 = (instruction >> 25) & 0b1111111;
    let funct3 = (instruction >> 12) & 0b111;

    let imm_i = (instruction as u32) >> 20;
    let imm_l = (instruction as u32) >> 20;
    let imm_u = (instruction as i32) >> 12;
    let imm_s = ((instruction >> 25) << 11) | ((instruction >> 7) & 0b11111);
    let imm_b = ((((instruction >> 31) & 1) << 12)
        | (((instruction >> 25) & 0b111111) << 5)
        | (((instruction >> 8) & 0b1111) << 1)
        | ((instruction >> 7) & 0b1) << 11)
        & 0b11111111111111111111111111111110;

    let imm_j = (((instruction >> 31) << 20)
        | (((instruction >> 21) & 0b0111111111) << 1)
        | (((instruction >> 19) & 0b1) << 11)
        | (((instruction >> 12) & 0b11111111) << 12))
        & 0b11111111111111111111111111111110;

    println!("instruction: {:b}", instruction);
    println!("opcode: {:b}", opcode);
    println!("rd: {:b}", rd);
    println!("funct3: {:b}", funct3);
    println!("funct7: {:b}", funct7);
    match opcode {
        ///////////////////////////////////////////// RV32I Base Instruction Set /////////////////////////////////////////////
        ///////////////////////////////////////////// RV32M Standard Extension //////////////////////////////////////////////
        // Arithmetic Int Instructions
        0b0110011 => {
            println!("in 0110011");
            match funct7 {
                0b0000000 => match funct3 {
                    0b111 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] & cpu_state.registers[rs2 as usize]
                    } // AND
                    0b110 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] | cpu_state.registers[rs2 as usize]
                    } // OR
                    0b100 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] ^ cpu_state.registers[rs2 as usize]
                    } // XOR
                    0b011 => {
                        println!("rs2 val: {}", cpu_state.registers[rs2 as usize]);
                        cpu_state.registers[rd as usize] = if cpu_state.registers[rs1 as usize]
                            < cpu_state.registers[rs2 as usize]
                        {
                            1
                        } else {
                            0
                        };
                    } // SLTU
                    0b010 => {
                        cpu_state.registers[rd as usize] = {
                            if (cpu_state.registers[rs1 as usize] as i32)
                                < (cpu_state.registers[rs2 as usize] as i32)
                            {
                                1
                            } else {
                                0
                            } // SLT
                        }
                    }
                    0b001 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] << cpu_state.registers[rs2 as usize]
                    } // SLL
                    0b101 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] >> cpu_state.registers[rs2 as usize]
                    } // SRL
                    0b000 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] + cpu_state.registers[rs2 as usize]
                    } // ADD
                    _ => panic!("Unknown funct3: {}", funct3),
                },
                0b0100000 => match funct3 {
                    0b000 => {
                        println!("rd: {}", rd);
                        println!("rs1: {}", rd);
                        println!("rs2: {}", rd);
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] - cpu_state.registers[rs2 as usize];
                        // SUB
                    }
                    0b101 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] >> cpu_state.registers[rs2 as usize]
                    } // SRA
                    _ => panic!("Unknown funct3: {}", funct3),
                },
                0b0000001 => match funct3 {
                    0b000 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] * cpu_state.registers[rs2 as usize]
                    } // MUL
                    0b001 => {
                        let result = (cpu_state.registers[rs1 as usize] as i32 as i64)
                            * (cpu_state.registers[rs2 as usize] as i32 as i64);
                        let upper_result = (result >> 32) as u32;
                        cpu_state.registers[rd as usize] = upper_result;
                    } // MULH
                    0b010 => {
                        let result = (cpu_state.registers[rs1 as usize] as i32 as i64)
                            * (cpu_state.registers[rs2 as usize] as u32 as i64);
                        let upper_result = (result >> 32) as u32;
                        cpu_state.registers[rd as usize] = upper_result;
                    } // MULHSU
                    0b011 => {
                        let result = (cpu_state.registers[rs1 as usize] as u32 as u64)
                            * (cpu_state.registers[rs2 as usize] as u32 as u64);
                        let upper_result = (result >> 32) as u32;
                        cpu_state.registers[rd as usize] = upper_result;
                    } // MULHU
                    0b100 => {
                        cpu_state.registers[rd as usize] = ((cpu_state.registers[rs1 as usize]
                            as i32)
                            / (cpu_state.registers[rs2 as usize] as i32))
                            as u32
                    } // DIV
                    0b101 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] / cpu_state.registers[rs2 as usize]
                    } // DIVU
                    0b110 => {
                        cpu_state.registers[rd as usize] = ((cpu_state.registers[rs1 as usize]
                            as i32)
                            % (cpu_state.registers[rs2 as usize] as i32))
                            as u32
                    } // REM
                    0b111 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] % cpu_state.registers[rs2 as usize]
                    } // REMU
                    _ => panic!("Unknown funct3: {}", funct3),
                },
                _ => panic!("Unknown funct7: {}", funct7),
            }
        }
        0b0010011 => {
            let funct3: u32 = (instruction >> 12) & 0b00000000000000000000000000001111;
            match funct3 {
                001 => cpu_state.registers[rd as usize] = cpu_state.registers[rs1 as usize] << rs2, // SLLI
                101 => match funct7 {
                    0b0000000 => {
                        cpu_state.registers[rd as usize] = cpu_state.registers[rs1 as usize] >> rs2
                    } // SRLI
                    0b0100000 => {
                        cpu_state.registers[rd as usize] =
                            ((cpu_state.registers[rs1 as usize] as i32) >> rs2) as u32
                    } // SRAI
                    _ => panic!("Unknown funct3: {}", funct3),
                },
                0b111 => {
                    cpu_state.registers[rd as usize] = cpu_state.registers[rs1 as usize] & imm_i
                } // ANDI
                0b110 => {
                    cpu_state.registers[rd as usize] = cpu_state.registers[rs1 as usize] | imm_i
                } // ORI
                0b100 => {
                    cpu_state.registers[rd as usize] = cpu_state.registers[rs1 as usize] ^ imm_i
                } // XORI
                0b011 => {
                    cpu_state.registers[rd as usize] = if cpu_state.registers[rs1 as usize] < imm_i
                    {
                        1
                    } else {
                        0
                    }
                } // SLTIU
                0b010 => {
                    cpu_state.registers[rd as usize] = {
                        if (cpu_state.registers[rs1 as usize] as i32) < (imm_i as i32) {
                            1
                        } else {
                            0
                        } // SLTI
                    }
                }
                0b000 => {
                    println!("rd: {}", rd);
                    println!("rs1: {}", rs1);
                    println!("imm_i: {}", imm_i);
                    println!("result: {}", cpu_state.registers[rd as usize]);
                    cpu_state.registers[rd as usize] = cpu_state.registers[rs1 as usize] + imm_i; // ADDI
                    println!("result: {}", cpu_state.registers[rd as usize]);
                }
                _ => panic!("Unknown funct3: {}", funct3),
            }
        }
        // Store instructions
        0b0100011 => {
            match funct3 {
                // TODO test sign extension of register value
                0b010 => cpu_state.set_mem(
                    ((cpu_state.registers[rs1 as usize] as i32) + (imm_s as i32)) as usize,
                    cpu_state.registers[rd as usize],
                ), // SW
                0b001 => {
                    let value = (cpu_state.registers[rd as usize]) as u32
                        & 0x0000FFFF
                            + cpu_state.read_mem(
                                ((cpu_state.registers[rs1 as usize] as i32) + (imm_s as i32))
                                    as usize,
                            )
                        & 0xFFFF0000;
                    cpu_state.set_mem(
                        ((cpu_state.registers[rs1 as usize] as i32) + (imm_s as i32)) as usize,
                        value,
                    );
                } // SH
                0b000 => {
                    let value = (cpu_state.registers[rd as usize]) as u32
                        & 0x000000FF
                            + cpu_state.read_mem(
                                ((cpu_state.registers[rs1 as usize] as i32) + (imm_s as i32))
                                    as usize,
                            )
                        & 0xFFFFFF00;
                    cpu_state.set_mem(
                        ((cpu_state.registers[rs1 as usize] as i32) + (imm_s as i32)) as usize,
                        value,
                    );
                } // SB
                _ => panic!("Unknown funct3: {}", funct3),
            }
        }
        // Load Instructions
        0b0000011 => {
            match funct3 {
                0b010 => {
                    println!(
                        "lw: {}",
                        ((cpu_state.registers[rs1 as usize] as i32) + (imm_l as i32)) as usize
                    );
                    cpu_state.registers[rd as usize] = cpu_state.read_mem(
                        ((cpu_state.registers[rs1 as usize] as i32) + (imm_l as i32)) as usize,
                    );
                } // LW
                0b001 => {
                    let val = cpu_state.read_mem(
                        ((cpu_state.registers[rs1 as usize] as i32) + (imm_s as i32)) as usize,
                    ) & 0x0000FFFF;
                    cpu_state.registers[rd as usize] = val
                        + if (val & 0x00008000) == 0x00008000 {
                            0xFFFF0000
                        } else {
                            0x00000000
                        };
                } // LH
                0b000 => {
                    let val = cpu_state.read_mem(
                        ((cpu_state.registers[rs1 as usize] as i32) + (imm_s as i32)) as usize,
                    ) & 0x000000FF;
                    cpu_state.registers[rd as usize] = val
                        + if (val & 0x00000080) == 0x00000080 {
                            0xFFFFFF00
                        } else {
                            0x00000000
                        };
                } // LB
                0b101 => {
                    cpu_state.registers[rd as usize] = cpu_state.read_mem(
                        ((cpu_state.registers[rs1 as usize] as i32) + (imm_l as i32)) as usize,
                    ) & 0x0000FFFF
                } // LHU
                0b100 => {
                    cpu_state.registers[rd as usize] = cpu_state.read_mem(
                        ((cpu_state.registers[rs1 as usize] as i32) + (imm_l as i32)) as usize,
                    ) & 0x000000FF
                } // LBU
                _ => panic!("Unknown funct3: {}", funct3),
            }
        }
        // Branch Instructions
        0b1100011 => {
            match funct3 {
                0b000 => {
                    println!("imm_b: {}", imm_b);
                    println!("pc: {}", *pc);
                    if cpu_state.registers[rs1 as usize] == cpu_state.registers[rs2 as usize] {
                        *pc = (*pc as i32 + ((imm_b as i32) << 0)) as u32;
                        println!("pc: {}", *pc);
                        return false;
                    }
                } // BEQ
                0b001 => {
                    println!("imm_b: {}", imm_b);
                    println!("pc: {}", *pc);
                    if cpu_state.registers[rs1 as usize] != cpu_state.registers[rs2 as usize] {
                        *pc = (*pc as i32 + ((imm_b as i32) << 0)) as u32;
                        println!("pc: {}", *pc);
                        return false;
                    }
                } // BNE
                0b100 => {
                    println!("imm_b: {}", imm_b);
                    println!("pc: {}", *pc);
                    if (cpu_state.registers[rs1 as usize] as i32)
                        < (cpu_state.registers[rs2 as usize] as i32)
                    {
                        *pc = (*pc as i32 + ((imm_b as i32) << 0)) as u32;
                        println!("pc: {}", *pc);
                        return false;
                    }
                } // BLT
                0b101 => {
                    println!("imm_b: {}", imm_b);
                    println!("pc: {}", *pc);
                    if (cpu_state.registers[rs1 as usize] as i32)
                        >= (cpu_state.registers[rs2 as usize] as i32)
                    {
                        *pc = (*pc as i32 + ((imm_b as i32) << 0)) as u32;
                        println!("pc: {}", *pc);
                        return false;
                    }
                } // BGE
                0b110 => {
                    if cpu_state.registers[rs1 as usize] < cpu_state.registers[rs2 as usize] {
                        *pc = (*pc as i32 + ((imm_b as i32) << 0)) as u32;
                        return false;
                    }
                } // BLTU
                0b111 => {
                    if cpu_state.registers[rs1 as usize] >= cpu_state.registers[rs2 as usize] {
                        *pc = (*pc as i32 + ((imm_b as i32) << 0)) as u32;
                        return false;
                    }
                } // BGEU
                _ => panic!("Unknown funct3: {}", funct3),
            }
        }
        // Jump and Link Instructions
        0b1100111 => {
            println!("jalr");
            println!("imm_i: {}", imm_i);
            println!("rs1: {}", rs1);
            println!("reg: {}", cpu_state.registers[rs1 as usize]);
            let jump =
                ((cpu_state.registers[rs1 as usize] as i32 + imm_i as i32) as u32) & 0xFFFFFFFE;
            cpu_state.registers[rd as usize] = (*pc + 4) as u32;
            *pc = jump as u32;
            return false;
        } // JALR
        0b1101111 => {
            println!("imm_j: {}", imm_j);
            println!("pc: {}", *pc);
            cpu_state.registers[rd as usize] = (*pc + 4) as u32;
            *pc = (*pc as i32 + ((imm_j as i32) << 0)) as u32;
            println!("pc: {}", *pc);
            return false;
        } // JAL

        // Load or Add Immediate Instructions
        0b0110111 => cpu_state.registers[rd as usize] = imm_u as u32, // LUI
        0b0010111 => cpu_state.registers[rd as usize] = (imm_u << 12) as u32 + (*pc as u32), // AUIPC

        // Fence Instructions
        0b0001111 => {} // FENCE

        // ECall Instructions
        0b1110011 => {
            match funct3 {
                0b000 => {
                    println!("ECALL");
                    match imm_i {
                        0 => {
                            println!("ECALL");
                            return true;
                        } // ECALL
                        1 => {} // EBREAK
                        _ => panic!("Unknown imm_i: {}", imm_i),
                    }
                }
                0b001 => {} // CSRRW
                0b010 => {} // CSRRS
                0b011 => {} // CSRRC
                0b101 => {} // CSRRWI
                0b110 => {} // CSRRSI
                0b111 => {} // CSRRCI
                _ => panic!("Unknown funct3: {}", funct3),
            }
        }

        ///////////////////////////////////////////// RV32F Standard Extension /////////////////////////////////////////////

        // FLoating Point Instructions
        0b0000111 => {
            match funct3 {
                0b010 => {
                    cpu_state.floating_point_registers[rd as usize] = (cpu_state.read_mem(
                        ((cpu_state.registers[rs1 as usize] as i32) + (imm_l as i32)) as usize,
                    )) as f32
                } // FLW
                0b011 => {
                    cpu_state.floating_point_registers[rd as usize] = (cpu_state.read_mem(
                        ((cpu_state.registers[rs1 as usize] as i32) + (imm_l as i32)) as usize,
                    )) as f32
                } // FLW
                _ => panic!("Unknown funct3: {}", funct3),
            }
            cpu_state.floating_point_registers[rd as usize] = (cpu_state
                .read_mem(((cpu_state.registers[rs1 as usize] as i32) + (imm_l as i32)) as usize))
                as f32
        } // FLW
        0b0100111 => cpu_state.set_mem(
            ((cpu_state.registers[rs1 as usize] as i32) + (imm_s as i32)) as usize,
            cpu_state.floating_point_registers[rs2 as usize] as u32,
        ), // FSW
        0b1010011 => {
            match funct7 {
                0b0000000 => {
                    cpu_state.floating_point_registers[rd as usize] = cpu_state
                        .floating_point_registers[rs1 as usize]
                        + cpu_state.floating_point_registers[rs2 as usize]
                } // FADD.S
                0b0000100 => {
                    cpu_state.floating_point_registers[rd as usize] = cpu_state
                        .floating_point_registers[rs1 as usize]
                        - cpu_state.floating_point_registers[rs2 as usize]
                } // FSUB.S
                0b0001000 => {
                    cpu_state.floating_point_registers[rd as usize] = cpu_state
                        .floating_point_registers[rs1 as usize]
                        * cpu_state.floating_point_registers[rs2 as usize]
                } // FMUL.S
                0b0001100 => {
                    cpu_state.floating_point_registers[rd as usize] = cpu_state
                        .floating_point_registers[rs1 as usize]
                        / cpu_state.floating_point_registers[rs2 as usize]
                } // FDIV.S
                0b0101100 => {
                    cpu_state.floating_point_registers[rd as usize] =
                        (cpu_state.floating_point_registers[rs1 as usize]).sqrt()
                } // FSQRT.S
                0b0010000 => match funct3 {
                    0b000 => {
                        cpu_state.floating_point_registers[rd as usize] =
                            cpu_state.floating_point_registers[rs1 as usize].abs()
                                * cpu_state.floating_point_registers[rs2 as usize].signum()
                    } // FSGNJ.S
                    0b001 => {
                        cpu_state.floating_point_registers[rd as usize] =
                            cpu_state.floating_point_registers[rs1 as usize].abs()
                                * cpu_state.floating_point_registers[rs2 as usize].signum()
                                * -1.0
                    } // FSGNJN.S
                    0b010 => {
                        cpu_state.floating_point_registers[rd as usize] = cpu_state
                            .floating_point_registers[rs1 as usize]
                            * cpu_state.floating_point_registers[rs2 as usize].signum()
                    } // FSGNJX.S
                    _ => panic!("Unknown funct3: {}", funct3),
                },
                0b0010100 => match funct3 {
                    0b000 => {
                        cpu_state.floating_point_registers[rd as usize] = if cpu_state
                            .floating_point_registers[rs1 as usize]
                            < cpu_state.floating_point_registers[rs2 as usize]
                        {
                            cpu_state.floating_point_registers[rs1 as usize]
                        } else {
                            cpu_state.floating_point_registers[rs2 as usize]
                        }
                    } // FMIN.S
                    0b001 => {
                        cpu_state.floating_point_registers[rd as usize] = if cpu_state
                            .floating_point_registers[rs1 as usize]
                            > cpu_state.floating_point_registers[rs2 as usize]
                        {
                            cpu_state.floating_point_registers[rs1 as usize]
                        } else {
                            cpu_state.floating_point_registers[rs2 as usize]
                        }
                    } // FMAX.S
                    _ => panic!("Unknown funct3: {}", funct3),
                },
                0b1100000 => match rs2 as u32 {
                    0b00000 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.floating_point_registers[rs1 as usize] as i32 as u32
                    } // FCVT.W.S
                    0b00001 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.floating_point_registers[rs1 as usize] as u32
                    } // FCVT.WU.S
                    _ => panic!("Unknown rs2: {}", rs2),
                },
                0b1110000 => match funct3 {
                    0b000 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.floating_point_registers[rs1 as usize] as u32
                    } // FMV.X.W
                    0b001 => {
                        cpu_state.registers[rd as usize] =
                            mask_generate(cpu_state.floating_point_registers[rs1 as usize])
                    } // FCLASS.S
                    _ => panic!("Unknown funct3: {}", funct3),
                },
                0b1010000 => match funct3 {
                    0b000 => {
                        cpu_state.registers[rd as usize] = if cpu_state.floating_point_registers
                            [rs1 as usize]
                            <= cpu_state.floating_point_registers[rs2 as usize]
                        {
                            1
                        } else {
                            0
                        }
                    } // FLE.S
                    0b001 => {
                        cpu_state.registers[rd as usize] = if cpu_state.floating_point_registers
                            [rs1 as usize]
                            < cpu_state.floating_point_registers[rs2 as usize]
                        {
                            1
                        } else {
                            0
                        }
                    } // FLT.S
                    0b010 => {
                        cpu_state.registers[rd as usize] = if cpu_state.floating_point_registers
                            [rs1 as usize]
                            == cpu_state.floating_point_registers[rs2 as usize]
                        {
                            1
                        } else {
                            0
                        }
                    } // FEQ.S
                    _ => panic!("Unknown funct3: {}", funct3),
                },
                0b1101000 => match rs2 as u32 {
                    0b00000 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.floating_point_registers[rs1 as usize] as i32 as u32
                    } // FCVT.S.W
                    0b00001 => {
                        cpu_state.registers[rd as usize] =
                            cpu_state.floating_point_registers[rs1 as usize] as u32
                    } // FCVT.S.WU
                    _ => panic!("Unknown rs2: {}", rs2),
                },
                0b1111000 => match funct3 {
                    0b000 => {
                        cpu_state.floating_point_registers[rd as usize] =
                            cpu_state.registers[rs1 as usize] as f32
                    } // FMV.W.X
                    _ => panic!("Unknown funct3: {}", funct3),
                },

                _ => panic!("Unknown funct7: {}", funct7),
            }
        }

        ///////////////////////////////////////////// Custom GPIO Extensions /////////////////////////////////////////////
        0b1111000 => match funct3 {
            000 => {
                // Set Config
                cpu_state.pc += 4;
                let config_address = cpu_state.read_mem(cpu_state.pc as usize) as usize;
                let bytes_slice = &cpu_state.memory_bytes
                    [config_address..(config_address + std::mem::size_of::<GPIOState>())];
                let struct_ref = unsafe { &*(bytes_slice.as_ptr() as *const GPIOState) };
                cpu_state.gpio_states[imm_i as usize] = *struct_ref;
                // TODO check for any connections to this gpio and update them
            }
            001 => {
                // Reset Config
                cpu_state.gpio_states[imm_i as usize] = GPIOState::new();
            }
            _ => panic!("Unknown funct3: {}", funct3),
        },

        _ => {
            panic!("Unknown opcode: {:b}", opcode);
        }
    }
    cpu_state.pc += 4;
    return false;
}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    use std::fs;
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    return buffer;
}

pub fn interpret(file_name: &str, cpu_state: &mut CPUState) -> io::Result<()> {
    let buffer: Vec<u8> = get_file_as_byte_vec(file_name);
    for i in 0..buffer.len() {
        let index: usize = (i / 4) * 4 + 3 - (i % 4);
        cpu_state.memory_bytes[index] = buffer[i];
        // cpu_state.memory[(i / 4) * 4 as usize] <<= 8;
        // cpu_state.memory[(i / 4) * 4 as usize] += (buffer[i] & 0xFF) as u32;
    }
    let mut count = 0;
    loop {
        count += 1;
        let ecall: bool = decode_instruction(cpu_state);
        cpu_state.registers[0] = 0;
        if ecall {
            println!("{}", cpu_state.registers[17]);
            if cpu_state.registers[17] == 10 {
                break;
            }
        }
        if count >= 10 {
            assert!(false);
        }
    }

    return Ok(());
}
