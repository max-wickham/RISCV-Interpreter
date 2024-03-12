
tests = [
    (
        'test_add',
        '''
li x2, 5
li x1, 7
add x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,12
    ),
    (
    'test_sub',
        '''
li x2, 5
li x1, 7
sub x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,2
    ),
    (
        'test_sll',
        '''
li x2, 2
li x1, 10
sll x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,40
    ),
    (
        'test_slt_true',
        '''
li x2, 10
li x1, 2
slt x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,1
    ),
    (
        'test_slt_false',
        '''
li x2, 2
li x1, 10
slt x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,0
    ),
    (
        'test_slt_same',
        '''
li x2, 2
li x1, 2
slt x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,0
    ),
    (
        'test_slt_signed',
        '''
li x2, -10
li x1, -20
slt x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,1
    ),
    (
        'test_sltu_false',
        '''
li x2, 10
li x1, 20
sltu x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,0
    ),
    (
        'test_sltu_true',
        '''
li x2, 20
li x1, 10
sltu x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,1
    ),
#     (
#         'test_sltu_signed',
#         '''
# li x2, 4086
# li x1, 4076
# sltu x3, x1, x2
# addi x17, x0, 10
# ecall
#         ''',
#         3,1
#     ),
    (
        'test_xor',
        f'''
li x2, {5}
li x1, {15}
xor x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,5^15
    ),
    (
        'test_srl',
        f'''
li x2, {3}
li x1, {10}
srl x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,10 >> 3
    ),
#     (
#         'test_srl_signed',
#         f'''
# li x2, {4}
# li x1, {-10}
# srl x3, x1, x2
# addi x17, x0, 10
# ecall
#         ''',
#         3,((-10 >> 4) & 0xFFFFFFFF) & 0x0FFFFFFF
#     ),
    (
    'test_sra',
    f'''
li x2, {3}
li x1, {10}
sra x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,10 >> 3
    ),
#     (
#     'test_sra_signed',
#     f'''
# li x2, {3}
# li x1, {-10}
# sra x3, x1, x2
# addi x17, x0, 10
# ecall
#         ''',
#         3,-10 >> 3
#     ),
    (
        'test_or',
        f'''
li x2, {3}
li x1, {10}
or x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,10 | 3
    ),
    (
        'test_and',
        f'''
li x2, {3}
li x1, {10}
and x3, x1, x2
addi x17, x0, 10
ecall
        ''',
        3,10 & 3
    ),
    (
        'test_beq_pos',
        f'''
li x2, {5}
li x1, {5}
beq x1, x2, equal_branch
li x3, 0
j end_program
equal_branch: li x3, 10
end_program: addi x17, x0, 10
ecall
        ''',
        3,10
    ),
    (
        'test_beq_false',
        f'''
li x2, {5}
li x1, {6}
beq x1, x2, equal_branch
li x3, 0
j end_program
equal_branch: li x3, 10
end_program: addi x17, x0, 10
ecall
        ''',
        3,0
    ),
(
        'test_bne_pos',
        f'''
li x2, {5}
li x1, {6}
bne x1, x2, equal_branch
li x3, 0
j end_program
equal_branch: li x3, 10
end_program: addi x17, x0, 10
ecall
        ''',
        3,10
    ),
    (
        'test_bne_false',
        f'''
li x2, {5}
li x1, {5}
bne x1, x2, equal_branch
li x3, 0
j end_program
equal_branch: li x3, 10
end_program: addi x17, x0, 10
ecall
        ''',
        3,0
    ),
    (
        'test_blt_pos',
        f'''
li x2, {5}
li x1, {4}
blt x1, x2, equal_branch
li x3, 0
j end_program
equal_branch: li x3, 10
end_program: addi x17, x0, 10
ecall
        ''',
        3,10
    ),
    (
        'test_blt_false',
        f'''
li x2, {5}
li x1, {8}
blt x1, x2, equal_branch
li x3, 0
j end_program
equal_branch: li x3, 10
end_program: addi x17, x0, 10
ecall
        ''',
        3,0
    ),
    (
        'test_bge_pos',
        f'''
li x2, {4}
li x1, {5}
bge x1, x2, equal_branch
li x3, 0
j end_program
equal_branch: li x3, 10
end_program: addi x17, x0, 10
ecall
        ''',
        3,10
    ),
    (
        'test_bge_false',
        f'''
li x2, {8}
li x1, {5}
bge x1, x2, equal_branch
li x3, 0
j end_program
equal_branch: li x3, 10
end_program: addi x17, x0, 10
ecall
        ''',
        3,0
    ),
    (
        'test_bge_eq',
        f'''
li x2, {4}
li x1, {4}
bge x1, x2, equal_branch
li x3, 0
j end_program
equal_branch: li x3, 10
end_program: addi x17, x0, 10
ecall
        ''',
        3,10
    ),
    (
        'test_jalr',
        f'''
lw x1, start_addr(x0)
jalr x6 0(x1)
li x3 10
j end_program
start: li x3 17
end_program: addi x17, x0, 10
ecall
start_addr:  .word start
        ''',
        3,17
    ),
#     (
#         'test_auipc',
#         f'''
# li x1, 0
# lw x1, result

# jalr x6 0(x1)
# li x3 10
# j end_program
# start: li x3 17
# end_program: addi x17, x0, 10
# ecall
# start_addr:  .word start
#         ''',
#         3,17
#     ),

]


# LB
# imm[11:0] rs1 001 rd 0000011 LH
# imm[11:0] rs1 010 rd 0000011 LW
# imm[11:0] rs1 100 rd 0000011 LBU
# imm[11:0] rs1 101 rd 0000011 LHU
# imm[11:5] rs2 rs1 000 imm[4:0] 0100011 SB
# imm[11:5] rs2 rs1 001 imm[4:0] 0100011 SH
# imm[11:5] rs2 rs1 010 imm[4:0] 0100011 SW
import json
tests = [ {"name":t[0],"code":t[1],"result":(t[2],t[3])} for t in tests]
with open("test.json","w") as f:
    json.dump(tests,f)
