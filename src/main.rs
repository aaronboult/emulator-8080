fn main() {
    
    let mut processor = Processor8080{
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        stack_pointer: 0,
        program_counter: 0,
        memory: vec![],
        flags: Flags{
            zero: false,
            sign: false,
            parity: false,
            carry: false,
            // auxiliary_carry: false,
        },
        enabled: false,
    };

    emulate(&mut processor);

}

/*
    Intel 8080 architecure
    Little endian
*/
struct Processor8080{
    a: u8, // ----
    b: u8, //    |
    c: u8, //    |
    d: u8, //    |--- Registers
    e: u8, //    |
    h: u8, //    |
    l: u8, // ----
    
    stack_pointer: u16,
    program_counter: u16,

    memory: Vec<u8>,

    flags: Flags,

    enabled: bool,
}

struct Flags{
    zero: bool,
    sign: bool, // True if negative
    parity: bool, // True if even
    carry: bool,
    // auxiliary_carry: bool,
}

fn unimplemented_opcode(processor: &Processor8080){
    panic!("Unimplemented opcode");
}

fn emulate(processor: &mut Processor8080){

    let opcode: u8 = processor.memory[processor.program_counter as usize];
    
    processor.program_counter += 1;

    match opcode {

        0x01 => {}, // NOP

        0x02 => {
            processor.c = processor.memory[(processor.program_counter + 1) as usize];
            processor.b = processor.memory[(processor.program_counter + 2) as usize];
            processor.program_counter += 2;
        }, // LXI B,operand


        /********************************************
        *                Move Opcodes               *
        ********************************************/
        //#region
        0x40 => processor.b = processor.b, // MOV B,B
        0x41 => processor.b = processor.c, // MOV B,C
        0x42 => processor.b = processor.d, // MOV B,D
        0x43 => processor.b = processor.e, // MOV B,E
        0x44 => processor.b = processor.h, // MOV B,H
        0x45 => processor.b = processor.l, // MOV B,L
        0x46 => processor.b = processor.memory[get_address_from_pair(processor.h, processor.l) as usize], // MOV B,(HL)
        0x47 => processor.b = processor.a, // MOV B,A
        
        0x48 => processor.c = processor.b, // MOV C,B
        0x49 => processor.c = processor.c, // MOV C,C
        0x4A => processor.c = processor.d, // MOV C,D
        0x4B => processor.c = processor.e, // MOV C,E
        0x4C => processor.c = processor.h, // MOV C,H
        0x4D => processor.c = processor.l, // MOV C,L
        0x4E => processor.c = processor.memory[get_address_from_pair(processor.h, processor.l) as usize], // MOV D,(HL)
        0x4F => processor.c = processor.a, // MOV C,A
        
        0x50 => processor.d = processor.b, // MOV D,B
        0x51 => processor.d = processor.c, // MOV D,C
        0x52 => processor.d = processor.d, // MOV D,D
        0x53 => processor.d = processor.e, // MOV D,E
        0x54 => processor.d = processor.h, // MOV D,H
        0x55 => processor.d = processor.l, // MOV D,L
        0x56 => processor.d = processor.memory[get_address_from_pair(processor.h, processor.l) as usize], // MOV D(HL)
        0x57 => processor.d = processor.a, // MOV D,A
        
        0x58 => processor.e = processor.b, // MOV E,B
        0x59 => processor.e = processor.c, // MOV E,C
        0x5A => processor.e = processor.d, // MOV E,D
        0x5B => processor.e = processor.e, // MOV E,E
        0x5C => processor.e = processor.h, // MOV E,H
        0x5D => processor.e = processor.l, // MOV E,L
        0x5E => processor.e = processor.memory[get_address_from_pair(processor.h, processor.l) as usize], // MOV E,(HL)
        0x5F => processor.e = processor.a, // MOV E,A
        
        0x60 => processor.h = processor.b, // MOV H,B
        0x61 => processor.h = processor.c, // MOV H,C
        0x62 => processor.h = processor.d, // MOV H,D
        0x63 => processor.h = processor.e, // MOV H,E
        0x64 => processor.h = processor.h, // MOV H,H
        0x65 => processor.h = processor.l, // MOV H,L
        0x66 => processor.h = processor.memory[get_address_from_pair(processor.h, processor.l) as usize], // MOV H,(HL)
        0x67 => processor.h = processor.a, // MOV B,A
        
        0x68 => processor.l = processor.b, // MOV L,B
        0x69 => processor.l = processor.c, // MOV L,C
        0x6A => processor.l = processor.d, // MOV L,D
        0x6B => processor.l = processor.e, // MOV L,E
        0x6C => processor.l = processor.h, // MOV L,H
        0x6D => processor.l = processor.l, // MOV L,L
        0x6E => processor.l = processor.memory[get_address_from_pair(processor.h, processor.l) as usize], // MOV L,(HL)
        0x6F => processor.l = processor.a, // MOV L,A
        
        0x70 => processor.memory[get_address_from_pair(processor.h, processor.l) as usize] = processor.b, // MOV (HL),B
        0x71 => processor.memory[get_address_from_pair(processor.h, processor.l) as usize] = processor.c, // MOV (HL),C
        0x72 => processor.memory[get_address_from_pair(processor.h, processor.l) as usize] = processor.d, // MOV (HL),D
        0x73 => processor.memory[get_address_from_pair(processor.h, processor.l) as usize] = processor.e, // MOV (HL),E
        0x74 => processor.memory[get_address_from_pair(processor.h, processor.l) as usize] = processor.h, // MOV (HL),H
        0x75 => processor.memory[get_address_from_pair(processor.h, processor.l) as usize] = processor.l, // MOV (HL),L
        0x76 => unimplemented_opcode(processor), // CPU enters STOPPED state until the next interrupt
        0x77 => processor.memory[get_address_from_pair(processor.h, processor.l) as usize] = processor.a, // MOV (HL),A
        
        0x78 => processor.a = processor.b, // MOV A,B
        0x79 => processor.a = processor.c, // MOV A,C
        0x7A => processor.a = processor.d, // MOV A,D
        0x7B => processor.a = processor.e, // MOV A,E
        0x7C => processor.a = processor.h, // MOV A,H
        0x7D => processor.a = processor.l, // MOV A,L
        0x7E => processor.a = processor.memory[get_address_from_pair(processor.h, processor.l) as usize], // MOV A,(HL)
        0x7F => processor.a = processor.a, // MOV A,A
        //#endregion


        /********************************************
        *                  Inc Pair                 *
        ********************************************/
        //#region
        0x03 => {
            let pair = seperate_16bit_pair(get_address_from_pair(processor.b, processor.c) + 1);
            processor.b = pair.1;
            processor.c = pair.0;
        },
        0x13 => {
            let pair = seperate_16bit_pair(get_address_from_pair(processor.d, processor.e) + 1);
            processor.d = pair.1;
            processor.e = pair.0;
        },
        0x23 => {
            let pair = seperate_16bit_pair(get_address_from_pair(processor.h, processor.l) + 1);
            processor.h = pair.1;
            processor.l = pair.0;
        },
        0x33 => processor.stack_pointer += 1,
        //#endregion


        /********************************************
        *                  Dec Pair                 *
        ********************************************/
        //#region
        0x0B => {
            let pair = seperate_16bit_pair(get_address_from_pair(processor.b, processor.c) - 1);
            processor.b = pair.1;
            processor.c = pair.0;
        },
        0x1B => {
            let pair = seperate_16bit_pair(get_address_from_pair(processor.d, processor.e) - 1);
            processor.d = pair.1;
            processor.e = pair.0;
        },
        0x2B => {
            let pair = seperate_16bit_pair(get_address_from_pair(processor.h, processor.l) - 1);
            processor.h = pair.1;
            processor.l = pair.0;
        },
        0x3B => processor.stack_pointer -= 1,
        //#endregion


        /********************************************
        *                 Add Opcodes               *
        ********************************************/
        //#region
        0x80 => add(processor, processor.b.into(), 0), // ADD B
        0x81 => add(processor, processor.c.into(), 0), // ADD C
        0x82 => add(processor, processor.d.into(), 0), // ADD D
        0x83 => add(processor, processor.e.into(), 0), // ADD E
        0x84 => add(processor, processor.h.into(), 0), // ADD H
        0x85 => add(processor, processor.l.into(), 0), // ADD L
        0x86 => {
            let address: u16 = get_address_from_pair(processor.h, processor.l);
            add(processor, processor.memory[address as usize].into(), 0);
        }, // ADD M - From memory address
        0x87 => add(processor, processor.a.into(), 0), // ADD A
        0x88 => add(processor, processor.b.into(), processor.c as u16), // ADC B
        0x89 => add(processor, processor.c.into(), processor.c as u16), // ADC C
        0x8A => add(processor, processor.d.into(), processor.c as u16), // ADC D
        0x8B => add(processor, processor.e.into(), processor.c as u16), // ADC E
        0x8C => add(processor, processor.h.into(), processor.c as u16), // ADC H
        0x8D => add(processor, processor.l.into(), processor.c as u16), // ADC L
        0x8E => {
            let address: u16 = get_address_from_pair(processor.h, processor.l);
            add(processor, processor.memory[address as usize].into(), processor.c as u16);
        }, // ADC M - From memory address
        0x8F => add(processor, processor.a.into(), processor.c as u16), // ADC A
        0xC6 => {
            add(processor, processor.memory[(processor.program_counter + 1) as usize].into(), 0);
            processor.program_counter += 1;
        }, // ADI - Immediate
        0xCE => {
            add(processor, processor.memory[(processor.program_counter + 1) as usize].into(), processor.c as u16);
            processor.program_counter += 1;
        }, // ACI - Immediate
        //#endregion


        /********************************************
        *              Subtract Opcodes             *
        ********************************************/
        //#region
        0x90 => subtract(processor, processor.b.into(), 0), // SUB B
        0x91 => subtract(processor, processor.c.into(), 0), // SUB C
        0x92 => subtract(processor, processor.d.into(), 0), // SUB D
        0x93 => subtract(processor, processor.e.into(), 0), // SUB E
        0x94 => subtract(processor, processor.h.into(), 0), // SUB H
        0x95 => subtract(processor, processor.l.into(), 0), // SUB L
        0x96 => {
            let address: u16 = get_address_from_pair(processor.h, processor.l);
            subtract(processor, processor.memory[address as usize].into(), 0);
        }, // SUB M - From memory address
        0x97 => subtract(processor, processor.a.into(), 0), // SUB A
        0x98 => subtract(processor, processor.b.into(), processor.c as u16), // SBB B
        0x99 => subtract(processor, processor.c.into(), processor.c as u16), // SBB C
        0x9A => subtract(processor, processor.d.into(), processor.c as u16), // SBB D
        0x9B => subtract(processor, processor.e.into(), processor.c as u16), // SBB E
        0x9C => subtract(processor, processor.h.into(), processor.c as u16), // SBB H
        0x9D => subtract(processor, processor.l.into(), processor.c as u16), // SBB L
        0x9E => {
            let address: u16 = get_address_from_pair(processor.h, processor.l);
            subtract(processor, processor.memory[address as usize].into(), processor.c as u16);
        }, // SBB M - From memory address
        0x9F => subtract(processor, processor.a.into(), processor.c as u16), // SBB A
        0xD6 => {
            subtract(processor, processor.memory[(processor.program_counter + 1) as usize].into(), 0);
            processor.program_counter += 1;
        }, // SUI - Immediate
        0xDE => {
            subtract(processor, processor.memory[(processor.program_counter + 1) as usize].into(), processor.c as u16);
            processor.program_counter += 1;
        }, // SBI - Immediate
        //#endregion


        /********************************************
        *                Not Opcodes                *
        ********************************************/
        //#region
        0x2F => processor.a = !processor.a, // CMA / NOT
        //#endregion


        /********************************************
        *                And Opcodes                *
        ********************************************/
        //#region
        0xA0 => logical(processor, processor.b, |x,y| (x&y) as u16), // ANA B
        0xA1 => logical(processor, processor.c, |x,y| (x&y) as u16), // ANA C
        0xA2 => logical(processor, processor.d, |x,y| (x&y) as u16), // ANA D
        0xA3 => logical(processor, processor.e, |x,y| (x&y) as u16), // ANA E
        0xA4 => logical(processor, processor.h, |x,y| (x&y) as u16), // ANA H
        0xA5 => logical(processor, processor.l, |x,y| (x&y) as u16), // ANA L
        0xA6 => logical(processor, processor.memory[get_address_from_pair(processor.h, processor.l) as usize], |x,y| (x&y) as u16), // ANA M
        0xA7 => logical(processor, processor.a, |x,y| (x&y) as u16), // ANA A
        0xE6 => {
            logical(processor, processor.memory[(processor.program_counter + 1) as usize], |x,y| (x&y) as u16);
            processor.program_counter += 1;
        }, // ANI
        //#endregion


        /********************************************
        *                 Or Opcodes                *
        ********************************************/
        //#region
        0xB0 => logical(processor, processor.b, |x,y| (x|y) as u16), // ORA B
        0xB1 => logical(processor, processor.c, |x,y| (x|y) as u16), // ORA C
        0xB2 => logical(processor, processor.d, |x,y| (x|y) as u16), // ORA D
        0xB3 => logical(processor, processor.e, |x,y| (x|y) as u16), // ORA E
        0xB4 => logical(processor, processor.h, |x,y| (x|y) as u16), // ORA H
        0xB5 => logical(processor, processor.l, |x,y| (x|y) as u16), // ORA L
        0xB6 => logical(processor, processor.memory[get_address_from_pair(processor.h, processor.l) as usize], |x,y| (x|y) as u16), // ORA M
        0xB7 => logical(processor, processor.a, |x,y| (x|y) as u16),  // ORA A
        0xF6 => {
            logical(processor, processor.memory[(processor.program_counter + 1) as usize], |x,y| (x|y) as u16);
            processor.program_counter += 1;
        }, // ORI
        //#endregion


        /********************************************
        *                XOR Opcodes                *
        ********************************************/
        //#region
        0xA8 => logical(processor, processor.b, |x,y| (x^y) as u16), // XRA B
        0xA9 => logical(processor, processor.c, |x,y| (x^y) as u16), // XRA C
        0xAA => logical(processor, processor.d, |x,y| (x^y) as u16), // XRA D
        0xAB => logical(processor, processor.e, |x,y| (x^y) as u16), // XRA E
        0xAC => logical(processor, processor.h, |x,y| (x^y) as u16), // XRA H
        0xAD => logical(processor, processor.l, |x,y| (x^y) as u16), // XRA L
        0xAE => logical(processor, processor.memory[get_address_from_pair(processor.h, processor.l) as usize], |x,y| (x^y) as u16),  // XRA M
        0xAF => logical(processor, processor.a, |x,y| (x^y) as u16), // XRA A
        0xEE => {
            logical(processor, processor.memory[(processor.program_counter + 1) as usize], |x,y| (x^y) as u16);
            processor.program_counter += 1;
        }, // XRI
        //#endregion


        /********************************************
        *               Return Opcodes              *
        ********************************************/
        //#region
        0xC9 => {
            ret(processor, true);
        }, // RET
        0xC0 => {
            ret(processor, !processor.flags.zero);
        }, // RNZ addr
        0xC8 => {
            ret(processor, processor.flags.zero);
        }, // RZ addr
        0xD0 => {
            ret(processor, !processor.flags.carry);
        }, // RNC addr
        0xD8 => {
            ret(processor, processor.flags.carry);
        }, // RC addr
        0xE0 => {
            ret(processor, !processor.flags.parity);
        }, // RPO addr - Parity odd
        0xE8 => {
            ret(processor, processor.flags.parity);
        }, // RPE addr - Parity even
        0xF0 => {
            ret(processor, !processor.flags.sign);
        }, // RP addr - Positive
        0xF8 => {
            ret(processor, processor.flags.sign);
        }, // RM addr - Minus
        //#endregion


        /********************************************
        *                Call Opcodes               *
        ********************************************/
        //#region
        0xCD => call(processor, true), // CALL addr
        0xC4 => call(processor, processor.flags.zero), // CNZ addr - If the zero bit is one
        0xCC => call(processor, !processor.flags.zero), // CZ addr - If the zero bit is zero
        0xD4 => call(processor, !processor.flags.carry), // CNC addr
        0xDC => call(processor, processor.flags.carry), // CC addr
        0xE4 => call(processor, !processor.flags.parity), // CPO addr - Parity odd
        0xEC => call(processor, processor.flags.parity), // CPE addr - Parity even
        0xF4 => call(processor, !processor.flags.sign), // CP addr - Positive
        0xFC => call(processor, processor.flags.sign), // CM addr - Minus
        //#endregion


        /********************************************
        *                Jump Opcodes               *
        ********************************************/
        //#region
        0xC3 => jump(processor, true), // JMP addr
        0xC2 => jump(processor, !processor.flags.zero), // JNZ addr - If the zero bit is zero
        0xCA => jump(processor, processor.flags.zero), // JZ addr - If the zero bit is one
        0xD2 => jump(processor, !processor.flags.carry), // JNC addr
        0xDA => jump(processor, processor.flags.carry), // JC addr
        0xE2 => jump(processor, !processor.flags.parity), // JPO addr - Parity odd
        0xEA => jump(processor, processor.flags.parity), // JPE addr - Parity even
        0xF2 => jump(processor, !processor.flags.sign), // JP addr - Positive
        0xFA => jump(processor, processor.flags.sign), // JM addr - Minus
        0xE9 => processor.program_counter = get_address_from_pair(processor.h, processor.l),
        //#endregion

        _ => unimplemented_opcode(processor),
    }

}

