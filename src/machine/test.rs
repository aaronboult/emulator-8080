use crate::machine::*;

pub fn test_interrupt(machine: &mut Machine){

}

pub fn test_in(processor: &mut Processor8080, port: u8) -> u8{

    write!(processor.logger, "Port: {}\t", port);

    0

}

pub fn test_out(processor: &mut Processor8080, port: u8, value: u8){

    write!(processor.logger, "Port: {}\tValue: {}", port, value);

}