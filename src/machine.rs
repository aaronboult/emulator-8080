use crate::cpu::*;

pub struct Machine{
    cpu: Processor8080,
}

impl Machine{

    pub fn new(game_id: u8, test: bool) -> Machine{

        let input_handler: fn(&Processor8080, u8);
        let output_handler: fn(&Processor8080, u8);
        let mut files: Vec<File> = vec![];

        match game_id {

            0 => {

                input_handler = test_in;
                output_handler = test_out;

            }, // Test

            1 => {

                input_handler = space_invaders_in;
                output_handler = space_invaders_out;

                files.push(File{
                    name: "space-invaders-source/SpaceInvaders.h".to_string(),
                    offset: 0x0,
                    size: 0x800
                });

                files.push(File{
                    name: "space-invaders-source/SpaceInvaders.g".to_string(),
                    offset: 0x800,
                    size: 0x800
                });

                files.push(File{
                    name: "space-invaders-source/SpaceInvaders.f".to_string(),
                    offset: 0x1000,
                    size: 0x800
                });

                files.push(File{
                    name: "space-invaders-source/SpaceInvaders.e".to_string(),
                    offset: 0x1800,
                    size: 0x800
                });

            }, // Space Invaders

            _ => panic!("Game ID is invalid"),
        }
    
        let mut new_arcade = Machine{
            cpu: Processor8080::new(input_handler, output_handler),
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

}

fn test_in(processor: &Processor8080, port: u8){

}

fn test_out(processor: &Processor8080, port: u8){

}

fn space_invaders_in(processor: &Processor8080, port: u8){

}

fn space_invaders_out(processor: &Processor8080, port: u8){

}