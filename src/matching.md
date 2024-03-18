{
    macro funct3 = 3..7,
    macro funct7 = 3..7,
    macro addi = opcode == 0b0001 && funct3 == 0b001 && funct7 == 0b002;
    macro addi = opcode ==


    match word {
        addi => {
            registers[rd] = registers[r1] + registers[r2] + immi;
        },
        addi => {

        }
    }
}


<!-- let funct3 = word_slice!(3..11){
        addi: 0b001,
        subi: 0b001,
    }

let funct7 = word_slice!(3..11){
    addi: 0b001,
    subi: 0b001,
}

vm!{

    addi: opcode, funct3, funct7 => {

    },

    subi: opcode, funct3, funct7 => {

    },

    subi: opcode, funct5, funct7 => {

    }

} -->


{
    parameter func_7 = 1..2;
    parameter func_3 = 6..7;
    parameter opcode = 6..8;

    value rd = 6..8;
    value r1 = 9..10;
    value r2 = 5..3;

    value imm_i = 6..8 1 3 7..5
    value imm_j = 6..8 1 3 7..5

    instruction_type j_type {

        depends {
            opcode,
            func_3,
            func_7,
        },

        inputs {
            rd, r1, r2
        },

    }

    instruction addi: j_type {
        opcode = 0b0001,
        funct_3 = 0b001,
        funct_7 = 0b002,
        depends = {rd, r1, r2}
    }

    instruction subi: j_type {
        opcode = 0b0001,
        funct_3 = 0b001,
        funct_7 = 0b002,
        depends = {rd, r1, r2}
    }
}


vm!{

    decode word {

        addi => {
            <!-- code -->
        }

        subi => {
            <!-- code -->
        }



    }

}
