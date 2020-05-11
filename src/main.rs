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
        0x41 => processor.b = processor.c, // MOV B,C
        0x42 => processor.b = processor.d, // MOV B,D
        0x43 => processor.b = processor.e, // MOV B,E


        /********************************************
        *                 Add Opcodes               *
        ********************************************/
        0x80 => {
            let answer: u16 = (processor.a + processor.b) as u16;
            set_flags(answer, processor);
            processor.a = answer as u8;
        }, // ADD B
        0x81 => {
            let answer: u16 = (processor.a + processor.c) as u16;
            set_flags(answer, processor);
            processor.a = answer as u8;
        }, // ADD C
        0x82 => {
            let answer: u16 = (processor.a + processor.d) as u16;
            set_flags(answer, processor);
            processor.a = answer as u8;
        }, // ADD D
        0x83 => {
            let answer: u16 = (processor.a + processor.e) as u16;
            set_flags(answer, processor);
            processor.a = answer as u8;
        }, // ADD E
        0x84 => {
            let answer: u16 = (processor.a + processor.h) as u16;
            set_flags(answer, processor);
            processor.a = answer as u8;
        }, // ADD H
        0x85 => {
            let answer: u16 = (processor.a + processor.l) as u16;
            set_flags(answer, processor);
            processor.a = answer as u8;
        }, // ADD L
        0x86 => {
            let address: u16 = get_address_from_pair(processor.h, processor.l);
            let answer: u16 = (processor.a + processor.memory[address as usize]) as u16;
            set_flags(answer, processor);
            processor.a = answer as u8;
        }, // ADD M - From memory address
        0x87 => {
            let answer: u16 = (processor.a + processor.a) as u16;
            set_flags(answer, processor);
            processor.a = answer as u8;
        }, // ADD A
        0xC6 => {
            let answer: u16 = (processor.a + processor.memory[(processor.program_counter + 1) as usize]) as u16;
            set_flags(answer, processor);
            processor.program_counter += 1;
        }, // ADD Immediate


        /********************************************
        *                Not Opcodes                *
        ********************************************/
        0x2F => processor.a = !processor.a, // CMA / NOT


        /********************************************
        *                And Opcodes                *
        ********************************************/
        0xA0 => logical(processor, processor.b, |x,y| (x&y) as u16), // ANA B
        0xA1 => logical(processor, processor.c, |x,y| (x&y) as u16), // ANA C
        0xA2 => logical(processor, processor.d, |x,y| (x&y) as u16), // ANA D
        0xA3 => logical(processor, processor.e, |x,y| (x&y) as u16), // ANA E
        0xA4 => logical(processor, processor.h, |x,y| (x&y) as u16), // ANA H
        0xA5 => logical(processor, processor.l, |x,y| (x&y) as u16), // ANA L
        0xA6 => logical(processor, processor.memory[get_address_from_pair(processor.h, processor.l) as usize], |x,y| (x&y) as u16), // ANA M
        0xA7 => logical(processor, processor.a, |x,y| (x&y) as u16), // ANA A


        /********************************************
        *                 Or Opcodes                *
        ********************************************/
        0xB0 => logical(processor, processor.b, |x,y| (x|y) as u16), // ORA B
        0xB1 => logical(processor, processor.c, |x,y| (x|y) as u16), // ORA C
        0xB2 => logical(processor, processor.d, |x,y| (x|y) as u16), // ORA D
        0xB3 => logical(processor, processor.e, |x,y| (x|y) as u16), // ORA E
        0xB4 => logical(processor, processor.h, |x,y| (x|y) as u16), // ORA H
        0xB5 => logical(processor, processor.l, |x,y| (x|y) as u16), // ORA L
        0xB6 => logical(processor, processor.memory[get_address_from_pair(processor.h, processor.l) as usize], |x,y| (x|y) as u16), // ORA M
        0xB7 => logical(processor, processor.a, |x,y| (x|y) as u16),  // ORA A


        /********************************************
        *                XOR Opcodes                *
        ********************************************/
        0xA8 => logical(processor, processor.b, |x,y| (x^y) as u16), // XRA B
        0xA9 => logical(processor, processor.c, |x,y| (x^y) as u16), // XRA C
        0xAA => logical(processor, processor.d, |x,y| (x^y) as u16), // XRA D
        0xAB => logical(processor, processor.e, |x,y| (x^y) as u16), // XRA E
        0xAC => logical(processor, processor.h, |x,y| (x^y) as u16), // XRA H
        0xAD => logical(processor, processor.l, |x,y| (x^y) as u16), // XRA L
        0xAE => logical(processor, processor.memory[get_address_from_pair(processor.h, processor.l) as usize], |x,y| (x^y) as u16),  // XRA M
        0xAF => logical(processor, processor.a, |x,y| (x^y) as u16), // XRA A


        /********************************************
        *               Return Opcodes              *
        ********************************************/
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


        /********************************************
        *                Call Opcodes               *
        ********************************************/
        0xCD => {
            call(processor, true);
        } // CALL addr
        0xC4 => {
            call(processor, processor.flags.zero);
        }, // CNZ addr
        0xCC => {
            call(processor, !processor.flags.zero);
        }, // CZ addr
        0xD4 => {
            call(processor, !processor.flags.carry);
        }, // CNC addr
        0xDC => {
            call(processor, processor.flags.carry);
        }, // CC addr
        0xE4 => {
            call(processor, !processor.flags.parity);
        }, // CPO addr - Parity odd
        0xEC => {
            call(processor, processor.flags.parity);
        }, // CPE addr - Parity even
        0xF4 => {
            call(processor, !processor.flags.sign);
        }, // CP addr - Positive
        0xFC => {
            call(processor, processor.flags.sign);
        }, // CM addr - Minus


        /********************************************
        *                Jump Opcodes               *
        ********************************************/
        0xC3 => {
            jump(processor, true);
        }, // JMP addr
        0xC2 => {
            jump(processor, !processor.flags.zero);
        }, // JNZ addr
        0xCA => {
            jump(processor, processor.flags.zero);
        }, // JZ addr
        0xD2 => {
            jump(processor, !processor.flags.carry);
        }, // JNC addr
        0xDA => {
            jump(processor, processor.flags.carry);
        }, // JC addr
        0xE2 => {
            jump(processor, !processor.flags.parity);
        }, // JPO addr - Parity odd
        0xEA => {
            jump(processor, processor.flags.parity);
        }, // JPE addr - Parity even
        0xF2 => {
            jump(processor, !processor.flags.sign);
        }, // JP addr - Positive
        0xFA => {
            jump(processor, processor.flags.sign);
        }, // JM addr - Minus
        0xE9 => {
            processor.program_counter = get_address_from_pair(processor.h, processor.l);
        }

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


// byte_1 is shifted 8 bits to the left
fn get_address_from_pair(byte_1: u8, byte_2: u8) -> u16 {

    ((byte_1 as u16) << 8) | (byte_2 as u16)

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

        processor.memory[(processor.stack_pointer - 1) as usize] = ((return_address >> 8) & 0xff) as u8; // Push return address onto the stack
        processor.memory[(processor.stack_pointer - 2) as usize] = (return_address & 0xff) as u8; // Little endian so it is pushed in reverse

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