fn check_parity(mut value: u16) -> bool {

    let mut is_even = true;

    while value != 0 {

        is_even = !is_even;

        value = value & (value - 1);

    }

    is_even

}

fn set_flags(answer: u16, processor: &mut Processor8080){

    processor.flags.zero = (answer & 0xff) == 0;

    processor.flags.sign = (answer & 0x80) != 0;

    processor.flags.carry = answer > 0xff;

    processor.flags.parity = check_parity(answer&0xff);

}


// byte_1 is highest order bits, byte_2 is lowest order bits
fn get_address_from_pair(byte_1: u8, byte_2: u8) -> u16 {

    ((byte_1 as u16) << 8) | (byte_2 as u16)

}

// Lowest order bits at position 0, highest order bits at position 1
fn seperate_16bit_pair(pair: u16) -> (u8, u8) {
    
    (
        ((pair >> 8) & 0xff) as u8,
        (pair & 0xff) as u8
    )

}

fn add(processor: &mut Processor8080, byte: u16, carry: u16){

    let answer: u16 = (processor.a as u16) + byte + carry;

    set_flags(answer, processor);

    processor.a = answer as u8;

}

fn subtract(processor: &mut Processor8080, byte: u16, carry: u16){

    let answer = (processor.a as u16) - (byte + carry);

    set_flags(answer, processor);

    processor.flags.carry = !processor.flags.carry;

    processor.a = answer as u8;
    
}

