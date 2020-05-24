/*
Intel 8080 Data Book: https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf
*/

use std::mem;

mod disassembler;

fn unimplemented_opcode(){
    panic!("Unimplemented opcode");
}


#[derive(Debug, Default)]
pub struct Processor8080{
    a: u8, // ----
    b: u8, //    |
    c: u8, //    |
    d: u8, //    |--- Registers
    e: u8, //    |
    h: u8, //    |
    l: u8, // ----
    
    pub stack_pointer: u16,
    program_counter: u16,

    pub memory: Vec<u8>,

    flags: Flags,

    pub enabled: bool,

    rom_size: u16,

    testing: bool,
}

impl Processor8080{

    pub fn test(&mut self){

        self.load_file("cpudiag.bin".to_string(), 0x100, 1453);

        self.memory[368] = 0x7;

        // Skip DAA test
        self.memory[0x59C] = 0xC3;
        self.memory[0x59D] = 0xC2;
        self.memory[0x59E] = 0x05;
    
        while self.memory.len() < 0x4000{
            self.memory.push(0);
        }

        self.program_counter = 0x100;

        self.rom_size = 1453;

        self.testing = true;

    }

    pub fn initialize(&mut self){
    
        self.load_file("space-invaders-source/SpaceInvaders.h".to_string(), 0x0, 0x800);
        self.load_file("space-invaders-source/SpaceInvaders.g".to_string(), 0x800, 0x800);
        self.load_file("space-invaders-source/SpaceInvaders.f".to_string(), 0x1000, 0x800);
        self.load_file("space-invaders-source/SpaceInvaders.e".to_string(), 0x1800, 0x800);
    
        while self.memory.len() < 0x4000{
            self.memory.push(0);
        }
    
        println!("{:?}", self.memory);
    
        self.stack_pointer = 0x2400;

        self.rom_size = 0x2400;

        self.testing = false;

    }

    fn load_file(&mut self, file_name: String, offset: usize, buffer_size: usize){

        while self.memory.len() < offset{

            self.memory.push(0);

        }

        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(file_name).expect("File not found");

        let mut buffer = vec![0u8; buffer_size];

        file.read_exact(&mut buffer).expect("Failed to read file");

        self.memory.append(&mut buffer);

    }

}

#[derive(Default, Debug)]
struct Flags{
    zero: bool,
    sign: bool, // True if negative
    parity: bool, // True if even
    carry: bool,
    // auxiliary_carry: bool,
}

