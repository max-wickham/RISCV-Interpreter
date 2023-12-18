'''Generate binary files from assembly scripts'''

import os

from tinyfive import Parser
from riscv_assembler.convert import AssemblyConverter as AC


ASSEMBLY_FOLDER = './test_gen/assembly_files'
BINARY_FOLDER = './test_gen/binary_files'

for filename in os.listdir(ASSEMBLY_FOLDER):
    if filename.endswith(".s") or filename.endswith(".S"):
        convert = AC(output_mode = 'f', nibble_mode = True, hex_mode = False)
        convert(f'{ASSEMBLY_FOLDER}/{filename}', f'{BINARY_FOLDER}/{filename[:-2]}.bin')
