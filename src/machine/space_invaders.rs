use crate::machine::*;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::Chunk;

pub fn setup(setup_config: &mut SetupConfiguration){

    println!("\n\n\n\
        Machine Interaction:\n\n\t\
            Escape Key  -> Close Emulator\n\t\
            T Key       -> Tilt Machine\n\t\
            C Key       -> Input Coin\n\t\
            1 Key       -> Player 1 Ready\n\t\
            2 Key       -> Player 2 Ready\n\n\
        Volume Controls:\n\n\t\
            Up Arrow    -> Volume Up\n\t\
            Down Arrow  -> Volume Down\n\t\
            M Key       -> Toggle Mute\n\n\
        Controls:\n\n\t\
            Left Arrow  -> Move Left\n\t\
            Right Arrow -> Move Right\n\t\
            Spacebar    -> Shoot
    ");
    
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

    let track_names = vec![
        "ufo", // UFO
        "shoot", // Shoot
        "player_killed", // Player die
        "invader_killed", // Invader die
        "fleet_move_1", // Fleet move 1
        "fleet_move_2", // Fleet move 2
        "fleet_move_3", // Fleet move 3
        "fleet_move_4", // Fleet move 4
        "ufo_hit", // UFO hit
    ];

    for (index, track_name) in track_names.iter().enumerate(){

        setup_config.audio_tracks.push(
            Chunk::from_file(format!("space-invaders-source/sounds/{}.wav", track_name)).expect("Failed to load audio file")
        );

        setup_config.audio_tracks[index].set_volume(64);

    }
    
    setup_config.ports[0] = 0b00001110;

    setup_config.ports[1] = 0b00001000;

    setup_config.window.set_title("Space Invaders").expect("Failed to set window title");

    setup_config.window.set_size(224 * 2, 256 * 2).expect("Failed to size window");

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

            Event::KeyDown { keycode: Some(Keycode::Up), .. } => machine.audio_controller.volume_up(),

            Event::KeyDown { keycode: Some(Keycode::Down), .. } => machine.audio_controller.volume_down(),

            Event::KeyDown { keycode: Some(Keycode::M), .. } => machine.audio_controller.toggle_mute(),

            Event::Quit {..} |
    
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {

                machine.cpu.logger.flush().expect("Failed to flush output buffer");

                machine.audio_controller.close();

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

        processor.custom_registers = vec![0; 4];

    }

    match port {

        1 => return ports[1], // Input

        2 => return ports[2], // Input

        3 => return (processor.custom_registers[1] >> (8 - processor.custom_registers[0])) as u8,
        
        _ => {},

    }
    
    0

}

fn space_invaders_out(processor: &mut Processor8080, port: u8, value: u8, ports: &mut Vec<u8>, audio_controller: &mut AudioController){

    /*
        Custom registers:
            0 -> Number of times to shift
            1 -> Shift result
    */

    if processor.custom_registers.len() == 0{

        processor.custom_registers = vec![0; 4];

    }

    match port {

        2 => processor.custom_registers[0] = (value & 0b111) as u16, // Set the shif amount to the last 3 bits of the provided value

        3 => {

            if (value & 0b00000001) != 0 && (ports[3] & 0b00000001) == 0{

                audio_controller.play_track(0, -1);
        
            }
            else if (value & 0b00000001) == 0 && (ports[3] & 0b00000001) != 0{

                audio_controller.stop_track(0);

            }

            // play_audio(value, ports[3], 0b00000001, 0, audio_controller); // UFO

            play_audio(value, ports[3], 0b00000010, 1, audio_controller); // Shoot

            play_audio(value, ports[3], 0b00000100, 2, audio_controller); // Player die

            play_audio(value, ports[3], 0b00001000, 3, audio_controller); // Invader die

            ports[3] = value;

        }, // Play Sound

        4 => processor.custom_registers[1] = (processor.custom_registers[1] >> 8) | ((value as u16) << 8), // Set the shift result

        5 => {

            play_audio(value, ports[5], 0b00000001, 4, audio_controller); // Fleet move 1
            
            play_audio(value, ports[5], 0b00000010, 5, audio_controller); // Fleet move 2

            play_audio(value, ports[5], 0b00000100, 6, audio_controller); // Fleet move 3

            play_audio(value, ports[5], 0b00001000, 7, audio_controller); // Fleet move 4

            play_audio(value, ports[5], 0b00010000, 8, audio_controller); // UFO hit

            ports[5] = value;

        }, // Play Sound

        _ => {},

    }

}

fn play_audio(value: u8, old_value: u8, and_value: u8, sound_index: u8, audio_controller: &mut AudioController){

    if (value & and_value) != 0 && (old_value & and_value) == 0{

        audio_controller.play_track(sound_index, 0);

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