fn jump(processor: &mut Processor8080, flag: bool){
    
    if flag {

        processor.program_counter = get_address_from_pair(
            processor.memory[(processor.program_counter + 2) as usize],
            processor.memory[(processor.program_counter + 1) as usize],
        );

    }
    else {

        processor.program_counter += 2;

    }

}

fn call(processor: &mut Processor8080, flag: bool){

    if flag {

        let return_address: u16 = processor.program_counter + 2;

        let pair = seperate_16bit_pair(return_address);

        processor.memory[(processor.stack_pointer - 1) as usize] = pair.0; // Push return address onto the stack
        processor.memory[(processor.stack_pointer - 2) as usize] = pair.1; // Little endian so it is pushed in reverse

        processor.stack_pointer -= 2;

        processor.program_counter = get_address_from_pair(
            processor.memory[(processor.program_counter + 2) as usize], 
            processor.memory[(processor.program_counter + 1) as usize],
        );

    }
    else {

        processor.program_counter += 2;

    }

}

fn ret(processor: &mut Processor8080, flag: bool){

    if flag {
        
        processor.program_counter = get_address_from_pair(
            processor.memory[(processor.stack_pointer + 1) as usize], 
            processor.memory[processor.stack_pointer as usize]
        );
        
        processor.stack_pointer += 2;

    }

}

fn logical(processor: &mut Processor8080, byte: u8, operator: fn(u8, u8) -> u16){

    let answer = operator(processor.a, byte);

    set_flags(answer, processor);

    processor.a = answer as u8;

}