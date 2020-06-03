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
    
    setup_config.ports[0] = 0b00001110;

    setup_config.ports[1] = 0b00001000;

    setup_config.window.set_title("Space Invaders").expect("Failed to set window title");

    setup_config.window.set_size(224, 256).expect("Failed to size window");

}

fn key_event(machine: &mut Machine){

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

                std::process::exit(0);
            
            },

            Event::KeyDown { keycode: Some(Keycode::V), .. } => {

                if machine.cpu.testing{

                    // Dump the VRAM to the log file
    
                    let mut output: String = "Byte Addr 0x | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7\n\n".to_string();
    
                    for byte_pos in 0..7168{
    
                        let mut to_append = format!("    0x{:04x}   |", byte_pos);
    
                        for bit in 0..8{
    
                            to_append = format!("{} {} ", to_append, (machine.cpu.memory[0x2400 + byte_pos] >> bit) & 0x01);
    
                            if bit != 7{
    
                                to_append += "|";
    
                            }
    
                        }
    
                        output = format!("{}{}\n", output, to_append);
    
                    }
    
                    write!(machine.cpu.logger, "{}", output).expect("Failed to write to output buffer");

                }

            },
    
            Event::KeyDown { keycode: Some(Keycode::T), .. } => machine.ports[2] |= 0b00000100, // Tilt
    
            Event::KeyDown { keycode: Some(Keycode::C), .. } => machine.ports[1] |= 0b00000001, // Coin entered
    
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => machine.ports[1] |= 0b00000100, // Player 1 ready
    
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => machine.ports[1] |= 0b00000010, // Player 2 ready
    
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                machine.ports[1] |= 0b00010000; // Player 1 shoot
                machine.ports[2] |= 0b00010000; // Player 2 shoot
            },
    
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                machine.ports[1] |= 0b00100000; // Player 1 Left
                machine.ports[2] |= 0b00100000; // Player 2 Left
            },
    
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                machine.ports[1] |= 0b01000000; // Player 1 Right
                machine.ports[2] |= 0b01000000; // Player 2 Right
            },


    
            Event::KeyUp { keycode: Some(Keycode::T), .. } => machine.ports[2] &= 0b11111011, // Tilt
    
            Event::KeyUp { keycode: Some(Keycode::C), .. } => machine.ports[1] &= 0b11111110, // Coin entered
    
            Event::KeyUp { keycode: Some(Keycode::Num1), .. } => machine.ports[1] &= 0b11111011, // Player 1 ready
    
            Event::KeyUp { keycode: Some(Keycode::Num2), .. } => machine.ports[1] &= 0b11111101, // Player 2 ready
    
            Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                machine.ports[1] &= 0b11101111; // Player 1 shoot
                machine.ports[2] &= 0b11101111; // Player 2 shoot
            },
    
            Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                machine.ports[1] &= 0b11011111; // Player 1 Left
                machine.ports[2] &= 0b11011111; // Player 2 Left
            },
    
            Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                machine.ports[1] &= 0b10111111; // Player 1 Right
                machine.ports[2] &= 0b10111111; // Player 2 Right
            },

            _ => {},

        }

    }

}

fn space_invaders_interrupt(machine: &mut Machine){

    if machine.cpu.cycles_elapsed >= (2_000_000 / 60 / 2) as u16{
    
        machine.cpu.generate_interrupt();
        
        machine.cpu.cycles_elapsed -= (2_000_000 / 60 / 2) as u16;

        machine.cpu.interrupt_value = if machine.cpu.interrupt_value == 1 { 2 } else { 1 };

    }

}

fn space_invaders_in(processor: &mut Processor8080, port: u8, ports: &Vec<u8>) -> u8{

    /*
        Custom registers:
            0 -> Number of times to shift
            1 -> Shift result
    */

    if processor.custom_registers.len() == 0{

        processor.custom_registers = vec![0, 0];

    }

    match port {

        1 => return ports[1], // Input

        2 => return ports[2], // Input

        3 => return (processor.custom_registers[1] >> (8 - processor.custom_registers[0])) as u8,
        
        _ => {},

    }
    
    0

}

fn space_invaders_out(processor: &mut Processor8080, port: u8, value: u8, _ports: &Vec<u8>){

    /*
        Custom registers:
            0 -> Number of times to shift
            1 -> Shift result
    */

    if processor.custom_registers.len() == 0{

        processor.custom_registers = vec![0, 0];

    }

    match port {

        2 => processor.custom_registers[0] = (value & 0b111) as u16, // Set the shif amount to the last 3 bits of the provided value

        3 => {}, // Play Sound

        4 => processor.custom_registers[1] = (processor.custom_registers[1] >> 8) | ((value as u16) << 8), // Set the shift result

        5 => {}, // Play Sound

        _ => {},

    }

}

fn draw(machine: &mut Machine){
    
    machine.canvas.clear();

    for current_byte_position in 0..7168{ // Read from the 7KB of VRAM

        for bit in (0..8).rev(){ // Read each bit from the byte, as each bit represents a pixel - start from the leftmost bit

            if (machine.cpu.memory[0x2400 + current_byte_position as usize] >> bit) & 0x01 != 0{ // If this pixel is on

                let x_pos = ((current_byte_position * 8) + bit) / 256;
                let y_pos = ((current_byte_position * 8) + bit) % 256;

                if y_pos >= 192 && y_pos < 224{ // If the pixel is in the 'RED' range

                    machine.canvas.set_draw_color(Color::RED);

                }
                else if (y_pos > 16 && y_pos <= 72) || (y_pos < 16 && x_pos >= 16 && x_pos < 134){ // If the pixel is in the 'GREEN' range

                    machine.canvas.set_draw_color(Color::GREEN);

                }
                else{ // The pixel is in the 'WHITE' range

                    machine.canvas.set_draw_color(Color::WHITE);

                }

                machine.canvas.draw_point(Point::new(x_pos, 256 - y_pos)).expect("Failed to draw window");

            }

        }

    }

    machine.canvas.set_draw_color(Color::BLACK);

    let window_size = machine.canvas.output_size().expect("Failed to read window size");

    machine.canvas.set_scale(window_size.0 as f32 / 224.0, window_size.1 as f32 / 256.0).expect("Failed to set canvas scale");

    machine.canvas.present();

}