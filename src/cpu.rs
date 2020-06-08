mod disassembler;

use crate::machine::AudioController;

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

    interrupt_enabled: bool,
    pub interrupt_value: u8,

    rom_size: u16,

    opcode_cycle_length: [u16; 256],

    input_handler: fn(&mut Self, u8, &Vec<u8>) -> u8,
    output_handler: fn(&mut Self, u8, u8, &mut Vec<u8>, &mut AudioController),

    pub testing: bool,
    pub debug: bool,

    pub logger: std::boxed::Box<dyn std::io::Write>,
}

#[derive(Default, Debug)]
struct Flags{
    zero: bool,
    sign: bool, // True if negative
    parity: bool, // True if even
    carry: bool,
    auxiliary_carry: bool,
}

impl Processor8080{

    pub fn new(
            input_handler: fn(&mut Self, u8, &Vec<u8>) -> u8, 
            output_handler: fn(&mut Self, u8, u8, &mut Vec<u8>, &mut AudioController), 
            log_to_file: bool
        ) -> Self{

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
                //  0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F <- Lowest order bits
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

        // Handle outputs - return instantly
        self.memory[0x06] = 0xC9;

        self.rom_size = 0;
    
        while self.memory.len() < 0x10000{ // Initialize 64KiB of memory

            self.memory.push(0);

        }

        self.program_counter = 0x100;

        self.testing = true;

        self.debug = true;

    }

    pub fn initialize(&mut self, files: Vec<FileToLoad>){

        for file in files{
            
            self.load_file(file.name, file.offset, file.size);

        }
    
        while self.memory.len() < 0x4000{ // Initialize 16KiB of memory
            
            self.memory.push(0);

        }
        
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

        if !self.interrupt_enabled{

            return;

        }
    
        push_address_onto_stack(self, self.program_counter);
    
        self.program_counter = (8 * self.interrupt_value) as u16;
        
        self.cycles_elapsed += 11;

        self.interrupt_enabled = false;
    
    }

    pub fn debug_output(&mut self){
        
        write!(self.logger, "\n\n==============\n\n").expect("Failed to write to output buffer");
        
        disassembler::check_opcode_8080(self.program_counter as usize, &self.memory, &mut self.logger);
    
        write!(self.logger, "Memory:\n\t0x{:x}\n\t0x{:x}\n\t0x{:x}\n", 
            self.memory[self.program_counter as usize],
            self.memory[(self.program_counter + 1) as usize],
            self.memory[(self.program_counter + 2) as usize],
        ).expect("Failed to write to output buffer");
    
        write!(self.logger, "Registers:\n\tA: 0x{:x}\n\tB: 0x{:x}\n\tC: 0x{:x}\n\tD: 0x{:x}\n\tE: 0x{:x}\n\tH: 0x{:x}\n\tL: 0x{:x}\n",
            self.a, self.b, self.c, self.d, self.e, self.h, self.l
        ).expect("Failed to write to output buffer");
    
        write!(self.logger, "Flags:\n\tZero: {}\n\tSign: {}\n\tParity: {}\n\tCarry: {}\n\tAuxiliary Carry: {}\n", 
            self.flags.zero, self.flags.sign, self.flags.parity, self.flags.carry, self.flags.auxiliary_carry
        ).expect("Failed to write to output buffer");
        
        write!(self.logger, "Program Counter:\n\tDecimal: {0}\n\tHex: {0:x}\n", self.program_counter).expect("Failed to write to output buffer");
    
        write!(self.logger, "Stack Pointer:\n\tDecimal: {0}\n\tHex: {0:x}\nMisc:\n\n", self.stack_pointer).expect("Failed to write to output buffer");
    
    }

    fn check_cpudiag_status(&mut self, audio_controller: &mut AudioController){

        if self.program_counter == 5{
    
            if self.c == 9{

                let mut offset: u16 = ((self.d as u16) << 8) | (self.e as u16);

                let mut letter = self.memory[(offset + 3) as usize] as char;

                let mut string: String = "".to_string();

                while letter != '$'{

                    string += &letter.to_string();

                    offset += 1;

                    letter = self.memory[(offset + 3) as usize] as char;

                }

                write!(self.logger, "{}", string).expect("Failed to write to output buffer");

                self.debug = false;

            }
            else if self.c == 2{

                write!(self.logger, "{}", self.e as char).expect("Failed to write to output buffer");

            }

        }
        else if self.program_counter == 0{

            self.logger.flush().expect("Failed to flush output buffer");

            audio_controller.close();

            std::process::exit(0);

        }

    }

