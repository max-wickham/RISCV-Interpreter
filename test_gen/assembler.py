# Write an assembler for RISCV that converts assembly code to machine code.

_opcodes_dict : dict[str,str] = {
    'add' : '0110011',
    'addi' : '0010011',
    'beq' : '1100011',
    'sb' : '0100011',
    'lui' : '0110111',
    'jal' : '1101111',
}

_func_3_dict : dict[str,str] = {
    'add' : '000',
    'addi' : '000',
    'beq' : '000',
    'sb' : '000',
}

_funct_7_dict : dict[str,str] = {
    'add' : '0000000',
}

_registers_dict : dict[str,str] = {}
for i in range(32):
    _registers_dict[f'x{i}'] = format(i,'05b')

_r_type_instructions : list[str] = ['add','sub','sll','slt','sltu','xor','srl','sra','or','and']
_i_type_instructions : list[str] = ['addi','slti','sltiu','xori','ori','andi','slli','srli','srai','lb','lh','lw','lbu','lhu','jalr']
_s_type_instructions : list[str] = ['sb','sh','sw']
_u_type_instructions : list[str] = ['lui','auipc']
_b_type_instructions : list[str] = ['beq','bne','blt','bge','bltu','bgeu']
_j_type_instructions : list[str] = ['jal']

def to_32_bit_hex(binary_string : str) -> str:
    return hex(int(binary_string,base = 2))[2:].zfill(8)

def _format_32_bit_string(number : int) -> str:
    return format(number, '032b')[-32:]

class Assembler:
    '''Convert assembly to array of bytes'''

    # _instruction_map = {
    #     'r' : _r_type,
    #     'i' : _i_type,
    #     's' : _s_type,
    #     'u' : _u_type,
    #     'b' : _b_type,
    #     'j' : _j_type
    # }

    def __init__(self) -> None:
        self._instruction_map = {**{
            instruction : self._r_type for instruction in _r_type_instructions
        }, **{
            instruction : self._i_type for instruction in _i_type_instructions
        }, **{
            instruction : self._s_type for instruction in _s_type_instructions
        }, **{
            instruction : self._u_type for instruction in _u_type_instructions
        }, **{
            instruction : self._b_type for instruction in _b_type_instructions
        }, **{
            instruction : self._j_type for instruction in _j_type_instructions
        }}

    def assembler(self,assembly_code):
        '''Receive assembly string and return array of bytes'''
        lines = assembly_code.splitlines()
        # remove comments
        lines = [line.split('#')[0] for line in lines]
        # remove tabs
        lines = [line.replace('\t',' ') for line in lines]
        # remove commas
        lines = [line.replace(',',' ') for line in lines]
        # split into words
        lines = [line.split(' ') for line in lines]
        # filter out whitespace and empty strings
        lines = [[word for word in line if (not word.isspace() and len(word) > 0)] for line in lines]
        # remove empty lines
        lines = [line for line in lines if len(line) > 0]

        hex_strings = []
        for line in lines:
            hex_strings.append(self._instruction_map[line[0]](line))

        return hex_strings


    def _r_type(self, line : list[str]) -> bytearray:
        instruction = line[0].lower()
        rd = line[1]
        rs1 = line[2]
        rs2 = line[3]

        binary = (
            (int(_funct_7_dict[instruction],base = 2) & 0b1111111) |
            ((int(_registers_dict[rs2],base = 2) & 0b11111) << 20) |
            ((int(_registers_dict[rs1],base = 2) & 0b11111) << 15) |
            ((int(_func_3_dict[instruction],base = 2) & 0b111) << 12) |
            ((int(_registers_dict[rd],base = 2) & 0b11111) << 7) |
            ((int(_opcodes_dict[instruction],base = 2) & 0b1111111))
            )

        return _format_32_bit_string(binary)

    def _i_type(self, line : list[str]):
        instruction : str = line[0].lower()
        rd = line[1]
        rs1 = line[2]
        imm = int(line[3])

        binary = (imm & 0xFFF) << 20
        binary |=(
            ((int(_registers_dict[rs1],base=2) & 0b11111) << 15) |
            ((int(_func_3_dict[instruction],base = 2) & 0b111) << 12) |
            ((int(_registers_dict[rd],base=2) & 0b11111) << 7) |
            ((int(_opcodes_dict[instruction],base = 2) & 0b1111111))
        )

        return _format_32_bit_string(binary)

    def _s_type(self, line : list[str]):
        instruction : str = line[0].lower()
        opcode = int(_opcodes_dict[instruction],base = 2)
        funct3 = int(_func_3_dict[instruction],base = 2)
        rs2 = int(_registers_dict[line[1]],base = 2)
        imm_rs1 = line[2][:-1].split('(')
        imm, rs1 = int(imm_rs1[0]), int(_registers_dict[imm_rs1[1]],base =2)
        binary = (
            (((imm >> 5) & 0xFF) << 25) |
            ((rs2 & 0b11111) << 12) |
            ((rs1 & 0b11111) << 15) |
            ((funct3 & 0b111) << 12) |
            (((imm) & 0x1F) << 7) |
            ((opcode & 0b1111111))
            )

        return _format_32_bit_string(binary)

    def _u_type(self, line : list[str]):
        instruction : str = line[0].lower()
        opcode = int(_opcodes_dict[instruction],base = 2)
        rd = int(_registers_dict[line[1]],base = 2)
        imm = int(line[2])

        binary = (
            ((imm & 0xFFFFF) << 12) |
            ((rd & 0b11111) << 7) |
            ((opcode & 0b1111111))
        )

        return _format_32_bit_string(binary)

    def _b_type(self, line : list[str]):
        instruction : str = line[0].lower()
        rs1 = line[1].lower()
        rs2 = line[2].lower()
        imm = int(line[3])
        binary = (
            ((int(_opcodes_dict[instruction],base=2)) & 0b1111111) |
            ((int(_registers_dict[rs1],base=2) & 0b11111) << 15) |
            ((int(_registers_dict[rs2],base=2) & 0b11111) << 20) |
            ((int(_func_3_dict[instruction],base=2) & 0b111) << 12)
            )
        # print(_format_32_bit_string(binary))
        # Extract and set bits for the immediate value
        binary |= ((imm >> 12) & 0b1) << 31
        # print(_format_32_bit_string(binary))
        binary |= ((imm >> 5) & 0b111111) << 25
        # print('x',_format_32_bit_string(((imm >> 5) & 0b111111) << 25))
        binary |= ((imm >> 11) & 0b1) << 7
        # print(_format_32_bit_string(binary))
        binary |= ((imm >> 1) & 0b1111) << 8
        # print(_format_32_bit_string(binary))
        # Convert to a 32-bit binary string and ensure it's 32 characters long
        # binary_string = format(binary, '032b')[-32:]
        return _format_32_bit_string(binary)

    def _j_type(self, line : list[str]):
        instruction : str = line[0].lower()
        opcode = int(_opcodes_dict[instruction],base = 2)
        rd = int(_registers_dict[line[1]],base = 2)
        imm = int(line[2])

        binary = (
            ((rd & 0b11111) << 7) |
            ((opcode & 0b1111111))
        )

        binary |= (
            (((imm >> 12) & 0b11111111) << 12 ) |
            (((imm >> 11) & 0b1) << 20) |
            (((imm >> 1) & 0b1111111111) << 21) |
            (((imm >> 20) & 0b1) << 31)
        )

        return _format_32_bit_string(binary)



# print(Assembler().assembler('add x1, x2, x3'))
