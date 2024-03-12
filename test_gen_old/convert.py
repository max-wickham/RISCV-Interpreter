'''Generate binary files from assembly scripts'''

import os
from assembler import Assembler, to_32_bit_hex,_format_32_bit_string

from tests import tests


def delete_files_in_directory(directory_path):
    try:
        files = os.listdir(directory_path)
        for file in files:
            file_path = os.path.join(directory_path, file)
            if os.path.isfile(file_path):
                os.remove(file_path)
        print("All files deleted successfully.")
    except OSError:
        print("Error occurred while deleting files.")


ASSEMBLY_FOLDER = './test_gen/assembly_files'
BINARY_FOLDER = './test_gen/binary_files'
RESULTS_FOLDER = './test_gen/result_files'
delete_files_in_directory(RESULTS_FOLDER)
delete_files_in_directory(BINARY_FOLDER)
delete_files_in_directory(ASSEMBLY_FOLDER)
names = []
for test_name, assembly_code, expected_register, expected_reg_value in tests:
    assert test_name not in names
    names.append(test_name)
    print(test_name)
    with open(f'{ASSEMBLY_FOLDER}/{test_name}.s', 'w', encoding='utf8') as f:
        f.write(assembly_code)
    with open(f'{BINARY_FOLDER}/{test_name}.bin', 'wb') as f:
        hex_strings = Assembler().assembler(assembly_code)
        print(len(hex_strings))
        print('strings',*[''.join(format(byte, '08b') for byte in bytes.fromhex(hex_string))
        for hex_string in hex_strings], sep='\n')
        hex_string = ''.join(hex_strings)
        binary_data = bytes.fromhex(hex_string)
        word = 7
        # print(''.join(format(byte, '08b') for byte in binary_data)[32*word:32+32*word])
        f.write(binary_data)
    with open(f'{RESULTS_FOLDER}/{test_name}.json', 'w', encoding='utf8') as f:
        f.write(f'[{expected_register},{expected_reg_value}]')
