pub fn check_opcode_8080(program_counter: usize, buffer: &Vec<u8>) -> usize {
    
    let mut read_bytes = 1;
    
    match buffer[program_counter] {
        0 => println!("0x{:02x} NOP\n", buffer[program_counter]),
        1 => {
            println!("0x{:02x} LXI B {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        2 => println!("0x{:02x} STAX B\n", buffer[program_counter]),
        3 => println!("0x{:02x} INX B\n", buffer[program_counter]),
        4 => println!("0x{:02x} INR B\n", buffer[program_counter]),
        5 => println!("0x{:02x} DCR B\n", buffer[program_counter]),
        6 => {
            println!("0x{:02x} MVI B  {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        7 => println!("0x{:02x} RLC\n", buffer[program_counter]),
        9 => println!("0x{:02x} DAD B\n", buffer[program_counter]),
        10 => println!("0x{:02x} LDAX B\n", buffer[program_counter]),
        11 => println!("0x{:02x} DCX B\n", buffer[program_counter]),
        12 => println!("0x{:02x} INR C\n", buffer[program_counter]),
        13 => println!("0x{:02x} DCR C\n", buffer[program_counter]),
        14 => {
            println!("0x{:02x} MVI C {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        15 => println!("0x{:02x} RRC\n", buffer[program_counter]),
        17 => {
            println!("0x{:02x} LXI D {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        18 => println!("0x{:02x} STAX D\n", buffer[program_counter]),
        19 => println!("0x{:02x} INX D\n", buffer[program_counter]),
        20 => println!("0x{:02x} INR D\n", buffer[program_counter]),
        21 => println!("0x{:02x} DCR D\n", buffer[program_counter]),
        22 => {
            println!("0x{:02x} MVI D  {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        23 => println!("0x{:02x} RAL\n", buffer[program_counter]),
        25 => println!("0x{:02x} DAD D\n", buffer[program_counter]),
        26 => println!("0x{:02x} LDAX D\n", buffer[program_counter]),
        27 => println!("0x{:02x} DCX D\n", buffer[program_counter]),
        28 => println!("0x{:02x} INR E\n", buffer[program_counter]),
        29 => println!("0x{:02x} DCR E\n", buffer[program_counter]),
        30 => {
            println!("0x{:02x} MVI E {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        31 => println!("0x{:02x} RAR\n", buffer[program_counter]),
        33 => {
            println!("0x{:02x} LXI H {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        34 => {
            println!("0x{:02x} SHLD {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        35 => println!("0x{:02x} INX H\n", buffer[program_counter]),
        36 => println!("0x{:02x} INR H\n", buffer[program_counter]),
        37 => println!("0x{:02x} DCR H\n", buffer[program_counter]),
        38 => {
            println!("0x{:02x} MVI H {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        39 => println!("0x{:02x} DAA\n", buffer[program_counter]),
        41 => println!("0x{:02x} DAD H\n", buffer[program_counter]),
        42 => {
            println!("0x{:02x} LHLD {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        43 => println!("0x{:02x} DCX H\n", buffer[program_counter]),
        44 => println!("0x{:02x} INR L\n", buffer[program_counter]),
        45 => println!("0x{:02x} DCR L\n", buffer[program_counter]),
        46 => {
            println!("0x{:02x} MVI L  {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        47 => println!("0x{:02x} CMA\n", buffer[program_counter]),
        49 => {
            println!("0x{:02x} LXI SP  {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        50 => {
            println!("0x{:02x} STA {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        51 => println!("0x{:02x} INX SP\n", buffer[program_counter]),
        52 => println!("0x{:02x} INR M\n", buffer[program_counter]),
        53 => println!("0x{:02x} DCR M\n", buffer[program_counter]),
        54 => {
            println!("0x{:02x} MVI M {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        55 => println!("0x{:02x} STC\n", buffer[program_counter]),
        57 => println!("0x{:02x} DAD SP\n", buffer[program_counter]),
        58 => {
            println!("0x{:02x} LDA {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        59 => println!("0x{:02x} DCX SP\n", buffer[program_counter]),
        60 => println!("0x{:02x} INR A\n", buffer[program_counter]),
        61 => println!("0x{:02x} DCR A\n", buffer[program_counter]),
        62 => {
            println!("0x{:02x} MVI A {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        63 => println!("0x{:02x} CMC\n", buffer[program_counter]),
        64 => println!("0x{:02x} MOV B B\n", buffer[program_counter]),
        65 => println!("0x{:02x} MOV B C\n", buffer[program_counter]),
        66 => println!("0x{:02x} MOV B D\n", buffer[program_counter]),
        67 => println!("0x{:02x} MOV B E\n", buffer[program_counter]),
        68 => println!("0x{:02x} MOV B H\n", buffer[program_counter]),
        69 => println!("0x{:02x} MOV B L\n", buffer[program_counter]),
        70 => println!("0x{:02x} MOV B M\n", buffer[program_counter]),
        71 => println!("0x{:02x} MOV B A\n", buffer[program_counter]),
        72 => println!("0x{:02x} MOV C B\n", buffer[program_counter]),
        73 => println!("0x{:02x} MOV C C\n", buffer[program_counter]),
        74 => println!("0x{:02x} MOV C D\n", buffer[program_counter]),
        75 => println!("0x{:02x} MOV C E\n", buffer[program_counter]),
        76 => println!("0x{:02x} MOV C H\n", buffer[program_counter]),
        77 => println!("0x{:02x} MOV C L\n", buffer[program_counter]),
        78 => println!("0x{:02x} MOV C M\n", buffer[program_counter]),
        79 => println!("0x{:02x} MOV C A\n", buffer[program_counter]),
        80 => println!("0x{:02x} MOV D B\n", buffer[program_counter]),
        81 => println!("0x{:02x} MOV D C\n", buffer[program_counter]),
        82 => println!("0x{:02x} MOV D D\n", buffer[program_counter]),
        83 => println!("0x{:02x} MOV D E\n", buffer[program_counter]),
        84 => println!("0x{:02x} MOV D H\n", buffer[program_counter]),
        85 => println!("0x{:02x} MOV D L\n", buffer[program_counter]),
        86 => println!("0x{:02x} MOV D M\n", buffer[program_counter]),
        87 => println!("0x{:02x} MOV D A\n", buffer[program_counter]),
        88 => println!("0x{:02x} MOV E B\n", buffer[program_counter]),
        89 => println!("0x{:02x} MOV E C\n", buffer[program_counter]),
        90 => println!("0x{:02x} MOV E D\n", buffer[program_counter]),
        91 => println!("0x{:02x} MOV E E\n", buffer[program_counter]),
        92 => println!("0x{:02x} MOV E H\n", buffer[program_counter]),
        93 => println!("0x{:02x} MOV E L\n", buffer[program_counter]),
        94 => println!("0x{:02x} MOV E M\n", buffer[program_counter]),
        95 => println!("0x{:02x} MOV E A\n", buffer[program_counter]),
        96 => println!("0x{:02x} MOV H B\n", buffer[program_counter]),
        97 => println!("0x{:02x} MOV H C\n", buffer[program_counter]),
        98 => println!("0x{:02x} MOV H D\n", buffer[program_counter]),
        99 => println!("0x{:02x} MOV H E\n", buffer[program_counter]),
        100 => println!("0x{:02x} MOV H H\n", buffer[program_counter]),
        101 => println!("0x{:02x} MOV H L\n", buffer[program_counter]),
        102 => println!("0x{:02x} MOV H M\n", buffer[program_counter]),
        103 => println!("0x{:02x} MOV H A\n", buffer[program_counter]),
        104 => println!("0x{:02x} MOV L B\n", buffer[program_counter]),
        105 => println!("0x{:02x} MOV L C\n", buffer[program_counter]),
        106 => println!("0x{:02x} MOV L D\n", buffer[program_counter]),
        107 => println!("0x{:02x} MOV L E\n", buffer[program_counter]),
        108 => println!("0x{:02x} MOV L H\n", buffer[program_counter]),
        109 => println!("0x{:02x} MOV L L\n", buffer[program_counter]),
        110 => println!("0x{:02x} MOV L M\n", buffer[program_counter]),
        111 => println!("0x{:02x} MOV L A\n", buffer[program_counter]),
        112 => println!("0x{:02x} MOV M B\n", buffer[program_counter]),
        113 => println!("0x{:02x} MOV M C\n", buffer[program_counter]),
        114 => println!("0x{:02x} MOV M D\n", buffer[program_counter]),
        115 => println!("0x{:02x} MOV M E\n", buffer[program_counter]),
        116 => println!("0x{:02x} MOV M H\n", buffer[program_counter]),
        117 => println!("0x{:02x} MOV M L\n", buffer[program_counter]),
        118 => println!("0x{:02x} HLT\n", buffer[program_counter]),
        119 => println!("0x{:02x} MOV M A\n", buffer[program_counter]),
        120 => println!("0x{:02x} MOV A B\n", buffer[program_counter]),
        121 => println!("0x{:02x} MOV A C\n", buffer[program_counter]),
        122 => println!("0x{:02x} MOV A D\n", buffer[program_counter]),
        123 => println!("0x{:02x} MOV A E\n", buffer[program_counter]),
        124 => println!("0x{:02x} MOV A H\n", buffer[program_counter]),
        125 => println!("0x{:02x} MOV A L\n", buffer[program_counter]),
        126 => println!("0x{:02x} MOV A M\n", buffer[program_counter]),
        127 => println!("0x{:02x} MOV A A\n", buffer[program_counter]),
        128 => println!("0x{:02x} ADD B\n", buffer[program_counter]),
        129 => println!("0x{:02x} ADD C\n", buffer[program_counter]),
        130 => println!("0x{:02x} ADD D\n", buffer[program_counter]),
        131 => println!("0x{:02x} ADD E\n", buffer[program_counter]),
        132 => println!("0x{:02x} ADD H\n", buffer[program_counter]),
        133 => println!("0x{:02x} ADD L\n", buffer[program_counter]),
        134 => println!("0x{:02x} ADD M\n", buffer[program_counter]),
        135 => println!("0x{:02x} ADD A\n", buffer[program_counter]),
        136 => println!("0x{:02x} ADC B\n", buffer[program_counter]),
        137 => println!("0x{:02x} ADC C\n", buffer[program_counter]),
        138 => println!("0x{:02x} ADC D\n", buffer[program_counter]),
        139 => println!("0x{:02x} ADC E\n", buffer[program_counter]),
        140 => println!("0x{:02x} ADC H\n", buffer[program_counter]),
        141 => println!("0x{:02x} ADC L\n", buffer[program_counter]),
        142 => println!("0x{:02x} ADC M\n", buffer[program_counter]),
        143 => println!("0x{:02x} ADC A\n", buffer[program_counter]),
        144 => println!("0x{:02x} SUB B\n", buffer[program_counter]),
        145 => println!("0x{:02x} SUB C\n", buffer[program_counter]),
        146 => println!("0x{:02x} SUB D\n", buffer[program_counter]),
        147 => println!("0x{:02x} SUB E\n", buffer[program_counter]),
        148 => println!("0x{:02x} SUB H\n", buffer[program_counter]),
        149 => println!("0x{:02x} SUB L\n", buffer[program_counter]),
        150 => println!("0x{:02x} SUB M\n", buffer[program_counter]),
        151 => println!("0x{:02x} SUB A\n", buffer[program_counter]),
        152 => println!("0x{:02x} SBB B\n", buffer[program_counter]),
        153 => println!("0x{:02x} SBB C\n", buffer[program_counter]),
        154 => println!("0x{:02x} SBB D\n", buffer[program_counter]),
        155 => println!("0x{:02x} SBB E\n", buffer[program_counter]),
        156 => println!("0x{:02x} SBB H\n", buffer[program_counter]),
        157 => println!("0x{:02x} SBB L\n", buffer[program_counter]),
        158 => println!("0x{:02x} SBB M\n", buffer[program_counter]),
        159 => println!("0x{:02x} SBB A\n", buffer[program_counter]),
        160 => println!("0x{:02x} ANA B\n", buffer[program_counter]),
        161 => println!("0x{:02x} ANA C\n", buffer[program_counter]),
        162 => println!("0x{:02x} ANA D\n", buffer[program_counter]),
        163 => println!("0x{:02x} ANA E\n", buffer[program_counter]),
        164 => println!("0x{:02x} ANA H\n", buffer[program_counter]),
        165 => println!("0x{:02x} ANA L\n", buffer[program_counter]),
        166 => println!("0x{:02x} ANA M\n", buffer[program_counter]),
        167 => println!("0x{:02x} ANA A\n", buffer[program_counter]),
        168 => println!("0x{:02x} XRA B\n", buffer[program_counter]),
        169 => println!("0x{:02x} XRA C\n", buffer[program_counter]),
        170 => println!("0x{:02x} XRA D\n", buffer[program_counter]),
        171 => println!("0x{:02x} XRA E\n", buffer[program_counter]),
        172 => println!("0x{:02x} XRA H\n", buffer[program_counter]),
        173 => println!("0x{:02x} XRA L\n", buffer[program_counter]),
        174 => println!("0x{:02x} XRA M\n", buffer[program_counter]),
        175 => println!("0x{:02x} XRA A\n", buffer[program_counter]),
        176 => println!("0x{:02x} ORA B\n", buffer[program_counter]),
        177 => println!("0x{:02x} ORA C\n", buffer[program_counter]),
        178 => println!("0x{:02x} ORA D\n", buffer[program_counter]),
        179 => println!("0x{:02x} ORA E\n", buffer[program_counter]),
        180 => println!("0x{:02x} ORA H\n", buffer[program_counter]),
        181 => println!("0x{:02x} ORA L\n", buffer[program_counter]),
        182 => println!("0x{:02x} ORA M\n", buffer[program_counter]),
        183 => println!("0x{:02x} ORA A\n", buffer[program_counter]),
        184 => println!("0x{:02x} CMP B\n", buffer[program_counter]),
        185 => println!("0x{:02x} CMP C\n", buffer[program_counter]),
        186 => println!("0x{:02x} CMP D\n", buffer[program_counter]),
        187 => println!("0x{:02x} CMP E\n", buffer[program_counter]),
        188 => println!("0x{:02x} CMP H\n", buffer[program_counter]),
        189 => println!("0x{:02x} CMP L\n", buffer[program_counter]),
        190 => println!("0x{:02x} CMP M\n", buffer[program_counter]),
        191 => println!("0x{:02x} CMP A\n", buffer[program_counter]),
        192 => println!("0x{:02x} RNZ\n", buffer[program_counter]),
        193 => println!("0x{:02x} POP B\n", buffer[program_counter]),
        194 => {
            println!("0x{:02x} JNZ {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        195 => {
            println!("0x{:02x} JMP {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        196 => {
            println!("0x{:02x} CNZ {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        197 => println!("0x{:02x} PUSH B\n", buffer[program_counter]),
        198 => {
            println!("0x{:02x} ADI {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        199 => println!("0x{:02x} RST 0\n", buffer[program_counter]),
        200 => println!("0x{:02x} RZ\n", buffer[program_counter]),
        201 => println!("0x{:02x} RET\n", buffer[program_counter]),
        202 => {
            println!("0x{:02x} JZ {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        204 => {
            println!("0x{:02x} CZ {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        205 => {
            println!("0x{:02x} CALL {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        206 => {
            println!("0x{:02x} ACI {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        207 => println!("0x{:02x} RST 1\n", buffer[program_counter]),
        208 => println!("0x{:02x} RNC\n", buffer[program_counter]),
        209 => println!("0x{:02x} POP D\n", buffer[program_counter]),
        210 => {
            println!("0x{:02x} JNC {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        211 => {
            println!("0x{:02x} OUT {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        212 => {
            println!("0x{:02x} CNC {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        213 => println!("0x{:02x} PUSH D\n", buffer[program_counter]),
        214 => {
            println!("0x{:02x} SUI {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        215 => println!("0x{:02x} RST 2\n", buffer[program_counter]),
        216 => println!("0x{:02x} RC\n", buffer[program_counter]),
        218 => {
            println!("0x{:02x} JC {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        219 => {
            println!("0x{:02x} IN {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        220 => {
            println!("0x{:02x} CC {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        222 => {
            println!("0x{:02x} SBI {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        223 => println!("0x{:02x} RST 3\n", buffer[program_counter]),
        224 => println!("0x{:02x} RPO\n", buffer[program_counter]),
        225 => println!("0x{:02x} POP H\n", buffer[program_counter]),
        226 => {
            println!("0x{:02x} JPO {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        227 => println!("0x{:02x} XTHL\n", buffer[program_counter]),
        228 => {
            println!("0x{:02x} CPO {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        229 => println!("0x{:02x} PUSH H\n", buffer[program_counter]),
        230 => {
            println!("0x{:02x} ANI {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        231 => println!("0x{:02x} RST 4\n", buffer[program_counter]),
        232 => println!("0x{:02x} RPE\n", buffer[program_counter]),
        233 => println!("0x{:02x} PCHL\n", buffer[program_counter]),
        234 => {
            println!("0x{:02x} JPE {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        235 => println!("0x{:02x} XCHG\n", buffer[program_counter]),
        236 => {
            println!("0x{:02x} CPE {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        238 => {
            println!("0x{:02x} XRI {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        239 => println!("0x{:02x} RST 5\n", buffer[program_counter]),
        240 => println!("0x{:02x} RP\n", buffer[program_counter]),
        241 => println!("0x{:02x} POP PSW\n", buffer[program_counter]),
        242 => {
            println!("0x{:02x} JP {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        243 => println!("0x{:02x} DI\n", buffer[program_counter]),
        244 => {
            println!("0x{:02x} CP {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        245 => println!("0x{:02x} PUSH PSW\n", buffer[program_counter]),
        246 => {
            println!("0x{:02x} ORI {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        247 => println!("0x{:02x} RST 6\n", buffer[program_counter]),
        248 => println!("0x{:02x} RM\n", buffer[program_counter]),
        249 => println!("0x{:02x} SPHL\n", buffer[program_counter]),
        250 => {
            println!("0x{:02x} JM {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        251 => println!("0x{:02x} EI\n", buffer[program_counter]),
        252 => {
            println!("0x{:02x} CM {:02x}{:02x}\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]);
            read_bytes = 3;
        },
        254 => {
            println!("0x{:02x} CPI {:02x}\n", buffer[program_counter], buffer[program_counter + 1]);
            read_bytes = 2;
        },
        255 => println!("0x{:02x} RST 7\n", buffer[program_counter]),
        
		_   => {},
    }

    read_bytes

}