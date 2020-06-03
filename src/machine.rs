extern crate sdl2;

use crate::cpu::*;

mod space_invaders;
mod test;

use std::time::SystemTime;

pub struct Machine{
    pub cpu: Processor8080,
    ports: Vec<u8>,
    interrupt_handler: fn(&mut Machine),
    key_event_handler: fn(&mut Machine),
    drawer: fn(&mut Machine),
    pub timestamp: SystemTime,

    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub sdl_context: sdl2::Sdl,
}

pub struct SetupConfiguration{
    input_handler: fn(&mut Processor8080, u8, &Vec<u8>) -> u8,
    output_handler: fn(&mut Processor8080, u8, u8, &Vec<u8>),
    key_event_handler: fn(&mut Machine),
    interrupt_handler: fn(&mut Machine),
    drawer: fn(&mut Machine),
    files: Vec<FileToLoad>,
    ports: Vec<u8>,

    window: sdl2::video::Window,
    sdl_context: sdl2::Sdl,
}

impl Machine{

    pub fn new(game_id: u8, log_to_file: bool, test: bool) -> Machine{
        
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Test Window", 128, 128)
                                        .position_centered()
                                        .resizable()
                                        .build()
                                        .expect("Failed to create window");

        let mut setup_config = SetupConfiguration{
            input_handler: test::test_in,
            output_handler: test::test_out,
            key_event_handler: test::key_event,
            interrupt_handler: test::test_interrupt,
            drawer: test::draw,
            files: vec![],
            ports: vec![0; 256],

            window: window,
            sdl_context: sdl_context
        };

        if !test{

            match game_id {
    
                0 => {
    
                    space_invaders::setup(&mut setup_config);
    
                }, // Space Invaders
    
                _ => panic!("Game ID is invalid"),
            }

        }
    
        let mut new_arcade = Machine{
            cpu: Processor8080::new(setup_config.input_handler, setup_config.output_handler, log_to_file),
            ports: setup_config.ports,
            key_event_handler: setup_config.key_event_handler,
            interrupt_handler: setup_config.interrupt_handler,
            drawer: setup_config.drawer,
            timestamp: SystemTime::now(),

            canvas: setup_config.window.into_canvas().build().expect("Failed to create canvas"),
            sdl_context: setup_config.sdl_context,
        };
    
        if test{
    
            new_arcade.cpu.test();
    
        }
        else {
    
            new_arcade.cpu.initialize(setup_config.files);
    
        }
    
        new_arcade
    
    }

    pub fn start(&mut self){

        let mut cycle = 0;

        let mut saved_cycle_count;

        loop {

            (self.key_event_handler)(self);

            if self.timestamp.elapsed().expect("Failed to calculate elapsed time").as_millis() > 1 / 60 * 1000{ // Mimmics running at 60Hz

                self.timestamp = self.get_time();

                while cycle <= (2_000_000 / 60) as u16{ // The number of cycles that should execute per frame

                    saved_cycle_count = self.cpu.cycles_elapsed;

                    self.cpu.emulate(&self.ports);

                    cycle += self.cpu.cycles_elapsed - saved_cycle_count;
        
                    (self.interrupt_handler)(self); // Handle any program-specific interrupts
    
                }

                cycle = 0;

                (self.drawer)(self); // Draw the window

            }

        }

    }

    pub fn _emulate_n(&mut self, n: usize){

        for _ in 0..n{

            self.cpu.emulate(&self.ports);

        }

    }

    pub fn get_time(&self) -> SystemTime{

        SystemTime::now()

    }

}