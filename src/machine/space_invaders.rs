use crate::machine::*;

pub fn setup(setup_config: &mut SetupConfiguration){
    
    setup_config.input_handler = space_invaders_in;
    setup_config.output_handler = space_invaders_out;

    setup_config.interrupt_handler = space_invaders_interrupt;

    setup_config.files.push(FileToLoad{
        name: "space-invaders-source/SpaceInvaders.h".to_string(),
        offset: 0x0,
        size: 0x800
    });

    setup_config.files.push(FileToLoad{
        name: "space-invaders-source/SpaceInvaders.g".to_string(),
        offset: 0x800,
        size: 0x800
    });

    setup_config.files.push(FileToLoad{
        name: "space-invaders-source/SpaceInvaders.f".to_string(),
        offset: 0x1000,
        size: 0x800
    });

    setup_config.files.push(FileToLoad{
        name: "space-invaders-source/SpaceInvaders.e".to_string(),
        offset: 0x1800,
        size: 0x800
    });
    
    setup_config.ports[0] = 0b01110000;

    setup_config.ports[1] = 0b00010000;

}

pub fn space_invaders_interrupt(machine: &mut Machine){

    if machine.timestamp.elapsed().expect("Failed to calculate elapsed time").as_millis() == 1/60{

        if machine.cpu.interrupt_enabled{

            machine.cpu.generate_interrupt(2); // Generate a video hardware interrupt

            machine.timestamp = machine.get_time();

        }

    }

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