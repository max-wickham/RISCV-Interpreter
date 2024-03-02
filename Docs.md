# Custom RISCV Extensions

## GPIO Extension

- Opcode **1111000**
    - Config GPIO Funct3 **000**#
        - Immediate value is the GPIO number
        - Next word is address of config
        - Next word is the length of the config
        - Config (any missing values will just use current values, i.e. could just set the value and not the pull up state)
            ```json
                {
                    pull_up : boolean,
                    value : boolean
                }
            ```
    - Reset GPIO Funct3 **001**
        - Set config to default values
        - Immediate value is the GPIO number
