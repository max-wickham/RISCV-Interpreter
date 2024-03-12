'''Test the assembler'''
import sys
import os
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from assembler import Assembler

def test_assembler():
    '''Test that the assembler works as expected'''

    code = '''
        add x1, x2, x3
        add x1, x3, x2
        add x7, x3, x2
        addi x7, x3, 12
        addi x7, x3, 0
        addi x7, x3, -16
        sb x0, 12(x5)
        sb x0, -16(x5)
        sb x0, 0(x5) # this is a comment at the end of a line
        # this is a comment
        lui x0, 12
        lui x0, -16
        lui x0, 0
        beq x0, x5, 12
        beq x0, x5, -16
        beq x0, x5, 0
        jal x0, 12
        jal x0, -16
        jal x0, 0

                        '''

    values = Assembler().assembler(code)
    results = ['003100b3',
               '002180b3',
               '002183b3',
               '00c18393',
               '00018393',
               'ff018393',
               '00028623',
               'fe028823',
               '00028023',
               '0000c037',
               'ffff0037',
               '00000037',
               '00500663',
               'fe5008e3',
               '00500063',
               '00c0006f',
               'ff1ff06f',
               '0000006f',
    ]

    for value, result in zip(values,results):
        assert value == result


    # TODO add in addresses
