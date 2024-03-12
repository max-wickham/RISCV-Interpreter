# Write an assembler for RISCV that converts assembly code to machine code.

_opcodes_dict : dict[str,str] = {
    'add' : '0110011',
    'sub' : '0110011',
    'sll' : '0110011',
    'slt' : '0110011',
    'sltu' : '0110011',
    'xor' : '0110011',
    'srl' : '0110011',
    'sra' : '0110011',
    'or' : '0110011',
    'and' : '0110011',
    'addi' : '0010011',
    'beq' : '1100011',
    'bne' : '1100011',
    'blt' : '1100011',
    'bge' : '1100011',
    'bltu' : '1100011',
    'bgeu' : '1100011',
    'sb' : '0100011',
    'lui' : '0110111',
    'li'   : '0010111',
    'jal' : '1101111',
    'jalr' : '1100111',
    'j' : '1101111',
    'lw' : '0000011',
    'lb' : '0000011',
    'lh' : '0000011',
    'lbu' : '0000011',
    'lhu' : '0000011',
    'sw' : '0100011',
    'sh' : '0100011',
}

_func_3_dict : dict[str,str] = {
    'add' : '000',
    'sub' : '000',
    'sll' : '001',
    'slt' : '010',
    'sltu' : '011',
    'xor' : '100',
    'srl' : '101',
    'sra' : '101',
    'or' : '110',
    'and' : '111',
    'addi' : '000',
    'beq' : '000',
    'bne' : '001',
    'blt' : '100',
    'bge' : '101',
    'bltu' : '110',
    'bgeu' : '111',
    'sb' : '000',
    'lw' : '010',
    'lb' : '000',
    'lh' : '001',
    'lbu' : '100',
    'lhu' : '101',
    'sw' : '010',
    'sh' : '001',
    'jalr' : '000',
}

_funct_7_dict : dict[str,str] = {
    'add' : '0000000',
    'sub' : '0100000',
    'sll' : '0000000',
    'slt' : '0000000',
    'sltu' : '0000000',
    'xor' : '0000000',
    'srl' : '0000000',
    'sra' : '0100000',
    'or' : '0000000',
    'and' : '0000000',
    'addi' : '0000000',
    'beq' : '0000000',
    'bne' : '0000000',
    'blt' : '0000000',
    'bge' : '0000000',
    'sb' : '0000000',
}
# 00000000001000001011000110110011
# 00000000001000001011000110110011

_registers_dict : dict[str,str] = {}
for i in range(32):
    _registers_dict[f'x{i}'] = format(i,'05b')