pub fn emulate(processor: &mut Processor8080){

    if processor.stack_pointer < processor.rom_size { // Ensuring ROM is not overwritten

        processor.stack_pointer = 0;

    }

    if processor.program_counter >= 0x4000{ // Preventing the program from trying to read outside the size of memory

        processor.program_counter = 0;

    }

    let opcode: u8 = processor.memory[processor.program_counter as usize];

    // if opcode != 0x00 && true{

        println!("\n\n==============\n\n");

        disassembler::check_opcode_8080(processor.program_counter as usize, &processor.memory);
    
        println!("Memory:\n\t0x{:x}", processor.memory[processor.program_counter as usize]);
        println!("\t0x{:x}", processor.memory[(processor.program_counter + 1) as usize]);
        println!("\t0x{:x}", processor.memory[(processor.program_counter + 2) as usize]);

        println!("Registers:\n\tA: 0x{:x}\n\tB: 0x{:x}\n\tC: 0x{:x}\n\tD: 0x{:x}\n\tE: 0x{:x}\n\tH: 0x{:x}\n\tL: 0x{:x}",
            processor.a, processor.b, processor.c, processor.d, processor.e, processor.h, processor.l
        );

        println!("Flags:\n\tZero: {}\n\tSign: {}\n\tParity: {}\n\tCarry: {}", 
            processor.flags.zero, processor.flags.sign, processor.flags.parity, processor.flags.carry
        );
    
        println!("Program Counter:\n\tDecimal: {}", processor.program_counter);
        println!("\tHex: {:x}", processor.program_counter);
    
        println!("Stack Pointer:\n\tDecimal: {}", processor.stack_pointer);
        println!("\tHex: {:x}\nMisc:\n", processor.stack_pointer);

    // }

    processor.program_counter += 1;

    match opcode {

        /********************************************
        *                  Special                  *
        ********************************************/
        //#region
        0x00 => {}, // NOP
        0xF3 => processor.enabled = false, // DI
        0xFB => processor.enabled = true, // EI
        0xD3 => {}, // OUT <- Needed
        0xDB => {}, // IN
        0x27 => {}, // DAA
        0x76 => panic!("Halting"), // HLT
        //#endregion


        /********************************************
        *               Store Register              *
        ********************************************/
        //#region
        0x02 => {
            let address = get_address_from_pair(
                &mut processor.b,
                &mut processor.c,
            );
            if address < 0x2000{
                return;
            }
            processor.memory[address as usize] = processor.a;
        }, // STAX B
        0x12 => {
            let address = get_address_from_pair(
                &mut processor.d,
                &mut processor.e,
            );
            if address < 0x2000{
                return;
            }
            processor.memory[address as usize] = processor.a;
        }, // STAX D
        0x32 => {
            let mut first_byte = processor.memory[(processor.program_counter + 1) as usize];
            let mut second_byte = processor.memory[processor.program_counter as usize];
            let address = get_address_from_pair(
                &mut first_byte,
                &mut second_byte,
            );
            if address < 0x2000{
                return;
            }
            processor.memory[address as usize] = processor.a;
            processor.program_counter += 2;
        }, // STA addr
        0x22 => {
            let mut first_byte = processor.memory[(processor.program_counter + 1) as usize];
            let mut second_byte = processor.memory[processor.program_counter as usize];
            let address = get_address_from_pair(
                &mut first_byte,
                &mut second_byte,
            );
            if address + 1 < 0x2000{
                return;
            }
            processor.memory[address as usize] = processor.l;
            processor.memory[(address + 1) as usize] = processor.h;
            processor.program_counter += 2;
        }, // SHLD addr
        0xEB => {
            mem::swap(&mut processor.h, &mut processor.d);
            mem::swap(&mut processor.l, &mut processor.e);
        }, // XCHG
        //endregion


        /********************************************
        *               Load Register               *
        ********************************************/
        //#region
        0x01 => {
            processor.b = processor.memory[(processor.program_counter + 1) as usize];
            processor.c = processor.memory[processor.program_counter as usize];
            processor.program_counter += 2;
        }, // LXI B,operand
        0x11 => {
            processor.d = processor.memory[(processor.program_counter + 1) as usize];
            processor.e = processor.memory[processor.program_counter as usize];
            processor.program_counter += 2;
        }, // LXI D,operand
        0x21 => {
            processor.h = processor.memory[(processor.program_counter + 1) as usize];
            processor.l = processor.memory[processor.program_counter as usize];
            processor.program_counter += 2;
        }, // LXI H,operand
        0x31 => {
            let mut first_byte = processor.memory[(processor.program_counter + 1) as usize];
            let mut second_byte = processor.memory[processor.program_counter as usize];
            let address = get_address_from_pair(&mut first_byte,&mut second_byte);
            processor.stack_pointer = address;
            processor.program_counter += 2;
        }, // LXI SP,operand
        0x3A => {
            let mut first_byte = processor.memory[(processor.program_counter + 1) as usize];
            let mut second_byte = processor.memory[processor.program_counter as usize];
            let address = get_address_from_pair(&mut first_byte,&mut second_byte);
            processor.a = processor.memory[address as usize];
            processor.program_counter += 2;
        }, // LDA addr
        0x2A => {
            let mut first_byte = processor.memory[(processor.program_counter + 1) as usize];
            let mut second_byte = processor.memory[processor.program_counter as usize];
            let address = get_address_from_pair(&mut first_byte,&mut second_byte);
            processor.l = processor.memory[address as usize];
            processor.h = processor.memory[(address + 1) as usize];
        }, // LHLD addr
        //#endregion


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
        0x46 => processor.b = processor.memory[get_address_from_pair(&mut processor.h, &mut processor.l) as usize], // MOV B,(HL)
        0x47 => processor.b = processor.a, // MOV B,A

        0x48 => processor.c = processor.b, // MOV C,B
        0x49 => processor.c = processor.c, // MOV C,C
        0x4A => processor.c = processor.d, // MOV C,D
        0x4B => processor.c = processor.e, // MOV C,E
        0x4C => processor.c = processor.h, // MOV C,H
        0x4D => processor.c = processor.l, // MOV C,L
        0x4E => processor.c = processor.memory[get_address_from_pair(&mut processor.h, &mut processor.l) as usize], // MOV D,(HL)
        0x4F => processor.c = processor.a, // MOV C,A

        0x50 => processor.d = processor.b, // MOV D,B
        0x51 => processor.d = processor.c, // MOV D,C
        0x52 => processor.d = processor.d, // MOV D,D
        0x53 => processor.d = processor.e, // MOV D,E
        0x54 => processor.d = processor.h, // MOV D,H
        0x55 => processor.d = processor.l, // MOV D,L
        0x56 => processor.d = processor.memory[get_address_from_pair(&mut processor.h, &mut processor.l) as usize], // MOV D(HL)
        0x57 => processor.d = processor.a, // MOV D,A

        0x58 => processor.e = processor.b, // MOV E,B
        0x59 => processor.e = processor.c, // MOV E,C
        0x5A => processor.e = processor.d, // MOV E,D
        0x5B => processor.e = processor.e, // MOV E,E
        0x5C => processor.e = processor.h, // MOV E,H
        0x5D => processor.e = processor.l, // MOV E,L
        0x5E => processor.e = processor.memory[get_address_from_pair(&mut processor.h, &mut processor.l) as usize], // MOV E,(HL)
        0x5F => processor.e = processor.a, // MOV E,A
        
        0x60 => processor.h = processor.b, // MOV H,B
        0x61 => processor.h = processor.c, // MOV H,C
        0x62 => processor.h = processor.d, // MOV H,D
        0x63 => processor.h = processor.e, // MOV H,E
        0x64 => processor.h = processor.h, // MOV H,H
        0x65 => processor.h = processor.l, // MOV H,L
        0x66 => processor.h = processor.memory[get_address_from_pair(&mut processor.h, &mut processor.l) as usize], // MOV H,(HL)
        0x67 => processor.h = processor.a, // MOV H,A
        
        0x68 => processor.l = processor.b, // MOV L,B
        0x69 => processor.l = processor.c, // MOV L,C
        0x6A => processor.l = processor.d, // MOV L,D
        0x6B => processor.l = processor.e, // MOV L,E
        0x6C => processor.l = processor.h, // MOV L,H
        0x6D => processor.l = processor.l, // MOV L,L
        0x6E => processor.l = processor.memory[get_address_from_pair(&mut processor.h, &mut processor.l) as usize], // MOV L,(HL)
        0x6F => processor.l = processor.a, // MOV L,A
        
        0x70 => {
            let result = get_hl_address_pair(processor);
            if result.is_err(){
                return;
            }
            let result = result.unwrap();
            processor.memory[result as usize] = processor.b;
        }, // MOV (HL),B
        0x71 => {
            let result = get_hl_address_pair(processor);
            if result.is_err(){
                return;
            }
            let result = result.unwrap();
            processor.memory[result as usize] = processor.c;
        }, // MOV (HL),C
        0x72 => {
            let result = get_hl_address_pair(processor);
            if result.is_err(){
                return;
            }
            let result = result.unwrap();
            processor.memory[result as usize] = processor.d;
        }, // MOV (HL),D
        0x73 => {
            let result = get_hl_address_pair(processor);
            if result.is_err(){
                return;
            }
            let result = result.unwrap();
            processor.memory[result as usize] = processor.e;
        }, // MOV (HL),E
        0x74 => {
            let result = get_hl_address_pair(processor);
            if result.is_err(){
                return;
            }
            let result = result.unwrap();
            processor.memory[result as usize] = processor.h;
        }, // MOV (HL),H
        0x75 => {
            let result = get_hl_address_pair(processor);
            if result.is_err(){
                return;
            }
            let result = result.unwrap();
            processor.memory[result as usize] = processor.l;
        }, // MOV (HL),L
        0x77 => {
            let result = get_hl_address_pair(processor);
            if result.is_err(){
                return;
            }
            let result = result.unwrap();
            processor.memory[result as usize] = processor.a;
        }, // MOV (HL),A
        
        0x78 => processor.a = processor.b, // MOV A,B
        0x79 => processor.a = processor.c, // MOV A,C
        0x7A => processor.a = processor.d, // MOV A,D
        0x7B => processor.a = processor.e, // MOV A,E
        0x7C => processor.a = processor.h, // MOV A,H
        0x7D => processor.a = processor.l, // MOV A,L
        0x7E => processor.a = processor.memory[get_address_from_pair(&mut processor.h, &mut processor.l) as usize], // MOV A,(HL)
        0x7F => processor.a = processor.a, // MOV A,A

        0x06 => {
            processor.b = processor.memory[processor.program_counter as usize];
            processor.program_counter += 1;
        }, // MVI B, D8	2		B <- byte 2
        0x0e => {
            processor.c = processor.memory[processor.program_counter as usize];
            processor.program_counter += 1;
        }, // MVI C,D8	2		C <- byte 2
        0x16 => {
            processor.d = processor.memory[processor.program_counter as usize];
            processor.program_counter += 1;
        }, // MVI D, D8	2		D <- byte 2
        0x1e => {
            processor.e = processor.memory[processor.program_counter as usize];
            processor.program_counter += 1;
        }, // MVI E,D8	2		E <- byte 2
        0x26 => {
            processor.h = processor.memory[processor.program_counter as usize];
            processor.program_counter += 1;
        }, // MVI H,D8	2		H <- byte 2
        0x2e => {
            processor.l = processor.memory[processor.program_counter as usize];
            processor.program_counter += 1;
        }, // MVI L, D8	2		L <- byte 2
        0x36 => {
            let address = get_address_from_pair(
                &mut processor.h,
                &mut processor.l,
            );
            if address < 0x2000{
                return;
            }
            processor.memory[address as usize] = processor.memory[processor.program_counter as usize];
            processor.program_counter += 1;
        }, // MVI M,D8	2		(HL) <- byte 2
        0x3e => {
            processor.a = processor.memory[processor.program_counter as usize];
            processor.program_counter += 1;
        }, // MVI A,D8	2		A <- byte 2

        0x0A => processor.a = processor.memory[get_address_from_pair(&mut processor.b, &mut processor.c) as usize], // LDAX B
        0x1A => processor.a = processor.memory[get_address_from_pair(&mut processor.d, &mut processor.e) as usize], // LDAX D
        //#endregion


        /********************************************
        *                 Double Add                *
        ********************************************/
        //#region
        0x09 => double_add(processor, processor.b, processor.c),
        0x19 => double_add(processor, processor.d, processor.e),
        0x29 => double_add(processor, processor.h, processor.l),
        0x39 => {
            let stack_pointer_split = seperate_16bit_pair(processor.stack_pointer);
            double_add(processor, stack_pointer_split.1, stack_pointer_split.0);
        },
        //#endregion


        /********************************************
        *                Inc Register               *
        ********************************************/
        //#region
        0x04 => {
            let answer: u16 = (processor.b as u16) + 1;
            step_register_flags(processor, answer);
            processor.b = answer as u8;
        }, // INR B
        0x0C => {
            let answer: u16 = (processor.c as u16) + 1;
            step_register_flags(processor, answer);
            processor.c = answer as u8;
        }, // INR C
        0x14 => {
            let answer: u16 = (processor.d as u16) + 1;
            step_register_flags(processor, answer);
            processor.d = answer as u8;
        }, // INR D
        0x1C => {
            let answer: u16 = (processor.e as u16) + 1;
            step_register_flags(processor, answer);
            processor.e = answer as u8;
        }, // INR E
        0x24 => {
            let answer: u16 = (processor.h as u16) + 1;
            step_register_flags(processor, answer);
            processor.h = answer as u8;
        }, // INR H
        0x2C => {
            let answer: u16 = (processor.l as u16) + 1;
            step_register_flags(processor, answer);
            processor.l = answer as u8;
        }, // INR L
        0x34 => {
            let address = get_hl_address_pair(processor);
            if address.is_err(){
                return;
            }
            let address = address.unwrap() as usize;
            let answer: u16 = (processor.memory[address] as u16) + 1;
            step_register_flags(processor, answer);
            processor.memory[address] = answer as u8;
        }, // INR M
        0x3C => {
            let answer: u16 = (processor.a as u16) + 1;
            step_register_flags(processor, answer);
            processor.a = answer as u8;
        }, // INR A
        //#endregion


        /********************************************
        *                Dec Register               *
        ********************************************/
        //#region
        0x05 => {
            let answer: u16 = ((processor.b as u32) + get_twos_complement(1) as u32) as u16;
            step_register_flags(processor, answer);
            processor.b = answer as u8;
        }, // DCR B
        0x0D => {
            let answer: u16 = ((processor.c as u32) + get_twos_complement(1) as u32) as u16;
            step_register_flags(processor, answer);
            processor.c = answer as u8;
        }, // DCR C
        0x15 => {
            let answer: u16 = ((processor.d as u32) + get_twos_complement(1) as u32) as u16;
            step_register_flags(processor, answer);
            processor.d = answer as u8;
        }, // DCR D
        0x1D => {
            let answer: u16 = ((processor.e as u32) + get_twos_complement(1) as u32) as u16;
            step_register_flags(processor, answer);
            processor.e = answer as u8;
        }, // DCR E
        0x25 => {
            let answer: u16 = ((processor.h as u32) + get_twos_complement(1) as u32) as u16;
            step_register_flags(processor, answer);
            processor.h = answer as u8;
        }, // DCR H
        0x2D => {
            let answer: u16 = ((processor.l as u32) + get_twos_complement(1) as u32) as u16;
            step_register_flags(processor, answer);
            processor.l = answer as u8;
        }, // DCR L
        0x35 => {
            let address = get_hl_address_pair(processor);
            if address.is_err(){
                return;
            }
            let address = address.unwrap() as usize;
            let answer: u16 = ((processor.memory[address] as u32) + get_twos_complement(1) as u32) as u16;
            step_register_flags(processor, answer);
            processor.memory[address] = answer as u8;
        }, // DCR M
        0x3D => {
            let answer: u16 = ((processor.a as u32) + get_twos_complement(1) as u32) as u16;
            step_register_flags(processor, answer);
            processor.a = answer as u8;
        }, // DCR A
        //#endregion


        /********************************************
        *                  Inc Pair                 *
        ********************************************/
        //#region
        0x03 => {
            let pair = seperate_16bit_pair(get_address_from_pair(&mut processor.b, &mut processor.c) + 1);
            processor.b = pair.0;
            processor.c = pair.1;
        }, // INX B
        0x13 => {
            let pair = seperate_16bit_pair(get_address_from_pair(&mut processor.d, &mut processor.e) + 1);
            processor.d = pair.0;
            processor.e = pair.1;
        }, // INX D
        0x23 => {
            let pair = seperate_16bit_pair(get_address_from_pair(&mut processor.h, &mut processor.l) + 1);
            processor.h = pair.0;
            processor.l = pair.1;
        }, // INX H
        0x33 => processor.stack_pointer += 1, // INX SP
        //#endregion


        /********************************************
        *                  Dec Pair                 *
        ********************************************/
        //#region
        0x0B => {
            let pair = seperate_16bit_pair(get_address_from_pair(&mut processor.b, &mut processor.c) + get_twos_complement(1));
            processor.b = pair.0;
            processor.c = pair.1;
        }, // DCX B
        0x1B => {
            let pair = seperate_16bit_pair(get_address_from_pair(&mut processor.d, &mut processor.e) + get_twos_complement(1));
            processor.d = pair.0;
            processor.e = pair.1;
        }, // DCX D
        0x2B => {
            let pair = seperate_16bit_pair(get_address_from_pair(&mut processor.h, &mut processor.l) + get_twos_complement(1));
            processor.h = pair.0;
            processor.l = pair.1;
        }, // DCX H
        0x3B => processor.stack_pointer += get_twos_complement(1), // DCX SP
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
            let address: u16 = get_address_from_pair(&mut processor.h, &mut processor.l);
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
            let address: u16 = get_address_from_pair(&mut processor.h, &mut processor.l);
            add(processor, processor.memory[address as usize].into(), processor.c as u16);
        }, // ADC M - From memory address
        0x8F => add(processor, processor.a.into(), processor.c as u16), // ADC A
        0xC6 => {
            add(processor, processor.memory[processor.program_counter as usize].into(), 0);
            processor.program_counter += 1;
        }, // ADI - Immediate
        0xCE => {
            add(processor, processor.memory[processor.program_counter as usize].into(), processor.c as u16);
            processor.program_counter += 1;
        }, // ACI - Immediate
        //#endregion


        /********************************************
        *              Subtract Opcodes             *
        ********************************************/
        //#region
        0x90 => subtract(processor, processor.b, 0), // SUB B
        0x91 => subtract(processor, processor.c, 0), // SUB C
        0x92 => subtract(processor, processor.d, 0), // SUB D
        0x93 => subtract(processor, processor.e, 0), // SUB E
        0x94 => subtract(processor, processor.h, 0), // SUB H
        0x95 => subtract(processor, processor.l, 0), // SUB L
        0x96 => {
            let address: u16 = get_address_from_pair(&mut processor.h, &mut processor.l);
            subtract(processor, processor.memory[address as usize], 0);
        }, // SUB M - From memory address
        0x97 => subtract(processor, processor.a, 0), // SUB A
        0x98 => subtract(processor, processor.b, processor.c), // SBB B
        0x99 => subtract(processor, processor.c, processor.c), // SBB C
        0x9A => subtract(processor, processor.d, processor.c), // SBB D
        0x9B => subtract(processor, processor.e, processor.c), // SBB E
        0x9C => subtract(processor, processor.h, processor.c), // SBB H
        0x9D => subtract(processor, processor.l, processor.c), // SBB L
        0x9E => {
            let address: u16 = get_address_from_pair(&mut processor.h, &mut processor.l);
            subtract(processor, processor.memory[address as usize].into(), processor.c);
        }, // SBB M - From memory address
        0x9F => subtract(processor, processor.a, processor.c), // SBB A
        0xD6 => {
            subtract(processor, processor.memory[processor.program_counter as usize], 0);
            processor.program_counter += 1;
        }, // SUI - Immediate
        0xDE => {
            subtract(processor, processor.memory[processor.program_counter as usize], processor.c);
            processor.program_counter += 1;
        }, // SBI - Immediate
        //#endregion


        /********************************************
        *               Stack Opcodes               *
        ********************************************/
        //#region
        0xC5 => push_onto_stack(processor, processor.b, processor.c), // PUSH B
        0xD5 => push_onto_stack(processor, processor.d, processor.e), // PUSH D
        0xE5 => push_onto_stack(processor, processor.h, processor.l), // PUSH H
        0xF5 => push_onto_stack(processor, processor.a, {
            let mut flags: u8 = 0b00000010;
            if processor.flags.sign{
                flags = flags | 0b10000000; // Or the flag by the last bit of the zero flag
            }
            if processor.flags.zero{
                flags = flags | 0b01000000; // Or the flag by the last bit of the zero flag
            }
            if processor.flags.parity{
                flags = flags | 0b00000100; // Or the flag by the last bit of the zero flag
            }
            if processor.flags.carry{
                flags = flags | 0b00000001; // Or the flag by the last bit of the zero flag
            }
            flags
        }), // PUSH PSW
        0xC1 => {
            processor.b = processor.memory[(processor.stack_pointer + 1) as usize];
            processor.c = processor.memory[processor.stack_pointer as usize];
            processor.stack_pointer += 2;
        }, // POP B
        0xD1 => {
            processor.d = processor.memory[(processor.stack_pointer + 1) as usize];
            processor.e = processor.memory[processor.stack_pointer as usize];
            processor.stack_pointer += 2;
        }, // POP D
        0xE1 => {
            processor.h = processor.memory[(processor.stack_pointer + 1) as usize];
            processor.l = processor.memory[processor.stack_pointer as usize];
            processor.stack_pointer += 2;
        }, // POP H
        0xF1 => {
            // SZ000P1C
            processor.a = processor.memory[(processor.stack_pointer + 1) as usize];
            let flag_values = processor.memory[processor.stack_pointer as usize];
            processor.flags.sign = flag_values & 0b10000000 != 0;
            processor.flags.zero = flag_values & 0b01000000 != 0;
            processor.flags.parity = flag_values & 0b00000100 != 0;
            processor.flags.carry = flag_values & 0b10000001 != 0;
            processor.stack_pointer += 2;
        }, // POP PSW
        0xF9 => processor.stack_pointer = get_address_from_pair(&mut processor.h, &mut processor.l), // SPHL
        0xE3 => {
            processor.h = processor.memory[processor.stack_pointer as usize + 1];
            processor.l = processor.memory[processor.stack_pointer as usize];
        }, // XTHL
        //#endregion


        /********************************************
        *                Not Opcodes                *
        ********************************************/
        //#region
        0x2F => processor.a = !processor.a, // CMA
        //#endregion


        /********************************************
        *                   Rotate                  *
        ********************************************/
        //#region
        0x07 => {
            processor.flags.carry = (processor.a & 0x80) != 0;
            processor.a = processor.a << 1;
            rotate_carry_logic(processor, 0x01, 0xFE);
        }, // RLC
        0x0F => {
            processor.flags.carry = (processor.a & 0x01) != 0;
            processor.a = processor.a >> 1;
            rotate_carry_logic(processor, 0x80, 0x7F);
        }, // RRC
        0x17 => {
            rotate_carry_logic(processor, 0x01, 0xFE);
            processor.flags.carry = (processor.a & 0x80) != 0;
            processor.a = processor.a << 1;
        }, // RAL
        0x1F => {
            rotate_carry_logic(processor, 0x80, 0x7F);
            processor.a = processor.a >> 1;
        }, // RAR
        //#endregion


        /********************************************
        *                  Compare                  *
        ********************************************/
        //#region
        0xB8 => compare(processor, processor.b), // CMP B
        0xB9 => compare(processor, processor.c), // CMP C
        0xBA => compare(processor, processor.d), // CMP D
        0xBB => compare(processor, processor.e), // CMP E
        0xBC => compare(processor, processor.h), // CMP H
        0xBD => compare(processor, processor.l), // CMP L
        0xBE => {
            let address = get_address_from_pair(&mut processor.h, &mut processor.l);
            compare(processor, processor.memory[address as usize])
        }, // CMP M
        0xBF => compare(processor, processor.a), // CMP A
        0xFE => {
            compare(processor, processor.memory[processor.program_counter as usize]);
            processor.program_counter += 1;
        }, // CPI data
        //#endregion


        /********************************************
        *                   Carry                   *
        ********************************************/
        //#region
        0x37 => processor.flags.carry = true, // CMC
        0x3F => processor.flags.carry = !processor.flags.carry, // STC
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
        0xA6 => {
            let address = get_address_from_pair(&mut processor.h, &mut processor.l);
            logical(processor, processor.memory[address as usize], |x,y| (x&y) as u16)
        }, // ANA M
        0xA7 => logical(processor, processor.a, |x,y| (x&y) as u16), // ANA A
        0xE6 => {
            logical(processor, processor.memory[processor.program_counter as usize], |x,y| (x&y) as u16);
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
        0xB6 => {
            let address = get_address_from_pair(&mut processor.h, &mut processor.l);
            logical(processor, processor.memory[address as usize], |x,y| (x|y) as u16)
        }, // ORA M
        0xB7 => logical(processor, processor.a, |x,y| (x|y) as u16),  // ORA A
        0xF6 => {
            logical(processor, processor.memory[processor.program_counter as usize], |x,y| (x|y) as u16);
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
        0xAE => {
            let address = get_address_from_pair(&mut processor.h, &mut processor.l);
            logical(processor, processor.memory[address as usize], |x,y| (x^y) as u16)
        },  // XRA M
        0xAF => logical(processor, processor.a, |x,y| (x^y) as u16), // XRA A
        0xEE => {
            logical(processor, processor.memory[processor.program_counter as usize], |x,y| (x^y) as u16);
            processor.program_counter += 1;
        }, // XRI
        //#endregion


        /********************************************
        *               Return Opcodes              *
        ********************************************/
        //#region
        0xC9 => ret(processor, true), // RET
        0xC0 => ret(processor, !processor.flags.zero), // RNZ addr
        0xC8 => ret(processor, processor.flags.zero), // RZ addr
        0xD0 => ret(processor, !processor.flags.carry), // RNC addr
        0xD8 => ret(processor, processor.flags.carry), // RC addr
        0xE0 => ret(processor, !processor.flags.parity), // RPO addr - Parity odd
        0xE8 => ret(processor, processor.flags.parity), // RPE addr - Parity even
        0xF0 => ret(processor, !processor.flags.sign), // RP addr - Positive
        0xF8 => ret(processor, processor.flags.sign), // RM addr - Minus
        //#endregion


        /********************************************
        *                RST Restart                *
        ********************************************/
        //#region
        0xC7 => reset(processor, 0), // RST 0
        0xCF => reset(processor, 8), // RST 1
        0xD7 => reset(processor, 10), // RST 2
        0xDF => reset(processor, 18), // RST 3
        0xE7 => reset(processor, 20), // RST 4
        0xEF => reset(processor, 28), // RST 5
        0xF7 => reset(processor, 30), // RST 6
        0xFF => reset(processor, 30), // RST 7
        //#endregion


        /********************************************
        *                Call Opcodes               *
        ********************************************/
        //#region
        0xCD => {
            if processor.testing{
                let addr = ((processor.memory[(processor.program_counter + 1) as usize] as u16) << 8) | (processor.memory[processor.program_counter as usize] as u16);
                if addr == 5{
                    if processor.c == 9{
                        let mut offset: u16 = ((processor.d as u16) << 8) | (processor.e as u16);
                        let mut letter = processor.memory[(offset + 3) as usize] as char;
                        let mut string: String = "".to_string();
                        while letter != '$'{
                            string += &letter.to_string();
                            offset += 1;
                            letter = processor.memory[(offset + 3) as usize] as char;
                        }
                        if string != " CPU HAS FAILED! ERROR EXIT="{
                            println!("{}", string);
                        }
                    }
                }
                else if addr == 0{
                    panic!("Just exiting");
                }
                else{
                    call(processor, true);
                }
            }
            else{
                call(processor, true);
            }
        }, // CALL addr
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
        0xE9 => processor.program_counter = get_address_from_pair(&mut processor.h, &mut processor.l),
        //#endregion

        _ => unimplemented_opcode(),

    }

}




/********************************************
*                 Addresses                 *
********************************************/
//#region
// Highest order bits at position 0, lowest order bits at position 1
fn seperate_16bit_pair(pair: u16) -> (u8, u8) {
    
    (
        ((pair >> 8) & 0xff) as u8,
        (pair & 0xff) as u8
    )

}

// byte_1 is highest order bits, byte_2 is lowest order bits
fn get_address_from_pair(byte_1: &mut u8, byte_2: &mut u8) -> u16 {

    let mut address = ((*byte_1 as u16) << 8) | (*byte_2 as u16);

    if address >= 0x4000{

        *byte_1 = 0;

        *byte_2 = 0;

        address = 0;

    }

    address

}

fn get_hl_address_pair(processor: &mut Processor8080) -> Result<u16, bool> {

    let address = get_address_from_pair(

        &mut processor.h,

        &mut processor.l,

    );

    if address < 0x2000{

        return Err(false);

    }

    Ok(address)

}
//#endregion


/********************************************
*                 Arithmetic                *
********************************************/
//#region
fn add(processor: &mut Processor8080, byte: u16, carry: u16){

    let answer: u16 = (processor.a as u16) + byte + carry;

    set_flags(answer, processor);

    processor.a = answer as u8;

}

fn subtract(processor: &mut Processor8080, byte: u8, carry: u8){
    
    let answer = (processor.a as u16) + get_twos_complement(byte) + get_twos_complement(carry);

    set_flags(answer, processor);

    processor.flags.carry = !processor.flags.carry;

    processor.a = answer as u8;
    
}

fn double_add(processor: &mut Processor8080, byte_a: u8, byte_b: u8){

    let new_address = ((processor.h as u32) << 8) | (processor.l as u32) + ((byte_a as u32) << 8) | (byte_b as u32);

    processor.flags.carry = new_address > 0xffff;

    let split_address = seperate_16bit_pair(new_address as u16);

    processor.h = split_address.1;

    processor.l = split_address.0;

}

fn rotate_carry_logic(processor: &mut Processor8080, or_value: u8, and_value: u8){

    if processor.flags.carry {

        processor.a = processor.a | or_value;

    }
    else {

        processor.a = processor.a & and_value;

    }

}

fn get_twos_complement(byte: u8) -> u16{
    
    ((!(byte as u16) as u32) + 1) as u16

}

fn step_register_flags(processor: &mut Processor8080, answer: u16){

    let carry_value = processor.flags.carry;

    set_flags(answer, processor);

    processor.flags.carry = carry_value;

}
//#endregion


/********************************************
*                  Logical                  *
********************************************/
//#region
fn logical(processor: &mut Processor8080, byte: u8, operator: fn(u8, u8) -> u16){

    let answer = operator(processor.a, byte);

    set_flags(answer, processor);

    processor.a = answer as u8;

}

fn compare(processor: &mut Processor8080, byte: u8){
    
    let answer: u8 = (processor.a as u32 + get_twos_complement(byte) as u32) as u8;

    processor.flags.zero = answer == 0;

    processor.flags.sign = (answer & 0x80) == 0x80;

    processor.flags.parity = check_parity(answer as u16);

    processor.flags.carry = processor.a < byte;

}
//#endregion


/********************************************
*                   Memory                  *
********************************************/
//#region
fn push_address_onto_stack(processor: &mut Processor8080, address: u16){
    
    let pair = seperate_16bit_pair(address);

    push_onto_stack(processor, pair.0, pair.1);

}

// byte_1 -> stack_pointer - 1, byte_2 -> stack_pointer - 2
fn push_onto_stack(processor: &mut Processor8080, byte_1: u8, byte_2: u8){
    
    processor.memory[(((processor.stack_pointer as u32) + (get_twos_complement(1) as u32)) as u16) as usize] = byte_1; // Push return address onto the stack
    processor.memory[(((processor.stack_pointer as u32) + (get_twos_complement(2) as u32)) as u16) as usize] = byte_2; // Little endian so it is pushed in reverse

    processor.stack_pointer = ((processor.stack_pointer as u32) + (get_twos_complement(2) as u32)) as u16;

}
//#endregion


/********************************************
*                   Flags                   *
********************************************/
//#region
fn set_flags(answer: u16, processor: &mut Processor8080){

    processor.flags.zero = (answer & 0xff) == 0;

    processor.flags.sign = (answer & 0x800) != 0;

    processor.flags.carry = answer > 0xff;

    processor.flags.parity = check_parity(answer & 0xff);

}

fn check_parity(mut value: u16) -> bool {

    let mut is_even = true;

    while value != 0 {

        is_even = !is_even;
        
        value = value & (value as u32 + get_twos_complement(1) as u32) as u16;

    }

    is_even

}
//#endregion


/********************************************
*                    Flow                   *
********************************************/
//#region
fn jump(processor: &mut Processor8080, flag: bool){
    
    if flag {

        processor.program_counter = get_address_from_pair(
            &mut {processor.memory[(processor.program_counter + 1) as usize]},
            &mut {processor.memory[processor.program_counter as usize]},
        );

    }
    else {

        processor.program_counter += 2;

    }

}

fn call(processor: &mut Processor8080, flag: bool){

    if flag {

        let return_address: u16 = processor.program_counter + 2;

        push_address_onto_stack(processor, return_address); // Stack overflows the program and thus overwrites the ROM

        processor.program_counter = get_address_from_pair(
            &mut {processor.memory[(processor.program_counter + 1) as usize]},
            &mut {processor.memory[processor.program_counter as usize]},
        );

    }
    else {

        processor.program_counter += 2;

    }

}

fn ret(processor: &mut Processor8080, flag: bool){

    if flag {
        
        processor.program_counter = get_address_from_pair(
            &mut {processor.memory[(processor.stack_pointer + 1) as usize]}, 
            &mut {processor.memory[processor.stack_pointer as usize]},
        );
        
        processor.stack_pointer += 2;

    }

}

fn reset(processor: &mut Processor8080, address: u16){

    push_address_onto_stack(processor, processor.program_counter);

    processor.program_counter = address;

}
//#endregion