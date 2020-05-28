use crate::machine::*;

pub fn key_event(_machine: &mut Machine){

}

pub fn test_interrupt(_machine: &mut Machine){

}

pub fn test_in(processor: &mut Processor8080, port: u8, _ports: &Vec<u8>) -> u8{

    write!(processor.logger, "Port: {}\t", port);

    0

}

pub fn test_out(processor: &mut Processor8080, port: u8, value: u8, _ports: &Vec<u8>){

    write!(processor.logger, "Port: {}\tValue: {}", port, value);

}

pub fn draw(_machine: &mut Machine){

}