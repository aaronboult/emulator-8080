use crate::machine::*;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn setup(setup_config: &mut SetupConfiguration){
    
    setup_config.input_handler = space_invaders_in;
    setup_config.output_handler = space_invaders_out;

    setup_config.key_event_handler = key_event;

    setup_config.interrupt_handler = space_invaders_interrupt;

    setup_config.drawer = draw;

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
    
    setup_config.ports[0] = 0b00010000;

    setup_config.ports[1] = 0b00000000;

    setup_config.window.set_title("Space Invaders").expect("Failed to set window title");

    setup_config.window.set_size(224, 256).expect("Failed to size window");

}

fn key_event(machine: &mut Machine){

    let mut event_pump = machine.sdl_context.event_pump().expect("Failed to retrieve event pump");

    for event in event_pump.poll_iter(){

        match event{

            Event::Quit {..} |
    
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => std::process::exit(0),
    
            Event::KeyDown { keycode: Some(Keycode::T), .. } => machine.ports[1] = machine.ports[1] | 0b00000100, // Tilt
    
            Event::KeyDown { keycode: Some(Keycode::C), .. } => machine.ports[0] = machine.ports[0] | 0b00000010, // Coin entered
    
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => machine.ports[0] = machine.ports[0] | 0b00000100, // Player 1 ready
    
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => machine.ports[0] = machine.ports[0] | 0b00000010, // Player 2 ready
    
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                machine.ports[0] = machine.ports[0] | 0b00010000; // Player 1 shoot
                machine.ports[1] = machine.ports[1] | 0b00010000; // Player 2 shoot
            },
    
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                machine.ports[0] = machine.ports[0] | 0b00100000; // Player 1 shoot
                machine.ports[1] = machine.ports[1] | 0b00100000; // Player 2 shoot
            },
    
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                machine.ports[0] = machine.ports[0] | 0b01000000; // Player 1 shoot
                machine.ports[1] = machine.ports[1] | 0b01000000; // Player 2 shoot
            },


    
            Event::KeyUp { keycode: Some(Keycode::T), .. } => machine.ports[1] = machine.ports[1] & 0b11111011, // Tilt
    
            Event::KeyUp { keycode: Some(Keycode::C), .. } => machine.ports[0] = machine.ports[0] & 0b11111101, // Coin entered
    
            Event::KeyUp { keycode: Some(Keycode::Num1), .. } => machine.ports[0] = machine.ports[0] & 0b11111011, // Player 1 ready
    
            Event::KeyUp { keycode: Some(Keycode::Num2), .. } => machine.ports[0] = machine.ports[0] & 0b11111101, // Player 2 ready
    
            Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                machine.ports[0] = machine.ports[0] & 0b11101111; // Player 1 shoot
                machine.ports[1] = machine.ports[1] & 0b11101111; // Player 2 shoot
            },
    
            Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                machine.ports[0] = machine.ports[0] & 0b11011111; // Player 1 shoot
                machine.ports[1] = machine.ports[1] & 0b11011111; // Player 2 shoot
            },
    
            Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                machine.ports[0] = machine.ports[0] & 0b10111111; // Player 1 shoot
                machine.ports[1] = machine.ports[1] & 0b10111111; // Player 2 shoot
            },

            _ => {},

        }

    }

}

pub fn space_invaders_interrupt(machine: &mut Machine){

    if machine.cpu.cycles_elapsed >= (2_000_000 / 60 / 2) as u16{

        if machine.cpu.interrupt_enabled{
    
            machine.cpu.generate_interrupt(); // Generate a video hardware interrupt
    
        }

        machine.cpu.cycles_elapsed -= (2_000_000 / 60 / 2) as u16;

        machine.cpu.interrupt_value = if machine.cpu.interrupt_value == 1 { 2 } else { 1 };

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

        0 => return 1, // Input

        1 => return 0, // Input

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

pub fn draw(machine: &mut Machine){
    
    machine.canvas.clear();

    for current_byte_position in 0..7168{ // Read from the 7KB of VRAM

        for bit in (0..8).rev(){ // Read each bit from the byte, as each bit represents a pixel - start from the leftmost bit

            let x_pos = (current_byte_position * 8 + bit) % 256;
            let y_pos = (current_byte_position * 8 + bit) / 256;

            if machine.cpu.memory[0x2400 + current_byte_position as usize] >> bit & 0x01 != 0{ // If this pixel is on

                if x_pos >= 192 && x_pos < 224{ // If the pixel is in the 'RED' range

                machine.canvas.set_draw_color(Color::RGB(255, 0, 0));

                }
                else if (x_pos > 16 && x_pos <= 72) || (x_pos < 16 && y_pos >= 16 && y_pos < 134){ // If the pixel is in the 'GREEN' range

                machine.canvas.set_draw_color(Color::RGB(0, 255, 0));

                }
                else{ // The pixel is in the 'WHITE' range

                machine.canvas.set_draw_color(Color::RGB(255, 255, 255));

                }

                machine.canvas.draw_point(Point::new(y_pos, 256 - x_pos)).expect("Failed to draw window");

            }
            // else{

            //     machine.canvas.set_draw_color(Color::RGB(0, 0, 0));

            //     machine.canvas.draw_point(Point::new(y_pos, 256 - x_pos)).expect("Failed to draw window");

            // }

        }

    }

    machine.canvas.present();

}