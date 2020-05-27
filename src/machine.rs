use crate::cpu::*;

mod space_invaders;
mod test;

use std::time::SystemTime;

pub struct Machine{
    pub cpu: Processor8080,
    ports: Vec<u8>,
    interrupt_handler: fn(&mut Machine),
    pub timestamp: SystemTime,
}

pub struct SetupConfiguration{
    input_handler: fn(&mut Processor8080, u8) -> u8,
    output_handler: fn(&mut Processor8080, u8, u8),
    interrupt_handler: fn(&mut Machine),
    files: Vec<FileToLoad>,
    ports: Vec<u8>,
}

impl Machine{

    pub fn new(game_id: u8, log_to_file: bool, test: bool) -> Machine{

        let mut setup_config = SetupConfiguration{
            input_handler: test::test_in,
            output_handler: test::test_out,
    
            interrupt_handler: test::test_interrupt,
    
            files: vec![],
    
            ports: vec![0; 256],
        };

        match game_id {

            0 => {

                space_invaders::setup(&mut setup_config);

            }, // Space Invaders

            _ => panic!("Game ID is invalid"),
        }
    
        let mut new_arcade = Machine{
            cpu: Processor8080::new(setup_config.input_handler, setup_config.output_handler, log_to_file),
            ports: setup_config.ports,
            interrupt_handler: setup_config.interrupt_handler,
            timestamp: SystemTime::now()
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

        loop {

            self.cpu.emulate();

            (self.interrupt_handler)(self); // Handle any program-specific interrupts

        }

    }

    pub fn start_n(&mut self, n: usize){

        for _ in 0..n{

            self.cpu.emulate();

        }

    }

    fn key_event(&mut self, key: u8, key_down: bool){

        match key {

            0 => {

                if key_down{

                    self.ports[1] = self.ports[1] | 0x20;

                }
                else{

                    self.ports[1] = self.ports[1] & 0xDF;

                }

            }, // Left

            1 => {

                if key_down{

                    self.ports[1] = self.ports[1] | 0x40;

                }
                else{

                    self.ports[1] = self.ports[1] & 0xBF;

                }

            }, // Right

            _ => {},

        }

    }

    pub fn get_time(&self) -> SystemTime{

        SystemTime::now()

    }

}