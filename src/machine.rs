use crate::cpu::*;
use crate::io_handlers::*;

pub struct Machine{
    cpu: Processor8080,
    ports: Vec<u8>,
}

impl Machine{

    pub fn new(game_id: u8, log_to_file: bool, test: bool) -> Machine{

        let input_handler: fn(&mut Processor8080, u8) -> u8;
        let output_handler: fn(&mut Processor8080, u8, u8);

        let mut files: Vec<FileToLoad> = vec![];

        let mut ports = vec![0; 256];

        match game_id {

            0 => {

                input_handler = test_in;
                output_handler = test_out;

            }, // Test

            1 => {

                input_handler = space_invaders_in;
                output_handler = space_invaders_out;

                files.push(FileToLoad{
                    name: "space-invaders-source/SpaceInvaders.h".to_string(),
                    offset: 0x0,
                    size: 0x800
                });

                files.push(FileToLoad{
                    name: "space-invaders-source/SpaceInvaders.g".to_string(),
                    offset: 0x800,
                    size: 0x800
                });

                files.push(FileToLoad{
                    name: "space-invaders-source/SpaceInvaders.f".to_string(),
                    offset: 0x1000,
                    size: 0x800
                });

                files.push(FileToLoad{
                    name: "space-invaders-source/SpaceInvaders.e".to_string(),
                    offset: 0x1800,
                    size: 0x800
                });
                
                ports[0] = 0b01110000;

                ports[1] = 0b00010000;

            }, // Space Invaders

            _ => panic!("Game ID is invalid"),
        }
    
        let mut new_arcade = Machine{
            cpu: Processor8080::new(input_handler, output_handler, log_to_file),
            ports: ports,
        };
    
        if test{
    
            new_arcade.cpu.test();
    
        }
        else {
    
            new_arcade.cpu.initialize(files);
    
        }
    
        new_arcade
    
    }

    pub fn start(&mut self){

        while self.cpu.enabled {

            emulate(&mut self.cpu);
            
        }

    }

    pub fn start_n(&mut self, n: usize){

        for _ in 0..n{

            emulate(&mut self.cpu);

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

}