_r_type_instructions : list[str] = ['add','sub','sll','slt','sltu','xor','srl','sra','or','and']
_i_type_instructions : list[str] = ['addi','slti','sltiu','xori','ori','andi','slli','srli','srai','lb','lh','lw','lbu','lhu','jalr']
_s_type_instructions : list[str] = ['sb','sh','sw']
_u_type_instructions : list[str] = ['lui','auipc']
_b_type_instructions : list[str] = ['beq','bne','blt','bge','bltu','bgeu']
_j_type_instructions : list[str] = ['jal']
_bracket_instructions : list[str] = ['lb','lh','lw','lbu','lhu','sb','sh','sw','jalr']

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
        assembly_code.replace(':\n',':')
        assembly_code.replace(':\r\n',':')

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
        # handle line labels
        labels = {}
        concat_lines = [''.join(line) for line in lines]
        for index, line in enumerate(concat_lines):
            if ':' in line:
                line.split(':')
                print()
                labels[line.split(':')[0]] = index*4
        # if a line contains only 1 word and that word is a label, concatenate it with the next line
        concatenated_lines = []
        index = 0
        while index < len(lines):
            line = lines[index]
            if ':' in line[0] and len(line) == 1 and i != len(lines) - 1:
                concatenated_lines.append(line + lines[i+1])
                index += 1
            else:
                concatenated_lines.append(line)
            index += 1
        lines = concatenated_lines


        print(labels)
        # filter out labels
        lines = [[word for word in line if (not ':' in word)] for line in lines]
        # Word type
        lines = [
            [line[0], str(labels[line[1]])] if ('.word' in line[0] and line[1] in labels) else line for line in lines
        ]
        print(lines)
        # branch types
        lines = [
            [str((labels[word] - index * 4)) if word in labels else word
             for word in line if (not word.isspace() and len(word) > 0)] for index,line in enumerate(lines)]
        # jalr type
        lines = [
            [str((labels[word.split('(')[0]])) + f'({word.split('(')[1]}'
             if ('(' in word and word.split('(')[0] in labels) else word
             for word in line] for index,line in enumerate(lines)
        ]
        print(lines)

        hex_strings = []
        for line in lines:
            # if ':' in line[0]:
            #     line = line[1:]
            if line[0] == 'li':
                lines = self._convert_li(line)
                for line in lines:
                    hex_strings.append(to_32_bit_hex(self._instruction_map[line[0]](line)))
            elif line[0] == 'ecall':
                hex_strings.append('00000073')
            elif line[0] == 'j':
                hex_strings.append(to_32_bit_hex(self._instruction_map['jal'](['jal','x0',line[1]])))
            elif '.word' in line[0]:
                hex_strings.append(to_32_bit_hex(_format_32_bit_string(int(line[1]) & 0xFFFFFFFF)))
                # print('word',_format_32_bit_string(int(line[1]) & 0xFFFFFFFF))
            else:
                hex_strings.append(to_32_bit_hex(self._instruction_map[line[0]](line)))
        return hex_strings

    def _convert_li(self, line : list[str]) -> list[str]:
        '''Convert li instruction to addi instruction'''
        rd = line[1]
        imm = int(line[2])

        if imm < 2048 and imm >= -2048:
            return [['addi',rd,'x0',str(imm)]]
        else:
            imm_hi = imm >> 12
            imm_lo = imm & 0xFFF
            return [['lui',rd,str(imm_hi)],['addi',rd,rd,str(imm_lo)]]

    def _r_type(self, line : list[str]) -> bytearray:
        instruction = line[0].lower()
        rd = line[1]
        rs1 = line[2]
        rs2 = line[3]

        binary = (
            (int(_funct_7_dict[instruction],base = 2) & 0b1111111) << 25 |
            ((int(_registers_dict[rs2],base = 2) & 0b11111) << 20) |
            ((int(_registers_dict[rs1],base = 2) & 0b11111) << 15) |
            ((int(_func_3_dict[instruction],base = 2) & 0b111) << 12) |
            ((int(_registers_dict[rd],base = 2) & 0b11111) << 7) |
            ((int(_opcodes_dict[instruction],base = 2) & 0b1111111))
            )

        return _format_32_bit_string(binary)

    def _i_type(self, line : list[str]):
        instruction : str = line[0].lower()
        opcode = int(_opcodes_dict[instruction],base = 2)
        funct_3 = int(_func_3_dict[instruction],base = 2)
        rd = int(_registers_dict[line[1]],base=2)
        if line[0].lower() in _bracket_instructions:
            rs1 = int(_registers_dict[line[2].split('(')[1][:-1]],base=2) & 0xFFFFFFFF
            imm = int(line[2].split('(')[0])
        else:
            rs1 = int(_registers_dict[line[2]],base=2)
            imm = int(line[3]) & 0xFFFFFFFF

        print('imm', imm)

        binary = (imm & 0xFFF) << 20
        binary |=(
            ((rs1 & 0b11111) << 15) |
            ((funct_3 & 0b111) << 12) |
            ((rd & 0b11111) << 7) |
            ((opcode & 0b1111111))
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

    # 00000000001000001010000110110011
    # 00000000001000001001000110110011

    def _b_type(self, line : list[str]):
        instruction : str = line[0].lower()
        rs1 = line[1].lower()
        rs2 = line[2].lower()
        imm = int(line[3])
        print('immmmm', imm)
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
