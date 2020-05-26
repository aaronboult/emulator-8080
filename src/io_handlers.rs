use crate::cpu::*;

pub fn test_in(processor: &mut Processor8080, port: u8) -> u8{

    0

}

pub fn test_out(processor: &mut Processor8080, port: u8, value: u8){

}

pub fn space_invaders_in(processor: &mut Processor8080, port: u8) -> u8{

    /*
        Custom registers:
            0 -> Number of times to shift
            1 -> Shift result
    */

    if processor.custom_registers.len() == 0{

        processor.custom_registers = vec![0, 0];

    }

    match port {

        1 => {}, // Input

        2 => {}, // Input

        3 => return (processor.custom_registers[1] >> (8 - processor.custom_registers[0])) as u8,
        
        _ => {},

    }

    0

}

pub fn space_invaders_out(processor: &mut Processor8080, port: u8, value: u8){

    /*
        Custom registers:
            0 -> Number of times to shift
            1 -> Shift result
    */

    if processor.custom_registers.len() == 0{

        processor.custom_registers = vec![0, 0];

    }

    match port {

        2 => processor.custom_registers[0] = (value & 0x07) as u16,

        3 => {}, // Play Sound

        4 => processor.custom_registers[1] = (processor.custom_registers[1] >> 8) | ((value as u16) << 2),

        5 => {}, // Play Sound

        _ => {},

    }

}

/*
Read
    00        INPUTS (Mapped in hardware but never used by the code)
    01        INPUTS
    02        INPUTS
    03        bit shift register read <- Reads the shift result
Write
    02        shift amount (3 bits) <- Sets the shift amount
    03        sound bits
    04        shift data <- Performs the shift
    05        sound bits
    06        watch-dog
*/