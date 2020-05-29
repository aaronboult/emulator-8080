/*
    Intel 8080 Data Book: https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf
    Original Repository: https://github.com/aaronboult/emulator-8080
*/

mod disassembler;

use std::mem;
use std::fs::File;
use std::io::{self, BufWriter, Write};

fn unimplemented_opcode(){
    panic!("Unimplemented opcode");
}

pub struct FileToLoad{
    pub name: String,
    pub offset: usize,
    pub size: usize,
}

pub struct Processor8080{
    a: u8, // ----
    b: u8, //    |
    c: u8, //    |
    d: u8, //    |--- Registers
    e: u8, //    |
    h: u8, //    |
    l: u8, // ----

    pub custom_registers: Vec<u16>,
    
    stack_pointer: u16,
    program_counter: u16,
    pub cycles_elapsed: u16,

    pub memory: Vec<u8>,

    flags: Flags,

    pub interrupt_enabled: bool,
    pub interrupt_value: u8,

    rom_size: u16,

    opcode_cycle_length: [u16; 256],

    input_handler: fn(&mut Self, u8, &Vec<u8>) -> u8,
    output_handler: fn(&mut Self, u8, u8, &Vec<u8>),

    testing: bool,
    pub debug: bool,

    pub logger: std::boxed::Box<dyn std::io::Write>,
}

#[derive(Default, Debug)]
struct Flags{
    zero: bool,
    sign: bool, // True if negative
    parity: bool, // True if even
    carry: bool,
    // auxiliary_carry: bool,
}

impl Processor8080{

    pub fn new(input_handler: fn(&mut Self, u8, &Vec<u8>) -> u8, output_handler: fn(&mut Self, u8, u8, &Vec<u8>), log_to_file: bool) -> Self{

        let logger;

        if log_to_file{

            logger = Box::new(BufWriter::new(File::create("log.txt").expect("Unable to create file"))) as Box<dyn Write>;

        }
        else{

            let stdout = Box::leak(Box::new(io::stdout()));

            logger = Box::new(BufWriter::new(stdout.lock())) as Box<dyn Write>

        }

        Processor8080{
            a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0,
            custom_registers: vec![],
            stack_pointer: 0, program_counter: 0,
            cycles_elapsed: 0,
            memory: vec![],
            flags: Default::default(),
            interrupt_enabled: false,
            interrupt_value: 1,
            rom_size: 0,
            input_handler: input_handler,
            output_handler: output_handler,
            testing: false,
            debug: false,
            logger: logger,

            opcode_cycle_length: 
                [
                //  0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
                    4,  10, 7,  5,  5,  5,  7,  4,  4,  10, 7,  5,  5,  5,  7,  4,  // 0
                    4,  10, 7,  5,  5,  5,  7,  4,  4,  10, 7,  5,  5,  5,  7,  4,  // 1
                    4,  10, 16, 5,  5,  5,  7,  4,  4,  10, 16, 5,  5,  5,  7,  4,  // 2
                    4,  10, 13, 5,  10, 10, 10, 4,  4,  10, 13, 5,  5,  5,  7,  4,  // 3
                    5,  5,  5,  5,  5,  5,  7,  5,  5,  5,  5,  5,  5,  5,  7,  5,  // 4
                    5,  5,  5,  5,  5,  5,  7,  5,  5,  5,  5,  5,  5,  5,  7,  5,  // 5
                    5,  5,  5,  5,  5,  5,  7,  5,  5,  5,  5,  5,  5,  5,  7,  5,  // 6
                    7,  7,  7,  7,  7,  7,  7,  7,  5,  5,  5,  5,  5,  5,  7,  5,  // 7
                    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,  // 8
                    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,  // 9
                    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,  // A
                    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4,  // B
                    5,  10, 10, 10, 11, 11, 7,  11, 5,  10, 10, 10, 11, 11, 7,  11, // C
                    5,  10, 10, 10, 11, 11, 7,  11, 5,  10, 10, 10, 11, 11, 7,  11, // D
                    5,  10, 10, 18, 11, 11, 7,  11, 5,  5,  10, 5,  11, 11, 7,  11, // E
                    5,  10, 10, 4,  11, 11, 7,  11, 5,  5,  10, 4,  11, 11, 7,  11  // F
                ],
        }
        
    }

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

    pub fn initialize(&mut self, files: Vec<FileToLoad>){

        for file in files{

            self.load_file(file.name, file.offset, file.size);

        }
    
        while self.memory.len() < 0x4000{
            self.memory.push(0);
        }
    
        write!(self.logger, "{:?}\n", self.memory);

        self.testing = false;
        
    }

    fn load_file(&mut self, file_name: String, offset: usize, buffer_size: usize){

        while self.memory.len() < offset{

            self.memory.push(0);

        }

        use std::io::Read;

        let mut file = File::open(file_name).expect("File not found");

        let mut buffer = vec![0u8; buffer_size];

        self.rom_size += buffer_size as u16;

        file.read_exact(&mut buffer).expect("Failed to read file");

        self.memory.append(&mut buffer);

    }


