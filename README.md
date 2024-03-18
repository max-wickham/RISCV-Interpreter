# RISCV Interpreter and Assembler

This is a Rust implementation of a RISCV interpreter and assembler. An assembler is included so as to eventually allow for the optimisation of the bytecode spec for interpretation. This is due to many of the design decisions made for RISCV not being optimal when interpreted in software and function better in hardware. Examples of this are having immediate fields with weird bit orders. Eventually in addition to an assembler a disassembler will be added so existing binaries can be optimised.

## Assembler

The assembler is built using the Lalrpop library to generate the grammar and parser. Some further optimisation need to be made to handle Pseudo instructions such as Li.

## Interpreter

The current implementation of the interpreter is hade written using match statements and has not been optimised. Eventually macros will be used to optimise code placement. When working on this I realised that it may make more sense to define a language that gives the opcodes, func values etc. of each instruction and then implement a compiler to create the most optimised version of the interpreter. This will allow for easy definition of RISC interpreters in the future with different instruction sets.

## Tests

Tests are currently defined in "tests/arithmetic_test.json" file and then a proc macro is used to convert these to rust functions. In older commits the tests were generated using a python implementation of the assembler.
