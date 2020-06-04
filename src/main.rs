/*
    Intel 8080 Data Book: https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf
    Original Repository: https://github.com/aaronboult/emulator-8080
*/

mod machine;
mod cpu;

use std::io;
use std::env;

const NUMBER_OF_PROGRAMS_EMULATED: u8 = 1;

fn main() {

    let (test, log_to_file) = {

        let args: Vec<String> = env::args().collect();

        let test = args.contains(&"-t".to_string()) || args.contains(&"--test".to_string());

        (
            test,
            test && (args.contains(&"-l".to_string()) || args.contains(&"--log-to-file".to_string())),
        )

    };


    display_options();


    let result = {
        
        let mut game_id = String::new();

        io::stdin().read_line(&mut game_id).expect("Failed to read from input stream");
    
        check_game_id(&game_id)

    };


    if result.is_ok(){

        let mut arcade_machine = machine::Machine::new(result.unwrap(), log_to_file, test); // Params: Game ID, Log To File, Test
    
        arcade_machine.start();

    }
    else{

        println!("The game ID you entered is invalid");

    }

}

fn display_options(){

    print!("\
    Select one of the following by typing the corresponding ID

            Game ID     |     Game Name
        -----------------------------------
                0       |  Space Invaders
    \n\
    :>> \
    ");

    io::Write::flush(&mut io::stdout()).expect("Failed to flush standard output");

}

fn check_game_id(game_id: &String) -> Result<u8, ()>{

    let id = game_id.trim().parse::<u8>();

    if id.is_ok(){

        let id_number = id.unwrap();

        if id_number < NUMBER_OF_PROGRAMS_EMULATED{

            return Ok(id_number);

        }

    }

    Err(())

}