    pub fn generate_interrupt(&mut self){
    
        push_address_onto_stack(self, self.program_counter);
    
        self.program_counter = (8 * self.interrupt_value) as u16;
        
        self.cycles_elapsed += 11;

        self.interrupt_enabled = false;
    
    }

    pub fn emulate(&mut self, ports: &Vec<u8>){
    
        let opcode: u8 = self.memory[self.program_counter as usize];

        self.cycles_elapsed += self.opcode_cycle_length[opcode as usize];

        if opcode != 0x00 && (self.debug || false){

            self.debug_output();
    
        }
    
        self.program_counter += 1;
    
        match opcode {
    
            /********************************************
            *                  Special                  *
            ********************************************/
            //#region
            0x00 => {}, // NOP
            0xF3 => self.interrupt_enabled = false, // DI
            0xFB => self.interrupt_enabled = true, // EI
            0xD3 => {
                (self.output_handler)(self, self.memory[self.program_counter as usize], self.a, ports);
                self.program_counter += 1;
            }, // OUT
            0xDB => {
                self.a = (self.input_handler)(self, self.memory[self.program_counter as usize], ports);
                self.program_counter += 1;
            }, // IN
            0x27 => {}, // DAA
            0x76 => panic!("Halting"), // HLT
            //#endregion
    
    
            /********************************************
            *               Store Register              *
            ********************************************/
            //#region
            0x02 => {
                let address = get_address_from_pair(&mut self.b, &mut self.c);
                if address < self.rom_size{
                    return;
                }
                self.memory[address as usize] = self.a;
            }, // STAX B
            0x12 => {
                let address = get_address_from_pair(&mut self.d, &mut self.e);
                if address < self.rom_size{
                    return;
                }
                self.memory[address as usize] = self.a;
            }, // STAX D
            0x32 => {
                let mut first_byte = self.memory[(self.program_counter + 1) as usize];
                let mut second_byte = self.memory[self.program_counter as usize];
                let address = get_address_from_pair(&mut first_byte, &mut second_byte);
                if address < self.rom_size{
                    return;
                }
                self.memory[address as usize] = self.a;
                self.program_counter += 2;
            }, // STA addr
            0x22 => {
                let mut first_byte = self.memory[(self.program_counter + 1) as usize];
                let mut second_byte = self.memory[self.program_counter as usize];
                let address = get_address_from_pair(&mut first_byte, &mut second_byte);
                if address + 1 < self.rom_size{
                    return;
                }
                self.memory[address as usize] = self.l;
                self.memory[(address + 1) as usize] = self.h;
                self.program_counter += 2;
            }, // SHLD addr
            0xEB => {
                mem::swap(&mut self.h, &mut self.d);
                mem::swap(&mut self.l, &mut self.e);
            }, // XCHG
            //endregion
    
    
            /********************************************
            *               Load Register               *
            ********************************************/
            //#region
            0x01 => {
                self.b = self.memory[(self.program_counter + 1) as usize];
                self.c = self.memory[self.program_counter as usize];
                self.program_counter += 2;
            }, // LXI B,operand
            0x11 => {
                self.d = self.memory[(self.program_counter + 1) as usize];
                self.e = self.memory[self.program_counter as usize];
                self.program_counter += 2;
            }, // LXI D,operand
            0x21 => {
                self.h = self.memory[(self.program_counter + 1) as usize];
                self.l = self.memory[self.program_counter as usize];
                self.program_counter += 2;
            }, // LXI H,operand
            0x31 => {
                let mut first_byte = self.memory[(self.program_counter + 1) as usize];
                let mut second_byte = self.memory[self.program_counter as usize];
                let address = get_address_from_pair(&mut first_byte, &mut second_byte);
                self.stack_pointer = address;
                self.program_counter += 2;
            }, // LXI SP,operand
            0x3A => {
                let mut first_byte = self.memory[(self.program_counter + 1) as usize];
                let mut second_byte = self.memory[self.program_counter as usize];
                let address = get_address_from_pair(&mut first_byte, &mut second_byte);
                self.a = self.memory[address as usize];
                self.program_counter += 2;
            }, // LDA addr
            0x2A => {
                let mut first_byte = self.memory[(self.program_counter + 1) as usize];
                let mut second_byte = self.memory[self.program_counter as usize];
                let address = get_address_from_pair(&mut first_byte, &mut second_byte);
                self.l = self.memory[address as usize];
                self.h = self.memory[(address + 1) as usize];
            }, // LHLD addr
            0x0A => self.a = self.memory[get_address_from_pair(&mut self.b, &mut self.c) as usize], // LDAX B
            0x1A => self.a = self.memory[get_address_from_pair(&mut self.d, &mut self.e) as usize], // LDAX D
            //#endregion
    
    
            /********************************************
            *                Move Opcodes               *
            ********************************************/
            //#region
            0x40 => self.b = self.b, // MOV B,B
            0x41 => self.b = self.c, // MOV B,C
            0x42 => self.b = self.d, // MOV B,D
            0x43 => self.b = self.e, // MOV B,E
            0x44 => self.b = self.h, // MOV B,H
            0x45 => self.b = self.l, // MOV B,L
            0x46 => self.b = self.memory[get_address_from_pair(&mut self.h, &mut self.l) as usize], // MOV B,(HL)
            0x47 => self.b = self.a, // MOV B,A
    
            0x48 => self.c = self.b, // MOV C,B
            0x49 => self.c = self.c, // MOV C,C
            0x4A => self.c = self.d, // MOV C,D
            0x4B => self.c = self.e, // MOV C,E
            0x4C => self.c = self.h, // MOV C,H
            0x4D => self.c = self.l, // MOV C,L
            0x4E => self.c = self.memory[get_address_from_pair(&mut self.h, &mut self.l) as usize], // MOV D,(HL)
            0x4F => self.c = self.a, // MOV C,A
    
            0x50 => self.d = self.b, // MOV D,B
            0x51 => self.d = self.c, // MOV D,C
            0x52 => self.d = self.d, // MOV D,D
            0x53 => self.d = self.e, // MOV D,E
            0x54 => self.d = self.h, // MOV D,H
            0x55 => self.d = self.l, // MOV D,L
            0x56 => self.d = self.memory[get_address_from_pair(&mut self.h, &mut self.l) as usize], // MOV D(HL)
            0x57 => self.d = self.a, // MOV D,A
    
            0x58 => self.e = self.b, // MOV E,B
            0x59 => self.e = self.c, // MOV E,C
            0x5A => self.e = self.d, // MOV E,D
            0x5B => self.e = self.e, // MOV E,E
            0x5C => self.e = self.h, // MOV E,H
            0x5D => self.e = self.l, // MOV E,L
            0x5E => self.e = self.memory[get_address_from_pair(&mut self.h, &mut self.l) as usize], // MOV E,(HL)
            0x5F => self.e = self.a, // MOV E,A
            
            0x60 => self.h = self.b, // MOV H,B
            0x61 => self.h = self.c, // MOV H,C
            0x62 => self.h = self.d, // MOV H,D
            0x63 => self.h = self.e, // MOV H,E
            0x64 => self.h = self.h, // MOV H,H
            0x65 => self.h = self.l, // MOV H,L
            0x66 => self.h = self.memory[get_address_from_pair(&mut self.h, &mut self.l) as usize], // MOV H,(HL)
            0x67 => self.h = self.a, // MOV H,A
            
            0x68 => self.l = self.b, // MOV L,B
            0x69 => self.l = self.c, // MOV L,C
            0x6A => self.l = self.d, // MOV L,D
            0x6B => self.l = self.e, // MOV L,E
            0x6C => self.l = self.h, // MOV L,H
            0x6D => self.l = self.l, // MOV L,L
            0x6E => self.l = self.memory[get_address_from_pair(&mut self.h, &mut self.l) as usize], // MOV L,(HL)
            0x6F => self.l = self.a, // MOV L,A
            
            0x70 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l) as usize;
                if (address as u16) < self.rom_size{
                    return;
                }
                self.memory[address] = self.b;
            }, // MOV (HL),B
            0x71 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l) as usize;
                if (address as u16) < self.rom_size{
                    return;
                }
                self.memory[address] = self.c;
            }, // MOV (HL),C
            0x72 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l) as usize;
                if (address as u16) < self.rom_size{
                    return;
                }
                self.memory[address] = self.d;
            }, // MOV (HL),D
            0x73 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l) as usize;
                if (address as u16) < self.rom_size{
                    return;
                }
                self.memory[address] = self.e;
            }, // MOV (HL),E
            0x74 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l) as usize;
                if (address as u16) < self.rom_size{
                    return;
                }
                self.memory[address] = self.h;
            }, // MOV (HL),H
            0x75 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l) as usize;
                if (address as u16) < self.rom_size{
                    return;
                }
                self.memory[address] = self.l;
            }, // MOV (HL),L
            0x77 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l) as usize;
                if (address as u16) < self.rom_size{
                    return;
                }
                self.memory[address] = self.a;
            }, // MOV (HL),A
            
            0x78 => self.a = self.b, // MOV A,B
            0x79 => self.a = self.c, // MOV A,C
            0x7A => self.a = self.d, // MOV A,D
            0x7B => self.a = self.e, // MOV A,E
            0x7C => self.a = self.h, // MOV A,H
            0x7D => self.a = self.l, // MOV A,L
            0x7E => self.a = self.memory[get_address_from_pair(&mut self.h, &mut self.l) as usize], // MOV A,(HL)
            0x7F => self.a = self.a, // MOV A,A
    
            0x06 => {
                self.b = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI B,D8
            0x0e => {
                self.c = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI C,D8
            0x16 => {
                self.d = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI D,D8
            0x1e => {
                self.e = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI E,D8
            0x26 => {
                self.h = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI H,D8
            0x2e => {
                self.l = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI L,D8
            0x36 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l);
                if address < self.rom_size{
                    return;
                }
                self.memory[address as usize] = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI M,D8
            0x3e => {
                self.a = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI A,D8
            //#endregion
    
    
            /********************************************
            *                 Double Add                *
            ********************************************/
            //#region
            0x09 => double_add(self, self.b, self.c), // DAD B
            0x19 => double_add(self, self.d, self.e), // DAD D
            0x29 => double_add(self, self.h, self.l), // DAD H
            0x39 => {
                let stack_pointer_split = seperate_16bit_pair(self.stack_pointer);
                double_add(self, stack_pointer_split.0, stack_pointer_split.1);
            }, // DAD SP
            //#endregion
    
    
            /********************************************
            *                Inc Register               *
            ********************************************/
            //#region
            0x04 => {
                let answer: u16 = (self.b as u16) + 1;
                step_register_flags(self, answer);
                self.b = answer as u8;
            }, // INR B
            0x0C => {
                let answer: u16 = (self.c as u16) + 1;
                step_register_flags(self, answer);
                self.c = answer as u8;
            }, // INR C
            0x14 => {
                let answer: u16 = (self.d as u16) + 1;
                step_register_flags(self, answer);
                self.d = answer as u8;
            }, // INR D
            0x1C => {
                let answer: u16 = (self.e as u16) + 1;
                step_register_flags(self, answer);
                self.e = answer as u8;
            }, // INR E
            0x24 => {
                let answer: u16 = (self.h as u16) + 1;
                step_register_flags(self, answer);
                self.h = answer as u8;
            }, // INR H
            0x2C => {
                let answer: u16 = (self.l as u16) + 1;
                step_register_flags(self, answer);
                self.l = answer as u8;
            }, // INR L
            0x34 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l) as usize;
                if (address as u16) < self.rom_size{
                    return;
                }
                let answer: u16 = (self.memory[address] as u16) + 1;
                step_register_flags(self, answer);
                self.memory[address] = answer as u8;
            }, // INR M
            0x3C => {
                let answer: u16 = (self.a as u16) + 1;
                step_register_flags(self, answer);
                self.a = answer as u8;
            }, // INR A
            //#endregion
    
    
            /********************************************
            *                Dec Register               *
            ********************************************/
            //#region
            0x05 => {
                let answer: u16 = ((self.b as u32) + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.b = answer as u8;
            }, // DCR B
            0x0D => {
                let answer: u16 = ((self.c as u32) + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.c = answer as u8;
            }, // DCR C
            0x15 => {
                let answer: u16 = ((self.d as u32) + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.d = answer as u8;
            }, // DCR D
            0x1D => {
                let answer: u16 = ((self.e as u32) + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.e = answer as u8;
            }, // DCR E
            0x25 => {
                let answer: u16 = ((self.h as u32) + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.h = answer as u8;
            }, // DCR H
            0x2D => {
                let answer: u16 = ((self.l as u32) + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.l = answer as u8;
            }, // DCR L
            0x35 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l) as usize;
                if (address as u16) < self.rom_size{
                    return;
                }
                let answer: u16 = ((self.memory[address] as u32) + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.memory[address] = answer as u8;
            }, // DCR M
            0x3D => {
                let answer: u16 = ((self.a as u32) + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.a = answer as u8;
            }, // DCR A
            //#endregion
    
    
            /********************************************
            *                  Inc Pair                 *
            ********************************************/
            //#region
            0x03 => {
                let pair = seperate_16bit_pair(get_address_from_pair(&mut self.b, &mut self.c) + 1);
                self.b = pair.0;
                self.c = pair.1;
            }, // INX B
            0x13 => {
                let pair = seperate_16bit_pair(get_address_from_pair(&mut self.d, &mut self.e) + 1);
                self.d = pair.0;
                self.e = pair.1;
            }, // INX D
            0x23 => {
                let pair = seperate_16bit_pair(get_address_from_pair(&mut self.h, &mut self.l) + 1);
                self.h = pair.0;
                self.l = pair.1;
            }, // INX H
            0x33 => self.stack_pointer += 1, // INX SP
            //#endregion
    
    
            /********************************************
            *                  Dec Pair                 *
            ********************************************/
            //#region
            0x0B => {
                let pair = seperate_16bit_pair((get_address_from_pair(&mut self.b, &mut self.c) as u32 + get_twos_complement(1) as u32) as u16);
                self.b = pair.0;
                self.c = pair.1;
            }, // DCX B
            0x1B => {
                let pair = seperate_16bit_pair((get_address_from_pair(&mut self.d, &mut self.e) as u32 + get_twos_complement(1) as u32) as u16);
                self.d = pair.0;
                self.e = pair.1;
            }, // DCX D
            0x2B => {
                let pair = seperate_16bit_pair((get_address_from_pair(&mut self.h, &mut self.l) as u32 + get_twos_complement(1) as u32) as u16);
                self.h = pair.0;
                self.l = pair.1;
            }, // DCX H
            0x3B => self.stack_pointer = (self.stack_pointer as u32 + get_twos_complement(1) as u32) as u16, // DCX SP
            //#endregion
    
    
            /********************************************
            *                 Add Opcodes               *
            ********************************************/
            //#region
            0x80 => add(self, self.b, false), // ADD B
            0x81 => add(self, self.c, false), // ADD C
            0x82 => add(self, self.d, false), // ADD D
            0x83 => add(self, self.e, false), // ADD E
            0x84 => add(self, self.h, false), // ADD H
            0x85 => add(self, self.l, false), // ADD L
            0x86 => {
                let address: u16 = get_address_from_pair(&mut self.h, &mut self.l);
                add(self, self.memory[address as usize], false);
            }, // ADD M - From memory address
            0x87 => add(self, self.a, false), // ADD A
            0x88 => add(self, self.b, self.flags.carry), // ADC B
            0x89 => add(self, self.c, self.flags.carry), // ADC C
            0x8A => add(self, self.d, self.flags.carry), // ADC D
            0x8B => add(self, self.e, self.flags.carry), // ADC E
            0x8C => add(self, self.h, self.flags.carry), // ADC H
            0x8D => add(self, self.l, self.flags.carry), // ADC L
            0x8E => {
                let address: u16 = get_address_from_pair(&mut self.h, &mut self.l);
                add(self, self.memory[address as usize], self.flags.carry);
            }, // ADC M - From memory address
            0x8F => add(self, self.a, self.flags.carry), // ADC A
            0xC6 => {
                add(self, self.memory[self.program_counter as usize], false);
                self.program_counter += 1;
            }, // ADI - Immediate
            0xCE => {
                add(self, self.memory[self.program_counter as usize], self.flags.carry);
                self.program_counter += 1;
            }, // ACI - Immediate
            //#endregion
    
    
            /********************************************
            *              Subtract Opcodes             *
            ********************************************/
            //#region
            0x90 => subtract(self, self.b, 0), // SUB B
            0x91 => subtract(self, self.c, 0), // SUB C
            0x92 => subtract(self, self.d, 0), // SUB D
            0x93 => subtract(self, self.e, 0), // SUB E
            0x94 => subtract(self, self.h, 0), // SUB H
            0x95 => subtract(self, self.l, 0), // SUB L
            0x96 => {
                let address: u16 = get_address_from_pair(&mut self.h, &mut self.l);
                subtract(self, self.memory[address as usize], 0);
            }, // SUB M - From memory address
            0x97 => subtract(self, self.a, 0), // SUB A
            0x98 => subtract(self, self.b, self.c), // SBB B
            0x99 => subtract(self, self.c, self.c), // SBB C
            0x9A => subtract(self, self.d, self.c), // SBB D
            0x9B => subtract(self, self.e, self.c), // SBB E
            0x9C => subtract(self, self.h, self.c), // SBB H
            0x9D => subtract(self, self.l, self.c), // SBB L
            0x9E => {
                let address: u16 = get_address_from_pair(&mut self.h, &mut self.l);
                subtract(self, self.memory[address as usize].into(), self.c);
            }, // SBB M - From memory address
            0x9F => subtract(self, self.a, self.c), // SBB A
            0xD6 => {
                subtract(self, self.memory[self.program_counter as usize], 0);
                self.program_counter += 1;
            }, // SUI - Immediate
            0xDE => {
                subtract(self, self.memory[self.program_counter as usize], self.c);
                self.program_counter += 1;
            }, // SBI - Immediate
            //#endregion
    
    
            /********************************************
            *               Stack Opcodes               *
            ********************************************/
            //#region
            0xC5 => push_onto_stack(self, self.b, self.c), // PUSH B
            0xD5 => push_onto_stack(self, self.d, self.e), // PUSH D
            0xE5 => push_onto_stack(self, self.h, self.l), // PUSH H
            0xF5 => push_onto_stack(self, self.a, {
                // Format: S Z 0 AC 0 P 1 C
                let mut flags: u8 = 0b00000010;
                if self.flags.sign{
                    flags = flags | 0b10000000;
                }
                if self.flags.zero{
                    flags = flags | 0b01000000;
                }
                if self.flags.parity{
                    flags = flags | 0b00000100;
                }
                if self.flags.carry{
                    flags = flags | 0b00000001;
                }
                flags
            }), // PUSH PSW
            0xC1 => {
                self.b = self.memory[(self.stack_pointer + 1) as usize];
                self.c = self.memory[self.stack_pointer as usize];
                self.stack_pointer += 2;
            }, // POP B
            0xD1 => {
                self.d = self.memory[(self.stack_pointer + 1) as usize];
                self.e = self.memory[self.stack_pointer as usize];
                self.stack_pointer += 2;
            }, // POP D
            0xE1 => {
                self.h = self.memory[(self.stack_pointer + 1) as usize];
                self.l = self.memory[self.stack_pointer as usize];
                self.stack_pointer += 2;
            }, // POP H
            0xF1 => {
                // Format: S Z 0 AC 0 P 1 C
                self.a = self.memory[(self.stack_pointer + 1) as usize];
                let flag_values = self.memory[self.stack_pointer as usize];
                self.flags.sign = flag_values & 0b10000000 != 0;
                self.flags.zero = flag_values & 0b01000000 != 0;
                self.flags.parity = flag_values & 0b00000100 != 0;
                self.flags.carry = flag_values & 0b00000001 != 0;
                self.stack_pointer += 2;
            }, // POP PSW
            0xF9 => self.stack_pointer = get_address_from_pair(&mut self.h, &mut self.l), // SPHL
            0xE3 => {
                mem::swap(&mut self.h, &mut self.memory[(self.stack_pointer + 1) as usize]);
                mem::swap(&mut self.l, &mut self.memory[self.stack_pointer as usize]);
            }, // XTHL
            //#endregion
    
    
            /********************************************
            *                Not Opcodes                *
            ********************************************/
            //#region
            0x2F => self.a = !self.a, // CMA
            //#endregion
    
    
            /********************************************
            *                   Rotate                  *
            ********************************************/
            //#region
            0x07 => {
                self.flags.carry = (self.a & 0b10000000) != 0; // Set the carry bit equal to the highest order bit of the accumulator
                self.a = self.a << 1;
                rotate_carry_logic(self, 0b00000001, 0b11111110); // Handle high order bit to low order bit transfer
            }, // RLC
            0x0F => {
                self.flags.carry = (self.a & 0b00000001) != 0; // Set the carry bit equal to the lowest order bit of the accumulator
                self.a = self.a >> 1;
                rotate_carry_logic(self, 0b10000000, 0b01111111); // Handle the low order bit to high order bit transfer
            }, // RRC
            0x17 => {
                let carry_temp = (self.a & 0b10000000) != 0; // Store the highest order bit as the new carry bit value
                self.a = self.a << 1; // Perform the shift, destroying the new carry bit value (though it is still stored)
                rotate_carry_logic(self, 0b00000001, 0b11111110); // Handle the carry bit to lowest order bit transfer
                self.flags.carry = carry_temp; // Set the new carry bit
            }, // RAL
            0x1F => {
                let carry_temp = (self.a & 0b00000001) != 0; // Store the lowest order bit as the new carry bit value
                self.a = self.a >> 1; // Perform the shift, destroying the new carry bit value (though it is still stored)
                rotate_carry_logic(self, 0b10000000, 0b01111111); // Handle the carry bit to highest order bit transfer
                self.flags.carry = carry_temp; // Set the new carry bit
            }, // RAR
            //#endregion
    
    
            /********************************************
            *                  Compare                  *
            ********************************************/
            //#region
            0xB8 => compare(self, self.b), // CMP B
            0xB9 => compare(self, self.c), // CMP C
            0xBA => compare(self, self.d), // CMP D
            0xBB => compare(self, self.e), // CMP E
            0xBC => compare(self, self.h), // CMP H
            0xBD => compare(self, self.l), // CMP L
            0xBE => {
                let address = get_address_from_pair(&mut self.h, &mut self.l);
                compare(self, self.memory[address as usize])
            }, // CMP M
            0xBF => compare(self, self.a), // CMP A
            0xFE => {
                compare(self, self.memory[self.program_counter as usize]);
                self.program_counter += 1;
            }, // CPI data
            //#endregion
    
    
            /********************************************
            *                   Carry                   *
            ********************************************/
            //#region
            0x37 => self.flags.carry = true, // STC
            0x3F => self.flags.carry = !self.flags.carry, // CMC
            //#endregion
    
    
            /********************************************
            *                And Opcodes                *
            ********************************************/
            //#region
            0xA0 => logical(self, self.b, |x,y| (x&y) as u16), // ANA B
            0xA1 => logical(self, self.c, |x,y| (x&y) as u16), // ANA C
            0xA2 => logical(self, self.d, |x,y| (x&y) as u16), // ANA D
            0xA3 => logical(self, self.e, |x,y| (x&y) as u16), // ANA E
            0xA4 => logical(self, self.h, |x,y| (x&y) as u16), // ANA H
            0xA5 => logical(self, self.l, |x,y| (x&y) as u16), // ANA L
            0xA6 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l);
                logical(self, self.memory[address as usize], |x,y| (x&y) as u16)
            }, // ANA M
            0xA7 => logical(self, self.a, |x,y| (x&y) as u16), // ANA A
            0xE6 => {
                logical(self, self.memory[self.program_counter as usize], |x,y| (x&y) as u16);
                self.program_counter += 1;
            }, // ANI
            //#endregion
    
    
            /********************************************
            *                 Or Opcodes                *
            ********************************************/
            //#region
            0xB0 => logical(self, self.b, |x,y| (x|y) as u16), // ORA B
            0xB1 => logical(self, self.c, |x,y| (x|y) as u16), // ORA C
            0xB2 => logical(self, self.d, |x,y| (x|y) as u16), // ORA D
            0xB3 => logical(self, self.e, |x,y| (x|y) as u16), // ORA E
            0xB4 => logical(self, self.h, |x,y| (x|y) as u16), // ORA H
            0xB5 => logical(self, self.l, |x,y| (x|y) as u16), // ORA L
            0xB6 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l);
                logical(self, self.memory[address as usize], |x,y| (x|y) as u16)
            }, // ORA M
            0xB7 => logical(self, self.a, |x,y| (x|y) as u16),  // ORA A
            0xF6 => {
                logical(self, self.memory[self.program_counter as usize], |x,y| (x|y) as u16);
                self.program_counter += 1;
            }, // ORI
            //#endregion
    
    
            /********************************************
            *                XOR Opcodes                *
            ********************************************/
            //#region
            0xA8 => logical(self, self.b, |x,y| (x^y) as u16), // XRA B
            0xA9 => logical(self, self.c, |x,y| (x^y) as u16), // XRA C
            0xAA => logical(self, self.d, |x,y| (x^y) as u16), // XRA D
            0xAB => logical(self, self.e, |x,y| (x^y) as u16), // XRA E
            0xAC => logical(self, self.h, |x,y| (x^y) as u16), // XRA H
            0xAD => logical(self, self.l, |x,y| (x^y) as u16), // XRA L
            0xAE => {
                let address = get_address_from_pair(&mut self.h, &mut self.l);
                logical(self, self.memory[address as usize], |x,y| (x^y) as u16)
            },  // XRA M
            0xAF => logical(self, self.a, |x,y| (x^y) as u16), // XRA A
            0xEE => {
                logical(self, self.memory[self.program_counter as usize], |x,y| (x^y) as u16);
                self.program_counter += 1;
            }, // XRI
            //#endregion
    
    
            /********************************************
            *               Return Opcodes              *
            ********************************************/
            //#region
            0xC9 => ret(self, true), // RET
            0xC0 => ret(self, !self.flags.zero), // RNZ addr
            0xC8 => ret(self, self.flags.zero), // RZ addr
            0xD0 => ret(self, !self.flags.carry), // RNC addr
            0xD8 => ret(self, self.flags.carry), // RC addr
            0xE0 => ret(self, !self.flags.parity), // RPO addr - Parity odd
            0xE8 => ret(self, self.flags.parity), // RPE addr - Parity even
            0xF0 => ret(self, !self.flags.sign), // RP addr - Positive
            0xF8 => ret(self, self.flags.sign), // RM addr - Minus
            //#endregion
    
    
            /********************************************
            *                RST Restart                *
            ********************************************/
            //#region
            0xC7 => reset(self, 0), // RST 0
            0xCF => reset(self, 8), // RST 1
            0xD7 => reset(self, 10), // RST 2
            0xDF => reset(self, 18), // RST 3
            0xE7 => reset(self, 20), // RST 4
            0xEF => reset(self, 28), // RST 5
            0xF7 => reset(self, 30), // RST 6
            0xFF => reset(self, 38), // RST 7
            //#endregion
    
    
            /********************************************
            *                Call Opcodes               *
            ********************************************/
            //#region
            0xCD => { // During the CPUDIAG test, a call to 0x05 is used to attempt to print information, so that behaviour is replicated here
                if self.testing{
                    let addr = ((self.memory[(self.program_counter + 1) as usize] as u16) << 8) | (self.memory[self.program_counter as usize] as u16);
                    if addr == 5{
                        if self.c == 9{
                            let mut offset: u16 = ((self.d as u16) << 8) | (self.e as u16);
                            let mut letter = self.memory[(offset + 3) as usize] as char;
                            let mut string: String = "".to_string();
                            while letter != '$'{
                                string += &letter.to_string();
                                offset += 1;
                                letter = self.memory[(offset + 3) as usize] as char;
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
                        call(self, true);
                    }
                }
                else{
                    call(self, true);
                }
            }, // CALL addr
            0xC4 => call(self, self.flags.zero), // CNZ addr - If the zero bit is one
            0xCC => call(self, !self.flags.zero), // CZ addr - If the zero bit is zero
            0xD4 => call(self, !self.flags.carry), // CNC addr
            0xDC => call(self, self.flags.carry), // CC addr
            0xE4 => call(self, !self.flags.parity), // CPO addr - Parity odd
            0xEC => call(self, self.flags.parity), // CPE addr - Parity even
            0xF4 => call(self, !self.flags.sign), // CP addr - Positive
            0xFC => call(self, self.flags.sign), // CM addr - Minus
            //#endregion
    
    
            /********************************************
            *                Jump Opcodes               *
            ********************************************/
            //#region
            0xC3 => jump(self, true), // JMP addr
            0xC2 => jump(self, !self.flags.zero), // JNZ addr - If the zero bit is zero
            0xCA => jump(self, self.flags.zero), // JZ addr - If the zero bit is one
            0xD2 => jump(self, !self.flags.carry), // JNC addr
            0xDA => jump(self, self.flags.carry), // JC addr
            0xE2 => jump(self, !self.flags.parity), // JPO addr - Parity odd
            0xEA => jump(self, self.flags.parity), // JPE addr - Parity even
            0xF2 => jump(self, !self.flags.sign), // JP addr - Positive
            0xFA => jump(self, self.flags.sign), // JM addr - Minus
            0xE9 => self.program_counter = get_address_from_pair(&mut self.h, &mut self.l), // PCHL
            //#endregion
    
            _ => {
                write!(self.logger, "Length: {:x}\n", self.memory.len());
                write!(self.logger, "0x20C0: {:x}\n", self.memory[0x20C0]);
                write!(self.logger, "Unimplemented Opcode:\n\tDenary: {0}\n\tHex: {0:x}\n", opcode);
                self.debug_output();
                unimplemented_opcode()
            },
    
        }
    
        if self.stack_pointer < self.rom_size || self.stack_pointer >= self.memory.len() as u16{ // Ensuring ROM is not overwritten
    
            self.stack_pointer = 0;
    
        }
    
        if self.program_counter >= 0x4000{ // Preventing the program from trying to read outside the size of memory
    
            self.program_counter = 0;
    
        }
    
    }

    pub fn debug_output(&mut self){
        
        write!(self.logger, "\n\n==============\n\n");
        
        disassembler::check_opcode_8080(self.program_counter as usize, &self.memory, &mut self.logger);
    
        write!(self.logger, "Memory:\n\t0x{:x}\n\t0x{:x}\n\t0x{:x}\n", 
            self.memory[self.program_counter as usize],
            self.memory[(self.program_counter + 1) as usize],
            self.memory[(self.program_counter + 2) as usize],
        );
    
        write!(self.logger, "Registers:\n\tA: 0x{:x}\n\tB: 0x{:x}\n\tC: 0x{:x}\n\tD: 0x{:x}\n\tE: 0x{:x}\n\tH: 0x{:x}\n\tL: 0x{:x}\n",
            self.a, self.b, self.c, self.d, self.e, self.h, self.l
        );
    
        write!(self.logger, "Flags:\n\tZero: {}\n\tSign: {}\n\tParity: {}\n\tCarry: {}\n", 
            self.flags.zero, self.flags.sign, self.flags.parity, self.flags.carry
        );
        
        write!(self.logger, "Program Counter:\n\tDecimal: {0}\n\tHex: {0:x}\n", self.program_counter);
    
        write!(self.logger, "Stack Pointer:\n\tDecimal: {0}\n\tHex: {0:x}\nMisc:\n\n", self.stack_pointer);
    
    }

}



/********************************************
*                 Addresses                 *
********************************************/
//#region
// Highest order bits at position 0, lowest order bits at position 1
fn seperate_16bit_pair(pair: u16) -> (u8, u8) {
    
    (
        (pair >> 8) as u8,
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
//#endregion


/********************************************
*                 Arithmetic                *
********************************************/
//#region
fn add(processor: &mut Processor8080, byte: u8, carry: bool){

    let answer: u16 = (processor.a as u16) + byte as u16 + {
        if carry{
            1
        }
        else{
            0
        }
    };

    set_flags(answer, processor);

    processor.a = answer as u8;

}

fn subtract(processor: &mut Processor8080, byte: u8, carry: u8){
    
    let answer = (processor.a as u32 + get_twos_complement(byte) as u32 + {
        if carry == 0 {
            0
        }
        else {
            get_twos_complement(carry) as u32
        }
    }) as u16;

    set_flags(answer, processor);

    processor.flags.carry = !processor.flags.carry;

    processor.a = answer as u8;
    
}

fn double_add(processor: &mut Processor8080, byte_a: u8, byte_b: u8){

    let new_address = ((processor.h as u32) << 8) | (processor.l as u32) + ((byte_a as u32) << 8) | (byte_b as u32);

    processor.flags.carry = new_address > 0xffff;

    let split_address = seperate_16bit_pair(new_address as u16);

    processor.h = split_address.0;

    processor.l = split_address.1;

}

// Handles the transfer of either the carry bit, high order bit or low order bit into
// any of the other respective positions; e.g the high order bit of the accumulator is
// transferred to the low order bit of the accumulator
fn rotate_carry_logic(processor: &mut Processor8080, or_value: u8, and_value: u8){

    if processor.flags.carry {

        processor.a = processor.a | or_value; // Sets the relevant bit

    }
    else {

        processor.a = processor.a & and_value; // Clears the relevant bit

    }

}

fn get_twos_complement(byte: u8) -> u16{
    
    ((!(byte as u16) as u32) + 1) as u16

}
//#endregion


/********************************************
*                  Logical                  *
********************************************/
//#region
fn logical(processor: &mut Processor8080, byte: u8, operator: fn(u8, u8) -> u16){

    let answer = operator(processor.a, byte);

    set_flags(answer, processor);

    processor.flags.carry = false; // Carry reset to false as there will never be a carry with a logical operation

    processor.a = answer as u8;

}

fn compare(processor: &mut Processor8080, byte: u8){
    
    let answer: u8 = (processor.a as u32 + get_twos_complement(byte) as u32) as u8;

    processor.flags.zero = answer == 0;

    processor.flags.sign = (answer & 0x80) == 0x80; // If the highest order bit is set

    processor.flags.parity = check_parity(answer as u16);

    processor.flags.carry = if (processor.a > 0 && byte > 0) || (processor.a & 0x80 != 0 && byte & 0x80 != 0){ // If the two values are the same sign
        processor.a < byte
    }
    else{
        !(processor.a < byte)
    };

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

// byte_1 is highest order bits, byte_2 is lowest order bits
fn push_onto_stack(processor: &mut Processor8080, byte_1: u8, byte_2: u8){

    processor.memory[((processor.stack_pointer as u32 + get_twos_complement(1) as u32) as u16) as usize] = byte_1;  // Push return address onto the stack
    processor.memory[((processor.stack_pointer as u32 + get_twos_complement(2) as u32) as u16) as usize] = byte_2;  // Highest order bits are pushed at SP - 1
                                                                                                                    // Lowerst order bits are pushed at SP - 2
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

fn step_register_flags(processor: &mut Processor8080, answer: u16){

    let carry_value = processor.flags.carry;

    set_flags(answer, processor);

    processor.flags.carry = carry_value;

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

        push_address_onto_stack(processor, processor.program_counter + 2);

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

    push_address_onto_stack(processor, processor.program_counter + 2);

    processor.program_counter = address;

}
//#endregion