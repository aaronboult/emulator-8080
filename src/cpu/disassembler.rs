use std::io::Write;

pub fn check_opcode_8080(program_counter: usize, buffer: &Vec<u8>, logger: &mut Box<dyn Write>) -> usize {
    
    let mut read_bytes = 1;
    
    match buffer[program_counter] {
        0 => write!(logger, "0x{:02x} NOP\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        1 => {
            write!(logger, "0x{:02x} LXI B {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        2 => write!(logger, "0x{:02x} STAX B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        3 => write!(logger, "0x{:02x} INX B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        4 => write!(logger, "0x{:02x} INR B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        5 => write!(logger, "0x{:02x} DCR B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        6 => {
            write!(logger, "0x{:02x} MVI B  {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        7 => write!(logger, "0x{:02x} RLC\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        9 => write!(logger, "0x{:02x} DAD B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        10 => write!(logger, "0x{:02x} LDAX B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        11 => write!(logger, "0x{:02x} DCX B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        12 => write!(logger, "0x{:02x} INR C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        13 => write!(logger, "0x{:02x} DCR C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        14 => {
            write!(logger, "0x{:02x} MVI C {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        15 => write!(logger, "0x{:02x} RRC\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        17 => {
            write!(logger, "0x{:02x} LXI D {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        18 => write!(logger, "0x{:02x} STAX D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        19 => write!(logger, "0x{:02x} INX D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        20 => write!(logger, "0x{:02x} INR D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        21 => write!(logger, "0x{:02x} DCR D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        22 => {
            write!(logger, "0x{:02x} MVI D  {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        23 => write!(logger, "0x{:02x} RAL\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        25 => write!(logger, "0x{:02x} DAD D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        26 => write!(logger, "0x{:02x} LDAX D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        27 => write!(logger, "0x{:02x} DCX D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        28 => write!(logger, "0x{:02x} INR E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        29 => write!(logger, "0x{:02x} DCR E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        30 => {
            write!(logger, "0x{:02x} MVI E {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        31 => write!(logger, "0x{:02x} RAR\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        33 => {
            write!(logger, "0x{:02x} LXI H {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        34 => {
            write!(logger, "0x{:02x} SHLD {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        35 => write!(logger, "0x{:02x} INX H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        36 => write!(logger, "0x{:02x} INR H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        37 => write!(logger, "0x{:02x} DCR H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        38 => {
            write!(logger, "0x{:02x} MVI H {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        39 => write!(logger, "0x{:02x} DAA\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        41 => write!(logger, "0x{:02x} DAD H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        42 => {
            write!(logger, "0x{:02x} LHLD {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        43 => write!(logger, "0x{:02x} DCX H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        44 => write!(logger, "0x{:02x} INR L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        45 => write!(logger, "0x{:02x} DCR L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        46 => {
            write!(logger, "0x{:02x} MVI L  {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        47 => write!(logger, "0x{:02x} CMA\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        49 => {
            write!(logger, "0x{:02x} LXI SP  {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        50 => {
            write!(logger, "0x{:02x} STA {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        51 => write!(logger, "0x{:02x} INX SP\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        52 => write!(logger, "0x{:02x} INR M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        53 => write!(logger, "0x{:02x} DCR M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        54 => {
            write!(logger, "0x{:02x} MVI M {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        55 => write!(logger, "0x{:02x} STC\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        57 => write!(logger, "0x{:02x} DAD SP\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        58 => {
            write!(logger, "0x{:02x} LDA {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        59 => write!(logger, "0x{:02x} DCX SP\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        60 => write!(logger, "0x{:02x} INR A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        61 => write!(logger, "0x{:02x} DCR A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        62 => {
            write!(logger, "0x{:02x} MVI A {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        63 => write!(logger, "0x{:02x} CMC\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        64 => write!(logger, "0x{:02x} MOV B B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        65 => write!(logger, "0x{:02x} MOV B C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        66 => write!(logger, "0x{:02x} MOV B D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        67 => write!(logger, "0x{:02x} MOV B E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        68 => write!(logger, "0x{:02x} MOV B H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        69 => write!(logger, "0x{:02x} MOV B L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        70 => write!(logger, "0x{:02x} MOV B M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        71 => write!(logger, "0x{:02x} MOV B A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        72 => write!(logger, "0x{:02x} MOV C B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        73 => write!(logger, "0x{:02x} MOV C C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        74 => write!(logger, "0x{:02x} MOV C D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        75 => write!(logger, "0x{:02x} MOV C E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        76 => write!(logger, "0x{:02x} MOV C H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        77 => write!(logger, "0x{:02x} MOV C L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        78 => write!(logger, "0x{:02x} MOV C M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        79 => write!(logger, "0x{:02x} MOV C A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        80 => write!(logger, "0x{:02x} MOV D B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        81 => write!(logger, "0x{:02x} MOV D C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        82 => write!(logger, "0x{:02x} MOV D D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        83 => write!(logger, "0x{:02x} MOV D E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        84 => write!(logger, "0x{:02x} MOV D H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        85 => write!(logger, "0x{:02x} MOV D L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        86 => write!(logger, "0x{:02x} MOV D M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        87 => write!(logger, "0x{:02x} MOV D A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        88 => write!(logger, "0x{:02x} MOV E B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        89 => write!(logger, "0x{:02x} MOV E C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        90 => write!(logger, "0x{:02x} MOV E D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        91 => write!(logger, "0x{:02x} MOV E E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        92 => write!(logger, "0x{:02x} MOV E H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        93 => write!(logger, "0x{:02x} MOV E L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        94 => write!(logger, "0x{:02x} MOV E M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        95 => write!(logger, "0x{:02x} MOV E A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        96 => write!(logger, "0x{:02x} MOV H B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        97 => write!(logger, "0x{:02x} MOV H C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        98 => write!(logger, "0x{:02x} MOV H D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        99 => write!(logger, "0x{:02x} MOV H E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        100 => write!(logger, "0x{:02x} MOV H H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        101 => write!(logger, "0x{:02x} MOV H L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        102 => write!(logger, "0x{:02x} MOV H M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        103 => write!(logger, "0x{:02x} MOV H A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        104 => write!(logger, "0x{:02x} MOV L B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        105 => write!(logger, "0x{:02x} MOV L C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        106 => write!(logger, "0x{:02x} MOV L D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        107 => write!(logger, "0x{:02x} MOV L E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        108 => write!(logger, "0x{:02x} MOV L H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        109 => write!(logger, "0x{:02x} MOV L L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        110 => write!(logger, "0x{:02x} MOV L M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        111 => write!(logger, "0x{:02x} MOV L A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        112 => write!(logger, "0x{:02x} MOV M B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        113 => write!(logger, "0x{:02x} MOV M C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        114 => write!(logger, "0x{:02x} MOV M D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        115 => write!(logger, "0x{:02x} MOV M E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        116 => write!(logger, "0x{:02x} MOV M H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        117 => write!(logger, "0x{:02x} MOV M L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        118 => write!(logger, "0x{:02x} HLT\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        119 => write!(logger, "0x{:02x} MOV M A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        120 => write!(logger, "0x{:02x} MOV A B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        121 => write!(logger, "0x{:02x} MOV A C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        122 => write!(logger, "0x{:02x} MOV A D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        123 => write!(logger, "0x{:02x} MOV A E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        124 => write!(logger, "0x{:02x} MOV A H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        125 => write!(logger, "0x{:02x} MOV A L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        126 => write!(logger, "0x{:02x} MOV A M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        127 => write!(logger, "0x{:02x} MOV A A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        128 => write!(logger, "0x{:02x} ADD B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        129 => write!(logger, "0x{:02x} ADD C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        130 => write!(logger, "0x{:02x} ADD D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        131 => write!(logger, "0x{:02x} ADD E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        132 => write!(logger, "0x{:02x} ADD H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        133 => write!(logger, "0x{:02x} ADD L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        134 => write!(logger, "0x{:02x} ADD M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        135 => write!(logger, "0x{:02x} ADD A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        136 => write!(logger, "0x{:02x} ADC B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        137 => write!(logger, "0x{:02x} ADC C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        138 => write!(logger, "0x{:02x} ADC D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        139 => write!(logger, "0x{:02x} ADC E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        140 => write!(logger, "0x{:02x} ADC H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        141 => write!(logger, "0x{:02x} ADC L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        142 => write!(logger, "0x{:02x} ADC M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        143 => write!(logger, "0x{:02x} ADC A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        144 => write!(logger, "0x{:02x} SUB B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        145 => write!(logger, "0x{:02x} SUB C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        146 => write!(logger, "0x{:02x} SUB D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        147 => write!(logger, "0x{:02x} SUB E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        148 => write!(logger, "0x{:02x} SUB H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        149 => write!(logger, "0x{:02x} SUB L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        150 => write!(logger, "0x{:02x} SUB M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        151 => write!(logger, "0x{:02x} SUB A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        152 => write!(logger, "0x{:02x} SBB B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        153 => write!(logger, "0x{:02x} SBB C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        154 => write!(logger, "0x{:02x} SBB D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        155 => write!(logger, "0x{:02x} SBB E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        156 => write!(logger, "0x{:02x} SBB H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        157 => write!(logger, "0x{:02x} SBB L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        158 => write!(logger, "0x{:02x} SBB M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        159 => write!(logger, "0x{:02x} SBB A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        160 => write!(logger, "0x{:02x} ANA B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        161 => write!(logger, "0x{:02x} ANA C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        162 => write!(logger, "0x{:02x} ANA D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        163 => write!(logger, "0x{:02x} ANA E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        164 => write!(logger, "0x{:02x} ANA H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        165 => write!(logger, "0x{:02x} ANA L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        166 => write!(logger, "0x{:02x} ANA M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        167 => write!(logger, "0x{:02x} ANA A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        168 => write!(logger, "0x{:02x} XRA B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        169 => write!(logger, "0x{:02x} XRA C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        170 => write!(logger, "0x{:02x} XRA D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        171 => write!(logger, "0x{:02x} XRA E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        172 => write!(logger, "0x{:02x} XRA H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        173 => write!(logger, "0x{:02x} XRA L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        174 => write!(logger, "0x{:02x} XRA M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        175 => write!(logger, "0x{:02x} XRA A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        176 => write!(logger, "0x{:02x} ORA B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        177 => write!(logger, "0x{:02x} ORA C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        178 => write!(logger, "0x{:02x} ORA D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        179 => write!(logger, "0x{:02x} ORA E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        180 => write!(logger, "0x{:02x} ORA H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        181 => write!(logger, "0x{:02x} ORA L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        182 => write!(logger, "0x{:02x} ORA M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        183 => write!(logger, "0x{:02x} ORA A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        184 => write!(logger, "0x{:02x} CMP B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        185 => write!(logger, "0x{:02x} CMP C\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        186 => write!(logger, "0x{:02x} CMP D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        187 => write!(logger, "0x{:02x} CMP E\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        188 => write!(logger, "0x{:02x} CMP H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        189 => write!(logger, "0x{:02x} CMP L\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        190 => write!(logger, "0x{:02x} CMP M\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        191 => write!(logger, "0x{:02x} CMP A\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        192 => write!(logger, "0x{:02x} RNZ\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        193 => write!(logger, "0x{:02x} POP B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        194 => {
            write!(logger, "0x{:02x} JNZ {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        195 => {
            write!(logger, "0x{:02x} JMP {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        196 => {
            write!(logger, "0x{:02x} CNZ {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        197 => write!(logger, "0x{:02x} PUSH B\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        198 => {
            write!(logger, "0x{:02x} ADI {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        199 => write!(logger, "0x{:02x} RST 0\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        200 => write!(logger, "0x{:02x} RZ\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        201 => write!(logger, "0x{:02x} RET\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        202 => {
            write!(logger, "0x{:02x} JZ {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        204 => {
            write!(logger, "0x{:02x} CZ {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        205 => {
            write!(logger, "0x{:02x} CALL {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        206 => {
            write!(logger, "0x{:02x} ACI {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        207 => write!(logger, "0x{:02x} RST 1\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        208 => write!(logger, "0x{:02x} RNC\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        209 => write!(logger, "0x{:02x} POP D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        210 => {
            write!(logger, "0x{:02x} JNC {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        211 => {
            write!(logger, "0x{:02x} OUT {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        212 => {
            write!(logger, "0x{:02x} CNC {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        213 => write!(logger, "0x{:02x} PUSH D\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        214 => {
            write!(logger, "0x{:02x} SUI {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        215 => write!(logger, "0x{:02x} RST 2\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        216 => write!(logger, "0x{:02x} RC\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        218 => {
            write!(logger, "0x{:02x} JC {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        219 => {
            write!(logger, "0x{:02x} IN {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        220 => {
            write!(logger, "0x{:02x} CC {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        222 => {
            write!(logger, "0x{:02x} SBI {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        223 => write!(logger, "0x{:02x} RST 3\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        224 => write!(logger, "0x{:02x} RPO\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        225 => write!(logger, "0x{:02x} POP H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        226 => {
            write!(logger, "0x{:02x} JPO {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        227 => write!(logger, "0x{:02x} XTHL\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        228 => {
            write!(logger, "0x{:02x} CPO {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        229 => write!(logger, "0x{:02x} PUSH H\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        230 => {
            write!(logger, "0x{:02x} ANI {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        231 => write!(logger, "0x{:02x} RST 4\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        232 => write!(logger, "0x{:02x} RPE\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        233 => write!(logger, "0x{:02x} PCHL\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        234 => {
            write!(logger, "0x{:02x} JPE {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        235 => write!(logger, "0x{:02x} XCHG\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        236 => {
            write!(logger, "0x{:02x} CPE {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        238 => {
            write!(logger, "0x{:02x} XRI {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        239 => write!(logger, "0x{:02x} RST 5\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        240 => write!(logger, "0x{:02x} RP\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        241 => write!(logger, "0x{:02x} POP PSW\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        242 => {
            write!(logger, "0x{:02x} JP {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        243 => write!(logger, "0x{:02x} DI\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        244 => {
            write!(logger, "0x{:02x} CP {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        245 => write!(logger, "0x{:02x} PUSH PSW\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        246 => {
            write!(logger, "0x{:02x} ORI {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        247 => write!(logger, "0x{:02x} RST 6\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        248 => write!(logger, "0x{:02x} RM\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        249 => write!(logger, "0x{:02x} SPHL\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        250 => {
            write!(logger, "0x{:02x} JM {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        251 => write!(logger, "0x{:02x} EI\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        252 => {
            write!(logger, "0x{:02x} CM {:02x}{:02x}\n\n", buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]).expect("Failed to write to output buffer");
            read_bytes = 3;
        },
        254 => {
            write!(logger, "0x{:02x} CPI {:02x}\n\n", buffer[program_counter], buffer[program_counter + 1]).expect("Failed to write to output buffer");
            read_bytes = 2;
        },
        255 => write!(logger, "0x{:02x} RST 7\n\n", buffer[program_counter]).expect("Failed to write to output buffer"),
        
		_   => {},
    }

    read_bytes

}