    pub fn emulate(&mut self, ports: &mut Vec<u8>, audio_controller: &mut AudioController){

        if self.testing{

            self.check_cpudiag_status(audio_controller);

        }
    
        let opcode: u8 = self.memory[self.program_counter as usize];
        
        self.cycles_elapsed += self.opcode_cycle_length[opcode as usize];

        if opcode != 0x00 && self.debug{ // Don't display NOP instructions to avoid log clutter

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
                (self.output_handler)(self, self.memory[self.program_counter as usize], self.a, ports, audio_controller);
                self.program_counter += 1;
            }, // OUT
            0xDB => {
                self.a = (self.input_handler)(self, self.memory[self.program_counter as usize], ports);
                self.program_counter += 1;
            }, // IN
            0x27 => {
                let mut correction = 0;
                let mut carry_temp = self.flags.carry;
                if self.a & 0x0f > 9 || self.flags.auxiliary_carry{
                    correction += 0x06;
                }
                if self.a >> 4 > 9 || carry_temp || (self.a >> 4 == 9 && correction != 0){ // Last condition: If the correction will inflate the highest order bits to be >9
                    correction += 0x60;
                    carry_temp = true;
                }
                add(self, correction, false);
                self.flags.carry = carry_temp;
            }, // DAA
            0x76 => panic!("Halting"), // HLT
            //#endregion
                
    
            /********************************************
            *                   Carry                   *
            ********************************************/
            //#region
            0x37 => self.flags.carry = true, // STC
            0x3F => self.flags.carry = !self.flags.carry, // CMC
            //#endregion
            

            /********************************************
            *                Move Opcodes               *
            ********************************************/
            //#region
            0x40 => {}, // MOV B,B - Does nothing
            0x41 => self.b = self.c, // MOV B,C
            0x42 => self.b = self.d, // MOV B,D
            0x43 => self.b = self.e, // MOV B,E
            0x44 => self.b = self.h, // MOV B,H
            0x45 => self.b = self.l, // MOV B,L
            0x46 => self.b = self.memory[get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16) as usize], // MOV B,(HL)
            0x47 => self.b = self.a, // MOV B,A
    
