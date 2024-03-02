



- [x] Basic Instructions
    This includes all the integer arithmetic instructions as well as load and store instructions
- [x] Floating Point Instructions
    Not including double precision
- [x] Create assembler in python
    - [x] R
    - [x] I
    - [x] S
    - [x] U
    - [x] B
    - [x] J
    - [x] Test
    - [x] Support .word
    - [_] Support proper sectioning
    - [_] Fix branch labels
- [x] Check that the immediate are correct
- [_] Test Basic Instructions
    - [x] Branch
    - [x] Jump
        - [_] Test link
    - [_] Immediate
    - [x] Register
    - [_] Store Load

- [x] Convert Memory to an array of bytes

- [_] Fix Up Assembler
    - [_] Add Pseudo instructions
    - [_] Ignore unnecessary directives
    - [_] Load bytes / half ????

- [_] Custom Instructions
    - [_] Interrupts
    - [_] Internet
    - [x] Digital GPIO

- [_] Test basic c scripts

- [_] Convert Esp32 API
    - [_] GPIO
    - [_] ADC
    - [_] Timer Interrupts
    - [_] Wifi messaging

- [_] More Assembler Instructions

- [_] Word Instructions
- [_] Sys Call
- [_] Atomic Operations, Fence
- [_] Vector Extensions
- [_] Doubles
- [_] Multi Core?
- [_] Convert the python to rust


## Bugs

- [_] Fix high li
- [_] SRA and SRL signed
- [_] Add proper branch labels to assembly
- [_] AUIPC may be adding 4 too much?




## Custom Instructions

- GPIO Extensions                   1111000
    - Config GPIO                   Funct3 000 next word is address of config string
        - Pull Up/Down
    - Reset GPIO                    Funct3 001
        Put in default state
    - Get/Set GPIO Level            Funct3 010 Immediate 1 or 0

- Communication Extensions
    - UART
    - SPI
    - I2C
    - 1Wire

- Web Extensions
    - HTTP
    - HTTPS?

    - Create Client
    - Client Perform (Do the action in the config)
    - Cancel request
    - Set url
    - Get/Set post field
    - Get/Set header
        - Get/Set username
        - Get/Set password
    - Set auth type
    - Get user data
    - Set user data


    Create Client
    Set Client Config
        ```json
            {
                "url" : "string",
                "headers" : {
                    "type" : "object",
                    "additionalProperties": {"type": "string"}
                }
                "username":"string",
                "password":"password",
                "auth_type":"string",
                "method":"string",
                "timeout_ms",number,
            }
        ```
    Set Body
    Open Client
        write all headers
    Write Body to Client
    Receive Config
    <!-- Is response chunked -->
    Receive Body
    Client Close
    ? Flush response,
    ? Is Response read



1111000

- GPIO Extension
    - Digital Read
    - Digital Write
1111001

- Interrupt Instruction
    - Specification of function to jump to
    - Timer Interrupts
    - GPIO Interrupts
1111010

- Sys Call Instructions
    - Sleep

1111011
