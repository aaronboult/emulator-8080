use crate::machine::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn key_event(machine: &mut Machine){

    let mut event_pump = machine.sdl_context.event_pump().expect("Failed to retrieve event pump");

    for event in event_pump.poll_iter(){

        match event{

            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                if machine.cpu.testing{
                    machine.cpu.debug = !machine.cpu.debug;
                }
            },

            Event::Quit {..} |
    
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {

                machine.cpu.logger.flush().expect("Failed to flush output buffer");

                machine.audio_controller.close();

                std::process::exit(0);

            },

            _ => {},

        }

    }

}

pub fn test_interrupt(machine: &mut Machine){

    machine.cpu.cycles_elapsed = 0;

}

pub fn test_in(_processor: &mut Processor8080, _port: u8, _ports: &Vec<u8>) -> u8{

    0

}

pub fn test_out(_processor: &mut Processor8080, _port: u8, _value: u8, _ports: &mut Vec<u8>, _audio_controller: &mut AudioController){

}

pub fn draw(_machine: &mut Machine){

}