            0x48 => self.c = self.b, // MOV C,B
            0x49 => {}, // MOV C,C - Does nothing
            0x4A => self.c = self.d, // MOV C,D
            0x4B => self.c = self.e, // MOV C,E
            0x4C => self.c = self.h, // MOV C,H
            0x4D => self.c = self.l, // MOV C,L
            0x4E => self.c = self.memory[get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16) as usize], // MOV D,(HL)
            0x4F => self.c = self.a, // MOV C,A
    
            0x50 => self.d = self.b, // MOV D,B
            0x51 => self.d = self.c, // MOV D,C
            0x52 => {}, // MOV D,D - Does nothing
            0x53 => self.d = self.e, // MOV D,E
            0x54 => self.d = self.h, // MOV D,H
            0x55 => self.d = self.l, // MOV D,L
            0x56 => self.d = self.memory[get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16) as usize], // MOV D(HL)
            0x57 => self.d = self.a, // MOV D,A
    
            0x58 => self.e = self.b, // MOV E,B
            0x59 => self.e = self.c, // MOV E,C
            0x5A => self.e = self.d, // MOV E,D
            0x5B => {}, // MOV E,E - Does nothing
            0x5C => self.e = self.h, // MOV E,H
            0x5D => self.e = self.l, // MOV E,L
            0x5E => self.e = self.memory[get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16) as usize], // MOV E,(HL)
            0x5F => self.e = self.a, // MOV E,A
            
            0x60 => self.h = self.b, // MOV H,B
            0x61 => self.h = self.c, // MOV H,C
            0x62 => self.h = self.d, // MOV H,D
            0x63 => self.h = self.e, // MOV H,E
            0x64 => {}, // MOV H,H - Does nothing
            0x65 => self.h = self.l, // MOV H,L
            0x66 => self.h = self.memory[get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16) as usize], // MOV H,(HL)
            0x67 => self.h = self.a, // MOV H,A
            
            0x68 => self.l = self.b, // MOV L,B
            0x69 => self.l = self.c, // MOV L,C
            0x6A => self.l = self.d, // MOV L,D
            0x6B => self.l = self.e, // MOV L,E
            0x6C => self.l = self.h, // MOV L,H
            0x6D => {}, // MOV L,L - Does nothing
            0x6E => self.l = self.memory[get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16) as usize], // MOV L,(HL)
            0x6F => self.l = self.a, // MOV L,A
            
            0x70 => write_to_memory(self, self.h, self.l, self.b), // MOV (HL),B
            0x71 => write_to_memory(self, self.h, self.l, self.c), // MOV (HL),C
            0x72 => write_to_memory(self, self.h, self.l, self.d), // MOV (HL),D
            0x73 => write_to_memory(self, self.h, self.l, self.e), // MOV (HL),E
            0x74 => write_to_memory(self, self.h, self.l, self.h), // MOV (HL),H
            0x75 => write_to_memory(self, self.h, self.l, self.l), // MOV (HL),L
            0x77 => write_to_memory(self, self.h, self.l, self.a), // MOV (HL),A
            
            0x78 => self.a = self.b, // MOV A,B
            0x79 => self.a = self.c, // MOV A,C
            0x7A => self.a = self.d, // MOV A,D
            0x7B => self.a = self.e, // MOV A,E
            0x7C => self.a = self.h, // MOV A,H
            0x7D => self.a = self.l, // MOV A,L
            0x7E => self.a = self.memory[get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16) as usize], // MOV A,(HL)
            0x7F => {}, // MOV A,A - Does nothing
    
            0x06 => {
                self.b = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI B,D8
            0x0E => {
                self.c = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI C,D8
            0x16 => {
                self.d = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI D,D8
            0x1E => {
                self.e = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI E,D8
            0x26 => {
                self.h = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI H,D8
            0x2E => {
                self.l = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI L,D8
            0x36 => {
                write_to_memory(self, self.h, self.l, self.memory[self.program_counter as usize]);
                self.program_counter += 1;
            }, // MVI M,D8
            0x3E => {
                self.a = self.memory[self.program_counter as usize];
                self.program_counter += 1;
            }, // MVI A,D8
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
            0xF9 => self.stack_pointer = ((self.h as u16) << 8) | (self.l as u16), // SPHL
            0xE3 => {
                mem::swap(&mut self.h, &mut self.memory[(self.stack_pointer + 1) as usize]);
                mem::swap(&mut self.l, &mut self.memory[self.stack_pointer as usize]);
            }, // XTHL
            //#endregion

    
            /********************************************
            *               Store Register              *
            ********************************************/
            //#region
            0x02 => write_to_memory(self, self.b, self.c, self.a), // STAX B
            0x12 => write_to_memory(self, self.d, self.e, self.a), // STAX D
            0x32 => {
                let first_byte = self.memory[(self.program_counter + 1) as usize];
                let second_byte = self.memory[self.program_counter as usize];
                write_to_memory(self, first_byte, second_byte, self.a);
                self.program_counter += 2;
            }, // STA addr
            0x22 => {
                let first_byte = self.memory[(self.program_counter + 1) as usize];
                let second_byte = self.memory[self.program_counter as usize];
                write_to_memory(self, first_byte, second_byte + 1, self.h);
                write_to_memory(self, first_byte, second_byte, self.l);
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
                let address = get_address_from_pair(&mut first_byte, &mut second_byte, (self.memory.len() - 1) as u16);
                self.stack_pointer = address;
                self.program_counter += 2;
            }, // LXI SP,operand
            0x3A => {
                let mut first_byte = self.memory[(self.program_counter + 1) as usize];
                let mut second_byte = self.memory[self.program_counter as usize];
                let address = get_address_from_pair(&mut first_byte, &mut second_byte, (self.memory.len() - 1) as u16);
                self.a = self.memory[address as usize];
                self.program_counter += 2;
            }, // LDA addr
            0x2A => {
                let mut first_byte = self.memory[(self.program_counter + 1) as usize];
                let mut second_byte = self.memory[self.program_counter as usize];
                let address = get_address_from_pair(&mut first_byte, &mut second_byte, (self.memory.len() - 1) as u16);
                self.h = self.memory[(address + 1) as usize];
                self.l = self.memory[address as usize];
                self.program_counter += 2;
            }, // LHLD addr
            0x0A => self.a = self.memory[get_address_from_pair(&mut self.b, &mut self.c, (self.memory.len() - 1) as u16) as usize], // LDAX B
            0x1A => self.a = self.memory[get_address_from_pair(&mut self.d, &mut self.e, (self.memory.len() - 1) as u16) as usize], // LDAX D
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
                let answer: u16 = (self.memory[
                    get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16) as usize
                ] as u16) + 1;
                step_register_flags(self, answer);
                write_to_memory(self, self.h, self.l, answer as u8);
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
                let answer: u16 = (self.b as u32 + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.b = answer as u8;
            }, // DCR B
            0x0D => {
                let answer: u16 = (self.c as u32 + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.c = answer as u8;
            }, // DCR C
            0x15 => {
                let answer: u16 = (self.d as u32 + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.d = answer as u8;
            }, // DCR D
            0x1D => {
                let answer: u16 = (self.e as u32 + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.e = answer as u8;
            }, // DCR E
            0x25 => {
                let answer: u16 = (self.h as u32 + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.h = answer as u8;
            }, // DCR H
            0x2D => {
                let answer: u16 = (self.l as u32 + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.l = answer as u8;
            }, // DCR L
            0x35 => {
                let answer: u16 = (self.memory[
                    get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16) as usize
                ] as u32 + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                write_to_memory(self, self.h, self.l, answer as u8);
            }, // DCR M
            0x3D => {
                let answer: u16 = (self.a as u32 + get_twos_complement(1) as u32) as u16;
                step_register_flags(self, answer);
                self.a = answer as u8;
            }, // DCR A
            //#endregion
    
    
            /********************************************
            *                  Inc Pair                 *
            ********************************************/
            //#region
            0x03 => {
                let address = (((self.b as u16) << 8) | (self.c as u16)) as u32;
                let pair = seperate_16bit_pair((address + 1) as u16); // Not using get_address_from_pair to avoid overflow resets
                self.b = pair.0;
                self.c = pair.1;
            }, // INX B
            0x13 => {
                let address = (((self.d as u16) << 8) | (self.e as u16)) as u32;
                let pair = seperate_16bit_pair((address + 1) as u16); // Not using get_address_from_pair to avoid overflow resets
                self.d = pair.0;
                self.e = pair.1;
            }, // INX D
            0x23 => {
                let address = (((self.h as u16) << 8) | (self.l as u16)) as u32;
                let pair = seperate_16bit_pair((address + 1) as u16); // Not using get_address_from_pair to avoid overflow resets
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
                let address = (((self.b as u16) << 8) | (self.c as u16)) as u32;
                let pair = seperate_16bit_pair((address + get_twos_complement(1) as u32) as u16); // Not using get_address_from_pair to avoid overflow resets
                self.b = pair.0;
                self.c = pair.1;
            }, // DCX B
            0x1B => {
                let address = (((self.d as u16) << 8) | (self.e as u16)) as u32;
                let pair = seperate_16bit_pair((address + get_twos_complement(1) as u32) as u16); // Not using get_address_from_pair to avoid overflow resets
                self.d = pair.0;
                self.e = pair.1;
            }, // DCX D
            0x2B => {
                let address = (((self.h as u16) << 8) | (self.l as u16)) as u32;
                let pair = seperate_16bit_pair((address + get_twos_complement(1) as u32) as u16); // Not using get_address_from_pair to avoid overflow resets
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
                let address = get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16);
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
                let address = get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16);
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
            *              Subtract Opcodes             *
            ********************************************/
            //#region
            0x90 => subtract(self, self.b, false), // SUB B
            0x91 => subtract(self, self.c, false), // SUB C
            0x92 => subtract(self, self.d, false), // SUB D
            0x93 => subtract(self, self.e, false), // SUB E
            0x94 => subtract(self, self.h, false), // SUB H
            0x95 => subtract(self, self.l, false), // SUB L
            0x96 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16);
                subtract(self, self.memory[address as usize], false);
            }, // SUB M - From memory address
            0x97 => subtract(self, self.a, false), // SUB A
            0x98 => subtract(self, self.b, self.flags.carry), // SBB B
            0x99 => subtract(self, self.c, self.flags.carry), // SBB C
            0x9A => subtract(self, self.d, self.flags.carry), // SBB D
            0x9B => subtract(self, self.e, self.flags.carry), // SBB E
            0x9C => subtract(self, self.h, self.flags.carry), // SBB H
            0x9D => subtract(self, self.l, self.flags.carry), // SBB L
            0x9E => {
                let address = get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16);
                subtract(self, self.memory[address as usize], self.flags.carry);
            }, // SBB M - From memory address
            0x9F => subtract(self, self.a, self.flags.carry), // SBB A
            0xD6 => {
                subtract(self, self.memory[self.program_counter as usize], false);
                self.program_counter += 1;
            }, // SUI - Immediate
            0xDE => {
                subtract(self, self.memory[self.program_counter as usize], self.flags.carry);
                self.program_counter += 1;
            }, // SBI - Immediate
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
                let address = get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16);
                compare(self, self.memory[address as usize])
            }, // CMP M
            0xBF => compare(self, self.a), // CMP A
            0xFE => {
                compare(self, self.memory[self.program_counter as usize]);
                self.program_counter += 1;
            }, // CPI data
            //#endregion

    
            /********************************************
            *                And Opcodes                *
            ********************************************/
            //#region
            0xA0 => and(self, self.b), // ANA B
            0xA1 => and(self, self.c), // ANA C
            0xA2 => and(self, self.d), // ANA D
            0xA3 => and(self, self.e), // ANA E
            0xA4 => and(self, self.h), // ANA H
            0xA5 => and(self, self.l), // ANA L
            0xA6 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16);
                and(self, self.memory[address as usize])
            }, // ANA M
            0xA7 => and(self, self.a), // ANA A
            0xE6 => {
                and(self, self.memory[self.program_counter as usize]);
                self.program_counter += 1;
            }, // ANI
            //#endregion
    
    
            /********************************************
            *                 Or Opcodes                *
            ********************************************/
            //#region
            0xB0 => or(self, self.b), // ORA B
            0xB1 => or(self, self.c), // ORA C
            0xB2 => or(self, self.d), // ORA D
            0xB3 => or(self, self.e), // ORA E
            0xB4 => or(self, self.h), // ORA H
            0xB5 => or(self, self.l), // ORA L
            0xB6 => {
                let address = get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16);
                or(self, self.memory[address as usize])
            }, // ORA M
            0xB7 => or(self, self.a),  // ORA A
            0xF6 => {
                or(self, self.memory[self.program_counter as usize]);
                self.program_counter += 1;
            }, // ORI
            //#endregion
    
    
            /********************************************
            *                XOR Opcodes                *
            ********************************************/
            //#region
            0xA8 => xor(self, self.b), // XRA B
            0xA9 => xor(self, self.c), // XRA C
            0xAA => xor(self, self.d), // XRA D
            0xAB => xor(self, self.e), // XRA E
            0xAC => xor(self, self.h), // XRA H
            0xAD => xor(self, self.l), // XRA L
            0xAE => {
                let address = get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16);
                xor(self, self.memory[address as usize])
            },  // XRA M
            0xAF => xor(self, self.a), // XRA A
            0xEE => {
                xor(self, self.memory[self.program_counter as usize]);
                self.program_counter += 1;
            }, // XRI
            //#endregion
    
    
            /********************************************
            *                Not Opcodes                *
            ********************************************/
            //#region
            0x2F => self.a = !self.a, // CMA
            //#endregion
    
    
            /********************************************
            *               Return Opcodes              *
            ********************************************/
            //#region
            0xC9 => ret(self, true), // RET
            0xC0 => ret(self, !self.flags.zero), // RNZ addr - If the zero bit is zero
            0xC8 => ret(self, self.flags.zero), // RZ addr - If the zero bit is one
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
            0xC7 => reset(self, 0x0), // RST 0
            0xCF => reset(self, 0x8), // RST 1
            0xD7 => reset(self, 0x10), // RST 2
            0xDF => reset(self, 0x18), // RST 3
            0xE7 => reset(self, 0x20), // RST 4
            0xEF => reset(self, 0x28), // RST 5
            0xF7 => reset(self, 0x30), // RST 6
            0xFF => reset(self, 0x38), // RST 7
            //#endregion
    
    
            /********************************************
            *                Call Opcodes               *
            ********************************************/
            //#region
            0xCD => call(self, true), // CALL addr
            0xC4 => call(self, !self.flags.zero), // CNZ addr - If the zero bit is zero
            0xCC => call(self, self.flags.zero), // CZ addr - If the zero bit is one
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
            0xE9 => self.program_counter = get_address_from_pair(&mut self.h, &mut self.l, (self.memory.len() - 1) as u16), // PCHL
            //#endregion
    
            _ => {
                write!(self.logger, "Unimplemented Opcode:\n\tDenary: {0}\n\tHex: {0:x}\n", opcode).expect("Failed to write to output buffer");
                self.debug_output();
                unimplemented_opcode()
            },
    
        }
    
        if self.stack_pointer < self.rom_size || self.stack_pointer >= (self.memory.len() - 1) as u16{ // Ensuring ROM is not overwritten
            
            self.stack_pointer = 0;
    
        }
    
        if self.program_counter >= (self.memory.len() - 1) as u16{ // Preventing the program from trying to read outside the size of memory
    
            self.program_counter = 0;
    
        }
    
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
fn get_address_from_pair(byte_1: &mut u8, byte_2: &mut u8, max_memory_index: u16) -> u16 {

    let mut address: u16 = ((*byte_1 as u16) << 8) | (*byte_2 as u16);

    if address >= max_memory_index{
        
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

    processor.flags.auxiliary_carry = ((answer ^ processor.a as u16 ^ byte as u16) & 0x10) != 0;

    processor.a = answer as u8;

}

fn subtract(processor: &mut Processor8080, byte: u8, carry: bool){

    let answer: u16 = (processor.a as u32 + get_twos_complement(byte) as u32 + {
        if carry{
            get_twos_complement(1) as u32
        }
        else{
            0
        }
    }) as u16;

    set_flags(answer, processor);

    processor.flags.auxiliary_carry = ((answer ^ processor.a as u16 ^ byte as u16) & 0x10) != 0;

    processor.a = answer as u8;
    
}

fn double_add(processor: &mut Processor8080, byte_a: u8, byte_b: u8){

    let new_address: u32 = (((processor.h as u32) << 8) | (processor.l as u32)) + (((byte_a as u32) << 8) | (byte_b as u32));

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

        processor.a |= or_value; // Sets the relevant bit

    }
    else {

        processor.a &= and_value; // Clears the relevant bit

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
fn and(processor: &mut Processor8080, byte: u8){

    processor.flags.auxiliary_carry = ((processor.a | byte) & 0x08) != 0;

    logical(processor, byte, |x,y| (x&y) as u16);

}

fn or(processor: &mut Processor8080, byte: u8){

    logical(processor, byte, |x,y| (x|y) as u16);

    processor.flags.auxiliary_carry = false;

}

fn xor(processor: &mut Processor8080, byte: u8){

    logical(processor, byte, |x,y| (x^y) as u16);

    processor.flags.auxiliary_carry = false;
    
}

fn logical(processor: &mut Processor8080, byte: u8, operator: fn(u8, u8) -> u16){

    let answer: u16 = operator(processor.a, byte);

    set_flags(answer, processor);

    processor.flags.carry = false; // Carry reset to false as there will never be a carry with a logical operation

    processor.a = answer as u8;

}

fn compare(processor: &mut Processor8080, byte: u8){
    
    let answer: u16 = (processor.a as u32 + get_twos_complement(byte) as u32) as u16;
    
    set_flags(answer, processor);

    processor.flags.auxiliary_carry = (!(processor.a as u16 ^ answer ^ byte as u16) & 0x10) != 0;

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

fn write_to_memory(processor: &mut Processor8080, byte_1: u8, byte_2: u8, value: u8){

    let mut address: u16 = get_address_from_pair(&mut {byte_1}, &mut {byte_2}, (processor.memory.len() - 1) as u16);

    if address < processor.rom_size{

        return;

    }

    if address >= 0x4000 && address < 0x6000{

        address -= 0x2000;

    }

    processor.memory[address as usize] = value;

}
//#endregion


/********************************************
*                   Flags                   *
********************************************/
//#region
pub fn set_flags(answer: u16, processor: &mut Processor8080){

    processor.flags.zero = (answer & 0xff) == 0;

    processor.flags.sign = (answer & 0x80) != 0;

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

    processor.flags.auxiliary_carry = (answer & 0xf0) == 0; // If the least significant bits are 0, a carry out of them occurred

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
            (processor.memory.len() - 1) as u16,
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
            (processor.memory.len() - 1) as u16,
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
            (processor.memory.len() - 1) as u16,
        );

        processor.stack_pointer += 2;

    }

}

fn reset(processor: &mut Processor8080, address: u16){

    push_address_onto_stack(processor, processor.program_counter + 2);

    processor.program_counter = address;

}
//#endregion