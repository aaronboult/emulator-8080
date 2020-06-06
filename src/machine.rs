extern crate sdl2;

mod space_invaders;
mod test;

use crate::cpu::*;

use std::time::SystemTime;

use sdl2::mixer;
use mixer::{Chunk, Channel};


pub struct Machine{
    pub cpu: Processor8080,
    ports: Vec<u8>,
    interrupt_handler: fn(&mut Machine),
    key_event_handler: fn(&mut Machine),
    drawer: fn(&mut Machine),
    pub timestamp: SystemTime,

    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub sdl_context: sdl2::Sdl,
    pub audio_controller: AudioController,
}

pub struct SetupConfiguration{
    input_handler: fn(&mut Processor8080, u8, &Vec<u8>) -> u8,
    output_handler: fn(&mut Processor8080, u8, u8, &mut Vec<u8>, &mut AudioController),
    key_event_handler: fn(&mut Machine),
    interrupt_handler: fn(&mut Machine),
    drawer: fn(&mut Machine),
    files: Vec<FileToLoad>,
    ports: Vec<u8>,

    window: sdl2::video::Window,
    sdl_context: sdl2::Sdl,

    audio_tracks: Vec<Chunk>,
}

impl Machine{

    pub fn new(game_id: u8, log_to_file: bool, test: bool) -> Machine{
        
        let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
        let video_subsystem = sdl_context.video().expect("Failed to retrieve SDL2 video subsystem");

        mixer::init(mixer::InitFlag::all()).expect("Failed to initialize audio mixer");

        mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, 8, 1024).expect("Failed to open audio mixer");

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
            sdl_context: sdl_context,

            audio_tracks: vec![],
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

            audio_controller: AudioController::new(setup_config.audio_tracks),
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

            if self.timestamp.elapsed().expect("Failed to calculate elapsed time").as_millis() as f32 >= 1_f32 / 60_f32 * 1000_f32{ // Mimmics running at 60Hz

                self.timestamp = self.get_time();

                while cycle <= (2_000_000 / 60) as u16{ // The number of cycles that should execute per frame

                    saved_cycle_count = self.cpu.cycles_elapsed;

                    self.cpu.emulate(&mut self.ports, &mut self.audio_controller);

                    cycle += self.cpu.cycles_elapsed - saved_cycle_count;
        
                    (self.interrupt_handler)(self); // Handle any program-specific interrupts
    
                }

                cycle = 0;

                (self.drawer)(self); // Draw the window

            }

        }

    }

    #[allow(dead_code)]
    pub fn emulate_n(&mut self, n: usize){

        for _ in 0..n{

            self.cpu.emulate(&mut self.ports, &mut self.audio_controller);

        }

    }

    pub fn get_time(&self) -> SystemTime{

        SystemTime::now()

    }

}

#[derive(Default)]
pub struct AudioController{
    audio_tracks: Vec<Chunk>,
    current_volume: i32,
    previous_volume: i32,
}

impl AudioController{

    fn new(audio_tracks: Vec<Chunk>) -> Self{

        let mut audio_controller = AudioController{
            audio_tracks: audio_tracks,
            current_volume: 26,
            previous_volume: 26,
        };

        audio_controller.set_global_volume(26);

        audio_controller

    }

    pub fn close(&mut self){

        mixer::close_audio();

    }

    pub fn play_track(&mut self, track_index: u8, number_of_repeats: i32){

        if mixer::get_playing_channels_number() != 8{

            Channel(-1).play(&self.audio_tracks[track_index as usize], number_of_repeats).expect("Failed to play audio track");
        
        }

    }

    pub fn stop_track(&mut self, track_index: u8){

        for channel_index in 0..8{

            if Channel(channel_index).is_playing(){

                if self.audio_tracks[track_index as usize].raw == Channel(channel_index).get_chunk().unwrap().raw{
    
                    Channel(channel_index).halt();
    
                }

            }

        }

    }

    fn set_global_volume(&mut self, volume: i32){
        
        for chunk in self.audio_tracks.iter_mut(){

            chunk.set_volume(volume);

        }

        for channel_index in 0..8{

            Channel(channel_index).set_volume(volume);

        }

    }

    pub fn volume_up(&mut self){

        if self.current_volume < 128{

            self.set_global_volume(self.current_volume + 1);

            self.current_volume += 1;

            self.previous_volume = self.current_volume;

        }

    }

    pub fn volume_down(&mut self){

        if self.current_volume > 0{

            self.set_global_volume(self.current_volume - 1);

            self.current_volume -= 1;

            self.previous_volume = self.current_volume;

        }

    }

    pub fn toggle_mute(&mut self){

        if self.current_volume == self.previous_volume{ // If the volume should be muted

            self.set_global_volume(0);

            self.current_volume = 0;

        }
        else{ // If the volume should be unmuted

            self.set_global_volume(self.previous_volume);

            self.current_volume = self.previous_volume;

        }

    }

}