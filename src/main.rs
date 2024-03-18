#[macro_use]
extern crate lalrpop_util;
mod assembler;
mod riscv_spec;
#[macro_use]
extern crate test_gen;
#[macro_use]
extern crate virtual_machine;
use virtual_machine::vm;


// macro_rules! immi {() => {};}
fn main() {
    println!("hello");
    vm! {
        let x = 2;
    };

    // let program_text = "
    // add x1, x2, x3
    // add x1, x3, x2
    // add x7, x3, x2
    // addi x7, x3, 12
    // addi x7, x3, 0
    // addi x7, x3, -16
    // sb x0, 12(x5)
    // sb x0, -16(x5)
    // sb x0, 0(x5) # this is a comment at the end of a line
    // # this is a comment
    // lui x0, 12
    // branch: lui x0, -16
    // lui x0, 0
    // beq x0, x5, 12
    // beq x0, x5, -16
    // beq x0, x5, 0
    // jal x0, 12
    // jal x0, -16
    // jal x0, 0
    // jal x0 branch
    // .word 0xAA 0xAB 10 0b0011
    // ";

    // let binary = assembler::assembler::assemble(&program_text.to_string());


}
